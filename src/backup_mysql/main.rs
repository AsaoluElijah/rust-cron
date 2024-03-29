use chrono::Local;
use cron::Schedule;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let expression = "0 0 2 * * *"; // Every day at 2 AM
    let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

    loop {
        let now = Local::now();
        if let Some(datetime) = schedule.upcoming(Local).take(1).next() {
            let until = datetime - now;
            thread::sleep(until.to_std().unwrap());

            let date_str = Local::now().format("%Y_%m_%d").to_string();
            let backup_file_name = format!("backup_{}.sql", date_str);

            // Replace `your_database_name`, `your_username`, and `your_password` accordingly
            let command = format!(
                "mysqldump -u elijah -ppassword UserDatabase > {}",
                backup_file_name
            );

            match Command::new("sh").arg("-c").arg(command).output() {
                Ok(output) => {
                    if output.status.success() {
                        println!("Backup created successfully: {}", backup_file_name);
                    } else {
                        eprintln!("Error creating backup");
                    }
                }
                Err(e) => eprintln!("Failed to execute command: {}", e),
            }
        }
    }
}
