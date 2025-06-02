use quote::quote;
use shared::functions::{declare_modules, find_modules};
use std::fs::{self, File};
use syn::{Ident, Item, LitStr};

fn main() {
    let encoders = find_modules();

    let common_code = generate_common();
    let export_code = generate_export(&encoders);

    let common_formatted = prettyplease::unparse(&syn::parse2(common_code).unwrap());
    let export_formatted = prettyplease::unparse(&syn::parse2(export_code).unwrap());

    fs::write("src/common.rs", common_formatted).unwrap();
    fs::write("src/export.rs", export_formatted).unwrap();

    let mut lib = File::create("src/lib.rs").unwrap();
    declare_modules(&mut lib, &encoders);
}

fn generate_common() -> proc_macro2::TokenStream {
    quote! {
        /// Auto-generated file. Any changes will be overwritten.
        pub use anyhow::Result;
        pub use shared::{
            constants::{RGB_CHANNELS, TILE_LENGTH, TILE_SIZE},
            traits::{Decoder, Encoder},
            types::{Address, MetadataLayer, Region, Size},
        };
        pub use std::{path::Path, sync::Arc};
        pub use zarrs::{
            array::{codec::GzipCodec, Array, ArrayBuilder, DataType, FillValue},
            array_subset::ArraySubset,
            filesystem::FilesystemStore,
            group::GroupBuilder,
        };

        pub fn interleave(channels: &[u8], output: &mut Box<[u8]>) {
            let rs = &channels[..TILE_LENGTH];
            let gs = &channels[TILE_LENGTH..TILE_LENGTH * 2];
            let bs = &channels[TILE_LENGTH * 2..];

            for (i, ((&r, &g), &b)) in rs.iter().zip(gs).zip(bs).enumerate() {
                let idx = i * 3;
                output[idx] = r;
                output[idx + 1] = g;
                output[idx + 2] = b;
            }
        }
    }
}

fn extract_encoder_names(encoders: Vec<String>) -> Vec<String> {
    let mut names = Vec::new();

    for encoder in encoders {
        let contents =
            fs::read_to_string(format!("src/{encoder}.rs")).expect("Failed to read file");

        let parsed = syn::parse_file(&contents).unwrap().items;

        for item in parsed {
            if let Item::Impl(item_impl) = item {
                if let Some(trait_path) = &item_impl.trait_ {
                    if trait_path
                        .1
                        .segments
                        .last()
                        .is_some_and(|seg| seg.ident == "Encoder")
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

fn generate_export(encoders: &[String]) -> proc_macro2::TokenStream {
    let encoder_names = extract_encoder_names(encoders.to_vec());

    let code = if encoders.is_empty() {
        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            use crate::common::*;

            pub fn get(_name: &str) -> Option<Box<dyn Encoder>> {
                None
            }

            pub fn names() -> Vec<&'static str> {
                vec![]
            }
        }
    } else {
        let match_arms = encoder_names
            .iter()
            .zip(encoders.iter())
            .map(|(name, encoder)| {
                let ident = Ident::new(encoder, proc_macro2::Span::call_site());
                quote! {
                    #name => Some(Box::new(crate::#ident::Module)),
                }
            });

        let names = encoder_names.iter().map(|name| {
            let name_lit = LitStr::new(name, proc_macro2::Span::call_site());
            quote! { #name_lit, }
        });

        quote! {
            /// Auto-generated file. Any changes will be overwritten.
            use crate::common::*;

            pub fn get(name: &str) -> Option<Box<dyn Encoder>> {
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
