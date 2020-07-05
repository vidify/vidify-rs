extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Error, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Conf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let result: Result<TokenStream, Error> = match &ast.data {
        Data::Struct(DataStruct{fields, ..}) => impl_conf_macro(&ast, &fields),
        _ => Err(Error::new(ast.ident.span(), "cannot derive Options for type")),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into()
    }
}

fn impl_conf_macro(input: &DeriveInput, fields: &Fields) -> Result<TokenStream, Error> {
    for field in fields {
    }

    let name = &input.ident;

    let gen = quote!{
        impl Conf for #name {
            fn test() {
                println!("Hello, {}", stringify!(#name));
            }
        }
    };

    Ok(gen.into())
}
