#[macro_use]
extern crate structure;

use std::fs::File;
use std::io;
use hex;
use structure::structure;
use positioned_io::ReadAt;

fn main() -> io::Result<()> {
    // open title.tmd
    let file = File::open("title.tmd")?;
    let mut contents: Vec<u64> = Vec::new();

    // title id
    let mut title_id = [0; 8];
    let _title_id_read = file.read_at(0x18C, &mut title_id)?;
    println!("Title ID: {:?}", hex::encode_upper(&title_id));

    // content count
    let mut content_count = [0; 2];
    let _content_count_read = file.read_at(0x1DE, &mut content_count)?;
    let mut s = structure!(">H");
    let c_count = s.unpack(&content_count)?.0;
    println!("Content count: {:?}", c_count);

    // tmd index
    let mut tmd_index = [0; 2];
    let _tmd_index_read = file.read_at(0x204, &mut tmd_index)?;
    println!("TMD Index: {:?}", tmd_index);

    // do the thing
    for c in 1..c_count {
        // content id
        let mut content_id = [0; 0x4];
        let _contend_id_read = file.read_at((0xB04 + (0x30 * c)).into(), &mut content_id)?;
        println!("Content ID: {:?}", hex::encode_upper(content_id));

        // content index
        let mut content_index = [0; 0x2];
        let _contend_index_read = file.read_at((0xB08 + (0x30 * c)).into(), &mut content_index)?;
        println!("Content Index: {:?}", content_index);

        // content type
        let mut content_type = [0; 2];
        let _contend_type_read = file.read_at((0xB0A + (0x30 * c)).into(), &mut content_type)?;
        let c_type = s.unpack(&content_type)?.0;
        println!("Content Type: {:?}", c_type);

        // content size
        let mut content_size = [0; 8];
        let _contend_size_read = file.read_at((0xB0C + (0x30 * c)).into(), &mut content_size)?;
        let mut s = structure!(">Q");
        let c_size = s.unpack(&content_size)?.0;
        println!("Content Size: {:?}", c_size);

        // content hash
        let mut content_hash = [0; 0x14];
        let _contend_hash_read = file.read_at((0xB14 + (0x30 * c)).into(), &mut content_hash)?;
        println!("Content Hash: {:?}", content_hash);
        
        /*
        contents.push(hex::encode_upper(content_id));
        contents.push(content_index);
        contents.push(c_type);
        contents.push(c_size);
        contents.push(content_hash[0]);
        */
    }
    Ok(())
}
