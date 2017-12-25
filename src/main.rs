extern crate time;

use std::io;
use std::io::Write;
use std::thread;
use time::Duration;

const DEFAULT_TASK_MINS: i64  = 25;
const DEFAULT_BREAK_MINS: i64 = 5;

fn main() {
    println!("Starting Pomodoro...\n");

    println!("What is your next task? ");

    let mut task_name = String::new();

    println!("Task Name: ");

    io::stdin().read_line(&mut task_name)
        .expect("Are you trying to break this thing? This is why you can't have nice things.");

    let task_minutes;
    let break_minutes;

    loop {
        println!("How long will this take? (minutes) [default = 25]: ");

        let mut task_min_input  = String::new();
        let mut break_min_input = String::new();

        io::stdin().read_line(&mut task_min_input)
            .expect("Failed to read line");

       task_minutes = match task_min_input.trim().parse() {
            Ok(mins) => mins,
            Err(_)   => DEFAULT_TASK_MINS,
        };

        println!("How long will the following break be? (minutes) [default = 5]: ");


        io::stdin().read_line(&mut break_min_input)
            .expect("Failed to read line");

        break_minutes = match break_min_input.trim().parse() {
            Ok(mins)    => mins,
            Err(_)      => DEFAULT_BREAK_MINS,
        };
    

        println!("task mins: {}\tbreak mins: {}", task_minutes, break_minutes);
        break;

    } // End loop.

    println!("Starting {}", task_name);
    countdown(task_minutes);
    println!("Time for a {} minute break!", break_minutes);
    countdown(break_minutes);
    println!("Break is over! Back to work!");
}


fn print_time(duration: Duration) {
    io::stdout().flush()
        .expect("Could not flush stdout");
    print!("\r{}:{}:{}   ", duration.num_hours(), duration.num_minutes() % 60, duration.num_seconds() % 60);
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
