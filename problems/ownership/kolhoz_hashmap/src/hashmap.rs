#![forbid(unsafe_code)]

use crate::types::{Data, Key};

pub fn new_hashmap(len: usize) -> Vec<Vec<(Key, Data)>> {
    let mut hashmap = Vec::new();
    for _i in 0..len {
        let tmp: Vec<(Key, Data)> = Vec::new();
        hashmap.push(tmp);
    }
    hashmap
}

pub fn insert(table: &mut Vec<Vec<(Key, Data)>>, key: Key, value: Data) -> &mut Data {
    if table.is_empty() {
        panic!("insert in empty kolhoz-table");
    }
    let i = key.get_hash() as usize % table.len();
    table[i].push((key, value));

    return &mut table[i].last_mut().unwrap().1;
}

pub fn get_one_or_default<'a>(
    table: &'a Vec<Vec<(Key, Data)>>,
    key: &Key,
    default_value: &'a Data,
) -> &'a Data {
    if table.is_empty() {
        return default_value;
    }
    let i = key.get_hash() as usize % table.len();
    for el in &table[i] {
        if el.0 == *key {
            return &el.1;
        }
    }
    default_value
}

pub fn multi_get<'a, 'b>(
    table: &'a Vec<Vec<(Key, Data)>>,
    keys: &'b Vec<Key>,
) -> Vec<(&'b Key, Vec<&'a Data>)> {
    let mut ans: Vec<(&Key, Vec<&Data>)> = Vec::new();
    for key in keys {
        let mut tmp: Vec<&Data> = Vec::new();
        if table.is_empty() {
            ans.push((key, tmp));
            continue;
        }
        let i = key.get_hash() as usize % table.len();
        for el in &table[i] {
            if el.0 == *key {
                tmp.push(&el.1);
            }
        }
        ans.push((key, tmp));
    }
    ans
}

pub fn find_keys_of_data<'a>(table: &'a Vec<Vec<(Key, Data)>>, value: &Data) -> Vec<&'a Key> {
    let mut ans: Vec<&Key> = Vec::new();
    for bucket in table {
        for el in bucket {
            if el.1 == *value {
                ans.push(&el.0)
            }
        }
    }
    ans
}

pub fn resize(table: &mut Vec<Vec<(Key, Data)>>, new_len: usize) {
    let mut tmp: Vec<Vec<(Key, Data)>> = new_hashmap(new_len);
    for bucket in &mut *table {
        while let Some(el) = bucket.pop() {
            if new_len > 0 {
                insert(&mut tmp, el.0, el.1);
            }
        }
    }
    *table = tmp;
}
