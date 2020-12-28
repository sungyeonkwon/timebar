#[macro_use]
extern crate serde_derive;
extern crate preferences;
use preferences::AppInfo;

pub mod bar;
pub mod cmd;
pub mod error;
pub mod helpers;

#[cfg(test)]
mod tests;

const PERCENTAGE_SCALAR: f64 = 3.0;
const PREF_DDAY: &str = "timebar/ddays";
const PREF_LIFE: &str = "timebar/life";
const APP_INFO: AppInfo = AppInfo {
  name: "timebar",
  author: "Sung Kwon",
};
