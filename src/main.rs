#[macro_use]
extern crate clap;

use clap::App;
use class_file::*;
use std::fs::File;
use std::io::prelude::*;

mod class_file;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut f = File::open(matches.value_of("CLASS_FILE").unwrap()).expect("Class file not found");
    let cf = read_classfile(&mut f);
    println!("{:#?}", cf);
}

fn read_classfile(f: &mut File) -> ClassFile {
    let mut cf: ClassFile = ClassFile {
        ..Default::default()
    };

    cf.magic = read_u32(f);
    cf.minor_version = read_u16(f);
    cf.major_version = read_u16(f);
    cf.constant_pool_count = read_u16(f);
    cf.constant_pool = read_constant_pool(f, cf.constant_pool_count);
    cf.access_flags = read_u16(f);
    cf.this_class = read_u16(f);
    cf.super_class = read_u16(f);
    cf.interfaces_count = read_u16(f);
    cf.interfaces = read_vec_u16(f, cf.interfaces_count as usize);
    cf.fields_count = read_u16(f);
    // cf.fields = read_fields(f, cf.fields_count);
    // cf.methods_count = 0u16;
    // cf.methods = read_methods(f, cf.methods_count);
    // cf.attributes_count = 0u16;
    // cf.attributes = read_attributes(f, cf.attributes_count);

    cf
}

fn read_constant_pool(f: &mut File, constant_pool_count: u16) -> Vec<Box<CpInfo>> {
    let mut cp: Vec<Box<CpInfo>> = Vec::new();
    let mut constant_pool_remaining = constant_pool_count - 1;

    loop {
        let tag = read_u8(f);

        let cp_info: Box<CpInfo> = match cp_tag_from(tag) {
            CpTag::Class => Box::new(ClassInfo {
                tag: tag,
                name_index: read_u16(f),
            }),

            CpTag::Fieldref => Box::new(FieldrefInfo {
                tag: tag,
                class_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::Methodref => Box::new(MethodrefInfo {
                tag: tag,
                class_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::InterfaceMethodref => Box::new(InterfaceMethodrefInfo {
                tag: tag,
                class_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::String => Box::new(StringInfo {
                tag: tag,
                string_index: read_u16(f),
            }),

            CpTag::Integer => Box::new(IntegerInfo {
                tag: tag,
                bytes: read_u32(f),
            }),

            CpTag::Float => Box::new(FloatInfo {
                tag: tag,
                bytes: read_u32(f),
            }),

            CpTag::Long => Box::new(LongInfo {
                tag: tag,
                high_bytes: read_u32(f),
                low_bytes: read_u32(f),
            }),

            CpTag::Double => Box::new(DoubleInfo {
                tag: tag,
                high_bytes: read_u32(f),
                low_bytes: read_u32(f),
            }),

            CpTag::NameAndType => Box::new(NameAndTypeInfo {
                tag: tag,
                name_index: read_u16(f),
                descriptor_index: read_u16(f),
            }),

            CpTag::Utf8 => {
                let length = read_u16(f);
                Box::new(Utf8Info {
                    tag: tag,
                    length: length,
                    bytes: read_vec_u8(f, length as usize),
                })
            }

            CpTag::MethodHandle => Box::new(MethodHandleInfo {
                tag: tag,
                reference_kind: read_u8(f),
                reference_index: read_u16(f),
            }),

            CpTag::MethodType => Box::new(MethodTypeInfo {
                tag: tag,
                descriptor_index: read_u16(f),
            }),

            CpTag::InvokeDynamic => Box::new(InvokeDynamicInfo {
                tag: tag,
                bootstrap_method_attr_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::Module => Box::new(ModuleInfo {
                tag: tag,
                name_index: read_u16(f),
            }),

            CpTag::Package => Box::new(PackageInfo {
                tag: tag,
                name_index: read_u16(f),
            }),
        };

        constant_pool_remaining = constant_pool_remaining - 1;

        cp.push(cp_info);

        if constant_pool_remaining == 0 {
            break;
        }
    }

    cp
}

fn read_fields(f: &mut File, fields_count: u16) -> Vec<FieldInfo> {
    unimplemented!()
}

fn read_methods(f: &mut File, methods_count: u16) -> Vec<MethodInfo> {
    unimplemented!()
}

fn read_attributes(f: &mut File, attributes_count: u16) -> Vec<AttributeInfo> {
    unimplemented!()
}

fn read_u8(f: &mut File) -> u8 {
    let mut buf = [0u8; 1];
    f.read_exact(&mut buf).expect("could not parse u8");

    buf[0] as u8
}

fn read_u16(f: &mut File) -> u16 {
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf).expect("could not parse u16");

    (buf[0] as u16) << 8 | (buf[1] as u16)
}

fn read_u32(f: &mut File) -> u32 {
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf).expect("could not parse u32");

    (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32)
}

fn read_vec_u8(f: &mut File, length: usize) -> Vec<u8> {
    let mut buf = vec![0u8; length];
    f.read_exact(buf.as_mut_slice()).expect("couldnt read cp_info: Utf8");

    buf
}

fn read_vec_u16(f: &mut File, length: usize) -> Vec<u16> {
    use std::slice;

    let mut buf = vec![0u16; length];
    let buf_u8 = unsafe {
        slice::from_raw_parts_mut(
            buf.as_mut_slice().as_mut_ptr() as *mut u8,
            buf.len() * 2,
        )
    };
    f.read_exact(buf_u8).expect("couldnt read cp_info: Utf8");

    buf
}
