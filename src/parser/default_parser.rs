use crate::parser::jtype::JType;
use crate::parser::parser::Parser;
use crate::parser::type_parser::JTypeParser;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct DefaultTypeParser {}

impl DefaultTypeParser {
    pub fn create() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DefaultJType {
    pub t: JType<HashMap<String, DefaultJType>, Vec<DefaultJType>, String, Decimal, bool, ()>,
}

impl JTypeParser for DefaultTypeParser {
    type JO<JS, JT> = HashMap<String, Self::JT>;
    type JA<JT> = Vec<Self::JT>;
    type JS = String;
    type JN = Decimal;
    type JB = bool;
    type JZ = ();
    type JT = DefaultJType;

    fn get_object(&self, obj: Vec<(Self::JS, Self::JT)>) -> Self::JO<Self::JS, Self::JT> {
        obj.into_iter().collect()
    }

    fn get_array(&self, arr: Vec<Self::JT>) -> Self::JA<Self::JT> {
        arr
    }

    fn get_string(&self, str: &str) -> Self::JS {
        str.to_string()
    }

    fn get_number(&self, num: &str) -> Self::JN {
        if let Ok(dec) = Decimal::from_str_exact(num) {
            return dec;
        }
        if let Ok(dec) = Decimal::from_scientific(num) {
            return dec;
        }
        panic!("Invalid string: {}", num)
    }

    fn get_boolean(&self, boo: bool) -> Self::JB {
        boo
    }

    fn get_null(&self) -> Self::JZ {}
    fn get_jtype(
        &self,
        jt: JType<
            Self::JO<Self::JS, Self::JT>,
            Self::JA<Self::JT>,
            Self::JS,
            Self::JN,
            Self::JB,
            Self::JZ,
        >,
    ) -> Self::JT {
        DefaultJType { t: jt }
    }
}

impl Parser for DefaultTypeParser {}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use nom::IResult;
    use rstest::*;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::collections::HashMap;

    #[fixture]
    fn default_parser() -> DefaultTypeParser {
        DefaultTypeParser {}
    }

    fn ok_factory(
        t: JType<HashMap<String, DefaultJType>, Vec<DefaultJType>, String, Decimal, bool, ()>,
    ) -> IResult<&'static str, DefaultJType> {
        Ok(("", DefaultJType { t }))
    }

    #[rstest]
    fn test_null(default_parser: DefaultTypeParser) {
        // given
        let null_string = "null";

        // when
        let json_null = default_parser.parse(null_string);

        // then
        assert_eq!(json_null, ok_factory(JType::Null(())));
    }

    #[rstest]
    #[case::val_true("true", true)]
    #[case::val_false("false", false)]
    fn test_boolean(#[case] input: &str, #[case] output: bool, default_parser: DefaultTypeParser) {
        // when
        let json_boolean = default_parser.parse(input);

        // then
        assert_eq!(json_boolean, ok_factory(JType::Boolean(output)))
    }

    #[rstest]
    #[case::zero("0.0", dec!(0.0))]
    #[case::positive("12.34", dec!(12.34))]
    #[case::negative("-55.66", dec!(-55.66))]
    #[case::scientific("9.7e-7", dec!(9.7e-7))]
    fn test_number(
        #[case] input: &str,
        #[case] output: Decimal,
        default_parser: DefaultTypeParser,
    ) {
        // when
        let json_number = default_parser.parse(input);

        // then
        assert_eq!(json_number, ok_factory(JType::Number(output)))
    }

    #[rstest]
    #[case(r#""ala ma kota..." "#, "ala ma kota...")]
    #[case(r#""kot ma na imię 'Filomen'""#, "kot ma na imię 'Filomen'")]
    #[case(r#""a na nazwisko \"Musk\"""#, r#"a na nazwisko \"Musk\""#)]
    fn test_string(#[case] input: &str, #[case] output: String, default_parser: DefaultTypeParser) {
        // when
        let json_string = default_parser.parse(input);

        // then
        assert_eq!(json_string, ok_factory(JType::String(output)))
    }

    #[rstest]
    fn test_array(default_parser: DefaultTypeParser) {
        // given
        let input = r#"
        [
            "ala",
            32,
            true,
            null,
            {}
        ]
        "#;
        let output = Vec::from([
            DefaultJType {
                t: JType::String("ala".to_string()),
            },
            DefaultJType {
                t: JType::Number(dec!(32)),
            },
            DefaultJType {
                t: JType::Boolean(true),
            },
            DefaultJType { t: JType::Null(()) },
            DefaultJType {
                t: JType::Object(HashMap::new()),
            },
        ]);

        // when
        let json_array = default_parser.parse(input);

        // then
        assert_eq!(json_array, ok_factory(JType::Array(output)))
    }

    #[rstest]
    fn test_object(default_parser: DefaultTypeParser) {
        // given
        let input = r#"
        {
            "name": "ala",
            "age": 32,
            "verified": true,
            "friends": null,
            "animals": [
                "cat",
                "dog"
            ]
        }
        "#;
        let output = HashMap::from([
            (
                "name".to_string(),
                DefaultJType {
                    t: JType::String("ala".to_string()),
                },
            ),
            (
                "age".to_string(),
                DefaultJType {
                    t: JType::Number(Decimal::from(32)),
                },
            ),
            (
                "verified".to_string(),
                DefaultJType {
                    t: JType::Boolean(true),
                },
            ),
            ("friends".to_string(), DefaultJType { t: JType::Null(()) }),
            (
                "animals".to_string(),
                DefaultJType {
                    t: JType::Array(Vec::from([
                        DefaultJType {
                            t: JType::String("cat".to_string()),
                        },
                        DefaultJType {
                            t: JType::String("dog".to_string()),
                        },
                    ])),
                },
            ),
        ]);

        // when
        let json_object = default_parser.parse(input);

        // then
        assert_eq!(json_object, ok_factory(JType::Object(output)))
    }
}
