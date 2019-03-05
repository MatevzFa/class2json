use serde::Serialize;

use class_file::Array;

pub type Attributes = Array<AttributeInfo>;

#[derive(Debug, Default, Serialize)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}