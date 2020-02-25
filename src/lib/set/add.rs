use super::super::domain::{Data, Primitive};
use super::super::errors::ApplicationError;
use std::collections::{HashMap, HashSet};

fn insert_values(set: &mut HashSet<Primitive>, values: Vec<Primitive>) -> usize {
    values
        .into_iter()
        .fold(0, |acc, val| (if set.insert(val) { acc + 1 } else { acc }))
}

pub fn command(
    store: &mut HashMap<String, Data>,
    key: &str,
    values: Vec<Primitive>,
) -> Result<usize, ApplicationError> {
    match store.get_mut(key) {
        Some(Data::Set(s)) => Ok(insert_values(s, values)),
        None => {
            let mut s = HashSet::new();
            let added = insert_values(&mut s, values);
            store.insert(key.to_string(), Data::Set(s));
            Ok(added)
        }
        Some(non_set_val) => Err(format!("{} is not a set", non_set_val).into()),
    }
}

#[cfg(test)]
mod test {
    use super::{command, Data, HashMap, HashSet, Primitive};
    use proptest::collection::vec;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn returns_n_added(key in "\".*\"", values in vec(any::<Primitive>(), 1..100)) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let size = values.iter().collect::<HashSet<&Primitive>>().drain().count();

            assert_eq!(command(&mut store, &key, values)?, size)
        }
    }
}
