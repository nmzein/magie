use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn wrap_with_store(attr: TokenStream, item: TokenStream) -> TokenStream {
    let wrapper_name = parse_macro_input!(attr as Ident);
    let input_fn = parse_macro_input!(item as ItemFn);

    let inner_fn_ident = &input_fn.sig.ident;
    let return_type = &input_fn.sig.output; // Extract the return type

    // Extract parameters (skip first) and their names
    let params = input_fn.sig.inputs.iter().skip(1);
    let param_names = input_fn.sig.inputs.iter().skip(1).map(|arg| match arg {
        syn::FnArg::Typed(pat_type) => &pat_type.pat,
        _ => unreachable!(),
    });

    // Generate wrapper function with custom name
    let expanded = quote! {
        #input_fn

        pub fn #wrapper_name(store_id: u32, #(#params),*) #return_type {
            if let Some(mut conn) = DB.store(store_id) {
                #inner_fn_ident(conn.lock().unwrap(), #(#param_names),*)
            } else {
                Err(anyhow::anyhow!("Store not found"))
            }
        }
    };

    expanded.into()
}
