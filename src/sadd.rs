use crate::domain::{Data, Primitive};
use crate::errors::ApplicationError;
use std::collections::{HashMap, HashSet};

pub fn command(
    store: &mut HashMap<String, Data>,
    key: String,
    value: Primitive,
) -> Result<usize, ApplicationError> {
    match store.get_mut(&key) {
        Some(Data::Set(s)) => {
            s.insert(value);
            Ok(s.len())
        }
        None => {
            let mut s = HashSet::new();
            s.insert(value);
            store.insert(key, Data::Set(s));
            Ok(1)
        }
        Some(non_set_val) => Err(format!("{} is not a set", non_set_val).into())
    }
}
