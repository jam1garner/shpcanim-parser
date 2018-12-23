extern crate byteorder;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

struct TpcbAnim {
    id : u32,
    unk_shorts : [u16; 2],
    unk_ints : [u32; 3],
    unk_floats : [f32; 12],
    unk_c : u32,
    unk_floats2 : [f32; 2],
    animation : Vec<(u32, [u8; 12])>
}

struct ShanFile {
    default_id  : u32, 
    animations : Vec<TpcbAnim>
}

fn read_shan_file(file_name : &str) -> ShanFile {
    let mut file = File::open(file_name).unwrap();
    file.seek(SeekFrom::Start(4));
    let val = file.read_u32::<LittleEndian>().unwrap();
    println!("Val = {}", val);
    ShanFile { default_id: 0, animations: vec![] }
}

fn main() {
    println!("Hello, world!");
    read_shan_file("C:\\Users\\jam1m\\Documents\\MEGAsync Downloads\\SHAN\\0x1e4e60c78.shan");
}
