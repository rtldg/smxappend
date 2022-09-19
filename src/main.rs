
/*
SPDX-License-Identifier: WTFPL
Copyright 2022 rtldg <rtldg@protonmail.com>

DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    0. You just DO WHAT THE FUCK YOU WANT TO.
*/

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use binrw::BinRead;
use binrw::BinWrite;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

#[derive(Debug, BinRead, BinWrite)]
#[brw(little)]
struct SectionHeader {
  nameoffs: u32,
  dataoffs: u32,
  size:     u32,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(repr(u8))]
enum CompressionType {
  None = 0,
  GZ   = 1,
}

#[derive(Debug, BinRead, BinWrite)]
#[brw(little, magic = b"FFPS")]
struct SmxHeader {
  version:         u16,
  compression:     CompressionType,
  disksize:        u32,
  imagesize:       u32,
  sections:        u8,
  stringtab:       u32,
  dataoffs:        u32,
  #[br(little, count = sections)]
  section_headers: Vec<SectionHeader>,
  #[br(count = (dataoffs - stringtab) as usize)]
  strings:         Vec<u8>,
  #[br(count = (disksize - dataoffs) as usize)]
  data:            Vec<u8>,
}

// assumes the smx is using compression... lol
fn main() -> anyhow::Result<()> {
  let infilename = env::args().nth(1).unwrap_or("smxreader.smx".into());
  let outfilename = env::args().nth(2).unwrap_or("smxreaderplus.smx".into());
  let mut header = SmxHeader::read(&mut BufReader::new(File::open(infilename)?))?;
  let my_section_name = ".src.zip\0";
  let my_section_data = b"asdfasdfasdfasdf";

  let my_section_start;

  header.data = {
    let mut gz = ZlibDecoder::new(&*header.data);
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;
    my_section_start = buffer.len();
    buffer.extend_from_slice(my_section_data);
    let mut gz = ZlibEncoder::new(Vec::new(), Compression::best());
    gz.write_all(&*buffer)?;
    gz.finish()?
  };

  let offset = std::mem::size_of::<SectionHeader>() + my_section_name.len();
  header.dataoffs += offset as u32;
  for section in header.section_headers.iter_mut() {
    section.dataoffs += offset as u32;
  }
  header.disksize = header.dataoffs + header.data.len() as u32;
  header.imagesize = header.dataoffs + (my_section_start + my_section_data.len()) as u32;
  header.stringtab += std::mem::size_of::<SectionHeader>() as u32;
  header.strings.extend_from_slice(my_section_name.as_bytes());
  header.section_headers.push(SectionHeader {
    nameoffs: header.dataoffs - my_section_name.len() as u32,
    dataoffs: header.dataoffs + my_section_start as u32,
    size:     my_section_data.len() as u32,
  });

  let fout = File::create(outfilename)?;
  let mut writer = BufWriter::new(fout);
  header.write_to(&mut writer)?;

  println!("{:?}", header);

  Ok(())
}
