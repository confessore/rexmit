use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct Transmission {
    pub id: String,
    pub href: String,
    pub initially_played: i64,
    pub last_played: i64
}

impl Transmission {

}