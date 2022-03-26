pub mod first_fit;
pub mod next_fit;
pub mod best_fit;
pub mod worst_fit;

#[derive(Debug)]
pub struct Partition {
    name: String,
    block_size: u32,
    available_size: u32,
    allocated_size: u32,
    allocated_blocks: Vec<usize>, // idx only
}

impl Partition {
    pub fn new(name: &str, block_size: u32) -> Partition {
        Partition {
            name: name.to_string(),
            block_size,
            available_size: block_size,
            allocated_size: 0,
            allocated_blocks: Vec::new(),
        }
    }

    pub fn consume(&mut self, size: u32, process_idx: usize) -> u32 {
        self.allocated_size += size;
        self.available_size -= size;
        self.allocated_blocks.push(process_idx);

        self.available_size
    }
}

pub struct Process {
    name: String,
    memory_size: u32,
}

impl Process {
    pub fn new(name: &str, memory_size: u32) -> Process {
        Process {
            name: name.to_string(),
            memory_size,
        }
    }
}

#[derive(Debug)]
pub struct OpLog {
    process_idx: usize,
    partition_idx: usize,
    partition_left: u32,
}

pub struct Memory {
    processes: Vec<Process>,
    partitions: Vec<Partition>,
    logs: Vec<OpLog>,
}

impl Memory {
    pub fn log(&mut self, process_idx: usize, partition_idx: usize, partition_left: u32) {
        self.logs.push(OpLog {
            process_idx,
            partition_idx,
            partition_left,
        });
    }

    pub fn print(&mut self) {
        for partition in self.partitions.iter() {
            print!("|=============|\n");
            println!("|{:^13}|", partition.name);
            for bloc in partition.allocated_blocks.iter().map(|&idx| self.processes.get(idx).unwrap()).into_iter() {
                println!("|-------------|");
                println!("|{:^13}|", bloc.name);
                println!("|{:^13}|", bloc.memory_size);
                println!("|-------------|");
            }
            print!("|{:^13}|\n", "avail");
            print!("|{:^13}|\n", partition.block_size - partition.allocated_size);
            println!("|-------------|");
        }

        println!("");

        for (i, log) in self.logs.iter().enumerate() {
            let process = &self.processes[log.process_idx];
            let partition = &self.partitions[log.partition_idx];

            print!("{}th: ", i);
            print!("allocate {}({}) to {}({}/{})", process.name, process.memory_size, partition.name, log.partition_left, partition.block_size);
            print!("\n");
        }
    }
}

#[cfg(test)]
fn build_test() -> Memory {
    Memory {
        partitions: vec![
            Partition::new("A", 16),
            Partition::new("B", 16),
            Partition::new("C", 32),
            Partition::new("D", 64),
            Partition::new("E", 20),
        ],
        processes: vec![
            Process::new("a", 12),
            Process::new("b", 10),
            Process::new("c", 22),
            Process::new("d", 15),
            Process::new("e", 6),
        ],
        logs: Vec::new(),
    }
}