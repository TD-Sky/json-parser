pub use nom::error::Error;
pub use nom::Err;

mod ast;
pub use self::ast::Value;

mod parser;

pub fn parse(data: &str) -> Result<Value, Err<Error<&str>>> {
    parser::root(data).map(|(_, v)| v)
}
