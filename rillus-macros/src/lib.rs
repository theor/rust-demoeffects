use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse::{Parse, self}, parse_macro_input, parse_quote, token::Token, FnArg, PatType, ReturnType};

#[proc_macro_attribute]
pub fn reflect(attr: TokenStream, item: TokenStream) -> TokenStream {
    let i2 = item.clone();
    let p: syn::ItemFn = syn::parse_macro_input!(i2);

    let name = format_ident!("{}_desc", p.sig.ident);
    eprintln!("INPUT: {}\n\t{}", &item, &name);

    let mut args: Vec<syn::ExprStruct> = Vec::new();
    for a in p.sig.inputs.iter() {
        match a {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let param_name = format!("{}", pat.to_token_stream());
                let param_ty = format!("{}", ty.to_token_stream());
                eprintln!("Parsed: {} {}", pat.to_token_stream(), ty.to_token_stream());
                args.push(parse_quote! {
                Param {
                    name: #param_name,
                    value_type: #param_ty,
                }
                                });
            }
            _ => {}
        }
        // eprintln!("Parsed: {:#?}", a);
    }

    let ret = match &p.sig.output {
    ReturnType::Default => "".to_string(),
    ReturnType::Type(_, t) => format!("{}", t.to_token_stream()), 
};
    // eprintln!("Parsed: {:#?}", p.sig.inputs);
    // let mut s =  "#[wasm_bindgen]".to_string();
    // s.push_str(&item.to_string());
    // s.parse().unwrap()
    // proc_macro::TokenStream::from(p.to_token_stream())
    // let args2 = args.iter().joi
    let res = quote! {
        #[wasm_bindgen]
        pub fn #name() ->  wasm_bindgen::JsValue {
            serde_wasm_bindgen::to_value(&Function {
                return_type: #ret,
                params: vec![ #(#args),* ],
            }).unwrap()
        }

        #[wasm_bindgen]
        #p
    };
    eprintln!("OUTPUT:\n{}", &res);
    res.into()
}
