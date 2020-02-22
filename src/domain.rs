use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Primitive {
    String(String),
    Number(i64),
}

pub enum Data {
    Primitive(Primitive),
    Set(HashSet<Primitive>),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Primitive(Primitive::String(s)) => write!(f, "{}", s),
            Data::Primitive(Primitive::Number(n)) => write!(f, "{}", n),
            Data::Set(_) => write!(f, "<Set>"),
        }
        // write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl From<Primitive> for Data {
    fn from(data: Primitive) -> Self {
        Data::Primitive(data)
    }
}

impl From<String> for Data {
    fn from(data: String) -> Self {
        Primitive::String(data).into()
    }
}

impl From<i64> for Data {
    fn from(data: i64) -> Self {
        Primitive::Number(data).into()
    }
}

pub enum Command {
    Echo(String),
    Set(String, Data),
    Get(String),
    Incr(String),
    Sadd(String, Primitive),
}
