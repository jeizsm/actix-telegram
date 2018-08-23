#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::*;
use quote::{ToTokens};

#[proc_macro_derive(NewType)]
pub fn new_type_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    let name = input.ident;
    let newtype = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
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
    let struct_name = input.ident;
    let mut method_name = struct_name.to_string();
    let lowercase = &method_name[0..1].to_lowercase();
    method_name.replace_range(0..1, lowercase);
    let attributes = input.attrs;
    let return_type = return_type(attributes);

    let file_fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                file_fields(fields)
            }
            Fields::Unit => None,
            _ => unreachable!()
        },
        _ => unreachable!(),
    };

    let request = match file_fields {
        Some(fields) => {
            quote! {
                let mut form = MultipartForm::default();
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
        use actix_web::client::MultipartForm;
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
    return_type.next().unwrap()
}

fn file_fields(fields: FieldsNamed) -> Option<FileFields> {
    let fields: Vec<_> = fields.named.into_iter().filter_map(|field| {
        let field_name = field.ident.unwrap();
        match field.ty {
            Type::Path(ty) => {
                file_field(ty).map(|(field_type, is_optional)| {
                    FileField {
                        field_type,
                        is_optional,
                        field_name,
                    }
                })
            },
            _ => unreachable!(),
        }
    }).collect();
    if fields.is_empty() {
        None
    } else {
        Some(FileFields(fields))
    }
}

fn file_field(ty: TypePath) -> Option<(Ident, bool)> {
    let mut is_optional = false;
    for segment in ty.path.segments.into_iter() {
        if segment.ident == "Option" { is_optional = true };

        match segment.arguments {
            PathArguments::AngleBracketed(args) => {
                for arg in args.args.into_iter() {
                    match arg {
                        GenericArgument::Type(Type::Path(ty)) => {
                            match file_field(ty) {
                                Some((ident, _)) => return Some((ident, is_optional)),
                                _ => (),
                            }
                        },
                        _ => (),
                    }
                }
            },
            _ => (),
        };

        let segment_string = segment.ident.to_string();
        if segment_string.starts_with("InputFile") || segment_string.starts_with("InputMedia") {
            return Some((segment.ident, is_optional))
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
                },
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
                },
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
            Some(InputFile::Memory { name, source, len, mime }) => {
                form.add_reader2(&name, source, Some(name.as_str()), mime, len);
            },
            Some(InputFile::Disk { path, mime }) => {
                let path: &Path = path.as_ref();
                let field_name = path.file_name().unwrap().to_str().unwrap();
                match mime {
                    Some(mime) => form.add_file_with_mime(field_name, &path, mime).unwrap(),
                    None => form.add_file(field_name, &path).unwrap(),
                }

            },
            None => (),
        }
    } else {
        quote! {
            InputFile::Memory { name, source, len, mime } => {
                form.add_reader2(&name, source, Some(name.as_str()), mime, len);
            },
            InputFile::Disk { path, mime } => {
                let path: &Path = path.as_ref();
                let field_name = path.file_name().unwrap().to_str().unwrap();
                match mime {
                    Some(mime) => form.add_file_with_mime(field_name, &path, mime).unwrap(),
                    None => form.add_file(field_name, &path).unwrap(),
                }
            }
        }
    }
}
