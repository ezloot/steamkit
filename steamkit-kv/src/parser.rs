use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::complete::{char, line_ending, none_of, not_line_ending, one_of, space0, space1},
    combinator::{cut, map, opt, recognize, value},
    complete::take,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: String,
    pub value: Value,
    pub macro_: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Map(Vec<Entry>),
}

fn comment(input: &str) -> IResult<&str, &str> {
    recognize(pair(char('/'), not_line_ending))(input)
}

fn unquoted_string(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of("\"{}\n\r\t ")))(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(
            char('"'),
            escaped(none_of("\"\\"), '\\', one_of("\"nt\\")),
            char('"'),
        ),
        value("", tag("\"\"")),
    ))(input)
}

fn parse_macro(input: &str) -> IResult<&str, String> {
    map(
        delimited(char('['), many1(none_of("[]\n")), char(']')),
        |macro_| macro_.into_iter().collect::<String>(),
    )(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    alt((quoted_string, unquoted_string))(input)
}

fn key_value_string(input: &str) -> IResult<&str, Entry> {
    map(
        tuple((
            string,
            preceded(space1, string),
            opt(preceded(space1, parse_macro)),
        )),
        |(key, value, macro_, ..)| Entry {
            key: key.to_owned(),
            value: Value::String(
                value
                    .to_owned()
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\"),
            ),
            macro_,
        },
    )(input)
}

fn whitespace_newline(input: &str) -> IResult<&str, &str> {
    recognize(tuple((space0, opt(comment), line_ending, space0)))(input)
}

fn key_value_map(input: &str) -> IResult<&str, Entry> {
    map(
        tuple((
            string,
            opt(preceded(space1, parse_macro)),
            tuple((whitespace_newline, char('{'), whitespace_newline)),
            key_value_map_body,
            tuple((many0(whitespace_newline), char('}'))),
        )),
        |(key, macro_, _, map, _)| Entry {
            key: key.to_owned(),
            value: Value::Map(map),
            macro_,
        },
    )(input)
}

fn key_value_map_body(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list0(many1(whitespace_newline), key_value)(input)
}

pub fn key_value(input: &str) -> IResult<&str, Entry> {
    alt((key_value_map, key_value_string))(input)
}
