use serde::ser::{Serialize, Serializer, SerializeSeq};

use bytecode;

pub fn serialize_utf8info_bytes<S>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    s.serialize_str(&String::from_utf8((v).clone()).unwrap())
}

pub fn serialize_bytecode<S>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    let mut seq = s.serialize_seq(Some(v.len()))?;

    let mut bytecode_iter = v.iter();

    loop {
        let byte = bytecode_iter.next();

        let byte = match byte {
            Some(byte) => *byte,
            None => break
        };

        let (bytecode, args_len, args_dist) = bytecode::to_string(byte);

        let str_buf = String::from(bytecode);

        if args_len > 0 {
            for arg_len in args_dist {
                let arg = match arg_len {
                    1 => bytecode_iter.next().unwrap(),
                    2 => bytecode_iter.next().unwrap() << 8 | bytecode_iter.next(),
                    4 => bytecode_iter.next().unwrap() << 24 | bytecode_iter.next() << 16 | bytecode_iter.next() << 8 | bytecode_iter.next().unwrap(),
                };

                
            }
        }


        seq.serialize_element(&str_buf)?;
    }

    seq.end()
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