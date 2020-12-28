extern crate termion;
use crate::error::{TimebarError, TimebarResult};
use crate::helpers::{draw, string_to_u32, Display};
use chrono::NaiveDate;
use std::io::stdin;
use std::process;
use termion::{clear, color, cursor};

pub fn life_handler() {
    print_life_intro();

    let mut args = String::new();
    stdin().read_line(&mut args).expect("Failed to read line");

    let config = LifeConfig::new(args.split(' ').collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let start = NaiveDate::from_ymd(config.year as i32, config.month, config.day)
        .and_hms(0, 0, 0)
        .timestamp() as u64;

    let lifespan = config.lifespan;
    let end = NaiveDate::from_ymd((config.year + lifespan) as i32, config.month, config.day)
        .and_hms(0, 0, 0)
        .timestamp() as u64;

    // Prepare display
    println!("\n{}{}", clear::All, cursor::Goto(1, 1));
    draw(Display::Life {
        start,
        end,
        lifespan,
    });
}

struct LifeConfig {
    day: u32,
    month: u32,
    year: u32,
    lifespan: u32,
}

impl LifeConfig {
    pub fn new(args: Vec<&str>) -> TimebarResult<LifeConfig> {
        let mut iterator = args.into_iter();
        let birthday = match iterator.next() {
            Some(arg) => arg,
            None => return Err(TimebarError::InvalidInput("birthday".to_string())),
        };

        let lifespan = match iterator.next() {
            Some(arg) => arg,
            None => return Err(TimebarError::InvalidInput("lifespan".to_string())),
        };

        let birthday: Vec<&str> = birthday.split('/').collect();

        if birthday.len() != 3 {
            return Err(TimebarError::InvalidDateFormat);
        }

        let day = string_to_u32(birthday[0].clone().trim()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let month = string_to_u32(birthday[1].clone().trim()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let year = string_to_u32(birthday[2].clone().trim()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let lifespan = string_to_u32(lifespan.clone().trim()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        Ok(LifeConfig {
            day,
            month,
            year,
            lifespan,
        })
    }
}

fn print_life_intro() {
    println!("Enter birthday and your expected lifespan.");
    println!(
        "For example: {}21/3/1985 85{}",
        color::Fg(color::Green),
        color::Fg(color::White)
    );
    println!("Note that the order is date/month/year, and your lifespan is separated by a space.");
}
