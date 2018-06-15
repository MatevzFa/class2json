use std::fs::File;
use std::io::prelude::*;


pub fn read_u8(f: &mut File) -> u8 {
    let mut buf = [0u8; 1];
    f.read_exact(&mut buf).expect("could not parse u8");

    buf[0] as u8
}

pub fn read_u16(f: &mut File) -> u16 {
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf).expect("could not parse u16");

    (buf[0] as u16) << 8 | (buf[1] as u16)
}

pub fn read_u32(f: &mut File) -> u32 {
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf).expect("could not parse u32");

    (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | (buf[3] as u32)
}

pub fn read_vec_u8(f: &mut File, length: usize) -> Vec<u8> {
    let mut buf = vec![0u8; length];
    f.read_exact(buf.as_mut_slice()).expect("couldnt read cp_info: Utf8");

    buf
}

pub fn read_vec_u16(f: &mut File, length: usize) -> Vec<u16> {
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