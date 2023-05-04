use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Queue {
    pub id: String,
}

impl Queue {}
