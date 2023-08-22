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

use crate::{Error, Result};

#[derive(Debug, Clone, Nom)]
#[nom(LittleEndian)]
pub struct Vpk {
    pub header: Header,
    #[nom(Parse = "limit(complete(tree), header.tree_length as usize)")]
    pub tree: HashMap<String, Entry>,
    #[nom(
        Cond = "header.version == 2 && header.archive_md5_length() > 0",
        SkipBefore = "header.data_length()",
        Count = "header.archive_md5_length() / 28"
    )]
    pub archive_md5s: Option<Vec<ArchiveMD5Entry>>,
    #[nom(Cond = "header.version == 2 && header.local_md5_length() > 0")]
    pub local_md5: Option<LocalMD5>,
    #[nom(Cond = "header.version == 2 && header.signature_length() > 0")]
    pub signature: Option<Signature>,
}

impl Vpk {
    pub fn parse(data: &[u8]) -> Result<Self> {
        let (_, vpk) = Parse::parse(data).map_err(|_| Error::Parse)?;
        Ok(vpk)
    }

    pub fn lookup(&self, path: &str) -> Option<&Entry> {
        self.tree.get(path)
    }
}

#[derive(Debug, Clone, Nom)]
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
    pub const V1_LENGTH: u64 = 12;
    pub const V2_LENGTH: u64 = 28;

    pub(crate) fn data_offset(&self) -> u64 {
        let mut offset = self.tree_length as u64;
        if self.version == 1 {
            offset += Self::V1_LENGTH;
        } else if self.version == 2 {
            offset += Self::V2_LENGTH;
        }
        offset
    }

    pub(crate) fn data_length(&self) -> usize {
        if self.version == 2 {
            if let Some(v2) = &self.v2 {
                return v2.data_length as usize;
            }
        }
        0
    }

    pub(crate) fn archive_md5_length(&self) -> usize {
        if self.version == 2 {
            if let Some(v2) = &self.v2 {
                return v2.archive_md5_length as usize;
            }
        }
        0
    }

    pub(crate) fn local_md5_length(&self) -> usize {
        if self.version == 2 {
            if let Some(v2) = &self.v2 {
                return v2.local_md5_length as usize;
            }
        }
        0
    }

    pub(crate) fn signature_length(&self) -> usize {
        if self.version == 2 {
            if let Some(v2) = &self.v2 {
                return v2.signature_length as usize;
            }
        }
        0
    }
}

#[derive(Debug, Clone, Nom)]
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

#[derive(Debug, Clone, Nom)]
#[nom(LittleEndian)]
pub struct Signature {
    pub public_key_length: u32,
    #[nom(Count = "public_key_length")]
    pub public_key: Vec<u8>,
    pub signature_length: u32,
    #[nom(Count = "signature_length")]
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Nom)]
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

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use nom::IResult;
    use nom_derive::Parse;

    use crate::Vpk;

    fn get_file_buffer(name: &str) -> Vec<u8> {
        let mut file = File::open(format!("assets/{name}_dir.vpk")).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        buf
    }

    #[test]
    fn parse_csgo() {
        let buf = get_file_buffer("csgo");
        let result: IResult<&[u8], Vpk> = Parse::parse(&buf);
        assert!(result.is_ok(), "failed to parse CS:GO pack");
        assert!(
            result.unwrap().0.is_empty(),
            "failed to fully parse CS:GO pack"
        );
    }

    #[test]
    fn parse_portal2() {
        let buf = get_file_buffer("portal2");
        let result: IResult<&[u8], Vpk> = Parse::parse(&buf);
        assert!(result.is_ok(), "failed to parse Portal 2 pack");
        assert!(
            result.unwrap().0.is_empty(),
            "failed to fully parse Portal 2 pack"
        );
    }
}
