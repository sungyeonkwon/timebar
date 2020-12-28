use crate::bar::dday::DdayConfig;
use crate::bar::life::LifeConfig;
use crate::{APP_INFO, PREF_DDAY, PREF_LIFE};
use preferences::Preferences;

pub fn remove_handler(args: Vec<String>) {
  // TODO: error handlining: args length

  let option = &*args[2];
  match option {
    "-a" => remove_all(),
    "life" => remove_life(true),
    _ => remove_name(option),
  }
}

fn remove_life(print_message: bool) {
  let empty: Vec<LifeConfig> = Vec::new();
  let saved = empty.save(&APP_INFO, PREF_LIFE);
  assert!(saved.is_ok());

  if print_message {
    println!("Successfully removed life entry");
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

fn remove_all() {
  // Remove d-day entries
  let empty: Vec<DdayConfig> = Vec::new();
  let saved = empty.save(&APP_INFO, PREF_DDAY);
  assert!(saved.is_ok());

  // Remove life entry
  remove_life(false);

  println!("Removed all of the saved entires in timebar.")
}
