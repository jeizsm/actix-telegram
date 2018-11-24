/*!
Getset, we're ready to go!

A procedural macro for generating the most basic getters and setters on fields.

Getters are generated as `fn field(&self) -> &type`, while setters are generated as `fn field(&mut self, val: type)`.

These macros are not intended to be used on fields which require custom logic inside of their setters and getters. Just write your own in that case!

```rust
#[macro_use]
extern crate getset;

#[derive(Getters, Setters, New, Default)]
#[get(vis = "pub")]
#[get(vis = "pub", mutable)]
#[set(vis = "pub", consume)]
#[set(vis = "pub")]
#[new(vis = "pub")]
pub struct Foo<T> where T: Copy + Clone + Default {
    /// Doc comments are supported!
    /// Multiline, even.
    #[get(copy)] #[get(mutable)] #[set]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    public: Option<T>,

    #[set(optional)]
    optional: Option<String>,
}

fn main() {
    let mut foo: Foo<i64> = Foo::new(1).consume_set_public(3);
    assert_eq!(foo.private(), 1);
    assert_eq!(*foo.public(), Some(3));
    foo.set_private(3);
    (*foo.private_mut()) += 1;
    assert_eq!(foo.private(), 4);
    foo.set_public(4);
    assert_eq!(*foo.public(), Some(4));
    foo.set_public(None);
    assert_eq!(*foo.public(), None);
    foo.set_optional(Some("test"));
    assert_eq!(foo.optional(), &Some("test".to_string()));
    foo.set_optional(None::<&str>);
    assert_eq!(*foo.optional(), None);
}
```
```compile_fail
#[macro_use]
extern crate getset;
mod submodule {
    #[derive(Getters, Default)]
    #[get = "pub"]
    pub struct Foo {
        #[get]
        private: i32,
        public: i32,
    }
}
fn main() {
    let mut foo = submodule::Foo::default();
    assert_eq!(*foo.private(), 2);
}
```
*/
#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

mod generate;
mod parse;
mod types;

use crate::types::{GenMode, GenParams};
use proc_macro::TokenStream;
use syn::*;
use quote::ToTokens;
use proc_macro2::Ident;

#[proc_macro_derive(Getters, attributes(get))]
pub fn getters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);
    let params = GenParams {
        attribute_name: "get",
        fn_name_prefix: None,
        fn_name_suffix: None,
        global_attr: parse::global_attr(&ast.attrs, "get"),
    };

    // Build the impl
    let gen = produce(&ast, &GenMode::Get, &params);

    // Return the generated impl
    gen.into()
}

#[proc_macro_derive(MutGetters, attributes(get_mut))]
pub fn mut_getters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).expect("Couldn't parse for getters");
    let params = GenParams {
        attribute_name: "get_mut",
        fn_name_prefix: None,
        fn_name_suffix: Some("_mut"),
        global_attr: parse::global_attr(&ast.attrs, "get_mut"),
    };

    // Build the impl
    let gen = produce(&ast, &GenMode::GetMut, &params);
    // Return the generated impl
    gen.into()
}

#[proc_macro_derive(Setters, attributes(set))]
pub fn setters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).expect("Couldn't parse for setters");
    let params = GenParams {
        attribute_name: "set",
        fn_name_prefix: Some("set_"),
        fn_name_suffix: None,
        global_attr: parse::global_attr(&ast.attrs, "set"),
    };

    // Build the impl
    let gen = produce(&ast, &GenMode::Set, &params);

    // Return the generated impl
    gen.into()
}

#[proc_macro_derive(New, attributes(new))]
pub fn new(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse2(input.into()).expect("Couldn't parse for setters");

    // Build the impl
    let gen = produce_new(&ast);

    // Return the generated impl
    gen.into()
}

fn produce_new(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields
            .iter()
            .map(|f| generate::implement_new(f))
            .collect::<Vec<_>>();

        let initialize = generated.iter().map(|(a, _)| a).collect::<Vec<_>>();
        let struct_initialize = generated.iter().map(|(_, a)| a).collect::<Vec<_>>();
        let global_attr = parse::global_attr(&ast.attrs, "new");
        let attr = global_attr.first().expect("new attribute").clone();
        let params = GenParams {
            attribute_name: "new",
            fn_name_prefix: None,
            fn_name_suffix: None,
            global_attr: global_attr,
        };

        let attributes = parse::meta(&attr, &params);
        let visibility: Option<Visibility> = attributes
            .vis
            .map(|vis| syn::parse_str(vis.as_ref()).expect("visibility"));
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #visibility fn new(#(#initialize)*) -> Self {
                    Self {
                        #(#struct_initialize)*
                    }
                }
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(New)] is only defined for structs, not for enums!");
    }
}

