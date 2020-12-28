use crate::error::{TimebarError, TimebarResult};
use crate::helpers::{
  get_current_timestamp, get_percentage, get_time_left, print_bar, string_to_u32,
};
use std::io::stdin;
use std::process;
use std::thread;
use std::time::Duration;
use termion::{clear, color, cursor};

pub fn timer_handler() {
  print_timer_intro();

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
  display_timer(start, end);
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

fn print_timer_intro() {
  println!("Enter timer duration in the format of hour:minute:second.");
  println!(
    "For example: {}0:30:0{}",
    color::Fg(color::Green),
    color::Fg(color::White)
  );
}

fn display_timer(start: u64, end: u64) {
  println!("\n{}{}", clear::All, cursor::Goto(1, 1));
  loop {
    draw(start, end, "timer");
    thread::sleep(Duration::from_millis(1000));
    println!("\n{}{}{}", cursor::Show, clear::All, cursor::Goto(1, 1))
  }
}

fn draw(start: u64, end: u64, display_type: &str) {
  let percentage = get_percentage(start, end);
  let info = get_time_left(end, display_type.to_string()).unwrap();

  print!("{}", color::Fg(color::White));
  print_bar(&percentage);
  println!("\nTime is ticking... You have:\n");
  print!("{}", color::Fg(color::Green));
  println!("{} in hours", info.hours);
  println!("{} in minutes", info.minutes);
  println!(
    "{}{}{} in seconds.",
    color::Fg(color::Yellow),
    info.seconds,
    color::Fg(color::Green),
  );
}
