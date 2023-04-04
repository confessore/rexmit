use crate::{
    schema::guilds::{self, dsl::*}
};
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub expiration: i64
}

impl Guild {

}