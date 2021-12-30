use std::{fmt::Display, io};

#[derive(Debug)]
pub struct StatsError {
    pub message: String,
}

impl Display for StatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&str> for StatsError {
    fn from(s: &str) -> Self {
        StatsError {
            message: s.to_string(),
        }
    }
}

impl From<io::Error> for StatsError {
    fn from(e: io::Error) -> Self {
        StatsError {
            message: e.to_string(),
        }
    }
}

impl From<std::num::TryFromIntError> for StatsError {
    fn from(_: std::num::TryFromIntError) -> Self {
        StatsError {
            message: "Number conversion error".to_string(),
        }
    }
}
