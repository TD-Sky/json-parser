use nom::IResult;
use std::fmt::Debug;
use tracing::debug;

pub(crate) fn string_debug<O: Debug>(func: &'static str, result: &IResult<&str, O>) {
    match result {
        Ok((_, s)) => debug!("[jsonom::parser::string::{func}] {s:?}"),
        Err(nom::Err::Incomplete(e)) => debug!("[{func}] Incomplete {e:?}"),
        _ => {}
    }
}
