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
