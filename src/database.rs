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

pub async fn set_joined(ctx: &Context, guild_id: u64, joined: bool) -> bool {
    let collection_option = get_guild_collection().await;
    if collection_option.is_some() {
        let collection = collection_option.unwrap();
        let partial_guild_option = context_get_guild(ctx, guild_id.into()).await;
        if partial_guild_option.is_some() {
            let guild = Guild::new_from_serenity_partial_guild(partial_guild_option);
            let result = collection.find_one_and_update(doc! { "id": &guild_id.to_string() }, doc! { "$set": { "joined": joined }}, None).await;
    
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