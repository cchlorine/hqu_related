use std::{fmt::Debug};

use crate::Item;

pub struct Opt<K: PartialEq, T> {
    storage: Vec<Item<K, T>>,
    cache_size: usize,
}

impl<K: PartialEq + Clone + Debug, T: Copy> Opt<K, T> {
    pub fn new(size: usize) -> Opt<K, T> {
        Opt {
            storage: Vec::new(),
            cache_size: size,
        }
    }

    pub fn get(&mut self, key: K) -> Option<T> {
        match self.storage.iter().find(|&item| item.key == key) {
            Some(item) => Some(item.value),
            None => None,
        }
    }

    pub fn decide_farthest(&mut self, future: Vec<K>) -> usize {
        let mut deciding_items = self
            .storage
            .clone()
            .iter()
            .enumerate()
            .map(|(idx, item)| (idx, item.key.clone()))
            .collect::<Vec<(usize, K)>>();

        let mut distance: Vec<usize> = vec![usize::MAX; deciding_items.len()];

        for (idx, key) in future.iter().enumerate() {
            for (deciding_idx, deciding_item) in deciding_items.iter().enumerate() {
                if deciding_item.1 == *key {
                    distance[deciding_item.0] = idx;
                    deciding_items.remove(deciding_idx);
                    break;
                }
            }

            if deciding_items.len() == 0 {
                break;
            }
        }

        distance
            .iter()
            .enumerate()
            .max_by_key(|item| item.1)
            .unwrap()
            .0
    }

    pub fn set(&mut self, key: K, value: T, future: Vec<K>) -> bool {
        let try_find = self.get(key.clone());

        if try_find.is_none() {
            if self.storage.len() >= self.cache_size {
                let idx = self.decide_farthest(future);
                self.storage.splice(idx..idx + 1, vec![Item { key, value }]);
            } else {
                self.storage.push(Item { key, value });
            }
        }

        return try_find.is_none();
    }

    pub fn get_cached(&mut self) -> Vec<Item<K, T>> {
        return self.storage.clone();
    }
}
