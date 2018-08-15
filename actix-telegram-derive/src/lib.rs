#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, Meta};

#[proc_macro_derive(NewType)]
pub fn new_type_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();

    let name = input.ident;
    let newtype = match input.data {
        Data::Struct(test) => match test.fields {
            Fields::Unnamed(fields) => {
                let unnamed = fields.unnamed;
                if unnamed.len() > 1 || unnamed.len() == 0 {
                    panic!("works only on newtypes")
                } else {
                    unnamed.first().unwrap().into_value().clone()
                }
            }
            _ => panic!("works only on unnamed structs"),
        },
        _ => panic!("works only on struct"),
    };

    let expanded = quote! {
        impl #name {
            pub fn new(field: #newtype) -> Self {
               #name(field)
            }

            pub fn get(self) -> #newtype {
                self.0
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(TelegramApi, attributes(return_type))]
pub fn telegram_api_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    match input.data {
        Data::Struct(_) => (),
        _ => panic!("works only on struct"),
    }
    let name = input.ident;
    let mut string_name = name.to_string();
    let lowercase = &string_name[0..1].to_lowercase();
    string_name.replace_range(0..1, lowercase);
    let attributes = input.attrs;
    let mut return_type = attributes.into_iter().filter_map(|attribute| {
        match attribute.interpret_meta() {
            Some(Meta::NameValue(meta)) => {
                if meta.ident == "return_type" {
                    match meta.lit {
                        syn::Lit::Str(a) => Some(a.parse::<syn::Type>().unwrap()),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    });
    let return_type = return_type.next().unwrap();

    let expanded = quote! {
        use actix::{Context, Handler, Message as ActixMessage};
        use actors::TelegramApi;
        use futures::Future;

        impl ActixMessage for #name {
            type Result = Result<#return_type, ()>;
        }

        impl Handler<#name> for TelegramApi {
            type Result = Box<Future<Item = #return_type, Error = ()>>;

            fn handle(&mut self, msg: #name, _ctx: &mut Context<Self>) -> Self::Result {
                Self::send_request(&self.token, #string_name, self.timeout, &msg)
            }
        }
    };

    expanded.into()
}
