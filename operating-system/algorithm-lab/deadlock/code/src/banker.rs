use std::{
    borrow::{BorrowMut},
};

pub struct Task {
    pub need: Vec<u32>,
    pub allocation: Vec<u32>,
}

pub struct DeadlockCheck {
    pub resources: Vec<u32>,
    pub tasks: Vec<Task>,
    pub current_resources: Vec<u32>,
}

fn satisfy(have: Vec<u32>, need: Vec<u32>) -> bool {
    need.clone().into_iter()
        .enumerate()
        .fold(true, |s, (i, j)| s && have[i] >= j)
}

fn sum_to_left(have: &mut Vec<u32>, allocated: Vec<u32>) {
    for index in 0..have.len() {
        have[index] = have[index] + allocated[index]
    }
}

impl DeadlockCheck {
    pub fn new(resources: Vec<u32>, tasks: Vec<Task>) -> DeadlockCheck {
        DeadlockCheck {
            tasks,
            resources,
            current_resources: Vec::new(),
        }
    }

    pub fn print_task(&mut self) {
        println!("[Resources allocation]: {:?}", self.resources);
        println!("[Tasks]");

        self.tasks
          .iter()
          .enumerate()
          .for_each(|(i, x)| println!("{}th task: allocated: {:?}, needed: {:?}", i, x.allocation, x.need))
    }

    pub fn detect(&mut self) -> (bool, Vec<usize>) {
        let mut index = 0;
        let tasks_len = self.tasks.len();
        let mut available = self.resources.clone();

        let mut finished_count = 0;
        let mut finished: Vec<bool> = vec![false].repeat(tasks_len);
        let mut safe_seq: Vec<usize> = Vec::new();

        while finished_count < tasks_len {
            if !finished[index] {
                let task = self.tasks.get_mut(index).unwrap();
                if satisfy(available.clone(), task.need.clone()) {
                    finished_count += 1;
                    finished[index] = true;
                    safe_seq.push(index);

                    sum_to_left(available.borrow_mut(), task.allocation.clone());
                    index = 0;
                    continue;
                }
            }

            index = (index + 1) % tasks_len;
            if index == 0 && finished_count == 0 {
              return (false, safe_seq)
            }
        }

        self.current_resources = available;
        return (true, safe_seq);
    }

    pub fn request(&mut self, request: Vec<u32>) -> bool {
        let mut available = self.current_resources.clone();
        let mut request_satisfied = true;

        for index in 0..request.len() {
            if available[index] < request[index] {
                request_satisfied = false;
                break;
            }
            available[index] -= request[index];
        }

        if request_satisfied {
            self.current_resources = available;
        }

        request_satisfied
    }
}

#[cfg(test)]
mod tests {
    use crate::banker::{DeadlockCheck, Task};

    #[test]
    fn test() {
        let resources: Vec<u32> = vec![3, 5, 2];
        let tasks: Vec<Task> = vec![
          Task {
            need: vec![4, 1, 0],
            allocation: vec![0, 2, 1],
          },

          Task {
            need: vec![2, 3, 1],
            allocation: vec![1, 0, 1],
          },

          Task {
            need: vec![4, 0, 4],
            allocation: vec![0, 1, 3],
          },

          Task {
            need: vec![4, 2, 2],
            allocation: vec![3, 2, 1],
          },

          Task {
            need: vec![5, 1, 3],
            allocation: vec![0, 2, 0],
          }
        ];

        let mut deadlock_checker = DeadlockCheck::new(resources, tasks);

        deadlock_checker.print_task();
        let (is_safe, sequence) = deadlock_checker.detect();

        println!("");
        if is_safe {
          println!("The sequence is safe.\nThe safe seq is: {:?}", sequence);
        } else {
          println!("The sequence is not safe");
        }
    }
}
