use serde::Serialize;

use class_file::Array;
use class_file::attributes::Attributes;

pub type Methods = Array<MethodInfo>;

#[derive(Debug, Default, Serialize)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Attributes,
}