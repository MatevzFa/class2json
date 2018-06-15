mod class_file;

use std::fs::File;
use std::io::prelude::*;

use class_file::*;

fn main() {
    let mut f = File::open("class_files/Sum.class").expect("Class file not found");
    let cf = read_classfile(&mut f);
    println!("{:#?}", cf);
}

fn read_classfile(f: &mut File) -> ClassFile {
    let cf: ClassFile = ClassFile {
        magic: read_u32(f),
        minor_version: read_u16(f),
        major_version: read_u16(f),
        constant_pool_count: read_u16(f),
        constant_pool: read_constant_pool(f, 15),
        access_flags: 0u16,
        this_class: 0u16,
        super_class: 0u16,
        interfaces_count: 0u16,
        interfaces: Vec::new(),
        fields_count: 0u16,
        fields: Vec::new(),
        methods_count: 0u16,
        methods: Vec::new(),
        attributes_count: 0u16,
        attributes: Vec::new(),
    };

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
                let mut buf: Vec<u8> = vec![0u8; length as usize];
                f.read_exact(buf.as_mut_slice())
                    .expect("couldnt read cp_info: Utf8");
                Box::new(Utf8Info {
                    tag: tag,
                    length: length,
                    bytes: buf,
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

        // println!("Found tag: {:X}", tag);

        cp.push(cp_info);

        if constant_pool_remaining == 0 {
            break;
        }
    }

    cp
}

fn read_u8(f: &mut File) -> u8 {
    let mut buf = [0u8; 1];
    f.read_exact(&mut buf).expect("could not parse u8");
    let r = { buf[0] as u8 };
    // println!("{:X}", r);
    r
}

fn read_u16(f: &mut File) -> u16 {
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf).expect("could not parse u16");
    let r = { (buf[0] as u16) << 8 | (buf[1] as u16) };
    // println!("{:X}", r);
    r
}

fn read_u32(f: &mut File) -> u32 {
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf).expect("could not parse u32");
    let r =
        { (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32) };
    // println!("{:X}", r);
    r
}
