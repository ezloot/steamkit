use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take},
    combinator::{complete, map},
    error::ParseError,
    multi::many_till,
    number::complete::le_u8,
    sequence::pair,
    IResult, InputIter, InputTake, Parser,
};
use nom_derive::{Nom, Parse};

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct Vpk {
    pub header: Header,
    #[nom(Parse = "limit(complete(tree), header.tree_length as usize)")]
    pub tree: HashMap<String, Entry>,
    #[nom(
        Cond = "header.version == 2 && header.v2.as_ref().unwrap().archive_md5_length > 0",
        SkipBefore = "header.v2.as_ref().unwrap().data_length",
        Count = "(header.v2.as_ref().unwrap().archive_md5_length / 28) as usize"
    )]
    pub archive_md5s: Option<Vec<ArchiveMD5Entry>>,
    #[nom(Cond = "header.version == 2 && header.v2.as_ref().unwrap().local_md5_length > 0")]
    pub local_md5: Option<LocalMD5>,
    #[nom(Cond = "header.version == 2 && header.v2.as_ref().unwrap().signature_length > 0")]
    pub signature: Option<Signature>,
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
    #[nom(Verify = "*archive_md5_length % 28 == 0")]
    pub archive_md5_length: u32,
    #[nom(Verify = "*local_md5_length == 48")]
    pub local_md5_length: u32,
    pub signature_length: u32,
}

#[derive(Debug, Clone, Nom)]
#[nom(LittleEndian)]
pub struct ArchiveMD5Entry {
    pub archive_index: u32,
    pub offset: u32,
    pub length: u32,
    pub checksum: [u8; 16],
}

#[derive(Debug, Clone, Nom)]
#[nom(LittleEndian)]
pub struct LocalMD5 {
    pub tree_checksum: [u8; 16],
    pub archive_md5_checksum: [u8; 16],
    pub unknown_checksum: [u8; 16],
}

#[derive(Debug, Nom)]
#[nom(LittleEndian)]
pub struct Signature {
    pub public_key_length: u32,
    #[nom(Count = "public_key_length")]
    pub public_key: Vec<u8>,
    pub signature_length: u32,
    #[nom(Count = "signature_length")]
    pub signature: Vec<u8>,
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
    #[nom(Count = "preload_length")]
    pub preload: Vec<u8>,
}

#[derive(Debug)]
pub struct Entry {
    pub path: String,
    pub dir_entry: DirectoryEntry,
}

fn cstring(input: &[u8]) -> IResult<&[u8], String> {
    let (input, (bytes, _)) = many_till(le_u8, tag(&[0]))(input)?;

    match String::from_utf8(bytes) {
        Ok(s) => Ok((input, s)),
        Err(_) => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::IsNot,
        })),
    }
}

fn limit<I: Clone, O, E: ParseError<I>, F>(
    mut parser: F,
    max_length: usize,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputIter + InputTake,
    F: Parser<I, O, E>,
{
    let mut limit = take(max_length);
    move |input: I| {
        let (input, limited_input) = limit.parse(input)?;
        parser.parse(limited_input).map(|(_, out)| (input, out))
    }
}

fn tree(input: &[u8]) -> IResult<&[u8], HashMap<String, Entry>> {
    map(
        many_till(
            pair(
                cstring,
                many_till(
                    pair(
                        cstring,
                        many_till(pair(cstring, DirectoryEntry::parse), tag(&[0u8])),
                    ),
                    tag(&[0u8]),
                ),
            ),
            tag(&[0u8]),
        ),
        |(entries, _)| {
            let mut m = HashMap::new();
            for (ext, (v, _)) in entries {
                for (dir, (v, _)) in v {
                    for (name, dir_entry) in v {
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

                        m.insert(path.clone(), Entry { path, dir_entry });
                    }
                }
            }
            m
        },
    )(input)
}
