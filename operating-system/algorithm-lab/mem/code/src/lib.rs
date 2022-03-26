use lru::Lru;
use opt::Opt;

use self::fifo::{Fifo};
use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct Item<K, T> {
    pub key: K,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct Action<K: Debug> {
    query: K,
    fault: bool,
    cached_items: Vec<Item<K, K>>,
}

#[derive(Clone)]
pub struct Task<K: Debug> {
    queries: Vec<K>,
    actions: Vec<Action<K>>,
    fault_count: u32,
    page_size: usize,
}

impl<K: PartialEq + Copy + Debug + Display> Task<K> {
    pub fn new(queries: Vec<K>, page_size: usize) -> Task<K> {
        Task {
            queries,
            actions: Vec::new(),
            fault_count: 0,
            page_size,
        }
    }

    pub fn fifo(&mut self) -> &mut Self {
        let mut fifo = Fifo::<K, K>::new(self.page_size);
        self.fault_count = 0;
        self.actions.clear();

        for &action in self.queries.iter() {
            let fault = fifo.set(action, action);
            if fault {
                self.fault_count += 1;
            }

            self.actions.push(Action {
                query: action.clone(),
                fault,
                cached_items: fifo.get_cached(),
            });
        }

        self
    }

    pub fn opt(&mut self) -> &mut Self {
        let mut opt = Opt::<K, K>::new(self.page_size);
        self.fault_count = 0;
        self.actions.clear();

        for (idx, &action) in self.queries.iter().enumerate() {
            let fault = opt.set(action, action, self.queries[idx..].to_vec());
            if fault {
                self.fault_count += 1;
            }

            self.actions.push(Action {
                query: action.clone(),
                fault,
                cached_items: opt.get_cached(),
            });
        }

        self
    }

    pub fn lru(&mut self) -> &mut Self {
        let mut lru = Lru::<K, K>::new(self.page_size);
        self.fault_count = 0;
        self.actions.clear();

        for &action in self.queries.iter() {
            let fault = lru.set(action, action);
            if fault {
                self.fault_count += 1;
            }

            self.actions.push(Action {
                query: action.clone(),
                fault,
                cached_items: lru.get_cached(),
            });
        }

        self
    }

    pub fn print(&mut self) {
        // Heading
        print!("|==");

        for _ in 0..self.queries.len() + 1 {
            print!("======");
        }

        print!("|\n|");
        print!("|{:>6}|", "nth");
        for idx in 0..self.queries.len() {
            print!("{:>6}", idx);
        }

        print!("|\n|--");

        for _ in 0..self.queries.len() + 1 {
            print!("------");
        }
        print!("|\n|");

        // Content
        print!("|{:>6}|", "Query");
        // query
        for action in &self.actions {
            print!("{:>6}", action.query);
        }
        print!("|\n|");

        // faulted
        print!("|{:>6}|", "Hit?");
        for action in &self.actions {
            print!("{:>6}", if action.fault { "Miss" } else { "Hit" });
        }
        print!("|\n|");

        for idx in 0..self.page_size {
            print!("|{:>6}|", idx);

            for action in &self.actions {
                match action.cached_items.get(idx) {
                    Some(item) => print!("{:>6}", item.key),
                    None => print!("{:>6}", "None"),
                }
            }
            print!("|\n|");
        }

        // Footer
        for _ in 0..self.queries.len() + 1 {
            print!("======");
        }

        print!("==|\n");

        print!(
            "Fault/Total = {:>6}/{:<6},     FaultRate: {:>6}",
            self.fault_count,
            self.queries.len(),
            self.fault_count as f32 / self.queries.len() as f32
        );
        print!(" |\n");
    }
}

pub mod fifo;
pub mod opt;
pub mod lru;

#[cfg(test)]
mod tests {
    use super::Task;

    fn new_task(page_size: usize) -> Task<u32> {
        Task::new(
            vec![
                4, 3, 2, 4, 2, 4, 2, 3, 5, 4, 3, 6, 5, 2, 1, 2, 3, 7, 1, 2, 6, 1,
            ],
            page_size,
        )
        .clone()
    }

    #[test]
    fn test() {
        let mut task = new_task(3);
        println!("===[[FIFO Mem Exchange]]===");
        task.fifo().print();
        assert_eq!(task.fault_count, 15);

        println!("\n\n===[[OPT Mem Exchange]]===");
        task.opt().print();

        println!("\n\n===[[LRU Mem Exchange]]===");
        task.lru().print();
        assert_eq!(task.fault_count, 14);
    }
}
