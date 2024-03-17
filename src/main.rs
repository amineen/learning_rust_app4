#![allow(unused)]
extern crate chrono;

use chrono::{DateTime, Local, NaiveDateTime, Utc};
fn main() {
    let current_time: DateTime<Utc> = Utc::now();

    let iso_time_string = current_time.to_rfc3339();

    let datetime_array = iso_time_string.split('T').collect::<Vec<&str>>();

    println!("Time: {}", datetime_array[1]);

    println!("Current time is: {}", current_time);
    println!("{}", iso_time_string);
}
