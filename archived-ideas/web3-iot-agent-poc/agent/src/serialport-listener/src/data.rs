use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default)]
pub enum DataCategory {
    #[default]
    Nil,
    Unknown,
    // Sensor(SensorInfo),
    Sensor,
}

impl Display for DataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "NIL"),
            // Self::Sensor(s) => write!(f, "SENSOR: {s}"),
            Self::Sensor => write!(f, "SENSOR"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl FromStr for DataCategory {
    type Err = anyhow::Error;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        dbg!(&input_str);
        let data_category = match input_str {
            // TODO:
            "SENSOR" => Self::Sensor,
            _ => Self::Unknown,
        };

        Ok(data_category)
    }
}

#[derive(Debug, Default)]
pub struct SensorInfo {
    label: String,
    timestamp: i64,
    data: (f32, f32, f32),
}
impl Display for SensorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SENSOR|{}|{}|({:1},{:1},{:1})",
            self.label, self.timestamp, self.data.0, self.data.1, self.data.2
        )
    }
}
