extern crate byteorder;
use byteorder::{ReadBytesExt, LittleEndian};

use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::env;

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

fn read_tpcb_file(file : &mut File, id : u16) -> std::io::Result<TpcbAnim> {
    let start = (file.seek(SeekFrom::Current(4))? - 4) as u32;
    let mut section_offsets = [0u32; 2];
    file.read_u32_into::<LittleEndian>(&mut section_offsets)?;
    file.seek(SeekFrom::Current(4))?;
    let mut unk_shorts = [0u16; 2];
    let mut unk_ints = [0u32; 3];
    let mut unk_floats = [0f32; 12];
    file.read_u16_into::<LittleEndian>(&mut unk_shorts)?;
    file.read_u32_into::<LittleEndian>(&mut unk_ints)?;
    file.read_f32_into::<LittleEndian>(&mut unk_floats)?;
    let unk_c = file.read_u32::<LittleEndian>()?;
    let mut unk_floats2 = [0f32; 2];
    file.read_f32_into::<LittleEndian>(&mut unk_floats2)?;
    
    let key_count = file.read_u32::<LittleEndian>()?;
    let mut animation : Vec<TpcbKeyFrame> = vec![];
    for i in 0..key_count {
        file.seek(SeekFrom::Start((start + section_offsets[0] + (i * 2)) as u64))?;
        let frame = file.read_u16::<LittleEndian>()?;
        file.seek(SeekFrom::Start((start + section_offsets[1] + (i * 0xC)) as u64))?;
        let mut data = [0u8; 0xC];
        file.read(&mut data)?;
        animation.push(TpcbKeyFrame { frame, data });
    }

    Ok(
        TpcbAnim { id, unk_shorts, unk_ints, unk_floats, unk_c, unk_floats2, animation}
    )
}

fn read_shan_file(file_name : &str) -> std::io::Result<ShanFile> {
    let mut file = File::open(file_name)?;
    file.seek(SeekFrom::Start(4))?;
    let default_id = file.read_u32::<LittleEndian>()?;
    let anim_count = file.read_u32::<LittleEndian>()?;
    let mut animations : Vec<TpcbAnim> = vec![];
    for i in 0..anim_count {
        file.seek(SeekFrom::Start((0x80 + (i * 4)) as u64))?;
        let id = file.read_u16::<LittleEndian>()?;
        file.seek(SeekFrom::Start((0x80 + (anim_count * 4) + (i * 4)) as u64))?;
        let anim_offset = file.read_u32::<LittleEndian>()?;
        file.seek(SeekFrom::Start(anim_offset as u64))?;
        animations.push(read_tpcb_file(&mut file, id)?);
    }
    
    Ok(ShanFile {
        default_id, animations
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No file passed. Usage:\nshpcanim [file]\n");
        return;
    }
    let shan_file = read_shan_file(&args[1]).unwrap();
    println!("{:#?}", shan_file);
}
