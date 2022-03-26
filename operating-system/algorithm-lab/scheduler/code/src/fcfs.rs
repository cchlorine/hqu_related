use crate::{Job, Scheduler, SchedulerResult, SchedulerProcess};

pub struct FcfsScheduler {
    jobs: Vec<Job>,
    results: SchedulerResult,
}

impl Scheduler for FcfsScheduler {
    fn new(mut jobs: Vec<Job>) -> FcfsScheduler {
        jobs.sort_by_key(|job| job.arrival_time);

        FcfsScheduler {
            jobs: jobs,
            results: SchedulerResult::new(),
        }
    }

    fn run(&mut self) {
        let mut current_time: u32 = 0;

        for current_job in self.jobs.iter_mut() {
            if current_time < current_job.arrival_time {
                current_time = current_job.arrival_time;
            }

            current_job.finish_time = Some(current_time + current_job.service_time);
            current_job.whole_time =
                Some(current_job.finish_time.unwrap() - current_job.arrival_time);
            current_job.weight_whole_time =
                Some(current_job.whole_time.unwrap() as f32 / current_job.service_time as f32);

            self.results.log(SchedulerProcess {
                job: Some(current_job.clone()),
                status: "FINISHED".to_string(),
                current_time,
                current_queue: vec![],
            });

            current_time += current_job.service_time;
        }
    }

    fn print(&mut self) {
        println!("\n\n[Scheduler algorithm: FCFS（先来先服务）]");
        self.results.print();
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_jobs, fcfs::FcfsScheduler, Job, Scheduler};

    #[test]
    fn test() {
        let jobs: Vec<Job> = build_jobs();
        let mut scheduler = FcfsScheduler::new(jobs);

        scheduler.run();
        scheduler.print();
    }
}
