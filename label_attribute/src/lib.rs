#[macro_use]
extern crate syn;
extern crate proc_macro;
extern crate proc_macro2 as pm2;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;

static INCREMENTAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[cfg_attr(feature = "extra-traits", derive(Debug))]
struct LabelStruct {
    attrs: Vec<syn::Attribute>,
    name: syn::Ident,
}

impl Parse for LabelStruct {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let _: Token![struct] = input.parse()?;
        let name = input.parse()?;
        let _: Token![;] = input.parse()?;
        Ok(LabelStruct { attrs, name })
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
enum LabelMeta {
    CustomName(pm2::Literal),
    AssocType(syn::Type),
    CratePath(syn::Path),
}

impl Parse for LabelMeta {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let lookahead = input.lookahead1();
        let (span, meta_name_str) = if lookahead.peek(Token![type]) {
            let tok: Token![type] = input.parse()?;
            (tok.span, "type".to_string())
        } else if lookahead.peek(Token![crate]) {
            let tok: Token![crate] = input.parse()?;
            (tok.span, "crate".to_string())
        } else {
            let meta_name: syn::Ident = input.parse()?;
            (meta_name.span(), meta_name.to_string())
        };
        let _: Token![=] = input.parse()?;

        const LABEL_NAME_ID: &str = "name";
        const CRATE_ID: &str = "crate";
        const TYPE_IDS: [&str; 2] = ["type", "assoc_type"];

        match &meta_name_str[..] {
            LABEL_NAME_ID => input.parse().map(LabelMeta::CustomName),
            CRATE_ID => input.parse().map(LabelMeta::CratePath),
            s if TYPE_IDS.contains(&s) => input.parse().map(LabelMeta::AssocType),
            _ => Err(syn::Error::new(
                span,
                format![
                    "expected {}, {}, or {}",
                    TYPE_IDS.join(", "),
                    CRATE_ID,
                    LABEL_NAME_ID
                ],
            )),
        }
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
struct LabelOptions {
    name: Option<pm2::Literal>,
    assoc_type: Option<syn::Type>,
    crate_path: Option<syn::Path>,
}

impl Parse for LabelOptions {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let metas: Punctuated<_, Token![,]> = input.parse_terminated(LabelMeta::parse)?;
        let mut opts = LabelOptions {
            name: None,
            assoc_type: None,
            crate_path: None,
        };
        for meta in &metas {
            match meta {
                LabelMeta::CustomName(name) => {
                    opts.name = Some(name.clone());
                }
                LabelMeta::AssocType(ty) => {
                    opts.assoc_type = Some(ty.clone());
                }
                LabelMeta::CratePath(path) => {
                    opts.crate_path = Some(path.clone());
                }
            }
        }
        Ok(opts)
    }
}

#[proc_macro_attribute]
pub fn label(attr: TokenStream, item: TokenStream) -> TokenStream {
    let label_options = parse_macro_input!(attr as LabelOptions);
    let label_struct = parse_macro_input!(item as LabelStruct);
    impl_label(&label_options, &label_struct)
}

fn impl_label(label_options: &LabelOptions, label_struct: &LabelStruct) -> TokenStream {
    // struct name
    let name = &label_struct.name;
    // string name (for identification)
    let name_str = match label_options.name {
        Some(ref cust_name) => quote! { #cust_name },
        None => quote! { stringify!(#name) },
    };

    // unique identifier
    let id = INCREMENTAL_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let id_ty = generate_uint(id);

    // attributes
    let attrs = &label_struct.attrs;

    // associated type
    let assoc_type = match label_options.assoc_type {
        Some(ref ty) => quote! { #ty },
        None => quote! { () },
    };

    let dummy_const = syn::Ident::new(&format!("_IMPL_LABEL_FOR_{}", name), pm2::Span::call_site());
    let use_lhlist = match label_options.crate_path {
        Some(ref path) => quote! {
            use #path as _lhlist;
        },
        None => quote! {
            extern crate lhlist as _lhlist;
        },
    };

    let generated = quote! {
        #(#attrs)*
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        struct #name;
        const #dummy_const: () = {
            #use_lhlist
            impl _lhlist::Label for #name {
                const NAME: &'static str = #name_str;
                type AssocType = #assoc_type;
                type Uid = #id_ty;
            }
        };
    };
    generated.into()
}

fn generate_uint_recurse(
    target: usize,
    curr_val: usize,
    curr_toks: pm2::TokenStream,
) -> pm2::TokenStream {
    if curr_val == 0 {
        curr_toks
    } else {
        // compute most significant bit
        let bit = if target & curr_val > 0 {
            quote! { typenum::B1 }
        } else {
            quote! { typenum::B0 }
        };
        // add most significant bit and recurse to add rest
        generate_uint_recurse(
            target,
            curr_val >> 1,
            quote! { typenum::UInt<#curr_toks, #bit> },
        )
    }
}

fn generate_uint(value: usize) -> pm2::TokenStream {
    let start = if value > 0 {
        value.next_power_of_two()
    } else {
        0
    };
    let gen = generate_uint_recurse(value, start, quote! { typenum::UTerm });
    gen
}
