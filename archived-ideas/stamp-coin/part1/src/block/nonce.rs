use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BlockNonce(u64);

impl Deref for BlockNonce {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for BlockNonce {
    fn from(data: u64) -> Self {
        Self(data)
    }
}

impl Display for BlockNonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let data = match self.0 {
        //     Some(data) => data,
        //     None => 0,
        // };
        // write!(f, "{}", data)
        write!(f, "{:?}", self.0)
    }
}
