pub mod fcfs;
pub mod rr;
pub mod sjf;

#[derive(Debug, Clone)]
pub struct Job {
    pub name: String,
    pub arrival_time: u32,
    pub service_time: u32,
    pub finish_time: Option<u32>,
    pub whole_time: Option<u32>,
    pub weight_whole_time: Option<f32>,
    pub served_time: u32,
}

#[derive(Debug)]
pub struct SchedulerProcess {
    job: Option<Job>,
    status: String,
    current_time: u32,
    current_queue: Vec<Job>,
}

pub struct SchedulerResult {
    pub task_num: f32,
    pub scheduled: Vec<SchedulerProcess>,
    pub serviced_time: f32,
    pub weight_serviced_time: f32,
}

impl SchedulerResult {
    pub fn new() -> SchedulerResult {
        SchedulerResult {
            task_num: 0.0,
            scheduled: Vec::new(),
            serviced_time: 0.0,
            weight_serviced_time: 0.0,
        }
    }

    pub fn len(&mut self) -> usize {
        self.scheduled.len()
    }

    pub fn log(&mut self, process: SchedulerProcess) {
        if process.job.is_some() {
            let job = process.job.as_ref().unwrap();
            if job.finish_time.is_some() {
                self.task_num += 1.0;
                self.serviced_time += job.whole_time.unwrap() as f32;
                self.weight_serviced_time += job.weight_whole_time.unwrap();
            }
        }

        self.scheduled.push(process);
    }

    pub fn print(&mut self) {
        for SchedulerProcess {
            job,
            status,
            current_time,
            current_queue,
        } in &self.scheduled
        {
            print!("| {0}th \t|", current_time);
            print!(" {:<10} | ", status);

            match job {
                Some(job) => print!("{:<10}", job.name),
                None => print!("{:<10}", ""),
            }

            if current_queue.len() > 0 {
                print!(" | QUEUE: ");

                for job in current_queue {
                    print!("{} ", job.name);
                }
            }

            print!("\n");
        }

        println!(
            "\n| {0: <8} | {1: <6} | {2: <6} | {3: <6} | {4: <6} | {5: <6} |",
            "名称", "到达时间", "服务时间", "完成时间", "周转时间", "带权周转时间"
        );

        for SchedulerProcess { job, .. } in &self.scheduled {
            match job {
                Some(job) => {
                    if job.finish_time.is_none() {
                        continue;
                    }
                    println!(
                        "| {0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} |",
                        job.name,
                        job.arrival_time,
                        job.service_time,
                        job.finish_time.unwrap(),
                        job.whole_time.unwrap(),
                        job.weight_whole_time.unwrap()
                    )
                }
                None => {}
            }
        }

        let average_wt = self.serviced_time / self.task_num;
        let average_wwt = self.weight_serviced_time / self.task_num;

        println!(
            "平均周转时间: {}, 平均带权周转时间: {}",
            average_wt, average_wwt
        );
    }
}

pub trait Scheduler {
    fn new(jobs: Vec<Job>) -> Self;
    fn run(&mut self);
    fn print(&mut self);
}

#[cfg(test)]
fn build_jobs() -> Vec<Job> {
    vec![
        Job {
            name: "A".to_string(),
            arrival_time: 0,
            service_time: 5,
            finish_time: None,
            whole_time: None,
            weight_whole_time: None,
            served_time: 0,
        },
        Job {
            name: "B".to_string(),
            arrival_time: 1,
            service_time: 7,
            finish_time: None,
            whole_time: None,
            weight_whole_time: None,
            served_time: 0,
        },
        Job {
            name: "C".to_string(),
            arrival_time: 3,
            service_time: 3,
            finish_time: None,
            whole_time: None,
            weight_whole_time: None,
            served_time: 0,
        },
        Job {
            name: "D".to_string(),
            arrival_time: 4,
            service_time: 8,
            finish_time: None,
            whole_time: None,
            weight_whole_time: None,
            served_time: 0,
        },
        Job {
            name: "E".to_string(),
            arrival_time: 6,
            service_time: 2,
            finish_time: None,
            whole_time: None,
            weight_whole_time: None,
            served_time: 0,
        },
    ]
    .clone()
}
