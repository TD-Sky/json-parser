mod string;
use self::string::string;

mod object;
use self::object::object;

mod utils;
use self::utils::char1;
use self::utils::token;

#[cfg(test)]
mod tests;

use crate::Value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of};
use nom::combinator;
use nom::combinator::{map, map_res, opt, recognize};
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::IResult;

/// 根元素：表或数组
pub(super) fn root(input: &str) -> IResult<&str, Value> {
    alt((map(object, Value::from), map(array, Value::from)))(input)
}

/// 解析Json的值。
/// 其中，去除基础类型的前缀空白符。
fn value(input: &str) -> IResult<&str, Value> {
    alt((
        map(token(null), Value::from),
        map(token(boolean), Value::from),
        map(token(integer), Value::from),
        map(token(string), Value::from),
        map(array, Value::from),
        map(object, Value::from),
    ))(input)
}

#[inline]
fn null(input: &str) -> IResult<&str, ()> {
    combinator::value((), tag("null"))(input)
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((
        combinator::value(true, tag("true")),
        combinator::value(false, tag("false")),
    ))(input)
}

fn integer(input: &str) -> IResult<&str, i64> {
    let signed_integer = recognize(preceded(opt(one_of("+-")), digit1));

    map_res(signed_integer, str::parse)(input)
}

fn array(input: &str) -> IResult<&str, Vec<Value>> {
    let elements = separated_list0(char1(','), value);

    let surround = delimited(char1('['), elements, char1(']'));

    context("array", surround)(input)
}
