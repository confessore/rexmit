use crate::{
    schema::transmissions::{self, dsl::*}
};
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Transmission {
    pub id: String,
    pub href: String,
    pub initially_played: u64,
    pub last_played: u64
}

impl Transmission {

}