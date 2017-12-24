extern crate time;

use std::io;
use std::thread;
use time::Duration;

const DEFAULT_TASK_MINS: u32 = 25;
const DEFAULT_BREAK_MINS: u32 = 5;

fn main() {
    println!("Starting Pomodoro...\n");

    println!("What is your next task?");

    let mut task_name = String::new();

    println!("Task Name: ");

    io::stdin().read_line(&mut task_name)
        .expect("Are you trying to break this thing? This is why you can't have nice things.");

    let mut task_minutes  = 25;
    let mut break_minutes = 5;

    loop {
        println!("How long will this take? (minutes) [default = 25]: ");

        let mut task_min_input  = String::new();
        let mut break_min_input = String::new();

        io::stdin().read_line(&mut task_min_input)
            .expect("Failed to read line");

       task_minutes = match task_min_input.trim().parse() {
            Ok(mins) => mins,
            Err(_)   => continue,
        };

        println!("How long will the following break be? (minutes) [default = 5]: ");


        io::stdin().read_line(&mut break_min_input)
            .expect("Failed to read line");

        break_minutes = match break_min_input.trim().parse() {
            Ok(mins)    => mins,
            Err(_)      => continue,
        };
    

        println!("task mins: {}\tbreak mins: {}", task_minutes, break_minutes);
        break;

    } // End loop.

    println!("Starting {}", task_name);

    let end_time = time::get_time() + Duration::seconds(60 * task_minutes);
    while time::get_time() < end_time {
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Time remaining: {}", end_time - time::get_time());
    }

}
