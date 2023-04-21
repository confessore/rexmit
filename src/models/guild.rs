use std::collections::VecDeque;

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub expiration: i64,
    pub queue: Vec<String>,
}

impl Guild {
    pub fn new(id: String, name: String) -> Guild {
        Guild { id, name, subscribed: false, expiration: 0, ..Default::default()  }
    }
} 
