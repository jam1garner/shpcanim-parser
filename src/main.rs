extern crate byteorder;
use byteorder::{ReadBytesExt, LittleEndian};

use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

#[derive(Debug)]
struct TpcbAnim {
    id : u32,
    unk_shorts : [u16; 2],
    unk_ints : [u32; 3],
    unk_floats : [f32; 12],
    unk_c : u32,
    unk_floats2 : [f32; 2],
    animation : Vec<(u32, [u8; 12])>
}

#[derive(Debug)]
struct ShanFile {
    default_id  : u32, 
    animations : Vec<TpcbAnim>
}

fn read_tpcb_file(file : &File, id : u32) -> TpcbAnim {
    
}

fn read_shan_file(file_name : &str) -> ShanFile {
    let mut file = File::open(file_name).unwrap();
    file.seek(SeekFrom::Start(4));
    let default_id = file.read_u32::<LittleEndian>().unwrap();
    let animCount = file.read_u32::<LittleEndian>().unwrap();
    let animations : Vec<TpcbAnim> = vec![];
    for i in 0..animCount {
        file.seek(SeekFrom::Start((0x80 + (i * 4)) as u64));
        let id = file.read_u32::<LittleEndian>().unwrap();
        file.seek(SeekFrom::Start((0x80 + (animCount * 4) + (i * 4)) as u64));
        let animOffset = file.read_u32::<LittleEndian>().unwrap();
        file.seek(SeekFrom::Start(animOffset as u64));
        animations.push(read_tpcb_file(&file, id));
    }
    
    ShanFile { default_id, animations}
}

fn main() {
    let shanFile = read_shan_file("C:\\Users\\jam1m\\Documents\\MEGAsync Downloads\\SHAN\\0x1e4e60c78.shan");
    println!("{:#?}", shanFile);
}
