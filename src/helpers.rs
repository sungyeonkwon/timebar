use crate::PERCENTAGE_SCALAR;
use std::str::FromStr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub enum BarType {
  Year,
  Life,
}

impl FromStr for BarType {
  type Err = ();

  fn from_str(input: &str) -> Result<BarType, Self::Err> {
    let lower_input = input.to_lowercase();
    if let "year" = &*lower_input {
      Ok(BarType::Year)
    } else if let "life" = &*lower_input {
      Ok(BarType::Life)
    } else {
      Err(())
    }
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

pub fn print_bar(percentage: f64) {
  let filled = (percentage / PERCENTAGE_SCALAR).round() as usize;
  let empty = ((100.0 - percentage) / PERCENTAGE_SCALAR).round() as usize;
  println!(
    "{}{} {:.1}%",
    "▓".repeat(filled),
    "░".repeat(empty),
    percentage,
  );
}
