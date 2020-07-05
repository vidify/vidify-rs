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
    println!("HELLO THIS IS A TEST HEHE");

    for attr in fields {
        println!("{:?}", field.attrs[0].tokens);
        for token in field.attrs[0].tokens.stream {
            match token {
                Ident => println!("key: {:?}", token.ident),
                proc_macro::Punct{ch, ..} => if ch == '=' {
                    continue
                } else {
                    panic!("wtf");
                },
                // proc_macro::Literal{Lit{symbol, ..}, ..} => println!("value: {:?}", symbol),
                _ => continue
            }
        }
    }

    let name = &input.ident;

    let gen = quote!{
        impl Conf for #name {
            fn test(&self) {
                println!("Hello, {}", stringify!(#name));

                let mut ret: Option<$type> = None;

                // First checking the arguments
                for name in $arg_names.iter() {
                    match $args.opt_value_from_str(name) {
                        // May fail in some cases.
                        Ok(val) => match val {
                            // May not exist.
                            Some(val) => {
                                ret = Some(val);
                                break;
                            },
                            None => continue
                        },
                        Err(_) => continue
                    }
                }

                match ret {
                    Some(val) => val,
                    None => {
                        // Then the config file, falling back to the default value
                        $ini.get_from(Some($info.conf_section), $info.conf_name, $info.default)
                        .parse::<$type>()
                        .expect(concat!("Could not parse the value of '",
                                        $conf_name, "' in the config file."))
                    }
                }
        }
    };

    Ok(gen.into())
}
