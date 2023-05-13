use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub id: String,
}

impl Source {}
