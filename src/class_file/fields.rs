use class_file::attributes::AttributeInfo;
use class_file::attributes::Attributes;

#[derive(Debug, Default)]
pub struct Fields {
    pub array: Vec<FieldInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Attributes,
}