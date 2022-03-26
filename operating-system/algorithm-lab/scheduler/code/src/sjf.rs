use crate::{Job, Scheduler, SchedulerProcess, SchedulerResult};

pub struct SjfScheduler {
    jobs: Vec<Job>,
    results: SchedulerResult,
}

impl Scheduler for SjfScheduler {
    fn new(mut jobs: Vec<Job>) -> SjfScheduler {
        jobs.sort_by_key(|job| job.arrival_time);

        SjfScheduler {
            jobs: jobs,
            results: SchedulerResult::new(),
        }
    }

    fn run(&mut self) {
        let mut current_time: u32 = 0;
        let mut running_queue = self.jobs.iter().map(|x| x.clone()).collect::<Vec<Job>>();

        while running_queue.len() > 0 {
            let current_running_queue = running_queue.iter().filter_map(|x| match x.arrival_time <= current_time && x.finish_time.is_none() {
                true => Some(x.clone()),
                false => None,
            }).collect::<Vec<Job>>();

            let current_job = running_queue
                .iter_mut()
                .enumerate()
                .filter(|(_, x) | x.arrival_time <= current_time && x.finish_time.is_none())
                .min_by_key(|(_, x)| x.service_time);

            match current_job {
                None => current_time += 1,
                Some((idx, job)) => {
                    job.finish_time = Some(current_time + job.service_time);
                    job.whole_time = Some(job.finish_time.unwrap() - job.arrival_time);
                    job.weight_whole_time =
                        Some(job.whole_time.unwrap() as f32 / job.service_time as f32);

                    self.results.log(SchedulerProcess {
                        job: Some(job.clone()),
                        current_time,
                        status: "FINISHED".to_string(),
                        current_queue: current_running_queue,

                    });

                    current_time += job.service_time;
                    running_queue.remove(idx);
                }
            }
        }
    }

    fn print(&mut self) {
        println!("\n\n[Scheduler algorithm: SJF（短作业优先）]");
        self.results.print();
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_jobs, sjf::SjfScheduler, Job, Scheduler};

    #[test]
    fn test() {
        let jobs: Vec<Job> = build_jobs();

        let mut scheduler = SjfScheduler::new(jobs);
        scheduler.run();
        scheduler.print();
    }
}
