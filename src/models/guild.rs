use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub expiration: i64
}

impl Guild {
    pub fn new(name: &str) -> Guild {
        Guild { name: String::from(name), id: String::from(""), subscribed: false, expiration: 0 }
    }
}