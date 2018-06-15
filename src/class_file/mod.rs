use self::attributes::*;
use self::constant_pool::*;
use self::fields::*;
use self::methods::*;
use std::boxed::Box;
use std::fmt;

pub mod constant_pool;
pub mod fields;
pub mod methods;
pub mod attributes;

#[derive(Debug, Default)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<Box<CpInfo>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}
