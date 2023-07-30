// TODO: https://tfpk.github.io/nominomicon/

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alphanumeric1, anychar, digit1, hex_digit1, multispace0, multispace1, newline,
        not_line_ending, one_of,
    },
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1, many_till},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum EnumValue {
    Number(i32),
    Hex(String),
    Expression(String),
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub removed: bool,
    pub obsolete: bool,
    pub reason: Option<String>,
    pub comment: Option<String>,
    pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub generic: Option<String>,
    pub flags: bool,
    pub variants: Vec<EnumVariant>,
}

pub fn hex(input: &str) -> IResult<&str, &str> {
    preceded(tag("0x"), recognize(many1(hex_digit1)))(input)
}

fn parse_ref(input: &str) -> IResult<&str, &str> {
    recognize(tuple((identifier, alt((tag("::"), tag("."))), identifier)))(input)
}

fn generic(input: &str) -> IResult<&str, &str> {
    delimited(tag("<"), alt((identifier, parse_ref)), tag(">"))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of(
        "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
    )))(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of(
        "#+-.!;:'/\\@:_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\t ",
    )))(input)
}

fn enum_variant_value(input: &str) -> IResult<&str, EnumValue> {
    alt((
        map(hex, |hex| EnumValue::Hex(hex.to_owned())),
        map(
            map_res(tuple((opt(tag("-")), digit1)), |(sign, digits)| {
                let sign = sign.unwrap_or("");
                format!("{sign}{digits}").parse()
            }),
            |num| EnumValue::Number(num),
        ),
    ))(input)
}

fn enum_variant(input: &str) -> IResult<&str, EnumVariant> {
    map(
        tuple((
            opt(tuple((multispace0, tag("//"), comment, newline))),
            preceded(multispace0, identifier),
            preceded(multispace1, tag("=")),
            preceded(multispace1, enum_variant_value),
            tag(";"),
            opt(tuple((
                preceded(multispace1, alt((tag("removed"), tag("obsolete")))),
                opt(preceded(
                    multispace1,
                    delimited(tag("\""), comment, tag("\"")),
                )),
            ))),
            opt(preceded(
                tuple((multispace0, tag("//"), multispace0)),
                not_line_ending,
            )),
            multispace0,
        )),
        |(_, name, _, value, _, extra, comment, _)| {
            let (status, reason) = extra.unwrap_or(("", None));
            EnumVariant {
                name: name.to_owned(),
                obsolete: status == "obsolete",
                removed: status == "removed",
                reason: reason.map(str::to_string),
                comment: comment.map(str::to_string),
                value,
            }
        },
    )(input)
}

pub fn parse_enum(input: &str) -> IResult<&str, Enum> {
    map(
        tuple((
            preceded(multispace0, tag("enum")),
            preceded(multispace1, identifier),
            opt(generic),
            opt(preceded(multispace1, tuple((tag("flag"), opt(tag("s")))))),
            multispace1,
            delimited(
                tag("{"),
                many0(enum_variant),
                tuple((tag("}"), opt(tag(";")))),
            ),
            multispace0,
        )),
        |(_, name, generic, flags, _, variants, _)| Enum {
            name: name.to_owned(),
            generic: generic.map(str::to_string),
            flags: flags.is_some(),
            variants,
        },
    )(input)
}

fn import(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            preceded(multispace0, tag("#import")),
            preceded(multispace0, delimited(tag("\""), comment, tag("\""))),
            multispace0,
        )),
        |(_, file, _)| file,
    )(input)
}

#[derive(Debug, Clone)]
pub struct Document {
    pub entries: Vec<DocumentEntry>,
}

#[derive(Debug, Clone)]
pub enum DocumentEntry {
    Import(String),
    Enum(Enum),
}

pub fn document(input: &str) -> IResult<&str, Document> {
    map(
        many0(preceded(multispace0, alt((
            map(import, |file| DocumentEntry::Import(file.to_owned())),
            map(parse_enum, |value| DocumentEntry::Enum(value)),
        )))),
        |entries| Document { entries },
    )(input)
}
