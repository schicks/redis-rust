use super::domain::Data;
use super::errors::{ApplicationError, Fallible};
use std::collections::HashMap;

pub fn command(store: &mut HashMap<String, Data>, key: &str) -> Result<usize, ApplicationError> {
  match store.get(key).fail_to(&format!("No value at {}", key))? {
    Data::Set(s) => Ok(s.len()),
    _ => Err(format!("Value at {} is not a set", key).into()),
  }
}

#[cfg(test)]
mod test {
  use super::super::domain::Primitive;
  use super::{command, Data, HashMap};
  use proptest::collection::hash_set;
  use proptest::prelude::*;

  proptest! {
      #[test]
      fn returns_set_size(key in "\".*\"", values in hash_set(any::<Primitive>(), 100)) {
          let mut store: HashMap<String, Data> = HashMap::new();
          let size = values.len();
          store.insert(key.clone(), values.into());

          assert_eq!(command(&mut store, &key)?, size)
      }
  }
}
