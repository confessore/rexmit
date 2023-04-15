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
    pub fn new(id: String, name: String) -> Guild {
        Guild { id, name, subscribed: false, expiration: 0 }
    }
}