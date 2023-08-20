mod error;

pub use error::*;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::char,
    combinator::{map, map_res, recognize},
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

#[derive(Debug)]
pub enum Header {
    V1 {
        signature: u32,
        version: u32,
        tree_length: u32,
    },
    V2 {
        signature: u32,
        version: u32,
        tree_length: u32,
        data_length: u32,
        md5_length: u32,
        external_md5_length: u32,
        signature_length: u32,
    },
}

impl Header {
    pub fn tree_length(&self) -> u32 {
        let test = &[0u8];
        match self {
            Self::V2 { tree_length, .. } | Self::V1 { tree_length, .. } => *tree_length,
        }
    }
}

impl Header {
    pub const V1_LENGTH: usize = 4 * 3;
    pub const V2_LENGTH: usize = 4 * 7;
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

pub fn header(input: &[u8]) -> IResult<&[u8], Header, Error<&[u8]>> {
    let (input, (signature, version, tree_length)) = tuple((le_u32, le_u32, le_u32))(input)?;

    // check vpk signature is valid
    if signature != Vpk::SIGNATURE {
        return Err(nom::Err::Error(Error::InvalidSignature(signature)));
    }

    match version {
        1 => Ok((
            input,
            Header::V1 {
                signature,
                version,
                tree_length,
            },
        )),
        2 => map(
            tuple((le_u32, le_u32, le_u32, le_u32)),
            |(data_length, md5_length, external_md5_length, signature_length)| Header::V2 {
                signature,
                version,
                tree_length,
                data_length,
                md5_length,
                external_md5_length,
                signature_length,
            },
        )(input),
        _ => Err(nom::Err::Error(Error::UnsupportedVersion(version))),
    }
}

pub fn cstring(input: &[u8]) -> IResult<&[u8], String, Error<&[u8]>> {
    let (input, (bytes, _)) = many_till(le_u8, tag(&[0]))(input)?;

    match String::from_utf8(bytes) {
        Ok(s) => Ok((input, s)),
        Err(_) => Err(nom::Err::Error(Error::Parse(nom::error::Error {
            input,
            code: nom::error::ErrorKind::IsNot,
        }))),
    }
}

pub fn tree(mut input: &[u8]) -> IResult<&[u8], (), Error<&[u8]>> {
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

                let (i, entry) =
                    DirectoryEntry::parse(input).map_err(|e| e.map(|e| Error::Parse(e)))?;
                input = i;

                // println!("{path} {entry:?}");
                // break;
            }
        }
    }

    Ok((input, ()))
}
