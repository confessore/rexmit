use std::env;

use mongodb::{
    Client as MongoClient,
    Collection, Database, bson::doc
};
use serenity::prelude::Context;

use crate::{models::guild::Guild, context::context_get_guild};

pub async fn get_rexmit_database() -> Option<Database> {
    let database_url: Result<String, env::VarError> = std::env::var("DATABASE_URL");
    if database_url.is_ok() {
        let client_result = MongoClient::with_uri_str(database_url.unwrap()).await;
        if client_result.is_ok() {
            let client = client_result.unwrap();
            let database = client.database("rexmit");
            return Some(database);
        }
    }
    println!("no rexmit database found");
    return None;
}

pub async fn get_guild_collection() -> Option<Collection<Guild>> {

    let database_option = get_rexmit_database().await;
    if database_option.is_some() {
        let database = database_option.unwrap();
        let collection: Collection<Guild> = database.collection("guilds");
        return Some(collection);
    }
    println!("no guild collection found");
    return None;
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
            let guild = Guild::new(guild_id);
            let insert_one_result_result = guild_collection.insert_one(&guild, None).await;
            match insert_one_result_result {
                Ok(_insert_one_result) => {
                    return Some(guild)},
                Err(why) => {
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            return None;
        }
    }
}

pub async fn update_guild_queue(guild: serenity::model::prelude::Guild, queue: Vec<String>) {
    let collection_option = get_guild_collection().await;
    if collection_option.is_some() {
        let collection = collection_option.unwrap();
        let mut guild = Guild::new_from_serenity_guild(Some(guild));
        guild.queue = queue;
        
        let result = collection.find_one_and_update(doc! { "id": &guild.id.to_string() }, doc! { "$set": { "queue": &guild.queue }}, None).await;

        println!("{:?}", result);
        match &result {
            Ok(option) => {
                match &option {
                    Some(guild) => {
                        println!("{:?}", guild);
                    }, 
                    None => {
                        let result = collection.insert_one(&guild, None).await;
                        println!("{:?}", result)
                    }
                }
            },
            Err(why) => {
                println!("{}", why)
            }
        }
    }
}

pub async fn clear_guild_queue(guild: serenity::model::prelude::Guild) {
    let collection_option = get_guild_collection().await;
    if collection_option.is_some() {
        let collection = collection_option.unwrap();
        let guild = Guild::new_from_serenity_guild(Some(guild));
        
        let result = collection.find_one_and_update(doc! { "id": &guild.id.to_string() }, doc! { "$set": { "queue": &guild.queue }}, None).await;

        println!("{:?}", result);
        match &result {
            Ok(option) => {
                match &option {
                    Some(guild) => {
                        println!("{:?}", guild);
                    }, 
                    None => {
                        let result = collection.insert_one(&guild, None).await;
                        println!("{:?}", result)
                    }
                }
            },
            Err(why) => {
                println!("{}", why)
            }
        }
    }
}

pub async fn pop_guild_queue(guild: serenity::model::prelude::Guild) {
    let collection_option = get_guild_collection().await;
    if collection_option.is_some() {
        let collection = collection_option.unwrap();
        
        let result = collection.find_one_and_update(doc! { "id": &guild.id.to_string() }, doc! { "$pop": { "queue": -1 }}, None).await;

        println!("{:?}", result);
        match &result {
            Ok(option) => {
                match &option {
                    Some(guild) => {
                        println!("{:?}", guild);
                    }, 
                    None => {
                        let guild = Guild::new_from_serenity_guild(Some(guild));
                        let result = collection.insert_one(&guild, None).await;
                        println!("{:?}", result)
                    }
                }
            },
            Err(why) => {
                println!("{}", why)
            }
        }
    }
}

pub async fn set_joined_to_channel(ctx: &Context, guild_id: u64, joined: bool) -> bool {
    let collection_option = get_guild_collection().await;
    if collection_option.is_some() {
        let collection = collection_option.unwrap();
        let partial_guild_option = context_get_guild(ctx, guild_id.into()).await;
        if partial_guild_option.is_some() {
            let guild = Guild::new_from_serenity_partial_guild(partial_guild_option);
            let result = collection.find_one_and_update(doc! { "id": &guild_id.to_string() }, doc! { "$set": { "joined_to_channel": joined }}, None).await;
    
            println!("{:?}", result);
            match &result {
                Ok(option) => {
                    match &option {
                        Some(guild) => {
                            println!("{:?}", guild);
                        }, 
                        None => {
                            let result = collection.insert_one(&guild, None).await;
                            println!("{:?}", result);
                        }
                    }
                },
                Err(why) => {
                    println!("{}", why);
                }
            }
            return true;
        }
    }
    println!("unable to set joined status");
    return false;
}


// wip
// some repeating here, consider modularizing by creating additional functions
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
                    for guild in &guilds {
                        println!("{}", guild);
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

/// sets a guild queue in mongo given a guild id and a queue
///
/// ### arguments
/// 
/// * `guild_id` - the discord issued id for the guild
/// * `queue` - the vector of string track urls 
/// 
/// ### returns 
/// 
/// some vector of string track urls
/// 
pub async fn set_guild_queue(guild_id: String, queue: Vec<String>) -> Option<Vec<String>> {
    let guild_option = get_guild_document(guild_id).await;
    match guild_option {
        Some(mut guild) => {
            guild.queue = queue;
            set_guild_document(&guild).await;
            return Some(guild.queue)
        },
        None =>
        {
            return None;
        }
    }
    /*let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            let filter = doc! { "id": &guild_id };
            let update = doc! { "$set": { "queue": &queue }};
            let guild_option_result = guild_collection.find_one_and_update(filter, update, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    match guild_option {
                        Some(guild) => {
                            println!("{:?}", guild);
                            return Some(guild.queue)
                        }, 
                        None => {
                            insert_new_guild(guild_id).await;
                            return None;
                        }
                    }
                },
                Err(why) => {
                    println!("{}", why);
                    return None;
                }
            }
        },
        None => {
            return None;
        }
    }*/
}