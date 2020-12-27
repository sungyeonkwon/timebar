use crate::error::{TimebarError, TimebarResult};
use crate::helpers::Display;
use crate::helpers::{draw, get_current_timestamp, string_to_u32};
use std::io::stdin;
use std::process;
use termion::{clear, color, cursor};

pub fn timer_handler() {
  println!("Enter timer duration in the format of hour:minute:second.");
  println!(
    "For example: {}0:30:0{}",
    color::Fg(color::Green),
    color::Fg(color::White)
  );

  let mut args = String::new();
  stdin().read_line(&mut args).expect("Failed to read line");

  let config = TimerConfig::new(args.split(' ').collect()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  let start = get_current_timestamp();
  let duration = config.seconds + config.minutes * 60 + config.hours * 60 * 60;
  let end = start + duration as u64;

  // Prepare display
  println!("\n{}{}", clear::All, cursor::Goto(1, 1));
  draw(Display::Timer { start, end });
}

struct TimerConfig {
  hours: u32,
  minutes: u32,
  seconds: u32,
}

impl TimerConfig {
  pub fn new(args: Vec<&str>) -> TimebarResult<TimerConfig> {
    let mut iterator = args.into_iter();
    let timer = match iterator.next() {
      Some(arg) => arg,
      None => return Err(TimebarError::InvalidInput("duration".to_string())),
    };

    let timer_config: Vec<&str> = timer.split(':').collect();

    if timer_config.len() != 3 {
      return Err(TimebarError::InvalidDurationFormat);
    }

    let hours = string_to_u32(timer_config[0].clone().trim()).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });
    let minutes = string_to_u32(timer_config[1].clone().trim()).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });
    let seconds = string_to_u32(timer_config[2].clone().trim()).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    Ok(TimerConfig {
      hours,
      minutes,
      seconds,
    })
  }
}
