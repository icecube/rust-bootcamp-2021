use std::env;
use std::fs;

struct Record {
    datetime: String,
    level: String,
    module: String,
    user: String,
    message: String,
}

fn load_logs(filename: &str) -> Vec<Record> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut records = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let (first, second) = line.split_at(line.find(": ").unwrap());
        let parts: Vec<&str> = first.split_whitespace().collect();
        records.push(Record{
            datetime: parts[0].to_string(),
            level: parts[1].to_string(),
            module: parts[2].to_string(),
            user: parts[4].to_string(),
            message: second[2..].to_string(),
        });
    }
    records
}

fn find_dglo_warn(logs: &Vec<Record>) {
    for message in logs.iter() {
        if message.level == "WARNING" && message.user == "dglo" {
            println!("{} {} {} - {}: {}", message.datetime, message.level, message.module, message.user, message.message);
        }
    }
}

fn find_time_lines(logs: &Vec<Record>, start: String, end: String) {
    for message in logs.iter() {
        if message.datetime >= start && message.datetime <= end {
            println!("{} {} {} - {}: {}", message.datetime, message.level, message.module, message.user, message.message);
        }
    }
}

fn find_mem_errors(logs: &Vec<Record>) {
    let mut mem = 0;
    let mut count = 0;
    for message in logs.iter() {
        if message.message.starts_with("memory error") {
            count += 1;
            let val: Vec<&str> = message.message.split_whitespace().collect();
            mem += val[2].parse::<u64>().unwrap();
        }
    }
    println!("Avg memory used: {} KB", mem/count);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Args: logfile starttime endtime");
        std::process::exit(1);
    }

    let logfilename = &args[1];

    let starttime = &args[2];
    let endtime = &args[3];

    let logs = load_logs(logfilename);
    println!("num lines = {}", logs.len());
    println!("---------");

    find_dglo_warn(&logs);

    println!("---------");

    find_time_lines(&logs, starttime.to_string(), endtime.to_string());

    println!("---------");

    find_mem_errors(&logs);
}
