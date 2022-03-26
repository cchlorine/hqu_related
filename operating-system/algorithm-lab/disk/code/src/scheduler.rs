#[derive(Debug, Clone)]
pub struct Request {
    target_position: i32,
    moved_distance: i32,
    prev_position: Option<i32>,
}

impl Request {
    pub fn schedule_at(&mut self, position: i32) {
        self.moved_distance = (position - self.target_position).abs();
        self.prev_position = Some(position);
    }

    pub fn diff_current(&mut self, position: i32) -> i32 {
        (self.target_position - position).abs()
    }
}

#[derive(Debug, Clone)]
pub enum SchedulerDirection {
    Upward = 1,
    Downward = -1,
}

pub struct Scheduler {
    requests: Vec<Request>,
    // THESE TWO IS FOR REAL SCAN, CSCAN
    // but in our case, we don't need
    section_start: i32,
    section_end: i32,
    direction: SchedulerDirection,
}

#[derive(Debug, Clone)]
pub struct SchedulerResult {
    requests: Vec<Request>,
    moved_distance: i32,
}

impl Scheduler {
    pub fn create_requests_from_vec(
        requests: Vec<i32>,
        section_start: Option<i32>,
        section_end: Option<i32>,
        direction: Option<SchedulerDirection>,
    ) -> Scheduler {
        let mut scheduler = Scheduler {
            requests: Vec::new(),
            section_start: section_start.unwrap_or(0),
            section_end: section_end.unwrap_or(0),
            direction: direction.unwrap_or(SchedulerDirection::Upward),
        };

        requests.iter().for_each(|request| {
            scheduler.requests.push(Request {
                target_position: *request,
                moved_distance: 0,
                prev_position: None,
            });
        });

        scheduler
    }

    pub fn find_start(&mut self, start_pos: i32) -> usize {
        // find the start position of the scheduler
        let mut current_idx = 0;
        let mut nearest_diff = (self.requests[0].target_position - start_pos).abs();

        self.requests.iter().enumerate().for_each(|(idx, val)| {
            let diff = (val.target_position - start_pos).abs();

            if diff < nearest_diff {
                current_idx = idx;
                nearest_diff = diff;
            }
        });

        current_idx
    }

    pub fn fcfs(&mut self, start_pos: i32) -> SchedulerResult {
        let mut result = SchedulerResult {
            requests: self.requests.clone(),
            moved_distance: 0,
        };

        let mut current_position: i32 = start_pos;

        result.requests.iter_mut().for_each(|req| {
            req.schedule_at(current_position);
            result.moved_distance += req.moved_distance;
            current_position = req.target_position;
        });

        result
    }

    pub fn ssts(&mut self, start_pos: i32) -> SchedulerResult {
        let mut result = SchedulerResult {
            requests: Vec::new(),
            moved_distance: 0,
        };

        // sort all tasks by their target position
        let mut running_tasks = self.requests.clone();
        running_tasks.sort_by_key(|req| req.target_position);

        let mut current_position = start_pos;
        // find the start position of the scheduler
        let mut current_idx = self.find_start(start_pos);

        // compare the two sides
        while running_tasks.len() > 0 {
            // pop out the scheduled item
            let mut current_task = running_tasks.remove(current_idx);
            current_task.schedule_at(current_position);
            result.moved_distance += current_task.moved_distance;
            current_position = current_task.target_position;

            // transfer ownership to result
            result.requests.push(current_task);

            // find the next task
            // if there is no tasks left
            if running_tasks.len() == 0 {
                break;
            // if there is only one task
            } else if running_tasks.len() == 1 {
                current_idx = 0;
            // more than one, and we should compare the cons two
            } else {
                let diff_ids = if current_idx == 0 {
                    [current_idx, current_idx + 1]
                } else {
                    [current_idx - 1, current_idx]
                };

                let diffs = diff_ids
                    .iter()
                    .map(|idx| (*idx, running_tasks[*idx].diff_current(current_position)))
                    .min_by_key(|(_, diff)| *diff)
                    .unwrap();

                current_idx = diffs.0;
            }
        }

        result
    }

