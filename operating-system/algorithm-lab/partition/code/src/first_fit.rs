use crate::Memory;

impl Memory {
    pub fn first_fit(&mut self) -> bool {
        for process_idx in 0..self.processes.len() {
            let process = &self.processes[process_idx];
            let mut flag = false;

            for (partition_idx, partition) in self.partitions.iter_mut().enumerate() {
                if partition.available_size >= process.memory_size {
                    flag = true;
                    let available_size = partition.consume(process.memory_size, process_idx);
                    self.log(process_idx, partition_idx, available_size);
                    break;
                }
            }

            if !flag {
                return false;
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
        assert_eq!(memory.first_fit(), true);
        println!("====[FirstFit Algorithm]====");
        memory.print();
    }
}
