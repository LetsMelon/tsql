use std::collections::BTreeMap;

/// Get, if possible, the first key-value tuple from a [`BTreeMap`] as reference.
///
/// This function is generic over the key `K: Ord` and value `V`.
pub fn get_first_element<K: Ord, V>(collection: &BTreeMap<K, V>) -> Option<(&K, &V)> {
    // `BTreeMap.keys()` returns the keys in an sorted iterator and with `Iterator.next()` we get the first value as an Option.
    // If the function returns `None`, we can return early and return with `None`.
    let key = collection.keys().next()?;

    // this get _should_ theoretically always return a value because if not how can we get a key out of the `BTreeMap`?
    let value = collection.get(key)?;

    Some((key, value))
}

#[cfg(test)]
mod tests {
    mod get_first_element {
        use std::collections::BTreeMap;

        use crate::helper::get_first_element;

        #[test]
        fn just_works() {
            let mut map = BTreeMap::new();
            assert_eq!(get_first_element(&map), None);

            map.insert("value", 1);
            assert_eq!(get_first_element(&map), Some((&"value", &1)));
        }
    }
}