    /**
     * In CSAN, we have two direction, UPWARD AND DOWNWARD
     *
     * When hit the boundary: change the direction
     */
    pub fn scan(&mut self, start_pos: i32) -> SchedulerResult {
        let mut result = SchedulerResult {
            requests: Vec::new(),
            moved_distance: 0,
        };

        let mut direction = self.direction.clone();

        // sort all tasks by their target position
        let mut running_tasks = self.requests.clone();
        running_tasks.sort_by_key(|req| req.target_position);

        let mut idx = 0;
        let mut current_pos = start_pos;

        while running_tasks.len() > 0 {
            let peek_task = running_tasks.get(idx).unwrap();

            if (matches!(direction, SchedulerDirection::Upward)
                && peek_task.target_position >= current_pos)
                || (matches!(direction, SchedulerDirection::Downward)
                    && peek_task.target_position <= current_pos)
            {
                let mut current_task = running_tasks.remove(idx);

                current_task.schedule_at(current_pos);
                current_pos = current_task.target_position;

                result.moved_distance += current_task.moved_distance;
                result.requests.push(current_task);
            } else {
                if matches!(direction, SchedulerDirection::Upward) {
                    idx += 1;
                } else {
                    if idx == 0 {
                        direction = SchedulerDirection::Upward;
                    } else {
                        idx -= 1;
                    }
                }
            }

            if idx > 0 && idx == running_tasks.len() {
                direction = SchedulerDirection::Downward;
                idx = running_tasks.len() - 1;
            }
        }

        result
    }

    /**
     * In CSCAN, we only handle requests in upward direction
     */
    pub fn cscan(&mut self, start_pos: i32) -> SchedulerResult {
        let mut result = SchedulerResult {
            requests: Vec::new(),
            moved_distance: 0,
        };

        // sort all tasks by their target position
        let mut running_tasks = self.requests.clone();
        running_tasks.sort_by_key(|req| req.target_position);

        let mut idx = 0;
        let mut current_pos = start_pos;
        let mut possible_pos = start_pos;

        while running_tasks.len() > 0 {
            let peek_task = running_tasks.get(idx).unwrap();

            if peek_task.target_position >= possible_pos {
                let mut current_task = running_tasks.remove(idx);

                current_task.schedule_at(current_pos);
                current_pos = current_task.target_position;
                possible_pos = current_pos;

                result.moved_distance += current_task.moved_distance;
                result.requests.push(current_task);
            } else {
                idx += 1;
            }

            if idx == running_tasks.len() {
                idx = 0;
                possible_pos = 0;
            }
        }

        result
    }
}

impl SchedulerResult {
    pub fn print(&mut self) {
        if self.requests.len() == 0 {
            println!("[EMPTY REQUESTS!]");
        } else {
            self.requests.iter().enumerate().for_each(|(idx, req)| {
                println!(
                    "Req#{}, target |{}->{}| \t moved distance {}",
                    idx,
                    req.prev_position.unwrap_or(-1),
                    req.target_position,
                    req.moved_distance
                );
            });

            println!("Total moved distance: {}", self.moved_distance);
            println!(
                "Average move distance: {}",
                self.moved_distance / self.requests.len() as i32
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Scheduler, SchedulerDirection};

    fn build() -> Scheduler {
        Scheduler::create_requests_from_vec(
            vec![86, 1470, 913, 1774, 948, 1509, 1022, 1750, 130],
            Some(0),
            Some(4999),
            Some(SchedulerDirection::Upward),
        )
    }

    #[test]
    fn test_fcfs() {
        println!("===TESTING FCFS===");
        let mut requests = build();
        let mut result = requests.fcfs(143);
        result.print();

        assert_eq!(result.moved_distance, 7081);
    }

    #[test]
    fn test_ssts() {
        println!("===TESTING SSTS===");
        let mut requests = build();
        let mut result = requests.ssts(143);
        result.print();

        assert_eq!(result.moved_distance, 1745);
    }

    #[test]
    fn test_scan() {
        println!("===TESTING SCAN===");
        let mut requests = build();
        let mut result = requests.scan(143);
        result.print();

        assert_eq!(result.moved_distance, 3319);
    }

    #[test]
    fn test_cscan() {
        println!("===TESTING CSCAN===");
        let mut requests = build();
        let mut result = requests.cscan(143);
        result.print();

        assert_eq!(result.moved_distance, 3363);
    }
}
