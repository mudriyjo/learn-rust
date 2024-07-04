use std::{
    collections::{self, hash_map::Entry, HashMap},
    hash::Hash,
};

fn just_print<T>(x: T)
where
    T: ToString,
{
    println!("{}", x.to_string())
}

#[derive(Debug)]
struct MyHashMap<K, V> {
    map: HashMap<K, Vec<V>>,
}
impl<K, V> MyHashMap<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        MyHashMap {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, k: K, v: V) {
        let entry = self.map.entry(k).or_default();
        entry.push(v);
    }

    fn iter(&self) -> MyHashMapIterator<K, V> {
        let mut it = self.map.iter();
        let map_entry = it.next();
        MyHashMapIterator {
            key_iter: it,
            current_map_entry: map_entry,
            current_vec_index: 0,
        }
    }
}

struct MyHashMapIterator<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    key_iter: collections::hash_map::Iter<'a, K, Vec<V>>,
    current_map_entry: Option<(&'a K, &'a Vec<V>)>,
    current_vec_index: usize,
}
impl<'a, K, V> Iterator for MyHashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.current_map_entry {
            if self.current_vec_index < entry.1.len() {
                let value = &entry.1[self.current_vec_index];
                self.current_vec_index += 1;
                return Some((entry.0, value));
            } else {
                self.current_map_entry = self.key_iter.next();
                self.current_vec_index = 0;

                if let Some((k, v)) = self.current_map_entry {
                    if self.current_vec_index < v.len() {
                        let value = &v[self.current_vec_index];
                        self.current_vec_index += 1;

                        return Some((k, value));
                    }
                }
            }
        }
        None
    }
}
fn main() {
    just_print(1);
    just_print("Hello");

    let mut my_hash_map = MyHashMap::new();
    my_hash_map.insert("hello", 1);
    my_hash_map.insert("hello", 2);
    my_hash_map.insert("bye", 3);
    println!("{my_hash_map:?}");

    my_hash_map
        .iter()
        .for_each(|el| println!("{:?} : {:?}", el.0, el.1));
}
