use super::super::domain::{Data, Primitive};
use super::super::errors::ApplicationError;
use std::collections::{HashMap, HashSet};

pub fn get_set<'a>(
    store: &'a HashMap<String, Data>,
    key: &str,
) -> Result<&'a HashSet<Primitive>, ApplicationError> {
    match store.get(key) {
        Some(Data::Set(set)) => Ok(set),
        Some(_) => Err(format!("Value at {} is not a set", key).into()),
        None => Err(format!("No value at key {}", key).into()),
    }
}

pub fn get_sets<'a>(
    store: &'a HashMap<String, Data>,
    keys: &[String],
) -> Result<Vec<&'a HashSet<Primitive>>, ApplicationError> {
    keys.iter().map(|key| get_set(&store, key)).fold(
        Ok(Vec::with_capacity(keys.len())),
        |acc, next| match (acc, next) {
            (Err(e), _) | (_, Err(e)) => Err(e),
            (Ok(mut acc), Ok(a)) => {
                acc.push(a);
                Ok(acc)
            }
        },
    )
}
