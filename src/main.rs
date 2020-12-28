use std::env;
use std::str::FromStr;
use timebar::bar::dday::dday_handler;
use timebar::bar::life::life_handler;
use timebar::bar::timer::timer_handler;
use timebar::bar::year::year_handler;
use timebar::cmd::ls::list_handler;
use timebar::cmd::rm::remove_handler;
use timebar::helpers::CommandType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = CommandType::from_str(&args[1]);

    match cmd {
        Ok(cmd) => match cmd {
            CommandType::Dday => dday_handler(args),
            CommandType::Life => life_handler(args),
            CommandType::List => list_handler(),
            CommandType::Remove => remove_handler(args),
            CommandType::Timer => timer_handler(),
            CommandType::Year => year_handler(),
        },
        Err(_error) => println!("{}", _error.to_string()),
    };
}
