extern crate time;
extern crate clap;
extern crate colored;

use colored::*;
use clap::{Arg, App};
use std::io;
use std::io::Write;
use std::thread;
use time::Duration;

const DEFAULT_TASK_MINS: i64  = 25;
const DEFAULT_BREAK_MINS: i64 = 5;

fn main() {

    let matches = App::new("Pom")
        .version("1.0")
        .author("Franklin Van Nes <franklin.vannes@gmail.com>")
        .about("A command line pomodoro timer that logs your productivity.")
        .arg(Arg::with_name("Task Minutes")
             .short("t")
             .long("task-mins")
             .value_name("TASKLENGTH")
             .help("The length of time in minutes to complete your task.")
             .takes_value(true))
        .arg(Arg::with_name("Break Minutes")
             .short("b")
             .long("break-mins")
             .value_name("BREAKLENGTH")
             .help("The length of your break in minutes.")
             .takes_value(true))
        .arg(Arg::with_name("TASK")
             .help("The name of the task, wrapped in quotes")
             .required(false)
             .index(1)).get_matches();


    let task_name: String;
    let task_minutes: i64;
    let break_minutes: i64;

    if matches.is_present("TASK") {
        task_name = String::from(matches.value_of("TASK").unwrap());
    } else {
        task_name = get_task_name_from_user();
    }

    if matches.is_present("Task Minutes") { 
        task_minutes =  match matches.value_of("Task Minutes").unwrap().trim().parse() {
            Ok(mins)    => mins,
            Err(_)      => DEFAULT_TASK_MINS,
        };
    } else {
        task_minutes = get_task_length_from_user(&task_name);
    }

    if matches.is_present("Break Minutes") {
        break_minutes =  match matches.value_of("Break Minutes").unwrap().trim().parse() {
            Ok(mins)    => mins,
            Err(_)      => DEFAULT_BREAK_MINS,
        };

    } else {
        break_minutes = get_break_length_from_user();
    }
    println!("{}", "Starting Pom".red());

    println!("task mins: {}\tbreak mins: {}", task_minutes, break_minutes);
    
    let start_string = format!("Starting {}", task_name);
    println!("{}", start_string.green().bold());
    countdown(task_minutes);
    println!("Time for a {} minute break!", break_minutes);
    countdown(break_minutes);
    println!("Break is over! Back to work!");
}


fn get_task_name_from_user() -> String{
    let mut task_name = String::new();
    println!("What is your next task? ");
    println!("Task Name: ");
    io::stdin().read_line(&mut task_name)
        .expect("Are you trying to break this thing? This is why you can't have nice things.");
    task_name.trim();
    return task_name;
}

fn get_task_length_from_user(task_name: &String) -> i64 {

    println!("How long will it take to {}? (minutes) [default = 25]: ", task_name);

    let mut task_min_input  = String::new();

    io::stdin().read_line(&mut task_min_input)
        .expect("Failed to read line");
    
    return match task_min_input.trim().parse() {
        Ok(mins)    => mins,
        Err(_)      => DEFAULT_TASK_MINS,
    };
}


fn get_break_length_from_user() -> i64  {
    println!("How long will the following break be? (minutes) [default = 5]: ");

    let mut break_min_input = String::new();

    io::stdin().read_line(&mut break_min_input)
        .expect("Failed to read line");

    return match break_min_input.trim().parse() {
        Ok(mins)    => mins,
        Err(_)      => DEFAULT_BREAK_MINS,
    };
}

fn print_time(duration: Duration) {
    io::stdout().flush()
        .expect("Could not flush stdout");
    let timer = format!("\r{}:{}:{}   ", duration.num_hours(), duration.num_minutes() % 60, duration.num_seconds() % 60);
    print!("{}", timer.green().bold());
}

fn countdown(num_minutes: i64) {
    let mut now  = time::now();
    let end_time = now + Duration::minutes(num_minutes);
    while now < end_time {
        print_time(end_time - now);
        thread::sleep(std::time::Duration::from_secs(1));
        now = time::now();
    }
}
