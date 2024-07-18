use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default)]
pub enum Event {
    #[default]
    Nil,
    Ping,
    InternalError,
    Action,
    Data,
    Unknown,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "NIL"),
            Self::Ping => write!(f, "PING"),
            Self::InternalError => write!(f, "INTERNAL_ERROR"),
            Self::Action => write!(f, "ACTION"),
            Self::Data => write!(f, "DATA"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let task = input_str
            .split('|')
            .next()
            .map(|task_str| match task_str {
                "PING" => Self::Ping,
                "ACTION" => Self::Action,
                "DATA" => Self::Data,
                "INTERNAL_ERROR" => Self::InternalError,
                _ => Self::Unknown,
            })
            .ok_or(anyhow::anyhow!("Invalid syntax: [{input_str:?}]"))?;
        Ok(task)
    }
}
