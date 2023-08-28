mod error;

pub use error::*;
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::escaped,
    character::complete::{char, none_of, not_line_ending, one_of, space0, space1},
    combinator::{cut, map, opt, recognize},
    multi::many1,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
    pub macro_: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Map(IndexMap<String, KeyValue>),
}

// impl ops::Add for KeyValue {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {

//     }
// }

fn comment(input: &str) -> IResult<&str, &str> {
    recognize(pair(char('/'), not_line_ending))(input)
}

fn unquoted_string(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of("\"{}\n\r\t ")))(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
    preceded(
        char('"'),
        cut(terminated(
            escaped(none_of("\"\\"), '\\', one_of("\"nt\\")),
            char('"'),
        )),
    )(input)
}

fn parse_macro(input: &str) -> IResult<&str, &str> {
    recognize(delimited(char('['), many1(none_of("[]\n")), char(']')))(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    alt((quoted_string, unquoted_string))(input)
}

fn key_value_string(input: &str) -> IResult<&str, KeyValue> {
    map(
        tuple((
            preceded(space0, string),
            preceded(space1, string),
            preceded(space1, opt(parse_macro)),
            preceded(space0, opt(comment)),
        )),
        |(key, value, macro_, ..)| KeyValue {
            key: key.to_owned(),
            value: Value::String(
                value
                    .to_owned()
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\"),
            ),
            macro_: macro_.map(|s| s.to_owned()),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::*;

    #[test]
    fn test_me() {
        let mut file = File::open("assets/items_game.txt").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        if let Value::String(s) =
            key_value_string("\"Hello World\" \"\\\"\\tHello World \\\\\"  [$123]   // hello there!")
                .unwrap()
                .1
                .value
        {
            println!("{s}");
        }
    }
}
