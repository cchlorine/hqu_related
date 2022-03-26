use std::borrow::BorrowMut;

use crate::Memory;

impl Memory {
    pub fn next_fit(&mut self) -> bool {
        let mut partition_idx = 0;
        for process_idx in 0..self.processes.len() {
            let process = &self.processes[process_idx];

            let mut current_partition_idx = partition_idx;

            loop {
                let partition = self.partitions[current_partition_idx].borrow_mut();
                if partition.block_size - partition.allocated_size >= process.memory_size {
                    let available_size = partition.consume(process.memory_size, process_idx);
                    self.log(process_idx, partition_idx, available_size);

                    partition_idx = (current_partition_idx + 1) % self.partitions.len();
                    break;
                }

                current_partition_idx = (current_partition_idx + 1) % self.partitions.len();
                if current_partition_idx == partition_idx {
                    return false;
                }
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
        assert_eq!(memory.next_fit(), true);
        println!("====[NextFit Algorithm]====");
        memory.print();
    }
}
