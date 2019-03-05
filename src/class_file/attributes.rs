#[derive(Debug, Default)]
pub struct Attributes {
    pub array: Vec<AttributeInfo>
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub info: Vec<u8>,
}