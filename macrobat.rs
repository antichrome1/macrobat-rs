//! A DSL for writing legible single-arm declartive macros

#![forbid(unsafe_code)]

use quote::quote;
use syn::{
    Ident as SynIdent,
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    Attribute, Result, Token,
};

struct Definition {
    attrs: Vec<Attribute>,
    name: SynIdent,
    matcher: proc_macro2::TokenStream,
    body: proc_macro2::TokenStream,
    export: bool,
    wrap: bool,
}

struct Definitions(Vec<Definition>);

impl Parse for Definitions {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut definitions = Vec::new();

        while !input.is_empty() {
            let attrs = input.call(Attribute::parse_outer)?;
            if input.is_empty() {
                break;
            }

            let first: SynIdent = input.call(SynIdent::parse_any)?;
            let name = if first == "macro" {
                input.parse()?
            } else {
                first
            };

            let matcher;
            parenthesized!(matcher in input);

            let body;
            braced!(body in input);

            let mut export = false;
            let mut wrap = false;

            while input.peek(Token![@]) {
                input.parse::<Token![@]>()?;
                let attr: SynIdent = input.parse()?;

                match attr.to_string().as_str() {
                    "export" => export = true,
                    "wrap" => wrap = true,
                    _ => {
                        return Err(syn::Error::new(attr.span(), "Expected `export` or `wrap`"));
                    }
                }
            }

            definitions.push(Definition {
                attrs,
                name,
                matcher: matcher.parse()?,
                body: body.parse()?,
                export,
                wrap,
            });
        }

        Ok(Self(definitions))
    }
}

#[proc_macro]
pub fn macros(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Definitions(definitions) = parse_macro_input!(input as Definitions);

    let macros = definitions.iter().map(|def| {
        let Definition {
            attrs,
            name,
            matcher,
            body,
            export,
            wrap,
        } = def;

        let export_attr = export.then(|| quote!(#[macro_export]));
        let expansion = if *wrap {
            quote!({ #body })
        } else {
            quote!(#body)
        };

        quote! {
            #(#attrs)*
            #export_attr
            macro_rules! #name {
                (#matcher) => { #expansion };
            }
        }
    });

    quote!(#(#macros)*).into()
}