fn produce(ast: &DeriveInput, mode: &GenMode, params: &GenParams) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields
            .iter()
            .map(|f| generate::implement(f, mode, params))
            .collect::<Vec<_>>();

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        panic!("#[derive(Getters)] is only defined for structs, not for enums!");
    }
}

#[proc_macro_derive(TelegramApi, attributes(return_type))]
pub fn telegram_api_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    let struct_name = input.ident;
    let mut method_name = struct_name.to_string();
    let lowercase = &method_name[0..1].to_lowercase();
    method_name.replace_range(0..1, lowercase);
    let attributes = input.attrs;
    let return_type = return_type(attributes);

    let file_fields: Option<Box<ToTokens>> = match struct_name.to_string().as_str() {
        "SetWebhook" => Some(Box::new(quote! {
            match msg.certificate {
                Some(InputFile::Memory { name, source, len }) => {
                    form.add_reader2("certificate", source, Some(name.as_str()), None, len);
                },
                Some(InputFile::Disk { path }) => {
                    let path: &Path = path.as_ref();
                    let field_name = path.file_name().unwrap().to_str().unwrap();
                    form.add_file("certificate", &path).unwrap();
                },
                None => (),
            }
        })),
        _ => match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields) => file_fields(fields),
                Fields::Unit => None,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
    };

    let request = match file_fields {
        Some(fields) => {
            quote! {
                let mut form = Form::default();
                let value = serde_json::to_value(&msg).unwrap();
                let object = value.as_object().unwrap();

                // add properties
                for (key, val) in object {
                    let val = match val {
                        Value::String(val) => val.to_string(),
                        etc => etc.to_string(),
                    };

                    form.add_text(key, val);
                }
                #fields;
                Self::send_multipart_request(&self.token, #method_name, self.timeout, form)
            }
        }
        None => {
            quote! {
                Self::send_request(&self.token, #method_name, self.timeout, &msg)
            }
        }
    };

    let expanded = quote! {
        use actix::{Context, Handler, Message as ActixMessage};
        use actors::TelegramApi;
        use futures::Future;

        #[allow(unused_imports)]
        use multipart_rfc7578::Form;
        #[allow(unused_imports)]
        use serde_json::{self, Value};
        #[allow(unused_imports)]
        use std::path::Path;

        impl ActixMessage for #struct_name {
            type Result = Result<#return_type, ()>;
        }

        impl Handler<#struct_name> for TelegramApi {
            type Result = Box<Future<Item = #return_type, Error = ()>>;

            fn handle(&mut self, msg: #struct_name, _ctx: &mut Context<Self>) -> Self::Result {
                #request
            }
        }
    };

    expanded.into()
}

fn return_type(attributes: Vec<Attribute>) -> Type {
    let mut return_type =
        attributes
            .into_iter()
            .filter_map(|attribute| match attribute.interpret_meta() {
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
            });
    return_type.next().unwrap()
}

fn file_fields(fields: FieldsNamed) -> Option<Box<ToTokens>> {
    let fields: Vec<_> = fields
        .named
        .into_iter()
        .filter_map(|field| {
            let field_name = field.ident.unwrap();
            match field.ty {
                Type::Path(ty) => file_field(ty).map(|(field_type, is_optional)| FileField {
                    field_type,
                    is_optional,
                    field_name,
                }),
                _ => unreachable!(),
            }
        }).collect();
    if fields.is_empty() {
        None
    } else {
        Some(Box::new(FileFields(fields)))
    }
}

