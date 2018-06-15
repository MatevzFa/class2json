#[macro_use]
extern crate clap;

use clap::App;
use class_file::attributes::*;
use class_file::ClassFile;
use class_file::constant_pool::*;
use class_file::fields::*;
use class_file::methods::*;
use read_util::*;
use std::fs::File;

mod read_util;
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
    cf.fields = read_fields(f, cf.fields_count);
    cf.methods_count = read_u16(f);
    cf.methods = read_methods(f, cf.methods_count);
    cf.attributes_count = read_u16(f);
    cf.attributes = read_attributes(f, cf.attributes_count);

    cf
}

fn read_constant_pool(f: &mut File, constant_pool_count: u16) -> Vec<Box<CpInfo>> {
    let mut cp: Vec<Box<CpInfo>> = Vec::new();
    let mut constant_pool_remaining = constant_pool_count - 1;

    loop {
        let tag_u8 = read_u8(f);
        let tag = cp_tag_from(tag_u8);

        let cp_info: Box<CpInfo> = match tag {
            CpTag::Class => Box::new(ClassInfo {
                tag: tag_u8,
                name_index: read_u16(f),
            }),

            CpTag::Fieldref => Box::new(FieldrefInfo {
                tag: tag_u8,
                class_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::Methodref => Box::new(MethodrefInfo {
                tag: tag_u8,
                class_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::InterfaceMethodref => Box::new(InterfaceMethodrefInfo {
                tag: tag_u8,
                class_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::String => Box::new(StringInfo {
                tag: tag_u8,
                string_index: read_u16(f),
            }),

            CpTag::Integer => Box::new(IntegerInfo {
                tag: tag_u8,
                bytes: read_u32(f),
            }),

            CpTag::Float => Box::new(FloatInfo {
                tag: tag_u8,
                bytes: read_u32(f),
            }),

            CpTag::Long => Box::new(LongInfo {
                tag: tag_u8,
                high_bytes: read_u32(f),
                low_bytes: read_u32(f),
            }),

            CpTag::Double => Box::new(DoubleInfo {
                tag: tag_u8,
                high_bytes: read_u32(f),
                low_bytes: read_u32(f),
            }),

            CpTag::NameAndType => Box::new(NameAndTypeInfo {
                tag: tag_u8,
                name_index: read_u16(f),
                descriptor_index: read_u16(f),
            }),

            CpTag::Utf8 => {
                let length = read_u16(f);
                Box::new(Utf8Info {
                    tag: tag_u8,
                    length: length,
                    bytes: read_vec_u8(f, length as usize),
                })
            }

            CpTag::MethodHandle => Box::new(MethodHandleInfo {
                tag: tag_u8,
                reference_kind: read_u8(f),
                reference_index: read_u16(f),
            }),

            CpTag::MethodType => Box::new(MethodTypeInfo {
                tag: tag_u8,
                descriptor_index: read_u16(f),
            }),

            CpTag::InvokeDynamic => Box::new(InvokeDynamicInfo {
                tag: tag_u8,
                bootstrap_method_attr_index: read_u16(f),
                name_and_type_index: read_u16(f),
            }),

            CpTag::Module => Box::new(ModuleInfo {
                tag: tag_u8,
                name_index: read_u16(f),
            }),

            CpTag::Package => Box::new(PackageInfo {
                tag: tag_u8,
                name_index: read_u16(f),
            }),
        };

        match tag {
            CpTag::Long => constant_pool_remaining -= 2,
            CpTag::Double => constant_pool_remaining -= 2,
            _ => constant_pool_remaining -= 1,
        }

        cp.push(cp_info);

        if constant_pool_remaining == 0 {
            break;
        }
    }

    cp
}

fn read_fields(f: &mut File, fields_count: u16) -> Vec<FieldInfo> {
    let mut r = Vec::new();

    for _ in 0..fields_count {
        let access_flags = read_u16(f);
        let name_index = read_u16(f);
        let descriptor_index = read_u16(f);
        let attributes_count = read_u16(f);

        r.push(FieldInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attributes: read_attributes(f, attributes_count),
        });
    }

    r
}

fn read_methods(f: &mut File, methods_count: u16) -> Vec<MethodInfo> {
    let mut r = Vec::new();

    for _ in 0..methods_count {
        let access_flags = read_u16(f);
        let name_index = read_u16(f);
        let descriptor_index = read_u16(f);
        let attributes_count = read_u16(f);

        r.push(MethodInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attributes: read_attributes(f, attributes_count),
        });
    }

    r
}

fn read_attributes(f: &mut File, attributes_count: u16) -> Vec<AttributeInfo> {
    let mut r = Vec::new();

    for _ in 0..attributes_count {
        let attribute_name_index = read_u16(f);
        let attribute_length = read_u32(f);

        r.push(AttributeInfo {
            attribute_name_index: attribute_name_index,
            attribute_length: attribute_length,
            info: read_vec_u8(f, attribute_length as usize),
        });
    }

    r
}
