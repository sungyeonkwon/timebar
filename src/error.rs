use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
pub enum TimebarError {
    InvalidInput(String),
    IoError(String),
    InvalidInteger,
    InvalidCommand,
    InvalidDateFormat,
    InvalidDurationFormat,
}

impl fmt::Display for TimebarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TimebarError::*;
        match self {
            InvalidInput(input) => {
                write!(f, "This part of the input could not be parsed: '{}'", input)
            }
            IoError(err) => write!(f, "IO error: {}", err),
            InvalidDateFormat => write!(
                f,
                "Please enter a date in the correct format of date/month/year."
            ),
            InvalidDurationFormat => write!(
                f,
                "Please enter a duration in the correct format of hours:minutes:seconds."
            ),
            InvalidCommand => write!(f, "Sorry, the command does not exist."),
            InvalidInteger => write!(f, "Must provide a positive integer."),
        }
    }
}

impl Error for TimebarError {}

impl From<io::Error> for TimebarError {
    fn from(io_error: io::Error) -> Self {
        TimebarError::IoError(io_error.to_string())
    }
}

pub type TimebarResult<T = ()> = Result<T, TimebarError>;
