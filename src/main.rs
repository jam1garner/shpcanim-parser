extern crate byteorder;
use byteorder::{ReadBytesExt, LittleEndian};

use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::Write;

#[derive(Debug)]
struct TpcbKeyFrame {
    frame : u16,
    data  : [u8; 12]
}

#[derive(Debug)]
struct TpcbAnim {
    id : u16,
    unk_shorts : [u16; 2],
    unk_ints : [u32; 3],
    unk_floats : [f32; 12],
    unk_c : u32,
    unk_floats2 : [f32; 2],
    animation : Vec<TpcbKeyFrame>
}

#[derive(Debug)]
struct ShanFile {
    default_id  : u32, 
    animations : Vec<TpcbAnim>
}

fn read_tpcb_file(file : &mut File, id : u16) -> TpcbAnim {
    let start = (file.seek(SeekFrom::Current(4)).unwrap() - 4) as u32;
    let mut sectionOffsets = [0u32; 2];
    file.read_u32_into::<LittleEndian>(&mut sectionOffsets).unwrap();
    file.seek(SeekFrom::Current(4));
    let mut unk_shorts = [0u16; 2];
    let mut unk_ints = [0u32; 3];
    let mut unk_floats = [0f32; 12];
    file.read_u16_into::<LittleEndian>(&mut unk_shorts).unwrap();
    file.read_u32_into::<LittleEndian>(&mut unk_ints).unwrap();
    file.read_f32_into::<LittleEndian>(&mut unk_floats).unwrap();
    let unk_c = file.read_u32::<LittleEndian>().unwrap();
    let mut unk_floats2 = [0f32; 2];
    file.read_f32_into::<LittleEndian>(&mut unk_floats2).unwrap();
    
    let keyCount = file.read_u32::<LittleEndian>().unwrap();
    let mut animation : Vec<TpcbKeyFrame> = vec![];
    for i in 0..keyCount {
        file.seek(SeekFrom::Start((start + sectionOffsets[0] + (i * 2)) as u64));
        let frame = file.read_u16::<LittleEndian>().unwrap();
        file.seek(SeekFrom::Start((start + sectionOffsets[1] + (i * 0xC)) as u64));
        let mut data = [0u8; 0xC];
        file.read(&mut data).unwrap();
        animation.push(TpcbKeyFrame { frame, data });
    }

    TpcbAnim { id, unk_shorts, unk_ints, unk_floats, unk_c, unk_floats2, animation}
}

fn read_shan_file(file_name : &str) -> ShanFile {
    let mut file = File::open(file_name).unwrap();
    file.seek(SeekFrom::Start(4));
    let default_id = file.read_u32::<LittleEndian>().unwrap();
    let animCount = file.read_u32::<LittleEndian>().unwrap();
    let mut animations : Vec<TpcbAnim> = vec![];
    for i in 0..animCount {
        file.seek(SeekFrom::Start((0x80 + (i * 4)) as u64));
        let id = file.read_u16::<LittleEndian>().unwrap();
        file.seek(SeekFrom::Start((0x80 + (animCount * 4) + (i * 4)) as u64));
        let animOffset = file.read_u32::<LittleEndian>().unwrap();
        file.seek(SeekFrom::Start(animOffset as u64));
        animations.push(read_tpcb_file(&mut file, id));
    }
    
    ShanFile { default_id, animations}
}

fn main() {
    let shanFile = read_shan_file("C:\\Users\\jam1m\\Documents\\MEGAsync Downloads\\SHAN\\0x1e4e60c78.shan");
    println!("{:#?}", shanFile);
}
