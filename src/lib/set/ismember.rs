use super::super::domain::{Data, Primitive};
use super::super::errors::ApplicationError;
use std::collections::HashMap;

pub fn command(
    store: &mut HashMap<String, Data>,
    key: &str,
    member: &Primitive,
) -> Result<bool, ApplicationError> {
    match store.get_mut(key) {
        Some(Data::Set(s)) => Ok(s.contains(member)),
        None => Err(format!("No value at key {}", key).into()),
        Some(non_set_val) => Err(format!("{} is not a set", non_set_val).into()),
    }
}
