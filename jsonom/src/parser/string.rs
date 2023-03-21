//! A string is:
//!
//! - Enclosed by double quotes
//! - Can contain any raw unescaped code point besides \ and "
//! - Matches the following escape sequences: \b, \f, \n, \r, \t, \", \\, \/
//! - Matches code points like Rust: \u{XXXX}, where XXXX can be up to 6
//!   hex characters
//! - An escape followed by whitespace consumes all whitespace between the
//!   escape and the next non-whitespace character

use nom::branch::alt;
use nom::bytes::streaming::{is_not, take_while_m_n};
use nom::character::streaming::{char, multispace1};
use nom::combinator::{map, map_opt, map_res, value, verify};
use nom::error::{context, FromExternalError, ParseError};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::num::ParseIntError;

/// A string fragment contains a fragment of a string being parsed:
///
/// - a non-empty Literal (a series of non-escaped characters)
/// - a single parsed Escaped Character
/// - a block of Escaped Whitespace
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
    EscapedWS,
}

/// Parse a string. Use a loop of parse_fragment and push all of the fragments
/// into an output string.
pub(super) fn string(input: &str) -> IResult<&str, String> {
    let build_string = fold_many0(
        // fold会施用此解析器多轮（成功一次为一轮）
        fragment,
        // Our init value, an empty string
        String::new,
        // Our folding function. For each fragment, append the fragment to the
        // string.
        |mut string, fragment| {
            match fragment {
                StringFragment::Literal(s) => string.push_str(s),
                StringFragment::EscapedChar(c) => string.push(c),
                StringFragment::EscapedWS => {}
            }
            string
        },
    );

    // If `build_string` could accept a raw " character,
    // the closing delimiter " would never match.
    let surround = delimited(char('"'), build_string, char('"'));

    context("string", surround)(input)
}

fn fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    alt((
        map(literal, StringFragment::Literal),
        map(escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, escaped_whitespace),
    ))(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
fn literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let not_quote_slash = is_not(r#""\"#);

    // 若输入满足`F`，则用`G`验证，通过则返回输入，否则返回验证错误；
    // 若输入不满足`F`，则返回`F`的错误。
    verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

/// Parse an escaped character: \n, \t, \r, \u{00AC}, etc.
fn escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    preceded(
        char('\\'),
        alt((
            unicode,
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
        )),
    )(input)
}

/// Parse a unicode sequence, of the form u{XXXX},
/// where XXXX is 1 to 6 hexadecimal numerals.
fn unicode<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, ParseIntError>,
{
    let hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    let delimited_hex = preceded(char('u'), delimited(char('{'), hex, char('}')));

    let u32 = map_res(delimited_hex, move |hex| u32::from_str_radix(hex, 16));

    // because not all u32 values are valid unicode code points,
    // we have to fallibly convert to char with from_u32.
    map_opt(u32, char::from_u32)(input)
}

/// Parse a backslash, followed by any amount of whitespace.
/// This is used to discard any escaped whitespace.
fn escaped_whitespace<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(char('\\'), multispace1)(input)
}

#[cfg(test)]
mod tests {
    use super::string;

    #[test]
    fn parse_string() {
        let input = r#""abc""#;
        let result = string(input);
        let s = "abc".to_owned();
        assert_eq!(result, Ok(("", s)));

        let input = r#""tab:\tafter tab, newline:\nnew line, quote: \", emoji: \u{1F602}, newline:\nescaped whitespace: \    abc""#;
        let result = string(input);
        let s = String::from("tab:\tafter tab, newline:\nnew line, quote: \", emoji: 😂, newline:\nescaped whitespace: abc");
        assert_eq!(result, Ok(("", s)));
    }
}
