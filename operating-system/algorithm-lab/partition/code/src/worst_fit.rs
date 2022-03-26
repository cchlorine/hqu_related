use std::borrow::BorrowMut;

use crate::Memory;

impl Memory {
    pub fn worst_fit(&mut self) -> bool {
        for process_idx in 0..self.processes.len() {
            let process = &self.processes[process_idx];

            let mut partition_indexes: Vec<(usize, u32)> = self.partitions
              .iter()
              .enumerate()
              .map(|(i, p)| (i, p.available_size))
              .collect();
            partition_indexes.sort_by_key(|&(_, available_size)| available_size);
            partition_indexes.reverse();

            let mut flag = false;
            for (partition_idx, available_size) in partition_indexes {
                if available_size >= process.memory_size {
                    flag = true;
                    let partition = self.partitions[partition_idx].borrow_mut();
                    let available_size = partition.consume(process.memory_size, process_idx);
                    self.log(process_idx, partition_idx, available_size);
                    break;
                }
            }

            if !flag {
                return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::build_test;

    #[test]
    fn test() {
        let mut memory = build_test();
        assert_eq!(memory.worst_fit(), true);
        println!("====[WorstFit Algorithm]====");
        memory.print();
    }
}
