use chrono::{Datelike, Duration, Local, NaiveDate, Timelike};
use dotenv::dotenv;
use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Debug)]
struct User {
    name: String,
    dob: NaiveDate, // Format: YYYY-MM-DD
    phone_number: String,
}

#[derive(Serialize)]
struct TwilioMessage {
    From: String,
    To: String,
    Body: String,
}

async fn send_sms(
    user: &User,
    from: &str,
    account_sid: &str,
    auth_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        account_sid
    );

    let msg = TwilioMessage {
        From: from.to_string(),
        To: user.phone_number.clone(),
        Body: format!("Happy Birthday, {}!", user.name),
    };

    let response = client
        .post(&url)
        .basic_auth(account_sid, Some(auth_token))
        .form(&msg)
        .send()
        .await?;

    if !response.status().is_success() {
        let err_body = response.text().await?;
        println!("Failed to send message. Error: {}", err_body);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "SMS send failed",
        )));
    } else {
        println!("Message sent successfully!");
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let account_sid = env::var("TWILIO_ACCOUNT_SID").unwrap();
    let auth_token = env::var("TWILIO_AUTH_TOKEN").unwrap();
    let twilio_number = env::var("TWILIO_NUMBER").unwrap();

    let users = vec![
        User {
            name: "Alice".into(),
            dob: NaiveDate::from_ymd_opt(1990, 3, 29).expect("Invalid date"),
            phone_number: "+12345678901".into(),
        },
        User {
            name: "Bob".into(),
            dob: NaiveDate::from_ymd_opt(1992, 6, 22).expect("Invalid date"),
            phone_number: "+10987654321".into(),
        },
    ];

    loop {
        let today = Local::now().naive_local();
        for user in users.iter() {
            if user.dob.month() == today.month() && user.dob.day() == today.day() {
                if let Err(e) = send_sms(user, &twilio_number, &account_sid, &auth_token).await {
                    println!("Error sending SMS: {:?}", e);
                }
            }
        }

        let now = Local::now();
        // Adjusting to avoid direct use of deprecated .date()
        let next_day_start = (now + Duration::try_days(1).unwrap())
            .with_hour(0)
            .unwrap()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            + Duration::try_seconds(1).unwrap(); // Direct use of seconds, assuming no overflow concern

        let duration_until_next_day = next_day_start.signed_duration_since(now);
        let tokio_sleep_duration = tokio::time::Duration::from_secs(
            duration_until_next_day.num_seconds().try_into().unwrap(),
        );
        tokio::time::sleep(tokio_sleep_duration).await;
    }
}
