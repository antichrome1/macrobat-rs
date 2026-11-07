#![doc = include_str!("README.md")]
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

/// Define multiple single-arm rust declarative macros in a familiar
/// function-like syntax
///
/// - `@wrap` wraps your code block in braces, which is useful when you
///   have multiple statements to do some computation and then return a
///   value.
/// - `@export` adds `#[macro_export]` attr to your macro.
///
/// # Example
/// ```
/// use macrobat::macros;
///
/// macros! {
///
///     macro add($x:expr, $y:expr) {
///         $x + $y
///     } @export
///
///     macro collatz_steps($x:expr) {
///         let mut n = $x;
///         let mut steps = 0u64;
///         while n > 1 {
///             n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
///             steps += 1;
///         }
///         steps
///     } @export @wrap
///
/// }
///
/// assert_eq!(3, add!(1, 2));
/// assert_eq!(111, collatz_steps!(27));
/// ```
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
