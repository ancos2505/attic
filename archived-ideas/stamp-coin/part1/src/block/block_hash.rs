use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockHash(Vec<u8>);
impl Deref for BlockHash {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for BlockHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}
impl From<Vec<u8>> for BlockHash {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}
impl From<[u8; 32]> for BlockHash {
    fn from(data: [u8; 32]) -> Self {
        Self(data.to_vec())
    }
}
impl BlockHash {
    pub fn genesis() -> Self {
        Self(vec![0; 32])
    }

    // fn new() -> Self {
    //     Self([255; 32])
    // }
}
