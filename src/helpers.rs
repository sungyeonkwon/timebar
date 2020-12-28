use crate::error::{TimebarError, TimebarResult};
use crate::PERCENTAGE_SCALAR;
use std::process;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use termion::color;

pub enum CommandType {
  Year,
  Life,
  Timer,
  Dday,
  List,
  Remove,
}

impl FromStr for CommandType {
  type Err = TimebarError;

  fn from_str(input: &str) -> TimebarResult<CommandType> {
    let lower_input = input.to_lowercase();

    match &*lower_input {
      "year" => Ok(CommandType::Year),
      "life" => Ok(CommandType::Life),
      "timer" => Ok(CommandType::Timer),
      "dday" => Ok(CommandType::Dday),
      "ls" => Ok(CommandType::List),
      "rm" => Ok(CommandType::Remove),
      _ => Err(TimebarError::InvalidCommand),
    }
  }
}

pub enum Display {
  Life { start: u64, end: u64, lifespan: u32 },
  Timer { start: u64, end: u64 },
}

pub fn string_to_u32(trimmed: &str) -> TimebarResult<u32> {
  match trimmed.parse::<u32>() {
    Ok(i) => Ok(i),
    Err(_error) => Err(TimebarError::InvalidInteger),
  }
}

pub fn limit_name(name: &str) -> String {
  if name.len() <= 12 {
    return name.to_string();
  }

  let modified_name = &name[0..12];
  println!(
    "Name must be less than 12 characters. It will be named as: {}",
    &modified_name
  );

  modified_name.to_string()
}

pub fn get_current_timestamp() -> u64 {
  let start = SystemTime::now();
  let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");

  since_the_epoch.as_secs()
}

pub fn get_percentage(start: u64, end: u64) -> f64 {
  let today = get_current_timestamp();
  let passed = today - start;
  let duration = end - start;

  passed as f64 / duration as f64 * 100.0
}

pub fn print_bar(percentage: &f64) {
  let (filled, empty) = get_filled_empty(&percentage);
  println!(
    "\n{}{} {:.1}%",
    "▓".repeat(filled),
    "░".repeat(empty),
    percentage,
  );
}

pub fn get_filled_empty(percentage: &f64) -> (usize, usize) {
  let filled = (percentage / PERCENTAGE_SCALAR).ceil() as usize;
  let empty = ((100.0 - percentage) / PERCENTAGE_SCALAR).ceil() as usize;

  (filled, empty)
}

pub fn seconds_to_days(seconds: u64) -> u64 {
  seconds / 60 / 60 / 24
}

pub fn get_time_left(end: u64, display_type: String) -> TimebarResult<TimeLeft> {
  let now = get_current_timestamp();

  if end <= now {
    if display_type == "life" {
      return Err(TimebarError::InvalidInput("lifespan".to_string()));
    } else {
      println!(
        "{}Time's up! Done.{}",
        color::Fg(color::Red),
        color::Fg(color::White)
      );
    }
    process::exit(1);
  }

  let seconds = (end as i64 - now as i64) as u32;
  let minutes = seconds / 60 as u32;
  let hours = minutes / 60 as u32;
  let days = hours / 24 as u32;
  let weeks = days / 7 as u32;
  // For an approximate result, divide the time value by 30.417
  let months = (days as f64 / 30.417).round() as u32;
  let years = days / 365 as u32;

  Ok(TimeLeft {
    years,
    months,
    weeks,
    days,
    hours,
    minutes,
    seconds,
  })
}

#[derive(Copy, Clone)]
pub struct TimeLeft {
  pub years: u32,
  pub months: u32,
  pub weeks: u32,
  pub days: u32,
  pub hours: u32,
  pub minutes: u32,
  pub seconds: u32,
}
