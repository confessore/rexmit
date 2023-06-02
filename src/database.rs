use std::env;

use chrono::{DateTime, Utc};
use mongodb::{bson::doc, Client as MongoClient, Collection, Database};
use serenity::{model::prelude::{GuildId, ChannelId}, prelude::Context};
use tracing::{debug, error};

use crate::{models::guild::Guild, command::check_msg};

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
            debug!("database url result is ok");
            let client_result = MongoClient::with_uri_str(database_url).await;
            match client_result {
                Ok(client) => {
                    debug!("client result is ok");
                    // storm better way to account for debug environment variable
                    let debug = env::var("DEBUG").expect("Expected a DEBUG == to 1 or 0 in the environment");
                    let mut database_name = "rexmit";
                    if debug == "1" {
                        database_name = "rexmit-dev"
                    }
                    let database = client.database(database_name);
                    return Some(database);
                }
                Err(why) => {
                    debug!("client result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        Err(why) => {
            debug!("database url result is err");
            error!("{}", why);
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
            debug!("database option is some");
            let collection: Collection<Guild> = database.collection("guilds");
            return Some(collection);
        }
        None => {
            debug!("database option is none");
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
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let guild_option_result = guild_collection.find_one(filter, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild is some");
                            return Some(guild);
                        }
                        None => {
                            debug!("guild is none");
                            return insert_new_guild(&guild_collection_option, guild_id).await;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
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
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild.id };
            let guild_option_result = guild_collection
                .find_one_and_replace(filter, guild, None)
                .await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild is some");
                            return Some(guild);
                        }
                        None => {
                            debug!("guild is none");
                            return insert_new_guild(
                                &guild_collection_option,
                                guild.id.to_string(),
                            )
                            .await;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
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
pub async fn insert_new_guild(
    guild_collection_option: &Option<Collection<Guild>>,
    guild_id: String,
) -> Option<Guild> {
    match guild_collection_option {
        Some(guild_collection) => {
            debug!("guild collection option reference is some");
            let guild = Guild::new(guild_id);
            let insert_one_result_result = guild_collection.insert_one(&guild, None).await;
            match insert_one_result_result {
                Ok(_insert_one_result) => {
                    debug!("insert one result result is ok");
                    return Some(guild);
                }
                Err(why) => {
                    debug!("insert one result result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option reference is none");
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
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let update = doc! { "$set": { "queue": &queue }};
            let guild_option_result = guild_collection
                .find_one_and_update(filter, update, None)
                .await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild option is some");
                            return Some(guild);
                        }
                        None => {
                            debug!("guild option is none");
                            let guild_option =
                                insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
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
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let queue: Vec<String> = vec![];
            let update = doc! { "$set": { "queue": queue }};
            let guild_option_result = guild_collection
                .find_one_and_update(filter, update, None)
                .await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild option is some");
                            return Some(guild);
                        }
                        None => {
                            debug!("guild option is none");
                            let guild_option =
                                insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
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
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let update = doc! { "$pop": { "queue": -1 }};
            let guild_option_result = guild_collection
                .find_one_and_update(filter, update, None)
                .await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild option is some");
                            return Some(guild);
                        }
                        None => {
                            debug!("guild option is none");
                            let guild_option =
                                insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
            return None;
        }
    }
}

/// sets a guild's joined to channel status as well as a guild's joined channel id
///
/// ### arguments
///
/// * `guild_id` - the discord issued id for the guild
/// * `voice_channel_id_option` - some string discord channel id or none
/// * `message_channel_id_option` - some string discord channel id or none
///
/// ### returns
///
/// some guild or none
///
pub async fn set_joined_to_channel(
    guild_id: String,
    voice_channel_id_option: Option<String>,
    message_channel_id_option: Option<String>,
) -> Option<Guild> {
    let guild_collection_option = get_guild_collection().await;
    match &guild_collection_option {
        Some(guild_collection) => {
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let mut update = doc! { "$set": { "joined_to_voice": false, "voice_channel_id": "", "message_channel_id": "" }};
            match voice_channel_id_option {
                Some(voice_channel_id) => {
                    debug!("voice channel id option is some");
                    match message_channel_id_option {
                        Some(message_channel_id) => {
                            debug!("message channel id option is some");
                            update = doc! { "$set": { "joined_to_voice": true, "voice_channel_id": voice_channel_id, "message_channel_id": message_channel_id }};
                        }
                        None => {
                            debug!("message channel id option is none");
                        }
                    }
                }
                None => {
                    debug!("voice channel id option is none");
                }
            }
            let guild_option_result = guild_collection
                .find_one_and_update(filter, update, None)
                .await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild option is some");
                            return Some(guild);
                        }
                        None => {
                            debug!("guild option is none");
                            let guild_option =
                                insert_new_guild(&guild_collection_option, guild_id).await;
                            return guild_option;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    println!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
            return None;
        }
    }
}

/// gets the ids of the guilds that are found to be joined to channel in mongo
///
/// ### arguments
///
/// * `none` - none
///
/// ### returns
///
/// some vector of string or none
///
pub async fn get_guild_ids_joined_to_channel() -> Option<Vec<String>> {
    let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            debug!("{}", "guild collection is some");
            let filter = doc! { "joined_to_voice": true };
            let cursor_result = guild_collection.find(filter, None).await;
            match cursor_result {
                Ok(mut cursor) => {
                    debug!("{}", "cursor result is ok");
                    let mut guilds: Vec<String> = vec![];
                    while let Ok(cursor_is_open) = cursor.advance().await {
                        debug!(
                            "cursor result is ok and it is {} that the cursor is open",
                            cursor_is_open
                        );
                        if !cursor_is_open {
                            break;
                        }
                        if cursor_is_open {
                            let guild_result = cursor.deserialize_current();
                            match guild_result {
                                Ok(guild) => {
                                    debug!("{}", "guild result is ok");
                                    guilds.push(guild.id);
                                }
                                Err(why) => {
                                    debug!("{}", "guild result is err");
                                    error!("{}", why)
                                }
                            }
                        }
                    }
                    return Some(guilds);
                }
                Err(why) => {
                    debug!("{}", "cursor result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("{}", "guild collection is none");
            return None;
        }
    }
}

/// counts guilds that are found to be joined to channel in mongo
///
/// ### arguments
///
/// * `none` - none
///
/// ### returns
///
/// some vector of string or none
///
pub async fn count_guilds_joined_to_channel() -> Option<u64> {
    let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            debug!("{}", "guild collection is some");
            let filter = doc! { "joined_to_voice": true };
            let count_result = guild_collection.count_documents(filter, None).await;
            match count_result {
                Ok(count) => {
                    debug!("{}", "count result is ok");
                    return Some(count);
                }
                Err(why) => {
                    debug!("{}", "count result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("{}", "guild collection is none");
            return None;
        }
    }
}

/// counts guilds with a reservation that are found to be joined to channel in mongo
///
/// ### arguments
///
/// * `none` - none
///
/// ### returns
///
/// some vector of string or none
///
pub async fn count_guilds_with_a_reservation_joined_to_channel() -> Option<u64> {
    let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            debug!("{}", "guild collection is some");
            let filter =
                doc! { "expiration": { "$gt": Utc::now().to_string() }, "joined_to_voice": true };
            let count_result = guild_collection.count_documents(filter, None).await;
            match count_result {
                Ok(count) => {
                    debug!("{}", "count result is ok");
                    return Some(count);
                }
                Err(why) => {
                    debug!("{}", "count result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("{}", "guild collection is none");
            return None;
        }
    }
}

/// counts free guilds that are found to be joined to channel in mongo
///
/// ### arguments
///
/// * `none` - none
///
/// ### returns
///
/// some vector of string or none
///
pub async fn count_free_guilds_joined_to_channel() -> Option<u64> {
    let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            debug!("{}", "guild collection is some");
            let filter =
                doc! { "expiration": { "$lt": Utc::now().to_string() }, "joined_to_voice": true };
            let count_result = guild_collection.count_documents(filter, None).await;
            match count_result {
                Ok(count) => {
                    debug!("{}", "count result is ok");
                    return Some(count);
                }
                Err(why) => {
                    debug!("{}", "count result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("{}", "guild collection is none");
            return None;
        }
    }
}

/// gets the first free guild that is found to be joined to channel in mongo
///
/// ### arguments
///
/// * `none` - none
///
/// ### returns
///
/// some vector of string or none
///
pub async fn get_first_free_guild_joined_to_channel() -> Option<GuildId> {
    let guild_collection_option = get_guild_collection().await;
    match guild_collection_option {
        Some(guild_collection) => {
            debug!("{}", "guild collection is some");
            let filter =
                doc! { "expiration": { "$lt": Utc::now().to_string() }, "joined_to_voice": true };
            let guild_option_result = guild_collection.find_one(filter, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("{}", "guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild option is some");
                            match guild.id.parse::<u64>() {
                                Ok(guild_id) => {
                                    debug!("guild id result is ok");
                                    return Some(GuildId(guild_id));
                                }
                                Err(why) => {
                                    debug!("guild id result is err");
                                    error!("{}", why);
                                    return None;
                                }
                            };
                        }
                        None => {
                            debug!("guild option is none");
                            return None;
                        }
                    }
                }
                Err(why) => {
                    debug!("{}", "guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("{}", "guild collection is none");
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
            debug!("guild collection option is some");
            let filter = doc! { "id": &guild_id };
            let guild_option_result = guild_collection.find_one(filter, None).await;
            match guild_option_result {
                Ok(guild_option) => {
                    debug!("guild option result is ok");
                    match guild_option {
                        Some(guild) => {
                            debug!("guild option is some");
                            return Some(guild.queue);
                        }
                        None => {
                            debug!("guild option is none");
                            insert_new_guild(&guild_collection_option, guild_id).await;
                            return None;
                        }
                    }
                }
                Err(why) => {
                    debug!("guild option result is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("guild collection option is none");
            return None;
        }
    }
}

/// gets a guild's reservation status from mongo given a guild id
///
/// ### arguments
///
/// * `guild_id` - the discord issued id for the guild
///
/// ### returns
///
/// some bool or none
///
pub async fn get_guild_has_reservation(guild_id: String) -> Option<bool> {
    let guild_option = get_guild_document(guild_id).await;
    match guild_option {
        Some(guild) => {
            debug!("guild option is some");
            return Some(guild.has_reservation());
        }
        None => {
            debug!("guild option is none");
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
            debug!("guild option is some");
            return Some(guild.expiration);
        }
        None => {
            debug!("guild option is none");
            return None;
        }
    }
}

pub async fn slot_is_available(ctx: &Context, guild_id: String) -> Option<bool> {
    match count_guilds_joined_to_channel().await {
        Some(used_slots) => {
            debug!("count guilds joined to channel is some");
            match env::var("MAX_SLOTS")
                .expect("Expected a MAX_SLOTS in the environment")
                .parse::<u64>()
            {
                Ok(max_slots) => {
                    debug!("max slots parse is ok");
                    if used_slots < max_slots {
                        // connect

                        return Some(true);
                    } else {
                        match get_guild_has_reservation(guild_id).await {
                            Some(guild_has_reservation) => {
                                debug!("get_guild_has_reservation is some");
                                if guild_has_reservation {
                                    match count_free_guilds_joined_to_channel().await {
                                        Some(used_free_slots) => {
                                            debug!("count free guilds joined to channel is some");
                                            if used_free_slots > 0 {
                                                match get_first_free_guild_joined_to_channel().await {
                                                    Some(guild_id) => {
                                                        debug!("get_first_free_guild_joined_to_channel is some");
                                                        match get_guild_document(guild_id.to_string()).await {
                                                            Some(guild) => {
                                                                debug!("get_guild_document is some");
                                                                match guild.message_channel_id.parse::<u64>() {
                                                                    Ok(message_channel_id) => {
                                                                        debug!("message_channel_id parse is ok");
                                                                        // boot the free user and connect the guild with a reservation


                                                                        check_msg(ChannelId(message_channel_id).say(&ctx.http, "retreated to attend to a reserved guild").await);
                                                                        return Some(true);
                                                                    }
                                                                    Err(why) => {
                                                                        debug!("message_channel_id parse is err");
                                                                        error!("{}", why);
                                                                        return None;
                                                                    }
                                                                }
                                                            }
                                                            None => {
                                                                debug!("get_guild_document is none");
                                                                return None;
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        debug!("get_first_free_guild_joined_to_channel is none");
                                                        return None;
                                                    }
                                                }
                                            } else {
                                                // take to the discord channel with a pitchfork
                                                // https://discord.gg/95eUjKqT7e
                                                //
                                                //
                                                return Some(false);
                                            }
                                        }
                                        None => {
                                            debug!("count free guilds joined to channel is none");
                                            return None;
                                        }
                                    }
                                } else {
                                    // first come first serve, maybe make a reservation?
                                    // post https link
                                    //
                                    //
                                    return Some(false);
                                }
                            }
                            None => {
                                debug!("get_guild_has_reservation is none");
                                return None;
                            }
                        }
                    }
                }
                Err(why) => {
                    debug!("max slots parse is err");
                    error!("{}", why);
                    return None;
                }
            }
        }
        None => {
            debug!("count guilds joined to channel is none");
            return None;
        }
    }
}
