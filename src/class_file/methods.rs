use class_file::attributes::AttributeInfo;
use class_file::attributes::Attributes;

#[derive(Debug, Default)]
pub struct Methods {
    pub array: Vec<MethodInfo>
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Attributes,
}