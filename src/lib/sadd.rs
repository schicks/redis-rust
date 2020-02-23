use super::domain::{Data, Primitive};
use super::errors::ApplicationError;
use std::collections::{HashMap, HashSet};

pub fn command(
    store: &mut HashMap<String, Data>,
    key: &str,
    value: Primitive,
) -> Result<usize, ApplicationError> {
    match store.get_mut(key) {
        Some(Data::Set(s)) => Ok(if s.insert(value) { 1 } else { 0 }),
        None => {
            let mut s = HashSet::new();
            s.insert(value);
            store.insert(key.to_string(), Data::Set(s));
            Ok(1)
        }
        Some(non_set_val) => Err(format!("{} is not a set", non_set_val).into()),
    }
}

#[cfg(test)]
mod test {
    use super::{command, Data, HashMap};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn returns_n_added(key in "\".*\"", val in "\".*\"") {
            let mut store: HashMap<String, Data> = HashMap::new();
            assert_eq!(command(&mut store, &key, val.into())?, 1)
        }
    }

    proptest! {
        #[test]
        fn sadd_more_than_once_is_0(key in "\".*\"", val in "\".*\"", n in 2i64..10) {
            let mut store: HashMap<String, Data> = HashMap::new();
            for _ in 1..n {
                command(&mut store, &key, val.to_string().into())?;
            }
            assert_eq!(command(&mut store, &key, val.into())?, 0)
        }
    }
}
