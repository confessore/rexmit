use crate::{
    schema::queues::{self, dsl::*}
};
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Queue {
    pub id: String,

}

impl Queue {

}