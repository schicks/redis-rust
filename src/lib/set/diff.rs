use super::super::domain::{Data, Primitive};
use super::super::errors::{ApplicationError, Fallible, Flattenable};
use std::collections::{HashMap, HashSet};

fn get_set<'a>(
  store: &'a HashMap<String, Data>,
  key: &str,
) -> Result<&'a HashSet<Primitive>, ApplicationError> {
  match store.get(key) {
    Some(Data::Set(set)) => Ok(set),
    Some(_) => Err(format!("Value at {} is not a set", key).into()),
    None => Err(format!("No value at key {}", key).into()),
  }
}

pub fn command(
  store: &mut HashMap<String, Data>,
  destination: &str,
  base_key: &str,
  keys: &[String],
) -> Result<usize, ApplicationError> {
  let mut result: HashSet<Primitive> = get_set(store, base_key)?.iter().cloned().collect();
  let mut iterated = keys.iter().map(|key| get_set(&store, key));
  let first = iterated.next().fail_to("No sets to difference").flatten()?;
  iterated
    .fold(Ok(vec![first]), |acc, next| match (acc, next) {
      (Err(e), _) | (_, Err(e)) => Err(e),
      (Ok(mut acc), Ok(a)) => {
        acc.push(a);
        Ok(acc)
      }
    })?
    .iter()
    .for_each(|set| {
      set.iter().for_each(|el| {
        result.remove(el);
      })
    });
  let size = result.len();
  store.insert(destination.to_string(), result.into());
  Ok(size)
}

#[cfg(test)]
mod test {
  use super::super::super::domain::Primitive;
  use super::{command, Data};
  use proptest::collection::hash_set;
  use proptest::prelude::*;
  use std::collections::HashMap;

  proptest! {
      #[test]
      fn difference_is_idempotent(
        dest in any::<String>(),
        snd_dest in any::<String>(),
        a in (any::<String>(), hash_set(any::<Primitive>(), 100)),
        b in (any::<String>(), hash_set(any::<Primitive>(), 100))
      ) {
          let mut store: HashMap<String, Data> = HashMap::new();
          let (a_key, a_set) = a;
          let (b_key, b_set) = b;
          store.insert(a_key.clone(), a_set.into());
          store.insert(b_key.clone(), b_set.into());
          command(&mut store, &dest, &a_key, &vec![b_key.clone()][..])?;
          command(&mut store, &snd_dest, &dest, &vec![b_key][..])?;
          assert_eq!(store.get(&dest), store.get(&snd_dest))
      }
  }

  proptest! {
      #[test]
      fn self_difference_is_empty(
        dest in any::<String>(),
        a in (any::<String>(), hash_set(any::<Primitive>(), 100))
      ) {
          let mut store: HashMap<String, Data> = HashMap::new();
          let (a_key, a_set) = a;
          store.insert(a_key.clone(), a_set.into());
          command(&mut store, &dest, &a_key.clone(), &vec![a_key][..])?;
          let result = store.get(&dest);
          if let Some(Data::Set(set)) = result {assert_eq!(set.len(), 0)} else { panic!("something odd found at key")}
      }
  }

  proptest! {
      #[test]
      fn returns_size_of_dest(
        dest in any::<String>(),
        a in (any::<String>(), hash_set(any::<Primitive>(), 100)),
        b in (any::<String>(), hash_set(any::<Primitive>(), 100))
      ) {
          let mut store: HashMap<String, Data> = HashMap::new();
          let (a_key, a_set) = a;
          let (b_key, b_set) = b;
          store.insert(a_key.clone(), a_set.into());
          store.insert(b_key.clone(), b_set.into());
          let resulting_size = command(&mut store, &dest, &a_key, &vec![b_key][..])?;
          let resulting_set = store.get(&dest);
          if let Some(Data::Set(set)) = resulting_set {assert_eq!(set.len(), resulting_size)} else { panic!("something odd found at key")}
      }
  }
}
