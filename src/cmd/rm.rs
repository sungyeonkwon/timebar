use crate::bar::dday::DdayConfig;
use crate::bar::life::LifeConfig;
use crate::{APP_INFO, PREF_DDAY, PREF_LIFE};
use preferences::Preferences;
use std::process;

pub fn remove_handler(args: Vec<String>) {
  if args.len() != 3 {
    // TODO: Re-prompt the d-day flow or use error object
    println!("Please enter valid arguments for rm command.");
    process::exit(1);
  }

  let option = &*args[2];
  match option {
    "-a" => remove_all(),
    _ => remove_name(option),
  }
}

fn remove_name(option: &str) {
  let existing_ddays = Vec::<DdayConfig>::load(&APP_INFO, PREF_DDAY).unwrap();
  let new_ddays = existing_ddays
    .clone()
    .into_iter()
    .filter(|dday| dday.name != option)
    .collect::<Vec<DdayConfig>>();

  let existing_lives = Vec::<LifeConfig>::load(&APP_INFO, PREF_LIFE).unwrap();
  let new_lives = existing_lives
    .clone()
    .into_iter()
    .filter(|dday| dday.name != option)
    .collect::<Vec<LifeConfig>>();

  if &new_ddays.len() != &existing_ddays.clone().len() {
    let saved = new_ddays.save(&APP_INFO, PREF_DDAY);
    assert!(saved.is_ok());
    println!("Successfully removed entry: {}", option);
  } else if &new_lives.len() != &existing_lives.clone().len() {
    let saved = new_lives.save(&APP_INFO, PREF_LIFE);
    assert!(saved.is_ok());
    println!("Successfully removed entry: {}", option);
  } else {
    println!("There is no saved entry under that name.");
  }
}

fn remove_lives() {
  let empty: Vec<LifeConfig> = Vec::new();
  let saved = empty.save(&APP_INFO, PREF_LIFE);
  assert!(saved.is_ok());
}

fn remove_ddays() {
  let empty: Vec<DdayConfig> = Vec::new();
  let saved = empty.save(&APP_INFO, PREF_DDAY);
  assert!(saved.is_ok());
}

fn remove_all() {
  remove_ddays();
  remove_lives();

  println!("Removed all of the saved entires in timebar.")
}
