use super::array;
use super::boolean;
use super::integer;
use super::null;
use crate::Value;

#[test]
fn parse_null() {
    assert_eq!(null("null"), Ok(("", ())));
}

#[test]
fn parse_boolean() {
    assert_eq!(boolean("true"), Ok(("", true)));
    assert_eq!(boolean("false"), Ok(("", false)));
}

#[test]
fn parse_number() {
    assert_eq!(integer("114514"), Ok(("", 114514)));
    assert_eq!(integer("+114514"), Ok(("", 114514)));
    assert_eq!(integer("-114514"), Ok(("", -114514)));
}

#[test]
fn parse_array() {
    let v = vec![
        Value::Null,
        14.into(),
        true.into(),
        false.into(),
        vec![1.into(), 2.into(), 3.into()].into(),
    ];
    assert_eq!(
        array(" [ null , 14 , true , false , [1 , 2 , 3] ] "),
        Ok((" ", v))
    );
}
