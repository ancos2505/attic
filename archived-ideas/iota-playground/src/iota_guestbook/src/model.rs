use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuestbookEntry {
    pub name: String,
    pub message: String,
}
