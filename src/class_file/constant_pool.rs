use std::fmt;

use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde::Serialize as SerializeDer;

pub trait CpInfo: fmt::Debug + erased_serde::Serialize {}

serialize_trait_object!(CpInfo);

#[derive(Debug, Default)]
pub struct ConstantPool {
    pub array: Vec<Box<CpInfo>>
}

impl Serialize for ConstantPool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.array.len()))?;

        for attr in &self.array {
            seq.serialize_element(&attr)?;
        }

        seq.end()
    }
}

#[derive(Debug, SerializeDer)]
pub struct ClassInfo {
    pub tag: u8,
    pub name_index: u16,
}

impl CpInfo for ClassInfo {}


#[derive(Debug, SerializeDer)]
pub struct FieldrefInfo {
    pub tag: u8,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl CpInfo for FieldrefInfo {}


#[derive(Debug, SerializeDer)]
pub struct MethodrefInfo {
    pub tag: u8,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl CpInfo for MethodrefInfo {}


#[derive(Debug, SerializeDer)]
pub struct InterfaceMethodrefInfo {
    pub tag: u8,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl CpInfo for InterfaceMethodrefInfo {}


#[derive(Debug, SerializeDer)]
pub struct StringInfo {
    pub tag: u8,
    pub string_index: u16,
}

impl CpInfo for StringInfo {}


#[derive(Debug, SerializeDer)]
pub struct IntegerInfo {
    pub tag: u8,
    pub bytes: u32,
}

impl CpInfo for IntegerInfo {}


#[derive(Debug, SerializeDer)]
pub struct FloatInfo {
    pub tag: u8,
    pub bytes: u32,
}

impl CpInfo for FloatInfo {}


#[derive(Debug, SerializeDer)]
pub struct LongInfo {
    pub tag: u8,
    pub high_bytes: u32,
    pub low_bytes: u32,
}

impl CpInfo for LongInfo {}


#[derive(Debug, SerializeDer)]
pub struct DoubleInfo {
    pub tag: u8,
    pub high_bytes: u32,
    pub low_bytes: u32,
}

impl CpInfo for DoubleInfo {}


#[derive(Debug, SerializeDer)]
pub struct NameAndTypeInfo {
    pub tag: u8,
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl CpInfo for NameAndTypeInfo {}

#[derive(SerializeDer)]
pub struct Utf8Info {
    pub tag: u8,
    pub length: u16,
    pub bytes: Vec<u8>,
}

impl CpInfo for Utf8Info {}

impl fmt::Debug for Utf8Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Utf8Info")
            .field("tag", &self.tag)
            .field("length", &self.length)
            .field("bytes", &String::from_utf8((&self.bytes).clone()).unwrap())
            .finish()
    }
}


#[derive(Debug, SerializeDer)]
pub struct MethodHandleInfo {
    pub tag: u8,
    pub reference_kind: u8,
    pub reference_index: u16,
}

impl CpInfo for MethodHandleInfo {}


#[derive(Debug, SerializeDer)]
pub struct MethodTypeInfo {
    pub tag: u8,
    pub descriptor_index: u16,
}

impl CpInfo for MethodTypeInfo {}


#[derive(Debug, SerializeDer)]
pub struct InvokeDynamicInfo {
    pub tag: u8,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

impl CpInfo for InvokeDynamicInfo {}


#[derive(Debug, SerializeDer)]
pub struct ModuleInfo {
    pub tag: u8,
    pub name_index: u16,
}

impl CpInfo for ModuleInfo {}


#[derive(Debug, SerializeDer)]
pub struct PackageInfo {
    pub tag: u8,
    pub name_index: u16,
}

impl CpInfo for PackageInfo {}


#[derive(Debug, SerializeDer)]
pub enum CpTag {
    Class,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    InvokeDynamic,
    Module,
    Package,
}

pub fn cp_tag_from(tag: u8) -> CpTag {
    match tag {
        7 => CpTag::Class,
        9 => CpTag::Fieldref,
        10 => CpTag::Methodref,
        11 => CpTag::InterfaceMethodref,
        8 => CpTag::String,
        3 => CpTag::Integer,
        4 => CpTag::Float,
        5 => CpTag::Long,
        6 => CpTag::Double,
        12 => CpTag::NameAndType,
        1 => CpTag::Utf8,
        15 => CpTag::MethodHandle,
        16 => CpTag::MethodType,
        18 => CpTag::InvokeDynamic,
        19 => CpTag::Module,
        20 => CpTag::Package,
        _ => panic!("Unknown u8 cptag {:X}", tag),
    }
}
