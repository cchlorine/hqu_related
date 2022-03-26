use crate::Item;


pub struct Fifo<K: PartialEq, T> {
    storage: Vec<Item<K, T>>,
    cache_size: usize,
}

impl<K: PartialEq + Clone, T: Copy> Fifo<K, T> {
    pub fn new(size: usize) -> Fifo<K, T> {
        Fifo {
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

    pub fn set(&mut self, key: K, value: T) -> bool {
        let try_find = self.get(key.clone());

        if try_find.is_none() {
            if self.storage.len() >= self.cache_size {
                self.storage.remove(0);
            }
            self.storage.push(Item { key, value });
        }

        return try_find.is_none();
    }

    pub fn get_cached(&mut self) -> Vec<Item<K, T>> {
        return self.storage.clone();
    }
}
