extern crate termion;
use crate::error::{TimebarError, TimebarResult};
use crate::helpers::{get_percentage, get_time_left, limit_name, print_bar, string_to_u32};
use crate::{APP_INFO, PREF_LIFE};
use chrono::NaiveDate;
use preferences::Preferences;
use std::io::stdin;
use std::process;
use std::thread;
use std::time::Duration;
use termion::{clear, color, cursor};

pub fn life_handler(args: Vec<String>) {
    if args.len() == 3 {
        let option = &args[2];
        let save = String::from("-s");

        match option {
            _ if option == &save => life_flow(true),
            _ => display_by_name(option),
        }
    } else if args.len() == 2 {
        life_flow(false);
    } else {
        // TODO: Re-prompt the d-day flow or use error object
        println!("Please enter valid arguments.");
    }
}

fn display_by_name(option: &str) {
    let existing_lives = Vec::<LifeConfig>::load(&APP_INFO, PREF_LIFE).unwrap();

    // If there's a save entry with the name, display the result
    for life in existing_lives {
        if life.name == option {
            let (start, end, lifespan) = config_to_draw_input(life.clone());
            display_life(start, end, lifespan, &life.name);
            return;
        }
    }
    // Else, print the result.
    println!("There is no life entry saved on that name.");
}

fn config_to_draw_input(config: LifeConfig) -> (u64, u64, u32) {
    let start = NaiveDate::from_ymd(config.year as i32, config.month, config.day)
        .and_hms(0, 0, 0)
        .timestamp() as u64;

    let lifespan = config.lifespan;
    let end = NaiveDate::from_ymd((config.year + lifespan) as i32, config.month, config.day)
        .and_hms(0, 0, 0)
        .timestamp() as u64;

    (start, end, lifespan)
}

fn display_life(start: u64, end: u64, lifespan: u32, name: &str) {
    println!("\n{}{}", clear::All, cursor::Goto(1, 1));
    loop {
        draw(start, end, "life", lifespan, name);
        thread::sleep(Duration::from_millis(1000));
        println!("\n{}{}{}", cursor::Show, clear::All, cursor::Goto(1, 1));
    }
}

fn draw(start: u64, end: u64, display_type: &str, lifespan: u32, name: &str) {
    let percentage = get_percentage(start, end);
    let info = get_time_left(end, display_type.to_string()).unwrap();

    print!("{}", color::Fg(color::White));
    print_bar(&percentage);
    println!(
        "\n{} is expected to exist on this planet for {} years.",
        name, lifespan
    );
    println!("\nTime remaining:");
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
    println!("\n{}Have a good day!", color::Fg(color::White));
}

fn life_flow(should_save: bool) {
    print_life_intro();

    let mut args = String::new();
    stdin().read_line(&mut args).expect("Failed to read line");

    let config = LifeConfig::new(args.split(' ').collect()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Save the entry
    if should_save {
        let mut lives = Vec::<LifeConfig>::load(&APP_INFO, PREF_LIFE).unwrap_or(Vec::new());
        lives.push(config.clone());
        let save_result = lives.save(&APP_INFO, PREF_LIFE);
        assert!(save_result.is_ok());
    }

    // Display the entry
    let (start, end, lifespan) = config_to_draw_input(config.clone());
    display_life(start, end, lifespan, &config.name);
}

// Deriving `Serialize` and `Deserialize` on a struct/enum automatically
// implements the `Preferences` trait.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LifeConfig {
    pub name: String,
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub lifespan: u32,
}

impl LifeConfig {
    pub fn new(args: Vec<&str>) -> TimebarResult<LifeConfig> {
        let mut iterator = args.into_iter();
        let name = match iterator.next() {
            Some(arg) => limit_name(arg),
            None => return Err(TimebarError::InvalidInput("name".to_string())),
        };

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
            name,
            day,
            month,
            year,
            lifespan,
        })
    }
}

fn print_life_intro() {
    println!("Enter your name, birthday and your expected lifespan.");
    println!(
        "For example: {}Brian 21/3/1985 101{}",
        color::Fg(color::Green),
        color::Fg(color::White)
    );
}
