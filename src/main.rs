use crate::parser::DefaultTypeParser;
use crate::parser::Parser;

mod parser;

fn main() {
    let parser = DefaultTypeParser::create();
    let json_string = r#"
    [
        {
            "name": "ala",
            "age": 32,
            "animals": ["cat", "dog"],
            "friends": true
        },
        ["bela", null, 3222],
        null
    ]
    "#;
    let (_, json) = parser.parse(json_string).unwrap();

    println!("{:#?}", json);

    println!("{:?}", str::parse::<f32>("9.7e-7"))
}
