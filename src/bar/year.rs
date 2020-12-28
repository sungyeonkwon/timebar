use crate::helpers::{get_current_timestamp, get_percentage, print_bar};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use std::fmt;

pub fn year_handler() {
    let now = get_current_timestamp();
    let current_year = NaiveDateTime::from_timestamp(now as i64, 0).year();
    let start = NaiveDate::from_ymd(current_year, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp() as u64;
    let end = NaiveDate::from_ymd(current_year, 12, 31)
        .and_hms(23, 59, 59)
        .timestamp() as u64; // 1 sec difference

    // Display info
    display_year(start, end);
}

fn display_year(start: u64, end: u64) {
    let percentage = get_percentage(start, end);
    let message = get_message(percentage);
    print_bar(&percentage);
    println!("{}", message);
}

fn get_message(percentage: f64) -> Message {
    let q1 = Message::Q1("\nYou've got a lot left.".to_owned());
    let q2 = Message::Q2("\nAlmost Halfway through!".to_owned());
    let q3 = Message::Q3("\nMore than half of this year's gone.".to_owned());
    let q4 = Message::Q4("\nThis year's almost over!".to_owned());

    match percentage {
        _ if percentage <= 40.0 => q1,
        _ if percentage <= 58.0 => q2,
        _ if percentage <= 75.0 => q3,
        _ => q4,
    }
}

#[derive(Debug)]
enum Message {
    Q1(String),
    Q2(String),
    Q3(String),
    Q4(String),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::Q1(v) => v.fmt(f),
            Message::Q2(v) => v.fmt(f),
            Message::Q3(v) => v.fmt(f),
            Message::Q4(v) => v.fmt(f),
        }
    }
}
