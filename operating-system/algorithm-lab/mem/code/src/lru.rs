use std::borrow::{Borrow};

use crate::Item;

pub struct Lru<K: PartialEq, T> {
    storage: Vec<Item<K, T>>,
    cache_size: usize,
}

impl<K: PartialEq + Clone, T: Copy> Lru<K, T> {
    pub fn new(size: usize) -> Lru<K, T> {
        Lru {
            storage: Vec::new(),
            cache_size: size,
        }
    }

    pub fn get(&mut self, key: K) -> Option<(usize, T)> {
        match self
            .storage
            .iter()
            .enumerate()
            .find(|(_, item)| item.key == key)
        {
            Some((idx, item)) => Some((idx, item.borrow().value)),
            None => None,
        }
    }

    pub fn set(&mut self, key: K, value: T) -> bool {
        let try_find = self.get(key.clone());

        match try_find {
            Some((idx, _)) => {
                self.storage.remove(idx);
                self.storage.push(Item { key, value });
            }
            None => {
                if self.storage.len() >= self.cache_size {
                    self.storage.remove(0);
                }
                self.storage.push(Item { key, value });
            }
        }

        return try_find.is_none();
    }

    pub fn get_cached(&mut self) -> Vec<Item<K, T>> {
        return self.storage.clone();
    }
}
