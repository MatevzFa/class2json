use serde::ser::{Serialize, Serializer, SerializeSeq};

use self::attributes::*;
use self::constant_pool::*;
use self::fields::*;
use self::methods::*;

pub mod constant_pool;
pub mod fields;
pub mod methods;
pub mod attributes;

#[derive(Debug, Default, serde::Serialize)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Fields,
    pub methods_count: u16,
    pub methods: Methods,
    pub attributes_count: u16,
    pub attributes: Attributes,
}

#[derive(Debug, Default)]
pub struct Array<T: Serialize> {
    pub array: Vec<T>
}

impl<T: Serialize> Serialize for Array<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;

        for attr in &self.array {
            seq.serialize_element(&attr)?;
        }

        seq.end()
    }
}
