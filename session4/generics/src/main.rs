use std::{collections::HashMap, hash::Hash};

fn just_print<T>(x: T)
where
    T: ToString,
{
    println!("{}", x.to_string())
}

#[derive(Debug)]
struct MyHashMap<K,V> 
{
    map: HashMap<K,Vec<V>>
}
impl <K,V> MyHashMap<K,V> 
where K: Eq + Hash
{
    fn new() -> Self {
        MyHashMap {map: HashMap::new()}
    }

    fn insert(&mut self, k: K, v:V) {
        let entry = self.map.entry(k).or_insert(Vec::new());
        entry.push(v);
    }
}
fn main() {
    just_print(1);
    just_print("Hello");

    let mut my_hash_map = MyHashMap::new();
    my_hash_map.insert("hello", 1);
    my_hash_map.insert("hello", 2);
    my_hash_map.insert("bye", 3);
    println!("{my_hash_map:?}")
}
