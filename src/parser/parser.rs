use crate::parser::jtype::JType;
use crate::parser::type_parser::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};

use nom::combinator::map;

use crate::parser::utils::ws;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use nom_regex::str::re_find;
use regex::Regex;

pub trait Parser: JTypeParser {
    fn parse_jobject<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JO<Self::JS, Self::JT>> {
        ws(delimited(
            tag("{"),
            ws(map(
                separated_list0(
                    ws(tag(",")),
                    separated_pair(
                        |s| self.parse_jstring(s),
                        ws(tag(":")),
                        |s| self.parse_jtype(s),
                    ),
                ),
                |obj| self.get_object(obj),
            )),
            tag("}"),
        ))(input)
    }

    fn parse_jarray<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JA<Self::JT>> {
        map(
            ws(delimited(
                tag("["),
                separated_list0(tag(","), move |t| self.parse_jtype(t)),
                tag("]"),
            )),
            |arr| self.get_array(arr),
        )(input)
    }

    fn parse_jstring<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JS> {
        map(
            ws(delimited(tag("\""), take_until("\""), tag("\""))),
            |str| self.get_string(str),
        )(input)
    }

    fn parse_jnumber<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JN> {
        let pattern = Regex::new(r"^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?").unwrap();
        map(ws(re_find(pattern)), |num| self.get_number(num))(input)
    }

    fn parse_jboolean<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JB> {
        map(
            ws(alt((
                map(tag("true"), |_| true),
                map(tag("false"), |_| false),
            ))),
            |boo| self.get_boolean(boo),
        )(input)
    }

    fn parse_jnull<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JZ> {
        ws(map(tag("null"), |_| self.get_null()))(input)
    }

    fn parse_jtype<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JT> {
        map(
            ws(alt((
                map(|obj| self.parse_jobject(obj), JType::Object),
                map(|arr| self.parse_jarray(arr), JType::Array),
                map(|str| self.parse_jstring(str), JType::String),
                map(|num| self.parse_jnumber(num), JType::Number),
                map(|boo| self.parse_jboolean(boo), JType::Boolean),
                map(|z| self.parse_jnull(z), JType::Null),
            ))),
            |jt| self.get_jtype(jt),
        )(input)
    }

    fn parse<'a>(&'a self, input: &'a str) -> IResult<&str, Self::JT> {
        self.parse_jtype(input)
    }
}
