use super::domain::{Data, Primitive};
use super::errors::ApplicationError;
use std::collections::HashMap;

pub fn command(store: &mut HashMap<String, Data>, key: &str) -> Result<i64, ApplicationError> {
    match store.get(key) {
        None => {
            store.insert(key.to_string(), 1.into());
            Ok(1)
        }
        Some(Data::Primitive(Primitive::Number(n))) => {
            let after = n + 1;
            store.insert(key.to_string(), after.into());
            Ok(after)
        }
        Some(_) => Err(format!("The value at {} is not a number.", key).into()),
    }
}

#[cfg(test)]
mod test {
    use super::{command, Data, HashMap};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn incr_empty_key_is_1(key in "\".*\"") {
            let mut store: HashMap<String, Data> = HashMap::new();
            assert_eq!(command(&mut store, &key)?, 1)
        }
    }

    proptest! {
        #[test]
        fn incr_n_times_is_n(key in "\".*\"", n in 1i64..10) {
            let mut store: HashMap<String, Data> = HashMap::new();
            for _ in 1..n {
                command(&mut store, &key)?;
            }
            assert_eq!(command(&mut store, &key)?, n)
        }
    }
}
