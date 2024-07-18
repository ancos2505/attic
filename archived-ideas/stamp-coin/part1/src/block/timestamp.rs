use serde::{Deserialize, Serialize};
use std::fmt::Display;
// use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTimestamp(u128);

impl BlockTimestamp {
    pub fn try_new() -> anyhow::Result<Self> {
        use crate::misc::now_millis;
        Ok(Self(now_millis()?))
    }
}

impl Display for BlockTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
