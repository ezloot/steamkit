mod error;

use std::collections::HashMap;

pub use error::*;
use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::many_till,
    number::complete::le_u8,
    IResult,
};
use nom_derive::{Nom, Parse};

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct Vpk {
    pub header: Header,
    #[nom(Parse = "limited(tree, header.tree_length as usize)")]
    pub tree: HashMap<String, Entry>,
}

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct Header {
    #[nom(Verify = "*signature == Self::SIGNATURE")]
    pub signature: u32,
    #[nom(Verify = "*version == 1 || *version == 2")]
    pub version: u32,
    pub tree_length: u32,
    #[nom(Cond = "version == 2")]
    pub v2: Option<HeaderV2>,
}

impl Header {
    pub const SIGNATURE: u32 = 0x55aa1234;
}

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct HeaderV2 {
    pub data_length: u32,
    pub archive_md5_length: u32,
    pub local_md5_length: u32,
    pub signature_length: u32,
}

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct DirectoryEntry {
    pub crc32: u32,
    pub preload_length: u16,
    pub archive_index: u16,
    pub archive_offset: u32,
    pub file_length: u32,
    pub suffix: u16,
    #[nom(Map = "|payload: &[u8]| payload.to_vec()", Take = "preload_length")]
    pub preload: Vec<u8>,
}

#[derive(Debug)]
pub struct Entry {
    pub path: String,
    pub dir_entry: DirectoryEntry,
}

pub fn cstring(input: &[u8]) -> IResult<&[u8], String> {
    let (input, (bytes, _)) = many_till(le_u8, tag(&[0]))(input)?;

    match String::from_utf8(bytes) {
        Ok(s) => Ok((input, s)),
        Err(_) => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::IsNot,
        })),
    }
}

fn limited<O, F>(parser: F, max_length: usize) -> impl Fn(&[u8]) -> IResult<&[u8], O>
where
    F: Fn(&[u8]) -> IResult<&[u8], O>,
{
    move |input: &[u8]| {
        let limited = take(max_length);
        let mut limited_parser = map_res(limited, |data: &[u8]| parser(data).map(|(_, out)| out));
        limited_parser(input)
    }
}

pub fn tree(mut input: &[u8]) -> IResult<&[u8], HashMap<String, Entry>> {
    let mut map = HashMap::new();

    loop {
        let (i, ext) = cstring(input)?;
        input = i;
        if ext.is_empty() {
            break;
        }

        loop {
            let (i, dir) = cstring(input)?;
            input = i;
            if dir.is_empty() {
                break;
            }

            loop {
                let (i, name) = cstring(input)?;
                input = i;
                if name.is_empty() {
                    break;
                }

                let mut path = name.clone();
                if path == " " {
                    path.clear();
                }

                if ext != " " {
                    path.push('.');
                    path.push_str(&ext);
                }

                if dir != " " {
                    path = format!("{dir}/{path}");
                }

                let (i, dir_entry) = DirectoryEntry::parse(input)?;
                input = i;

                map.insert(path.clone(), Entry { path, dir_entry });
            }
        }
    }

    Ok((input, map))
}
