use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    Nil,
    Unknown,
    Reboot,
    LedTurnOn,
    LedTurnOff,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "NIL"),
            Self::Reboot => write!(f, "REBOOT"),
            Self::LedTurnOn => write!(f, "LED_TURN_ON"),
            Self::LedTurnOff => write!(f, "LED_TURN_OFF"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let task = input_str
            .split('|')
            .next()
            .map(|task_str| match task_str {
                "REBOOT" => Self::Reboot,
                "LED_TURN_ON" => Self::LedTurnOn,
                "LED_TURN_OFF" => Self::LedTurnOff,
                _ => Self::Unknown,
            })
            .ok_or(anyhow::anyhow!("Invalid syntax: [{input_str:?}]"))?;
        Ok(task)
    }
}
