use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{
        digit1, hex_digit1, multispace0, multispace1, not_line_ending, one_of, space0, space1,
    },
    combinator::{map, opt, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub enum EnumValue {
    Number(String),
    Hex(String),
    Or(Vec<String>),
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

fn hex(input: &str) -> IResult<&str, &str> {
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

fn enum_variant_value(input: &str) -> IResult<&str, EnumValue> {
    alt((
        map(hex, |hex| EnumValue::Hex(hex.to_owned())),
        map(recognize(tuple((opt(tag("-")), digit1))), |num: &str| {
            EnumValue::Number(num.to_owned())
        }),
        map(
            separated_list1(tuple((multispace0, tag("|"), multispace0)), identifier),
            |list| EnumValue::Or(list.into_iter().map(str::to_owned).collect()),
        ),
    ))(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    map(preceded(tag("//"), not_line_ending), |s: &str| s.trim())(input)
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

pub fn parse_enum(input: &str) -> IResult<&str, Enum> {
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
        many0(preceded(
            multispace0,
            alt((
                map(import, |file| DocumentEntry::Import(file.to_owned())),
                map(parse_enum, |value| DocumentEntry::Enum(value)),
            )),
        )),
        |entries| Document { entries },
    )(input)
}