fn file_field(ty: TypePath) -> Option<(Ident, bool)> {
    let mut is_optional = false;
    for segment in ty.path.segments.into_iter() {
        if segment.ident == "Option" {
            is_optional = true
        };

        match segment.arguments {
            PathArguments::AngleBracketed(args) => {
                for arg in args.args.into_iter() {
                    match arg {
                        GenericArgument::Type(Type::Path(ty)) => match file_field(ty) {
                            Some((ident, _)) => return Some((ident, is_optional)),
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        };

        let segment_string = segment.ident.to_string();
        if segment_string.starts_with("InputFile") || segment_string.starts_with("InputMedia") {
            return Some((segment.ident, is_optional));
        }
    }
    None
}

#[derive(Debug)]
struct FileField {
    field_type: Ident,
    field_name: Ident,
    is_optional: bool,
}

#[derive(Debug)]
struct FileFields(Vec<FileField>);

impl ToTokens for FileFields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for field in self.0.iter() {
            let field_name = &field.field_name;
            let quote = match field.field_type.to_string().as_str() {
                "InputFile" => {
                    let expanded = expand_input_file_field(field.is_optional);
                    quote! {
                        match msg.#field_name {
                            #expanded
                        }
                    }
                }
                "InputFileOrString" => {
                    let expanded = expand_input_file_or_string_field(field.is_optional);
                    quote! {
                        match msg.#field_name {
                            #expanded
                        }
                    }
                }
                "InputMedia" => {
                    let expanded = expand_input_media_field(field.is_optional);
                    quote! {
                        match msg.#field_name {
                            #expanded
                        }
                    }
                }
                _ => {
                    let expanded = expand_input_media_photo_or_video_field(field.is_optional);
                    quote! {
                        for media in msg.#field_name {
                            match media {
                                #expanded
                            }
                        }
                    }
                }
            };
            quote.to_tokens(tokens);
        }
    }
}

fn expand_input_media_photo_or_video_field(_is_optional: bool) -> proc_macro2::TokenStream {
    let match_media = expand_input_file_or_string_field(false);
    let match_thumb = expand_input_file_or_string_field(true);
    quote! {
        InputMediaPhotoOrInputMediaVideo::InputMediaPhoto(input_media) => {
            match input_media.media {
                #match_media
            }
        },
        InputMediaPhotoOrInputMediaVideo::InputMediaVideo(input_media) => {
            match input_media.media {
                #match_media
            }
            match input_media.thumb {
                #match_thumb
            }
        },
    }
}

fn expand_input_media_field(_is_optional: bool) -> proc_macro2::TokenStream {
    let match_media = expand_input_file_or_string_field(false);
    let match_thumb = expand_input_file_or_string_field(true);
    quote! {
        InputMedia::InputMediaAnimation(input_media) => {
            match input_media.media {
                #match_media
            }
            match input_media.thumb {
                #match_thumb
            }
        },
        InputMedia::InputMediaDocument(input_media) => {
            match input_media.media {
                #match_media
            }
            match input_media.thumb {
                #match_thumb
            }
        },
        InputMedia::InputMediaPhoto(input_media) => {
            match input_media.media {
                #match_media
            }
        },
        InputMedia::InputMediaVideo(input_media) => {
            match input_media.media {
                #match_media
            }
            match input_media.thumb {
                #match_thumb
            }
        },
        InputMedia::InputMediaAudio(input_media) => {
            match input_media.media {
                #match_media
            }
            match input_media.thumb {
                #match_thumb
            }
        }
    }
}

fn expand_input_file_or_string_field(is_optional: bool) -> proc_macro2::TokenStream {
    let expanded = expand_input_file_field(false);
    if is_optional {
        quote! {
            Some(InputFileOrString::InputFile(input_file)) => {
                match input_file {
                    #expanded
                }
            },
            Some(InputFileOrString::String(_)) => {
                ()
            },
            None => (),
        }
    } else {
        quote! {
            InputFileOrString::InputFile(input_file) => {
                match input_file {
                    #expanded
                }
            },
            InputFileOrString::String(_) => {
                ()
            },
        }
    }
}

fn expand_input_file_field(is_optional: bool) -> proc_macro2::TokenStream {
    if is_optional {
        quote! {
            Some(InputFile::Memory { name, source, len }) => {
                form.add_reader2(&name, source, Some(name.as_str()), None, len);
            },
            Some(InputFile::Disk { path }) => {
                let path: &Path = path.as_ref();
                let field_name = path.file_name().unwrap().to_str().unwrap();
                form.add_file(field_name, &path).unwrap();
            },
            None => (),
        }
    } else {
        quote! {
            InputFile::Memory { name, source, len } => {
                form.add_reader2(&name, source, Some(name.as_str()), None, len);
            },
            InputFile::Disk { path } => {
                let path: &Path = path.as_ref();
                let field_name = path.file_name().unwrap().to_str().unwrap();
                form.add_file(field_name, &path).unwrap();
            }
        }
    }
}
