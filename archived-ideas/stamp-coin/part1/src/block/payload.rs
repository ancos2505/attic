use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockPayload(String);
impl Display for BlockPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for BlockPayload {
    fn from(data: String) -> Self {
        // Self(data)
        Self(base64::encode(data))
    }
}
