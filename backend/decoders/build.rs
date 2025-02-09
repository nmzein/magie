use quote::quote;
use shared::functions::{declare_modules, find_modules};
use std::{
    collections::HashMap,
    fs::{self, File},
};
use syn::{parse2, parse_file, Expr, Ident, ImplItem, Item, Lit, LitStr, Stmt};

fn main() {
    let decoders = find_modules();

    let common_code = generate_common(&decoders);
    let export_code = generate_export(&decoders);

    let common_formatted = prettyplease::unparse(&parse2(common_code).unwrap());
    let export_formatted = prettyplease::unparse(&parse2(export_code).unwrap());

    fs::write("src/common.rs", common_formatted).unwrap();
    fs::write("src/export.rs", export_formatted).unwrap();

    let mut lib = File::create("src/lib.rs").unwrap();
    declare_modules(&mut lib, &decoders);
}

fn generate_common(decoders: &[String]) -> proc_macro2::TokenStream {
    if decoders.is_empty() {
        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            pub use shared::traits::Decoder;
        }
    } else {
        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            pub use anyhow::Result;
            pub use image::{ImageBuffer, Rgb};
            pub use shared::{
                structs::{Region, Size},
                traits::Decoder,
            };
            pub use std::path::Path;
        }
    }
}

struct ModuleInfo {
    name: String,
    extensions: Vec<String>,
}

fn extract_module_info(decoders: Vec<String>) -> Vec<ModuleInfo> {
    let mut module_infos = Vec::new();

    for decoder in decoders {
        let contents =
            fs::read_to_string(format!("src/{decoder}.rs")).expect("Failed to read file");

        let parsed = parse_file(&contents).unwrap().items;

        for item in parsed {
            if let Item::Impl(item_impl) = item {
                if let Some(trait_path) = &item_impl.trait_ {
                    if trait_path
                        .1
                        .segments
                        .last()
                        .is_some_and(|seg| seg.ident == "Decoder")
                    {
                        let mut module_info = ModuleInfo {
                            name: String::new(),
                            extensions: Vec::new(),
                        };

                        for impl_item in item_impl.items {
                            if let ImplItem::Fn(method) = impl_item {
                                match method.sig.ident.to_string().as_str() {
                                    "name" => {
                                        if let Some(Stmt::Expr(Expr::Lit(expr_lit), _)) =
                                            method.block.stmts.first()
                                        {
                                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                                module_info.name = lit_str.value();
                                            }
                                        }
                                    }
                                    "extensions" => {
                                        if let Some(Stmt::Expr(Expr::Macro(expr_macro), _)) =
                                            method.block.stmts.first()
                                        {
                                            let tokens = expr_macro.mac.tokens.to_string();
                                            for token in tokens.split(',') {
                                                let token = token.trim();
                                                if token.is_empty() {
                                                    continue;
                                                }
                                                if let Some(ext) = token.split('"').nth(1) {
                                                    if !ext.is_empty() {
                                                        module_info
                                                            .extensions
                                                            .push(ext.to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }

                        if !module_info.name.is_empty() {
                            module_infos.push(module_info);
                        }
                    }
                }
            }
        }
    }

    module_infos
}

fn create_extension_map(
    module_infos: &[ModuleInfo],
    decoders: &[String],
) -> HashMap<String, Vec<String>> {
    let mut extensions_map: HashMap<String, Vec<String>> = HashMap::new();

    for (info, decoder) in module_infos.iter().zip(decoders) {
        for ext in &info.extensions {
            extensions_map
                .entry(ext.clone())
                .or_default()
                .push(decoder.clone());
        }
    }

    extensions_map
}

fn generate_export(decoders: &[String]) -> proc_macro2::TokenStream {
    let module_infos = extract_module_info(decoders.to_vec());
    let extension_map = create_extension_map(&module_infos, decoders);
    let mut extension_map: Vec<_> = extension_map.into_iter().collect();
    extension_map.sort_by(|a, b| a.0.cmp(&b.0));

    let match_arms = extension_map.iter().map(|(ext, decoders)| {
        let modules = decoders.iter().map(|d| {
            let ident = Ident::new(d, proc_macro2::Span::call_site());
            quote! { Box::new(crate::#ident::Module::default()), }
        });
        quote! {
            #ext => vec![
                #(#modules)*
            ],
        }
    });

    let names = module_infos.iter().map(|info| {
        let name_lit = LitStr::new(&info.name, proc_macro2::Span::call_site());
        quote! { #name_lit, }
    });

    if decoders.is_empty() {
        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            use crate::common::*;

            pub fn get(_extension: &str) -> Vec<Box<dyn Decoder>> {
                vec![]
            }

            pub fn names() -> Vec<&'static str> {
                vec![]
            }
        }
    } else {
        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            use crate::common::*;

            pub fn get(extension: &str) -> Vec<Box<dyn Decoder>> {
                match extension {
                    #(#match_arms)*
                    _ => vec![],
                }
            }

            pub fn names() -> Vec<&'static str> {
                vec![
                    #(#names)*
                ]
            }
        }
    }
}
