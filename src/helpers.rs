use crate::error::{TimebarError, TimebarResult};
use crate::PERCENTAGE_SCALAR;
use std::process;
use std::str::FromStr;
use std::thread;
use std::time;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use termion::{clear, color, cursor};

pub enum BarType {
  Year,
  Life,
  Timer,
}

impl FromStr for BarType {
  type Err = TimebarError;

  fn from_str(input: &str) -> TimebarResult<BarType> {
    let lower_input = input.to_lowercase();

    match &*lower_input {
      "year" => Ok(BarType::Year),
      "life" => Ok(BarType::Life),
      "timer" => Ok(BarType::Timer),
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

// Called every 1 second to update the display info.
pub fn draw(display: Display) {
  loop {
    match display {
      Display::Life {
        start,
        end,
        lifespan,
      } => display_info(start, end, Some(lifespan)),
      Display::Timer { start, end } => display_info(start, end, None),
    }

    thread::sleep(time::Duration::from_millis(1000));
    println!("\n{}{}{}", cursor::Show, clear::All, cursor::Goto(1, 1));
  }
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

fn display_info(start: u64, end: u64, lifespan: Option<u32>) {
  let percentage = get_percentage(start, end);
  let is_life = lifespan.is_some();
  let info = calculate_time_left(end, is_life).unwrap();

  print!("{}", color::Fg(color::White));
  print_bar(&percentage);

  if is_life {
    println!(
      "\nYou expect to exist on this planet for {} years.",
      lifespan.unwrap_or(0)
    );
    println!("\nYou have:");

    println!("{}", color::Fg(color::Green));
    println!("{} in years", info.years);
    println!("{} in months", info.months);
    println!("{} in weeks", info.weeks);
    println!("{} in days", info.days);
  } else {
    println!("\nTime is ticking... You have:");
  }
  print!("{}", color::Fg(color::Green));
  println!("{} in hours", info.hours);
  println!("{} in minutes", info.minutes);
  println!(
    "{}{}{} in seconds.",
    color::Fg(color::Yellow),
    info.seconds,
    color::Fg(color::Green)
  );

  if is_life {
    print!("{}", color::Fg(color::White));
    println!("\nHave a good day!");
  }
}

fn calculate_time_left(end: u64, is_life: bool) -> TimebarResult<TimeLeft> {
  let now = get_current_timestamp();

  if end <= now {
    if is_life {
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
struct TimeLeft {
  years: u32,
  months: u32,
  weeks: u32,
  days: u32,
  hours: u32,
  minutes: u32,
  seconds: u32,
}
