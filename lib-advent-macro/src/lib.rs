use proc_macro::TokenStream;
use quote::quote;

/// ```
/// //#[derive(Parse, ParseFromStr, PartialEq, Eq, Debug)]
/// //#[prse = "Hello, my name is {name} and I am {statuses: and :}"]
/// //struct Person {
/// //    name: String,
/// //    statuses: Vec<String>,
/// //}
/// ```
#[proc_macro_derive(ParseFromStr)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_parse_from_str_macro(&ast)
}

fn impl_parse_from_str_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl FromStr for #name {
            type Err = prse::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Parse::from_str(s)
            }
        }
    };
    generated.into()
}
