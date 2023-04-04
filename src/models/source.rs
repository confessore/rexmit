use crate::{
    schema::sources::{self, dsl::*}
};
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize
};
use songbird::input::Input;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    
}

impl Source {
 
}