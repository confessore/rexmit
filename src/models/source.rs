use crate::{
    schema::sources::{self, dsl::*}
};
use diesel::prelude::*;
use serde::{
    Serialize,
    Deserialize
};
use songbird::Input;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub input: Input,
    
}

impl Source {
 
}