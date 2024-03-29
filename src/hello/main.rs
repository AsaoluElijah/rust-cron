use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::thread;

fn main() {
    let expression = "0/5 * * * * *"; // Run in 5 seconds
    let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

    for datetime in schedule.upcoming(Utc).take(10) {
        let now = Utc::now();
        let until = datetime - now;
        thread::sleep(until.to_std().unwrap());
        println!("Hello, world!"); // Task to be executed
    }
}
