mod error;

use std::collections::HashMap;

pub use error::*;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::char,
    combinator::{map, map_res, peek, recognize},
    multi::many_till,
    number::complete::{le_u32, le_u8},
    sequence::{pair, tuple},
    IResult,
};
use nom_derive::{Nom, Parse};

pub struct Vpk {}

impl Vpk {
    pub const SIGNATURE: u32 = 0x55aa1234;
}

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct Header {
    pub signature: u32,
    pub version: u32,
    #[nom(Parse = "{ |i| HeaderData::parse(i, version) }")]
    pub data: HeaderData,
}

#[derive(Debug, Nom)]
#[nom(Selector = "u32")]
#[nom(LittleEndian)]
pub enum HeaderData {
    #[nom(Selector = "1")]
    V1 { tree_length: u32 },
    #[nom(Selector = "2")]
    V2 {
        tree_length: u32,
        data_length: u32,
        md5_length: u32,
        external_md5_length: u32,
        signature_length: u32,
    },
}

impl HeaderData {
    pub fn tree_length(&self) -> u32 {
        match self {
            Self::V2 { tree_length, .. } | Self::V1 { tree_length, .. } => *tree_length,
        }
    }
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
    pub payload: Vec<u8>,
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
