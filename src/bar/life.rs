extern crate termion;
use crate::helpers::{get_current_timestamp, get_percentage, print_bar};
use chrono::NaiveDate;
use std::io::stdin;
use std::process;
use std::thread;
use std::time;
use termion::{clear, color, cursor};

pub fn life_handler() {
    println!("Enter birthday and your expected lifespan.");
    println!(
        "For example: {}21/3/1985 85{}",
        color::Fg(color::Green),
        color::Fg(color::White)
    );

    println!(
        "Please note that the order is date/month/year, and your lifespan is separated by a space."
    );

    let mut args = String::new();

    stdin().read_line(&mut args).expect("Failed to read line");

    let config = LifeConfig::new(args.split(' ').collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let birth_ts = NaiveDate::from_ymd(config.year as i32, config.month, config.day)
        .and_hms(0, 0, 0)
        .timestamp() as u64;

    let lifespan = config.lifespan;
    let death_year = config.year + lifespan;
    let death_ts = NaiveDate::from_ymd(death_year as i32, config.month, config.day)
        .and_hms(0, 0, 0)
        .timestamp() as u64;

    println!("\n{}{}", clear::All, cursor::Goto(1, 1));
    draw(birth_ts, death_ts, lifespan);
}

pub fn string_to_u32(trimmed: &str) -> Result<u32, &'static str> {
    match trimmed.parse::<u32>() {
        Ok(i) => Ok(i),
        Err(error) => panic!("Must provide a positive integer: {}", error),
    }
}

struct LifetimeInfo {
    lifespan: u32,
    years: u32,
    months: u32,
    weeks: u32,
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

struct LifeConfig {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub lifespan: u32,
}

impl LifeConfig {
    pub fn new(args: Vec<&str>) -> Result<LifeConfig, &'static str> {
        let mut iterator = args.into_iter();
        let birthday = match iterator.next() {
            Some(arg) => arg,
            None => return Err("Couldn't get a valid birthday"),
        };

        let lifespan = match iterator.next() {
            Some(arg) => arg,
            None => return Err("Couldn't get a valid lifespan"),
        };

        let birthday: Vec<&str> = birthday.split('/').collect();

        if birthday.len() != 3 {
            panic!("Birthday and lifespan must be in a correct format");
        }

        let day = string_to_u32(birthday[0].clone().trim()).unwrap();
        let month = string_to_u32(birthday[1].clone().trim()).unwrap();
        let year = string_to_u32(birthday[2].clone().trim()).unwrap();
        let lifespan = string_to_u32(lifespan.clone().trim()).unwrap();

        Ok(LifeConfig {
            day,
            month,
            year,
            lifespan,
        })
    }
}

fn display(info: LifetimeInfo) {
    println!("{}", color::Fg(color::White));
    println!(
        "You expect to exist on this planet for {} years.",
        info.lifespan
    );
    println!("\nYou have:");

    println!("{}", color::Fg(color::Green));
    println!("{} in years", info.years);
    println!("{} in months", info.months);
    println!("{} in weeks", info.weeks);
    println!("{} in days", info.days);
    println!("{} in hours", info.hours);
    println!("{} in minutes", info.minutes);
    println!(
        "{}{}{} in seconds.",
        color::Fg(color::Yellow),
        info.seconds,
        color::Fg(color::Green)
    );

    println!("{}", color::Fg(color::White));
    println!("Have a good day!");
}

fn calculate_leftovers(death_ts: u64, lifespan: u32) -> Result<LifetimeInfo, &'static i32> {
    let now = get_current_timestamp();

    if death_ts <= now {
        panic!("Your expected death doesn't seem to be in the future");
    }

    let seconds = (death_ts as i64 - now as i64) as u32;
    let minutes = seconds / 60 as u32;
    let hours = minutes / 60 as u32;
    let days = hours / 24 as u32;
    let weeks = days / 7 as u32;
    // For an approximate result, divide the time value by 30.417
    let months = (days as f64 / 30.417).round() as u32;
    let years = days / 365 as u32;

    Ok(LifetimeInfo {
        lifespan,
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
    })
}

// Draw every 1 second
fn draw(birth_ts: u64, death_ts: u64, lifespan: u32) {
    loop {
        let percentage = get_percentage(birth_ts, death_ts);
        let info = calculate_leftovers(death_ts, lifespan).unwrap();
        print_bar(percentage);
        display(info);
        thread::sleep(time::Duration::from_millis(1000));
        println!("\n{}{}{}", cursor::Show, clear::All, cursor::Goto(1, 1),);
    }
}
