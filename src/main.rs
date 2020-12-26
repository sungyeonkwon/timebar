use std::env;
use std::str::FromStr;
use timebar::bar::dday::dday_handler;
use timebar::bar::life::life_handler;
use timebar::bar::year::year_handler;
use timebar::helpers::BarType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bar = BarType::from_str(&args[1]).unwrap();

    match bar {
        BarType::Year => year_handler(),
        BarType::Life => life_handler(),
        BarType::Dday => dday_handler(),
    };
}
