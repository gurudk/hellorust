#![allow(unused)]
fn main() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
        where
            K: Clone + Eq + Hash,
            V: Default,
    {
        match map.get_mut(&key) {
            Some(mut value) => &mut value,
            None => {
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }
}