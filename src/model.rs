use std::str::FromStr;
use std::fmt::Display;

/// The year for the advent of code solutions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Year {
    Year2021,
    Year2022
}

impl FromStr for Year {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2021" => Ok(Self::Year2021),
            "2022" => Ok(Self::Year2022),
            _  => Err(anyhow::anyhow!("Year not implemented (yet) for advent of code! {}", s))
        }
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Year2021 => "2021",
            Self::Year2022 => "2022"
        };
        write!(f, "{}", output)
    }
}

/// The solution to the question that should be answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Question {
    First,
    Second,
    Both
}

impl FromStr for Question {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "first" | "First" | "FIRST" => Ok(Self::First),
            "second" | "Second" | "SECOND" => Ok(Self::Second),
            "both" | "Both" | "BOTH" => Ok(Self::Both),
            _  => Err(anyhow::anyhow!("Provided a Question type that isn't supported! {}", s))
        }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
