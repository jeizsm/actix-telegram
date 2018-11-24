use syn::Meta;

#[derive(Debug)]
pub struct MetaAttributes {
    pub vis: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub mutable: bool,
    pub copy: bool,
    pub consume: bool,
    pub optional: bool,
}

pub struct GenParams {
    pub attribute_name: &'static str,
    pub fn_name_prefix: Option<&'static str>,
    pub fn_name_suffix: Option<&'static str>,
    pub global_attr: Vec<Meta>,
}

pub enum GenMode {
    Get,
    Set,
    GetMut,
}
