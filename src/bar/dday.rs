use crate::error::{TimebarError, TimebarResult};
use crate::helpers::{
  get_current_timestamp, get_percentage, limit_name, print_bar, seconds_to_days, string_to_u32,
};
use crate::{APP_INFO, PREF_DDAY};
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use preferences::Preferences;
use std::io::stdin;
use std::process;
use termion::color;

pub fn dday_handler(args: Vec<String>) {
  if args.len() == 3 {
    let option = &args[2];
    let save = String::from("-s");

    match option {
      _ if option == &save => dday_flow(true),
      _ => display_by_name(option),
    }
  } else if args.len() == 2 {
    dday_flow(false);
  } else {
    // TODO: Re-prompt the d-day flow or use error object
    println!("Please enter valid arguments.");
  }
}

fn dday_flow(should_save: bool) {
  print_dday_intro();

  let mut prompt_args = String::new();

  stdin()
    .read_line(&mut prompt_args)
    .expect("Failed to read line");

  let config = DdayConfig::new(prompt_args.split(' ').collect()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  let new_dday = DdayConfig {
    name: config.name,
    start_date: config.start_date,
    end_date: config.end_date,
  };

  // Save the entry
  if should_save {
    let mut ddays = Vec::<DdayConfig>::load(&APP_INFO, PREF_DDAY).unwrap();
    ddays.push(new_dday.clone());
    let save_result = ddays.save(&APP_INFO, PREF_DDAY);
    assert!(save_result.is_ok());
  }

  // Display the entry
  display_dday(new_dday);
}

fn display_dday(dday: DdayConfig) {
  let start = string_to_date(dday.start_date).timestamp() as u64;
  let end = string_to_date(dday.end_date).timestamp() as u64;
  let percentage = get_percentage(start, end);
  let today = get_current_timestamp();
  let days_gone = seconds_to_days(today - start);
  let days_left = seconds_to_days(end - today);
  print_bar(&percentage);
  println!(
    "\n{}: {} days are gone, {} days to go!",
    dday.name, days_gone, days_left
  );
}

fn display_by_name(option: &str) {
  let existing_ddays = Vec::<DdayConfig>::load(&APP_INFO, PREF_DDAY).unwrap();

  // If there's a save entry with the name, display the result
  for dday in existing_ddays {
    if dday.name == option {
      display_dday(dday);
      return;
    }
  }
  // Else, print the result.
  println!("There is no d-day entry saved on that name.");
}

fn string_to_date(date_string: String) -> NaiveDateTime {
  let date: Vec<&str> = date_string.split('/').collect();

  let day = string_to_u32(date[0].clone().trim()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  let month = string_to_u32(date[1].clone().trim()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  let year = string_to_u32(date[2].clone().trim()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  NaiveDate::from_ymd(year as i32, month, day).and_hms(0, 0, 0)
}

fn validate_date(arg: &str, end_date: &str) -> TimebarResult<String> {
  if arg.split('/').collect::<Vec<&str>>().len() != 3 {
    return Err(TimebarError::InvalidDateFormat);
  }

  if end_date.len() >= 1 {
    // Passed from validating start_date
    // start_date timestamp must be smaller than end_date.
    let start_ts = string_to_date(arg.to_string()).timestamp() as u64;
    let end_ts = string_to_date(end_date.to_string()).timestamp() as u64;
    if start_ts >= end_ts {
      return Err(TimebarError::InvalidDateRange);
    }
    // start_date timestamp must be smaller than end_date.
    if start_ts > get_current_timestamp() {
      return Err(TimebarError::InvalidDateRange);
    }
  } else {
    // Passed from validating end_date
    // end_date timestamp must be bigger than current timestamp.
    let end_ts = string_to_date(arg.to_string()).timestamp() as u64;
    if end_ts <= get_current_timestamp() {
      return Err(TimebarError::InvalidDateRange);
    }
  }

  Ok(arg.trim().to_string())
}

// Deriving `Serialize` and `Deserialize` on a struct/enum automatically
// implements the `Preferences` trait.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DdayConfig {
  pub name: String,
  pub start_date: String,
  pub end_date: String,
}

impl DdayConfig {
  pub fn new(args: Vec<&str>) -> TimebarResult<DdayConfig> {
    let mut iterator = args.into_iter();
    let name = match iterator.next() {
      Some(arg) => limit_name(arg),
      None => return Err(TimebarError::InvalidInput("name".to_string())),
    };

    let end_date = match iterator.next() {
      Some(arg) => validate_date(arg, ""),
      None => return Err(TimebarError::InvalidInput("d-day".to_string())),
    };
    let end_date = end_date.unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    let start_date = match iterator.next() {
      Some(arg) => validate_date(arg, &end_date),
      None => {
        // If start date is absent, take it as starting from today
        let today = Utc::today();
        let string_list = vec![
          today.day().to_string(),
          today.month().to_string(),
          today.year().to_string(),
        ];
        let joined = string_list.join("/");

        Ok(joined)
      }
    };
    let start_date = start_date.unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    Ok(DdayConfig {
      name,
      end_date,
      start_date,
    })
  }
}

fn print_dday_intro() {
  println!("Enter a name and d-day separated by space.");
  println!(
    "Should the counting start from a date that is not today, provide an optional argument."
  );
  println!(
    "Example 1: {}launch 31/1/2022{}",
    color::Fg(color::Green),
    color::Fg(color::White)
  );
  println!(
    "Example 2: {}launch 31/1/2022 31/1/2020{}",
    color::Fg(color::Green),
    color::Fg(color::White)
  );
}
