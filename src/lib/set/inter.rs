use super::super::domain::{Data, Primitive};
use super::super::errors::ApplicationError;
use super::utilities::get_sets;
use std::collections::{HashMap, HashSet};

pub fn command(
    store: &mut HashMap<String, Data>,
    keys: &[String],
) -> Result<HashSet<Primitive>, ApplicationError> {
    if keys.len() < 2 {
        return Err(String::from("Not enough sets to intersect").into());
    };
    let sets = get_sets(store, keys)?;
    let minimal = if let Some(set) = sets.iter().min_by_key(|set| set.len()) {
        set
    } else {
        return Err(String::from("Not enough sets to intersect").into());
    };
    let result: HashSet<Primitive> = minimal
        .iter()
        .filter(|el| sets.iter().all(|set| set.contains(el)))
        .cloned()
        .collect();
    Ok(result)
}

pub fn store_command(
    store: &mut HashMap<String, Data>,
    destination: &str,
    keys: &[String],
) -> Result<usize, ApplicationError> {
    let result: HashSet<Primitive> = command(store, keys)?;
    let size = result.len();
    store.insert(destination.to_string(), result.into());
    Ok(size)
}

#[cfg(test)]
mod test {
    use super::super::super::domain::Primitive;
    use super::{command, store_command, Data};
    use proptest::collection::hash_set;
    use proptest::prelude::*;
    use proptest::string::{string_regex, RegexGeneratorStrategy};
    use std::collections::{HashMap, HashSet};

    fn valid_keys() -> RegexGeneratorStrategy<String> {
        match string_regex("[^\\s]+") {
            Ok(s) => s,
            Err(_) => panic!("strategy failed"),
        }
    }

    proptest! {
        #[test]
        fn intersection_is_idempotent(
            dest in valid_keys(),
            snd_dest in valid_keys(),
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100)),
            b in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let (b_key, b_set) = b;
            store.insert(a_key.clone(), a_set.into());
            store.insert(b_key.clone(), b_set.into());
            store_command(&mut store, &dest, &vec![a_key, b_key.clone()][..])?;
            store_command(&mut store, &snd_dest, &vec![dest.clone(), b_key][..])?;
            assert_eq!(store.get(&dest), store.get(&snd_dest))
        }
    }

    proptest! {
        #[test]
        fn disjoint_intersection_is_empty(
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100)),
            b in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let (b_key, b_set) = b;
            store.insert(a_key.clone(), a_set.difference(&b_set).cloned().collect::<HashSet<Primitive>>().into());
            store.insert(b_key.clone(), b_set.into());
            let set = command(&mut store, &vec![a_key, b_key.clone()][..])?;
            assert_eq!(set.len(), 0)
        }
    }

    proptest! {
        #[test]
        fn intersection_has_leq_size(
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100)),
            b in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let (b_key, b_set) = b;
            let a_set_len = a_set.len();
            let b_set_len = b_set.len();
            store.insert(a_key.clone(), a_set.into());
            store.insert(b_key.clone(), b_set.into());
            let set = command(&mut store, &vec![a_key, b_key.clone()][..])?;
            assert!(set.len() <= a_set_len);
            assert!(set.len() <= b_set_len);
        }
    }

    proptest! {
        #[test]
        fn self_intersection_is_self(
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let a_len = a_set.len();
            store.insert(a_key.clone(), a_set.into());
            let set = command(&mut store,  &vec![a_key.clone(), a_key][..])?;
            assert_eq!(set.len(), a_len)
        }
    }

    proptest! {
        #[test]
        fn returns_size_of_dest(
            dest in valid_keys(),
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100)),
            b in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let (b_key, b_set) = b;
            store.insert(a_key.clone(), a_set.into());
            store.insert(b_key.clone(), b_set.into());
            let resulting_size = store_command(&mut store, &dest, &vec![a_key, b_key][..])?;
            let resulting_set = store.get(&dest);
            if let Some(Data::Set(set)) = resulting_set {assert_eq!(set.len(), resulting_size)} else { panic!("something odd found at key")}
        }
    }
}
