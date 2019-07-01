use serde::Serialize;

use class_file::Array;
use serialization::serialize_bytecode;

pub type Attributes = Array<AttributeInfo>;

#[derive(Debug, Default, Serialize)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,

    #[serde(serialize_with = "serialize_bytecode")]
    pub info: Vec<u8>,
}