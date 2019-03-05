use serde::ser::{Serialize, Serializer, SerializeSeq};

pub fn serialize_utf8info_bytes<S>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    s.serialize_str(&String::from_utf8((v).clone()).unwrap())
}

pub fn serialize_u8_hex<S>(v: &u8, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    s.serialize_str(format!("{:02x}", v).as_str())
}

pub fn serialize_u16_hex<S>(v: &u16, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    s.serialize_str(format!("{:04x}", v).as_str())
}

pub fn serialize_u32_hex<S>(v: &u32, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    s.serialize_str(format!("{:08x}", v).as_str())
}