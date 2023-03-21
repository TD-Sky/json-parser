use super::utils::char1;
use super::utils::token;
use super::value;
use crate::ast::Value;
use nom::bytes::complete::take_while;
use nom::character::complete::{alpha1, char};
use nom::combinator::{map, recognize};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use std::collections::HashMap;

/// 解析 Object 的键名，规则：
///
/// - 首字符只能是字母；
/// - 其余部分为字母/数字/下划线
fn key(input: &str) -> IResult<&str, String> {
    let identifier = recognize(preceded(
        alpha1,
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ));

    let build_string = map(delimited(char('"'), identifier, char('"')), String::from);

    token(build_string)(input)
}

#[inline]
fn key_value(input: &str) -> IResult<&str, (String, Value)> {
    separated_pair(key, char1(':'), value)(input)
}

pub(super) fn object(input: &str) -> IResult<&str, HashMap<String, Value>> {
    let key_value_pairs = separated_list0(char1(','), key_value);

    let surround = map(
        delimited(char1('{'), key_value_pairs, char1('}')),
        HashMap::from_iter,
    );

    context("object", surround)(input)
}

#[cfg(test)]
mod tests {
    use super::key;
    use super::key_value;
    use super::object;
    use crate::Value;

    #[test]
    fn parse_key() {
        let s = "abc_123".to_owned();
        assert_eq!(key(r#" "abc_123" "#), Ok((" ", s)));

        assert!(key(r#" "123abc" "#).is_err());
    }

    #[test]
    fn parse_key_value_pairs() {
        let entry = (String::from("Senpai"), Value::Number(114514));
        assert_eq!(key_value(r#" "Senpai" : 114514 "#), Ok((" ", entry)));
    }

    #[test]
    fn parse_object() {
        let input = r#"
        {
            "name": "Senpai" ,
            "age": 24 ,
            "job": "Student" ,
            "hobby": 114514
        }
        "#;

        let result = object(input).unwrap().1;

        println!("{result:#?}");
    }
}
