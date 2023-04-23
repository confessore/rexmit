use std::collections::VecDeque;

use serde::{Serialize, Deserialize};
use serenity::model::prelude::PartialGuild;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub expiration: i64,
    pub joined: bool,
    pub queue: Vec<String>
}

impl Guild {
    pub fn new(partial_guild_option: Option<PartialGuild>) -> Guild {
        match &partial_guild_option {
            Some(partial_guild) => {
                let id = &partial_guild.id;
                let name = &partial_guild.name;
                Guild {
                    id: id.to_string(),
                    name: name.to_string(),
                    ..Default::default() 
                }
            },
            None => { 
                Guild { 
                    ..Default::default()
                }
            }
        }
    }
} 
