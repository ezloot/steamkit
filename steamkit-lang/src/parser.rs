use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{
        digit1, hex_digit1, multispace0, multispace1, not_line_ending, one_of, space0, space1,
    },
    combinator::{map, opt, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Reference(String),
    FixedLengthArray { type_: Box<Self>, length: usize },
}

impl From<&str> for DataType {
    fn from(s: &str) -> Self {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<type>[^<>]+)<(?<length>\d+)>").unwrap());

        if let Some(captures) = RE.captures(s) {
            let type_ = captures.name("type").unwrap().as_str();
            let length = captures.name("length").unwrap().as_str();

            return Self::FixedLengthArray {
                type_: Box::new(Self::from(type_)),
                length: usize::from_str(length).unwrap(),
            };
        }

        match s {
            "byte" => Self::U8,
            "ushort" => Self::U16,
            "uint" => Self::U32,
            "ulong" => Self::U64,
            "char" => Self::I8,
            "short" => Self::I16,
            "int" => Self::I32,
            "long" => Self::I64,
            "float" => Self::F32,
            "double" => Self::F64,
            _ => Self::Reference(s.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnumVariantValue {
    Number(String),
    Hex(String),
    Union(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub removed: bool,
    pub obsolete: bool,
    pub reason: Option<String>,
    pub comment: Option<String>,
    pub value: EnumVariantValue,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub generic: Option<DataType>,
    pub flags: bool,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct ClassMember {
    pub name: String,
    pub type_: DataType,
    pub modifier: Option<String>,
    pub constant: bool,
    pub value: Option<ClassMemberValue>,
}

#[derive(Debug, Clone)]
pub enum ClassMemberValue {
    Number(String),
    Hex(String),
    Reference(String),
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub generic: Option<DataType>,
    pub removed: bool,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub entries: Vec<DocumentEntry>,
}

#[derive(Debug, Clone)]
pub enum DocumentEntry {
    Import(String),
    Enum(Enum),
    Class(Class),
}

fn hex(input: &str) -> IResult<&str, &str> {
    preceded(tag("0x"), recognize(many1(hex_digit1)))(input)
}

fn parse_ref(input: &str) -> IResult<&str, &str> {
    recognize(tuple((identifier, alt((tag("::"), tag("."))), identifier)))(input)
}

fn generic(input: &str) -> IResult<&str, DataType> {
    map(delimited(tag("<"), is_not("<>\n"), tag(">")), |s| {
        DataType::from(s)
    })(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of(
        "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
    )))(input)
}

fn parse_type(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of(
        "<>:._abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
    )))(input)
}

fn enum_variant_value(input: &str) -> IResult<&str, EnumVariantValue> {
    alt((
        map(hex, |hex| EnumVariantValue::Hex(hex.to_owned())),
        map(recognize(tuple((opt(tag("-")), digit1))), |num: &str| {
            EnumVariantValue::Number(num.to_owned())
        }),
        map(
            separated_list1(tuple((multispace0, tag("|"), multispace0)), identifier),
            |list| EnumVariantValue::Union(list.into_iter().map(str::to_owned).collect()),
        ),
    ))(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    map(
        preceded(pair(space0, tag("//")), not_line_ending),
        |s: &str| s.trim(),
    )(input)
}

fn reason(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), is_not("\"\n"), tag("\""))(input)
}

fn enum_variant(input: &str) -> IResult<&str, EnumVariant> {
    map(
        tuple((
            // ignore full-line comments
            opt(tuple((multispace0, comment))),
            // consume key
            preceded(multispace0, identifier),
            preceded(space1, tag("=")),
            // consume value
            preceded(space1, enum_variant_value),
            preceded(space0, tag(";")),
            opt(tuple((
                preceded(space1, alt((tag("removed"), tag("obsolete")))),
                opt(preceded(space1, reason)),
            ))),
            // handle inline comments
            opt(preceded(space0, comment)),
            multispace0,
        )),
        |(_, name, _, value, _, extra, comment, _)| {
            let (status, reason) = extra.unwrap_or(("", None));
            EnumVariant {
                name: name.to_owned(),
                obsolete: status == "obsolete",
                removed: status == "removed",
                reason: reason.map(str::to_string),
                comment: comment.map(str::trim).map(str::to_string),
                value,
            }
        },
    )(input)
}

fn parse_enum(input: &str) -> IResult<&str, Enum> {
    map(
        tuple((
            preceded(
                preceded(multispace0, opt(tuple((tag("public"), space1)))),
                tag("enum"),
            ),
            preceded(multispace1, identifier),
            opt(generic),
            opt(preceded(multispace1, tuple((tag("flag"), opt(tag("s")))))),
            multispace1,
            delimited(
                tag("{"),
                many0(enum_variant),
                tuple((multispace0, tag("}"), opt(tag(";")))),
            ),
            multispace0,
        )),
        |(_, name, generic, flags, _, variants, _)| Enum {
            name: name.to_owned(),
            generic,
            flags: flags.is_some(),
            variants,
        },
    )(input)
}

fn class_member_value(input: &str) -> IResult<&str, ClassMemberValue> {
    alt((
        map(hex, |hex| ClassMemberValue::Hex(hex.to_owned())),
        map(recognize(tuple((opt(tag("-")), digit1))), |num: &str| {
            ClassMemberValue::Number(num.to_owned())
        }),
        map(parse_ref, |value| {
            ClassMemberValue::Reference(value.to_owned())
        }),
    ))(input)
}

fn class_member(input: &str) -> IResult<&str, ClassMember> {
    map(
        tuple((
            preceded(
                multispace0,
                alt((
                    // const, modifier,
                    map(
                        tuple((
                            tag("const"),
                            preceded(space1, parse_type),
                            preceded(space1, identifier),
                        )),
                        |(_, type_, name)| ClassMember {
                            name: name.to_owned(),
                            type_: DataType::from(type_),
                            modifier: None,
                            constant: true,
                            value: None,
                        },
                    ),
                    map(
                        tuple((
                            parse_type,
                            preceded(space1, parse_type),
                            preceded(space1, identifier),
                        )),
                        |(modifier, type_, name)| ClassMember {
                            name: name.to_owned(),
                            type_: DataType::from(type_),
                            modifier: Some(modifier.to_owned()),
                            constant: false,
                            value: None,
                        },
                    ),
                    map(
                        tuple((parse_type, preceded(space1, identifier))),
                        |(type_, name)| ClassMember {
                            name: name.to_owned(),
                            type_: DataType::from(type_),
                            modifier: None,
                            constant: false,
                            value: None,
                        },
                    ),
                )),
            ),
            opt(preceded(
                tuple((space1, tag("="), space1)),
                class_member_value,
            )),
            preceded(space0, tag(";")),
            multispace0,
        )),
        |(mut class, value, ..)| {
            class.value = value;
            class
        },
    )(input)
}

fn class(input: &str) -> IResult<&str, Class> {
    map(
        tuple((
            preceded(
                preceded(multispace0, opt(tuple((tag("public"), space1)))),
                tag("class"),
            ),
            preceded(multispace1, identifier),
            opt(generic),
            opt(preceded(multispace1, tag("removed"))),
            multispace1,
            delimited(
                tag("{"),
                many0(class_member),
                tuple((multispace0, tag("}"), opt(tag(";")))),
            ),
            multispace0,
        )),
        |(_, name, generic, removed, _, members, _)| Class {
            name: name.to_owned(),
            generic,
            removed: removed.is_some(),
            members,
        },
    )(input)
}

fn import(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            preceded(multispace0, tag("#import")),
            preceded(multispace0, delimited(tag("\""), is_not("\"\n"), tag("\""))),
            multispace0,
        )),
        |(_, file, _)| file,
    )(input)
}

fn document(input: &str) -> IResult<&str, Document> {
    map(
        many0(preceded(
            multispace0,
            alt((
                map(import, |file| DocumentEntry::Import(file.to_owned())),
                map(parse_enum, |enum_| DocumentEntry::Enum(enum_)),
                map(class, |class| DocumentEntry::Class(class)),
            )),
        )),
        |entries| Document { entries },
    )(input)
}

pub fn parse(input: &str) -> anyhow::Result<Document> {
    let (_, document) = document(input).map_err(|_| anyhow::anyhow!("failed to parse"))?;
    Ok(document)
}
