use std::collections::HashSet;

#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Primitive {
    String(String),
    Number(i64),
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::String(s) => write!(f, "\"{}\"", s),
            Primitive::Number(n) => write!(f, "{}", n),
        }
    }
}

impl From<String> for Primitive {
    fn from(primitive: String) -> Self {
        Primitive::String(primitive)
    }
}

impl From<i64> for Primitive {
    fn from(primitive: i64) -> Self {
        Primitive::Number(primitive)
    }
}

#[derive(Debug, PartialEq)]
pub enum Data {
    Primitive(Primitive),
    Set(HashSet<Primitive>),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Primitive(p) => p.fmt(f),
            Data::Set(_) => write!(f, "<Set>"),
        }
    }
}

impl From<Primitive> for Data {
    fn from(data: Primitive) -> Self {
        Data::Primitive(data)
    }
}

impl From<String> for Data {
    fn from(data: String) -> Self {
        Data::Primitive(Primitive::String(data))
    }
}

impl From<i64> for Data {
    fn from(data: i64) -> Self {
        Data::Primitive(Primitive::Number(data))
    }
}

impl From<HashSet<Primitive>> for Data {
    fn from(data: HashSet<Primitive>) -> Self {
        Data::Set(data)
    }
}

pub enum Command {
    Echo(String),
    Set(String, Data),
    Get(String),
    Incr(String),
    Sadd(String, Vec<Primitive>),
    Scard(String),
    Sismember(String, Primitive),
    Sdiff(String, Vec<String>),
    SdiffStore(String, String, Vec<String>),
    Sinter(Vec<String>),
    SinterStore(String, Vec<String>),
    Sunion(Vec<String>),
    SunionStore(String, Vec<String>),
}
