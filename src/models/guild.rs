use serde::{Serialize, Deserialize};
use serenity::model::prelude::PartialGuild;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub subscribed: bool,
    pub expiration: i64,
    pub joined_to_channel: bool,
    pub joined_channel_id: String,
    pub queue: Vec<String>
}

impl Guild {
    pub fn new_from_serenity_partial_guild(partial_guild_option: Option<PartialGuild>) -> Guild {
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

    pub fn new_from_serenity_guild(guild_option: Option<serenity::model::prelude::Guild>) -> Guild {
        match &guild_option {
            Some(guild) => {
                let id = &guild.id;
                let name = &guild.name;
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
