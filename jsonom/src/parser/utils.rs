//! 用来组合出更简短的解析器

use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// 省略前缀空白符，使用解析器`f`解析token
#[inline]
pub(crate) fn token<'a, O, E, G>(f: G) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    G: Parser<&'a str, O, E>,
{
    preceded(multispace0, f)
}

/// 省略单个符号的前缀空白符
#[inline]
pub(crate) fn char1<'a, E>(c: char) -> impl FnMut(&'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str>,
{
    token(char(c))
}
