use super::super::domain::{Data, Primitive};
use super::super::errors::ApplicationError;
use super::utilities::get_sets;
use std::collections::{HashMap, HashSet};

pub fn command(
    store: &mut HashMap<String, Data>,
    keys: &[String],
) -> Result<HashSet<Primitive>, ApplicationError> {
    let sets = get_sets(store, keys)?;
    let mut result: HashSet<Primitive> = HashSet::new();
    sets.iter().for_each(|set| {
        set.iter().for_each(|el| {
            result.insert(el.clone());
        })
    });
    Ok(result)
}

pub fn store_command(
    store: &mut HashMap<String, Data>,
    destination: &str,
    keys: &[String],
) -> Result<usize, ApplicationError> {
    let union: HashSet<Primitive> = command(store, keys)?;
    let size = union.len();
    store.insert(destination.to_string(), union.into());
    return Ok(size);
}

#[cfg(test)]
mod test {
    use super::super::super::domain::{Data, Primitive};
    use super::store_command;
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
        fn union_is_idempotent(
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
        fn disjoint_union_has_sum_size(
            dest in valid_keys(),
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100)),
            b in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let (b_key, b_set) = b;
            let disjoint_to_b = a_set.difference(&b_set).cloned().collect::<HashSet<Primitive>>();
            let sum_size = b_set.len() + disjoint_to_b.len();
            store.insert(a_key.clone(), disjoint_to_b.into());
            store.insert(b_key.clone(), b_set.into());
            store_command(&mut store, &dest, &vec![a_key, b_key.clone()][..])?;
            if let Some(Data::Set(set)) = store.get(&dest) {assert_eq!(set.len(), sum_size)} else { panic!("something odd found at key")}
        }
    }

    proptest! {
        #[test]
        fn union_has_leq_sum_size(
            dest in valid_keys(),
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100)),
            b in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let (b_key, b_set) = b;
            let a_len = a_set.len();
            let b_len = b_set.len();
            store.insert(a_key.clone(), a_set.into());
            store.insert(b_key.clone(), b_set.into());
            store_command(&mut store, &dest, &vec![a_key, b_key.clone()][..])?;
            if let Some(Data::Set(set)) = store.get(&dest) {
                assert!(set.len() >= a_len);
                assert!(set.len() >= b_len);
                assert!(set.len() <= (a_len + b_len))
            } else {
                panic!("something odd found at key")
            }
        }
    }

    proptest! {
        #[test]
        fn self_union_is_self(
            dest in valid_keys(),
            a in (valid_keys(), hash_set(any::<Primitive>(), 1..100))
        ) {
            let mut store: HashMap<String, Data> = HashMap::new();
            let (a_key, a_set) = a;
            let a_len = a_set.len();
            store.insert(a_key.clone(), a_set.into());
            store_command(&mut store, &dest, &vec![a_key.clone(), a_key][..])?;
            let result = store.get(&dest);
            if let Some(Data::Set(set)) = result {assert_eq!(set.len(), a_len)} else { panic!("something odd found at key")}
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
