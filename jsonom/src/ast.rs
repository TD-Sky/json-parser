use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    // TODO: support more number types
    Number(i64),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Boolean(bool),
    Null,
}

impl From<i64> for Value {
    #[inline]
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<()> for Value {
    #[inline]
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for Value {
    #[inline]
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for Value {
    #[inline]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<Value>> for Value {
    #[inline]
    fn from(values: Vec<Value>) -> Self {
        Self::Array(values)
    }
}

impl From<HashMap<String, Value>> for Value {
    #[inline]
    fn from(value: HashMap<String, Value>) -> Self {
        Self::Object(value)
    }
}
