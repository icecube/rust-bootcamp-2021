use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Trace {
    timestamp: String,
    level: String,
    logger: String,
    user: String,
    message: String
}

fn get_arg(args: &Vec<String>, arg: &str) -> String {
    for i in 0..args.len() {
        if args[i] == arg {
            return String::from(&args[i+1])
        }
    }
    return String::from("error")
}

fn has_arg(args: &Vec<String>, arg: &str) -> bool {
    for i in args.iter() {
        if i == arg { return true; }
    }
    return false;
}

fn parse_memory_error(trace: &Trace) -> u64 {
    let memory_error: Vec<&str> = trace.message.split_whitespace().collect();
    let memory_used = memory_error[2].parse::<u64>().unwrap();
    return memory_used;
}

// "2021-04-07T04:02:29.476935 INFO tracker - aludez: 10 jobs processed in last minute"
fn parse_trace(line: &str) -> Trace {
    // ["2021-04-07T04:02:29.476935 INFO tracker" "aludez: 10 jobs processed in last minute"]
    let chop: Vec<&str> = line.split(" - ").collect();
    // ["2021-04-07T04:02:29.476935" "INFO" "tracker"]
    let meta: Vec<&str> = chop[0].split_whitespace().collect();
    // "aludez: 10 jobs processed in last minute"
    let mess = chop[1];

    // "aludez: 10 jobs processed in last minute"
    let first_colon = mess.find(":").unwrap();
    // "aludez"
    let user = &mess[0..first_colon];
    // " 10 jobs processed in last minute".trim()
    let message = &mess[(first_colon+1)..].trim();

    Trace {
        timestamp: meta[0].to_string(), // "2021-04-07T04:02:29.476935"
        level: meta[1].to_string(),     // "INFO"
        logger: meta[2].to_string(),    // "tracker"
        user: user.to_string(),         // "aludez"
        message: message.to_string()    // "10 jobs processed in last minute"
    }
}

fn print_trace(trace: &Trace) {
    println!("{} {} {} - {}: {}", trace.timestamp, trace.level, trace.logger, trace.user, trace.message);
}

fn print_usage() {
    println!("usage: logfile [--after TIME] [--before TIME] [--level LEVEL] [--logger LOGGER] [--user USER] [--memory] logfile");
    println!("");
    println!("positional arguments:");
    println!("  log              log file to parse");
    println!("");
    println!("optional arguments:");
    println!("  --after TIME     display traces after timestamp");
    println!("  --before TIME    display traces before timestamp");
    println!("  --level LEVEL    display traces at log level");
    println!("  --logger LOGGER  display traces from a specific logger");
    println!("  --user USER      display traces from user");
    println!("  --memory         compute average memory error");
}

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    // if we have none, usage and bail
    if args.len() == 1 {
        print_usage();
        exit(0);
    }

    // read the log into memory and parse the traces
    let filename = &args[args.len()-1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut traces: Vec<Trace> = Vec::<Trace>::new();
    for line in lines {
        traces.push(parse_trace(line));
    }

    // time to thin the herd a bit
    if has_arg(&args, "--after") {
        let timestamp = get_arg(&args, "--after");
        traces.retain(|x| x.timestamp > timestamp);
    }
    if has_arg(&args, "--before") {
        let timestamp = get_arg(&args, "--before");
        traces.retain(|x| x.timestamp < timestamp);
    }
    if has_arg(&args, "--level") {
        let level = get_arg(&args, "--level");
        traces.retain(|x| x.level == level);
    }
    if has_arg(&args, "--logger") {
        let logger = get_arg(&args, "--logger");
        traces.retain(|x| x.logger == logger);
    }
    if has_arg(&args, "--user") {
        let user = get_arg(&args, "--user");
        traces.retain(|x| x.user == user);
    }

    // display the remaining traces (if any)
    for trace in traces.iter() {
        print_trace(&trace)
    }

    // if we have to compute memory error statistics, do that now
    if has_arg(&args, "--memory") {
        let mut count: u32 = 0;
        let mut memory: u64 = 0;
        for trace in traces.iter() {
            if trace.message.starts_with("memory error") {
                count = count + 1;
                memory += parse_memory_error(&trace);
            }
        }
        println!("Average memory error is: {} KB", (memory as f32)/(count as f32));
    }
}
