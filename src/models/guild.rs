use std::collections::VecDeque;

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub expiration: i64,
    pub joined: bool,
    pub queue: Vec<String>,
}

impl Guild {
    pub fn new(id: String, name: String, joined: bool) -> Guild {
        Guild { id, name, ..Default::default()  }
    }
} 
