use nom::IResult;

use std::io::stdin;

use nom_regex::str::re_capture;
use regex::Regex;

fn parser(input: &str) -> IResult<&str, Vec<&str>> {
    let pattern = Regex::new(r#""([^\\"]*)""#).unwrap();
    re_capture(pattern)(input)
}

fn main() {
    let stdin = stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let (a, b) = parser(input.as_str()).unwrap();
        println!("result: ({}) ({:?})", a, b)
    }
}
