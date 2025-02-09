use quote::quote;
use shared::functions::{declare_modules, find_modules};
use std::fs::{self, File};
use syn::{Ident, Item, LitStr};

fn main() {
    let generators = find_modules();

    let common_code = generate_common();
    let export_code = generate_export(&generators);

    let common_formatted = prettyplease::unparse(&syn::parse2(common_code).unwrap());
    let export_formatted = prettyplease::unparse(&syn::parse2(export_code).unwrap());

    fs::write("src/common.rs", common_formatted).unwrap();
    fs::write("src/export.rs", export_formatted).unwrap();

    let mut lib = File::create("src/lib.rs").unwrap();
    declare_modules(&mut lib, &generators);
}

fn generate_common() -> proc_macro2::TokenStream {
    quote! {
        /// Auto-generated file. Any changes will be overwritten.
        pub use anyhow::Result;
        pub use shared::{structs::AnnotationLayer, traits::Generator};
        pub use std::path::Path;
    }
}

fn extract_generator_names(generators: Vec<String>) -> Vec<String> {
    let mut names = Vec::new();

    for generator in generators {
        let contents =
            fs::read_to_string(format!("src/{generator}.rs")).expect("Failed to read file");

        let parsed = syn::parse_file(&contents).unwrap().items;

        for item in parsed {
            if let Item::Impl(item_impl) = item {
                if let Some(trait_path) = &item_impl.trait_ {
                    if trait_path
                        .1
                        .segments
                        .last()
                        .is_some_and(|seg| seg.ident == "Generator")
                    {
                        for impl_item in item_impl.items {
                            if let syn::ImplItem::Fn(method) = impl_item {
                                if method.sig.ident == "name" {
                                    if let Some(syn::Stmt::Expr(syn::Expr::Lit(expr_lit), _)) =
                                        method.block.stmts.first()
                                    {
                                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                            names.push(lit_str.value());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    names
}

fn generate_export(generators: &[String]) -> proc_macro2::TokenStream {
    let generator_names = extract_generator_names(generators.to_vec());

    let code = if generators.is_empty() {
        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            use crate::common::*;

            pub fn get(_name: &str) -> Option<Box<dyn Generator>> {
                None
            }

            pub fn names() -> Vec<&'static str> {
                vec![]
            }
        }
    } else {
        let match_arms = generator_names
            .iter()
            .zip(generators.iter())
            .map(|(name, generator)| {
                let ident = Ident::new(generator, proc_macro2::Span::call_site());
                quote! {
                    #name => Some(Box::new(crate::#ident::Module)),
                }
            });

        let names = generator_names.iter().map(|name| {
            let name_lit = LitStr::new(name, proc_macro2::Span::call_site());
            quote! { #name_lit, }
        });

        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            use crate::common::*;

            pub fn get(name: &str) -> Option<Box<dyn Generator>> {
                match name {
                    #(#match_arms)*
                    _ => None,
                }
            }

            pub fn names() -> Vec<&'static str> {
                vec![
                    #(#names)*
                ]
            }
        }
    };

    code
}
