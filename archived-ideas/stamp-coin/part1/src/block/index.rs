use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockIndex(u32);
impl Deref for BlockIndex {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for BlockIndex {
    fn from(data: u32) -> Self {
        Self(data)
    }
}
