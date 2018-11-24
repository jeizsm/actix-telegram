use crate::types::{GenParams, MetaAttributes};
use syn::*;

pub fn meta(meta: &Meta, params: &GenParams) -> MetaAttributes {
    let mut attributes = MetaAttributes {
        vis: None,
        prefix: params.fn_name_prefix.map(ToOwned::to_owned),
        suffix: params.fn_name_suffix.map(ToOwned::to_owned),
        mutable: false,
        copy: false,
        consume: false,
        optional: false,
    };
    match meta {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(s), ..
        }) => {
            attributes.vis = Some(s.value());
            attributes
        }
        Meta::List(MetaList { nested, .. }) => {
            nested
                .iter()
                .for_each(|nested_meta| parse_nested_meta(nested_meta, &mut attributes));
            attributes
        }
        _ => attributes,
    }
}

pub fn parse_nested_meta(nested_meta: &NestedMeta, attributes: &mut MetaAttributes) {
    match nested_meta {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            lit: Lit::Str(s),
            ident,
            ..
        })) => match ident.to_string().as_ref() {
            "vis" => attributes.vis = Some(s.value()),
            "prefix" => {
                attributes.prefix = {
                    let value = s.value();
                    if value.is_empty() {
                        Some(s.value())
                    } else {
                        Some(format!("{}_", s.value()))
                    }
                }
            }
            "suffix" => {
                attributes.suffix = {
                    let value = s.value();
                    if value.is_empty() {
                        Some(s.value())
                    } else {
                        Some(format!("_{}", s.value()))
                    }
                }
            }
            _ => (),
        },
        NestedMeta::Meta(Meta::Word(ident)) => match ident.to_string().as_ref() {
            "mutable" => {
                if attributes.suffix.is_none() {
                    attributes.suffix = Some("_mut".to_owned());
                }
                attributes.mutable = true;
            }
            "copy" => {
                attributes.copy = true;
            }
            "consume" => {
                if attributes.suffix.is_none() {
                    attributes.prefix = Some("consume_set_".to_owned());
                }
                attributes.consume = true;
            }
            "optional" => {
                attributes.optional = true;
            }
            _ => (),
        },
        _ => (),
    }
}

pub fn attr_tuple(attr: &Attribute) -> Option<(Ident, Meta)> {
    let meta = attr.interpret_meta();
    meta.map(|v| (v.name(), v))
}

pub fn global_attr(attrs: &[syn::Attribute], attribute_name: &str) -> Vec<Meta> {
    attrs
        .iter()
        .filter_map(|v| {
            let (attr_name, meta) = attr_tuple(v).expect("attribute");
            if attr_name == attribute_name {
                Some(meta)
            } else {
                None
            }
        }).collect()
}


pub fn parse_type(ty: &Type) -> (bool, &Type) {
    if let Type::Path(ref path_type) = ty {
        let segment = &path_type.path.segments[0];
        if segment.ident == "Option" {
            if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                let arg = &args.args[0];
                if let GenericArgument::Type(ty) = arg {
                    return (true, ty);
                }
            }
        }
    }
    (false, ty)
}
