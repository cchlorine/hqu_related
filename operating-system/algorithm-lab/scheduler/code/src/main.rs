use clearscreen;
use scheduler::{Job, fcfs::FcfsScheduler, sjf::SjfScheduler, rr::RrScheduler};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
};
use scheduler::Scheduler;
fn clr() {
    clearscreen::clear().expect("failed to clear screen");
}

fn select_algo() -> u32 {
    clr();
    print!(
        "{}{}{}{}{}",
        "Now you can run the following algorithms:\n", "1. FCFS\n", "2. SJF\n", "3. RR\n", "4. FCFS+RR\n",
    );

    let mut algo_select = String::new();
    stdin().read_line(&mut algo_select).unwrap();

    match algo_select.trim().parse::<u32>() {
        Ok(i) => {
            if i > 4 || i < 1 {
                println!("Invalid select. Please try again.");
                return select_algo();
            }

            return i;
        }
        Err(..) => {
            println!("Invalid select. Please try again.");
            return select_algo();
        }
    };
}

fn read_from_file() -> Vec<Job> {
    let mut file_path = String::new();
    println!("Input the file path: (Empty means using default config)");
    let default_path = "./data/scheduler_test.csv".to_string();

    match stdin().read_line(&mut file_path) {
        Ok(..) => file_path = file_path.trim().to_string(),
        Err(..) => file_path = default_path,
    }

    if file_path.len() == 0 {
        file_path = "./data/scheduler_test.csv".to_string()
    }

    println!("{}", file_path.len());

    let file_path = Path::new(file_path.trim());

    let file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why),
        Ok(file) => file,
    };

    let mut ret = Vec::<Job>::new();

    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut line = line.split(',');
            let name = line.next().unwrap();
            let arrival_time = line.next().unwrap().parse::<u32>().unwrap();
            let service_time = line.next().unwrap().parse::<u32>().unwrap();
            let job = Job {
                name: name.to_string(),
                arrival_time,
                service_time,
                finish_time: None,
                whole_time: None,
                weight_whole_time: None,
                served_time: 0,
            };

            ret.push(job);
        }
    }

    ret
}

fn read_from_cmd() -> Vec<Job> {
    clr();

    let mut jobs = Vec::<Job>::new();

    loop {
        let mut name = String::new();
        println!("Input the job name: (use exit to break the input)");
        stdin().read_line(&mut name).unwrap();

        if name.trim() == "exit" {
            break;
        }

        let mut arrival_time = String::new();
        println!("Input the arrival time: ");
        stdin().read_line(&mut arrival_time).unwrap();

        let mut service_time = String::new();
        println!("Input the service time: ");
        stdin().read_line(&mut service_time).unwrap();

        let job = Job {
            name: name.trim().to_string(),
            arrival_time: arrival_time.trim().parse::<u32>().unwrap(),
            service_time: service_time.trim().parse::<u32>().unwrap(),
            finish_time: None,
            whole_time: None,
            weight_whole_time: None,
            served_time: 0,
        };

        jobs.push(job);
    }

    jobs
}

fn read_data() -> Vec<Job> {
    println!("Now we need you to input data, select one way:\n1.Read from file\n2.Input interactively");
    let mut method = String::new();
    stdin().read_line(&mut method).unwrap();

    match method.trim().parse::<u32>() {
        Ok(i) => {
            if i > 2 || i < 1 {
                println!("Invalid input. Please try again.");
                return read_data();
            }

            if i == 1 {
                return read_from_file();
            } else {
                return read_from_cmd();
            }
        }
        Err(..) => {
            println!("Invalid input. Please try again.");
            return read_data();
        }
    };
}

fn main() {
    clr();
    println!(
        "{}",
        "Welcome to Operating System Algorithms written in Rust.\n"
    );

    let jobs = read_data();
    let algo = select_algo();

    match algo {
        1 => {
            let mut scheduler = FcfsScheduler::new(jobs);
            scheduler.run();
            scheduler.print();
        }
        2 => {
            let mut scheduler = SjfScheduler::new(jobs);
            scheduler.run();
            scheduler.print();
        }
        3 => {
            println!("Enter rr slice num: ");
            let mut num = String::new();
            stdin().read_line(&mut num).unwrap();
            let slice_num: u32 = match num.trim().parse::<u32>() {
                Ok(num) => num,
                Err(..) => panic!("Invalid input. Please try again."),
            };

            let mut scheduler = RrScheduler::new(jobs);
            scheduler.run(slice_num);
            scheduler.print();
        }
        4 => {
            let mut fcfs_scheduler = FcfsScheduler::new(jobs.clone());
            fcfs_scheduler.run();
            fcfs_scheduler.print();

            let mut sjf_scheduler = SjfScheduler::new(jobs.clone());
            sjf_scheduler.run();
            sjf_scheduler.print();
        }
        _ => {}
    };

    println!("\n{}", "Press any key to continue.");
    let mut padding = String::new();
    stdin().read_line(&mut padding).unwrap();
    main();
}
