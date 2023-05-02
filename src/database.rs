use std::env;

use chrono::{DateTime, Utc};
use mongodb::{
    Client as MongoClient,
    Collection, Database, bson::doc
};

use crate::models::guild::Guild;


/// gets the rexmit database from mongo
/// 
/// ### arguments
/// 
/// * `none` - none
/// 
/// ### returns 
/// 
/// some database
/// 
pub async fn get_rexmit_database() -> Option<Database> {
    let database_url_result: Result<String, env::VarError> = std::env::var("DATABASE_URL");
    match database_url_result {
        Ok(database_url) => {
            println!("database url result is ok");
            let client_result = MongoClient::with_uri_str(database_url).await;
            match client_result {
                Ok(client) => {
                    println!("client result is ok");
                    let database = client.database("rexmit");
                    return Some(database);
                },
                Err(why) => {
                    println!("client result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        Err(why) => {
            println!("database url result is err");
            println!("{}", why);
            return None;
        }
    }
}

/// gets the rexmit guild collection from mongo
/// 
/// ### arguments
/// 
/// * `none` - none
/// 
/// ### returns 
/// 
/// some collection of guild
/// 
pub async fn get_guild_collection() -> Option<Collection<Guild>> {
    let database_option = get_rexmit_database().await;
    match database_option {
        Some(database) => {
            println!("database option is some");
            let collection: Collection<Guild> = database.collection("guilds");
            return Some(collection);
        },
        None => {
            println!("database option is none");
            return None;
        }
    }
}

/// gets a guild document from mongo given a discord guild id
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn get_guild_document(guild_id: String) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let guild_option_result = guild_collection.find_one(filter, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild is some");
                            return Some(guild);
                        }, 
                        None => {
                            println!("guild is none");
                            return insert_new_guild(&guild_collection_option, guild_id).await;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}

/// sets a guild document in mongo given a guild
///
/// ### arguments
/// 
/// * `guild` - the rexmit guild model
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn set_guild_document(guild: &Guild) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild.id };
            let guild_option_result = guild_collection.find_one_and_replace(filter, guild, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild is some");
                            return Some(guild);
                        }, 
                        None => {
                            println!("guild is none");
                            return insert_new_guild(&guild_collection_option, guild.id.to_string()).await;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}

/// inserts a new guild document into mongo given a discord guild id
///
/// ### arguments
/// 
/// * `guild_collection_option` - the mongo guild collection option reference
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn insert_new_guild(guild_collection_option: &Option<Collection<Guild>>, guild_id: String) -> Option<Guild> {
    match guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option reference is some");
            let guild = Guild::new(guild_id);
            let insert_one_result_result = guild_collection.insert_one(&guild, None).await;
            match insert_one_result_result {
                Ok(_insert_one_result) => {
                    println!("insert one result result is ok");
                    return Some(guild)},
                Err(why) => {
                    println!("insert one result result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option reference is none");
            return None;
        }
    }
}

/// sets a guild's queue in mongo given a guild id and a queue
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// * `queue` - a vector of string track urls
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn set_guild_queue(guild_id: String, queue: Vec<String>) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let update = doc! { "$set": { "queue": &queue }};
            let guild_option_result = guild_collection.find_one_and_update(filter, update, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild option is some");
                            return Some(guild);
                        },
                        None => {
                            println!("guild option is none");
                            let guild_option = insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}

/// clears a guild's queue in mongo given a discord guild id
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn clear_guild_queue(guild_id: String) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let queue: Vec<String> = vec![];
            let update = doc! { "$set": { "queue": queue }};
            let guild_option_result = guild_collection.find_one_and_update(filter, update, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild option is some");
                            return Some(guild);
                        },
                        None => {
                            println!("guild option is none");
                            let guild_option = insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}

/// directly pops the 0 index of a guild's queue in mongo given a guild id and returns the guild document
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn pop_guild_queue(guild_id: String) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let update = doc! { "$pop": { "queue": -1 }};
            let guild_option_result = guild_collection.find_one_and_update(filter, update, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild option is some");
                            return Some(guild);
                        }, 
                        None => {
                            println!("guild option is none");
                            let guild_option = insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}

/// sets a guild's joined to channel status as well as a guild's joined channel id
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// * `channel_id_option` - some string discord channel id or none
/// 
/// ### returns 
/// 
/// some guild or none
/// 
pub async fn set_joined_to_channel(guild_id: String, channel_id_option: Option<String>) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let mut update = doc! { "$set": { "joined_to_channel": false, "joined_channel_id": "" }};
            match channel_id_option {
                Some(channel_id) => {
                    println!("channel id option is some");
                    update = doc! { "$set": { "joined_to_channel": true, "joined_channel_id": channel_id }};
                },
                None => {
                    println!("channel id option is none");
                }
            }
            let guild_option_result = guild_collection.find_one_and_update(filter, update, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild option is some");
                            return Some(guild);
                        }, 
                        None => {
                            println!("guild option is none");
                            let guild_option = insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}

/// gets guilds that are found to be joined to channel in mongo
///
/// ### arguments
/// 
/// * `none` - none
/// 
/// ### returns 
/// 
/// some vector of string or none
/// 
pub async fn get_guilds_joined_to_channel() -> Option<Vec<String>> {
    let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            println!("{}", "guild collection is some");
            let filter = doc! { "joined_to_channel": true };
            let cursor_result = guild_collection.find(filter, None).await;
            match cursor_result {
                Ok(mut cursor) => {
                    println!("{}", "cursor result is ok");
                    let mut guilds: Vec<String> = vec![];
                    while let Ok(cursor_is_open) = cursor.advance().await {
                        println!("cursor result is ok and it is {} that the cursor is open", cursor_is_open);
                        if !cursor_is_open {
                            break;
                        }
                        if cursor_is_open {
                            let guild_result = cursor.deserialize_current();
                            match guild_result {
                                Ok(guild) => {
                                    println!("{}", "guild result is ok");
                                    guilds.push(guild.id);
                                },
                                Err(why) => {
                                    println!("{}", "guild result is err");
                                    println!("{}", why)
                                }
                            }
                        }
                    }
                    return Some(guilds);
                },
                Err(why) => {
                    println!("{}", "cursor result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("{}", "guild collection is none");
            return None;
        }
    }
}

/// gets a guild queue in mongo given a guild id
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some vector of string track urls
/// 
pub async fn get_guild_queue(guild_id: String) -> Option<Vec<String>> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            println!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let guild_option_result = guild_collection.find_one(filter, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    println!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            println!("guild option is some");
                            return Some(guild.queue)
                        }, 
                        None => {
                            println!("guild option is none");
                            insert_new_guild(&guild_collection_option, guild_id).await;
                            return None;
                        }
                    }
                },
                Err(why) => {
                    println!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            println!("guild collection option is none");
            return None;
        }
    }
}


/// gets a guild's subscription status from mongo given a guild id
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some bool or none
/// 
pub async fn get_guild_is_subscribed(guild_id: String) -> Option<bool> {
    let guild_option = get_guild_document(guild_id).await;
    match guild_option {
        Some(guild) => {
            println!("guild option is some");
            return Some(guild.is_subscribed());
        },
        None =>
        {
            println!("guild option is none");
            return None;
        }
    }
}

/// gets a guild's expiration from mongo given a guild id
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// 
/// ### returns 
/// 
/// some datetime utc or none
/// 
pub async fn get_guild_expiration(guild_id: String) -> Option<DateTime<Utc>> {
    let guild_option = get_guild_document(guild_id).await;
    match guild_option {
        Some(guild) => {
            println!("guild option is some");
            return Some(guild.expiration);
        },
        None =>
        {
            println!("guild option is none");
            return None;
        }
    }
}

