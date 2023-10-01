#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    set: BTreeMap<i32, K>,
    last_use: HashMap<K, i32>,
    map: HashMap<K, V>,
    time: i32,
    capacity: usize,
    size: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Ord,
{
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!();
        }
        Self {
            set: BTreeMap::new(),
            last_use: HashMap::new(),
            map: HashMap::new(),
            time: 0,
            capacity,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn touch_key(&mut self, key: &K) {
        if self.last_use.contains_key(key) {
            let tm = self.last_use[key];
            self.set.remove(&tm);
        }
        self.set.insert(self.time, key.clone());
        *self.last_use.entry(key.clone()).or_insert(self.time) = self.time;
        self.time += 1;
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if !self.last_use.contains_key(key) {
            return None;
        }
        self.touch_key(key);
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if !self.last_use.contains_key(key) {
            return None;
        }
        self.touch_key(key);
        self.map.get_mut(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.contains_key(&key) {
            self.touch_key(&key);
            let res = self.map.remove(&key);
            self.map.insert(key, value);
            return res;
        }
        self.size += 1;
        self.touch_key(&key);
        self.map.insert(key, value);
        if self.size > self.capacity {
            let a = self.set.pop_first().unwrap();
            self.map.remove(&a.1);
            self.last_use.remove(&a.1);
            self.size -= 1;
        }
        None
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.set.clear();
        self.last_use.clear();
        self.time = 0;
        self.size = 0;
    }
}
