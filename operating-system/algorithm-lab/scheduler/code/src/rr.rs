use crate::{Job, SchedulerProcess, SchedulerResult};

pub struct RrScheduler {
    jobs: Vec<Job>,
    results: SchedulerResult,
}

impl RrScheduler {
    pub fn new(jobs: Vec<Job>) -> RrScheduler {
        let mut rr_jobs: Vec<Job> = jobs.clone();
        rr_jobs.sort_by_key(|job| job.arrival_time);

        RrScheduler {
            jobs: rr_jobs,
            results: SchedulerResult::new(),
        }
    }

    pub fn run(&mut self, slice_size: u32) {
        let mut cycle_count = 0;
        let mut tick_count = 0;
        let mut _index = 0;
        let mut finished_count = 0;

        let task_len = self.jobs.len();
        let mut running_tasks: Vec<usize> = Vec::new();
        println!("slice size: {}", slice_size);
        while finished_count < task_len {
            let current_queue = running_tasks
                .iter()
                .map(|&x| self.jobs.get(x).unwrap().clone())
                .collect::<Vec<Job>>();

            // ENQUEUE
            if _index < task_len {
                let current_task = self.jobs.get(_index);
                if let Some(job) = current_task {
                    if job.arrival_time == cycle_count {
                        running_tasks.push(_index);
                        _index += 1;
                    }
                }
            }

            while running_tasks.len() == 0 {
                cycle_count += 1;

                self.results.log(SchedulerProcess {
                    job: None,
                    status: "WAITING".to_string(),
                    current_time: cycle_count,
                    current_queue: current_queue.clone(),
                });
                continue;
            }

            // FORCE MOVE TO NEXT TASK
            if tick_count == slice_size {
                tick_count = 0;
                let current_index = *running_tasks.first().unwrap();
                running_tasks.remove(0);
                running_tasks.push(current_index);
            }

            // SCHEDULE
            let current_task_index = *running_tasks.first().unwrap();
            let current_task = self.jobs.get_mut(current_task_index).unwrap();

            if current_task.arrival_time <= cycle_count {
                tick_count += 1;
                current_task.served_time += 1;

                // FINISHED: has finished its job
                if current_task.served_time == current_task.service_time {
                    current_task.finish_time = Some(cycle_count + 1);
                    current_task.whole_time =
                        Some(current_task.finish_time.unwrap() - current_task.arrival_time);
                    current_task.weight_whole_time = Some(
                        current_task.whole_time.unwrap() as f32 / current_task.service_time as f32,
                    );

                    tick_count = 0;
                    finished_count += 1;
                    running_tasks.remove(0);
                }

                self.results.log(SchedulerProcess {
                    job: Some(current_task.clone()),
                    status: match current_task.served_time == current_task.service_time {
                        true => "FINISHED".to_string(),
                        false => "RUNNING".to_string(),
                    },
                    current_time: cycle_count,
                    current_queue: current_queue.clone(),
                });

                // TASKS HAS ALL FINISHED
                if finished_count >= task_len {
                    break;
                }
            }

            cycle_count += 1;
        }
    }

    pub fn print(&mut self) {
        println!("\n\n[Scheduler algorithm: RR（时间片轮转)]");
        self.results.print();
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_jobs, rr::RrScheduler, Job};

    #[test]
    fn test() {
        let jobs: Vec<Job> = build_jobs();

        let mut scheduler = RrScheduler::new(jobs);
        scheduler.run(2);
        scheduler.print();
    }
}
