use crate::bar::dday::DdayConfig;
use crate::bar::life::LifeConfig;
use crate::{APP_INFO, PREF_DDAY, PREF_LIFE};
use preferences::Preferences;

pub fn list_handler() {
  let existing_ddays = Vec::<DdayConfig>::load(&APP_INFO, PREF_DDAY).unwrap_or(Vec::new());
  let lives = Vec::<LifeConfig>::load(&APP_INFO, PREF_LIFE).unwrap_or(Vec::new());

  display_table(existing_ddays, lives);
}

fn display_table(ddays: Vec<DdayConfig>, lives: Vec<LifeConfig>) {
  if ddays.clone().len() == 0 && lives.clone().len() == 0 as usize {
    println!("No saved entries found in timebar.");
    return;
  }

  // Print header
  println!("{0: <11} | {1: <11} | {2}", "TYPE", "NAME", "DURATION");

  // Print lives
  for life in lives {
    println!(
      "{0: <11} | {1: <11} | {2} years",
      "life", life.name, life.lifespan
    );
  }

  // Print d-days
  for dday in ddays {
    println!(
      "{0: <11} | {1: <11} | {2} - {3} ",
      "d-day",
      dday.name,
      dday.start_date.to_string().trim(),
      dday.end_date.to_string().trim(),
    );
  }
}
