//! Example demonstrating how to make use of individual track audio events,
//! and how to use the `TrackQueue` system.
//!
//! Requires the "cache", "standard_framework", and "voice" features be enabled in your
//! Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["cache", "framework", "standard_framework", "voice"]
//! ```
use std::{
    env,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use mongodb::{
    Client as MongoClient,
    Collection, bson::{doc, Bson}, Database
};
use rexmit::models::guild::Guild;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler, Cache},
    framework::{
        standard::{
            macros::{command, group},
            Args,
            CommandResult,
        },
        StandardFramework,
    },
    http::Http,
    model::{channel::Message, gateway::Ready, prelude::{ChannelId, Activity, GuildId, VoiceState, PartialGuild}},
    prelude::{GatewayIntents, Mentionable},
    Result as SerenityResult, futures::stream::Collect,
};

use songbird::{
    input::{
        self,
        restartable::Restartable,
    },
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    SerenityInit,
    TrackEvent, events::context_data::VoiceData,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::listening("~q <youtube url>")).await;
        println!("{} is connected!", ready.user.name);
    }

    /*async fn voice_state_update(&self, _ctx: Context, _old: Option<VoiceState>, _new: VoiceState) {
        let channel_result = _ctx.http.get_channel(_new.channel_id.unwrap().into()).await;
        match channel_result.unwrap().guild() {
            Some(guild_channel) => {
                let members_result = guild_channel.members(&_ctx.cache).await;
                match members_result {
                    Ok(members) => {
                        println!("{}", members.len())
                    },
                    Err(why) => {
                        println!("{}", why);
                    }
                }
            },
            None => {
                println!("{}", "no guild")
            }
        };
    }*/
}

#[group]
#[commands(
    d, deafen, j, join, l, leave, m, mute, q, queue, s, skip, c, clear, stop, p, ping, ud, undeafen, um, unmute
)]
struct General;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();



    let debug = env::var("DEBUG").expect("Expected a DEBUG == to 1 or 0 in the environment");

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");

    let framework = StandardFramework::new()
        .configure(|c| {
            let mut prefix = "~";
            if debug == "1" {
                prefix = ">";
            }
            c.prefix(prefix)
        })
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}

#[command]
async fn d(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return deafen(ctx, msg, _args).await;
}

#[command]
async fn deafen(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        check_msg(msg.channel_id.say(&ctx.http, "Already deafened").await);
    } else {
        if let Err(e) = handler.deafen(true).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Deafened").await);
    }

    Ok(())
}

async fn get_rexmit_database() -> Option<Database> {
    let database_url: Result<String, env::VarError> = std::env::var("DATABASE_URL");
    if database_url.is_ok() {
        let client_result = MongoClient::with_uri_str(database_url.unwrap()).await;
        if client_result.is_ok() {
            let client = client_result.unwrap();
            let database = client.database("rexmit");
            return Some(database);
        }
    }
    return None;
}

async fn get_guild_collection() -> Option<Collection<Guild>> {

    let database_option = get_rexmit_database().await;
    if database_option.is_some() {
        let database = database_option.unwrap();
        let collection: Collection<Guild> = database.collection("guilds");
        return Some(collection);
    }
    return None;
}

async fn context_get_guild(ctx: &Context, guild_id: u64) -> Option<PartialGuild> {
    let partial_guild_result = ctx.http.get_guild(guild_id).await;
    if partial_guild_result.is_ok() {
        let partial_guild = partial_guild_result.unwrap();
        return Some(partial_guild)
    }
    return None;
}

async fn set_joined(ctx: &Context, msg: &Message) -> bool {
    if msg.guild_id.is_some() {
        let guild_id = msg.guild_id.unwrap();
        let collection_option = get_guild_collection().await;
        if collection_option.is_some() {
            let collection = collection_option.unwrap();
            let partial_guild_option = context_get_guild(ctx, guild_id.into()).await;
            if partial_guild_option.is_some() {
                let partial_guild = partial_guild_option.unwrap();
                
                let guild = Guild::new(guild_id.to_string(), partial_guild.clone().name, true);
                let result = collection.find_one_and_update(doc! { "id": &guild_id.to_string() }, doc! { "$set": { "joined": true }}, None).await;
        
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
    }
    return false;
}

#[command]
#[only_in(guilds)]
async fn j(ctx: &Context, msg: &Message) -> CommandResult {
    return join(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    set_joined(ctx, msg).await;
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

        let guild_id = msg.guild_id.unwrap();

        let chan_id = msg.channel_id;

        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndNotifier {
                chan_id,
                http: send_http,
            },
        );

        let send_http = ctx.http.clone();
        let send_cache = ctx.cache.clone();

        handle.add_global_event(
            Event::Periodic(Duration::from_secs(1800), None),
            Periodic {
                voice_chan_id: connect_to,
                chan_id,
                http: send_http,
                cache: send_cache,
                ctx: ctx.clone()
            },
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }

    Ok(())
}

struct TrackEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            check_msg(
                self.chan_id
                    .say(&self.http, &format!("Track ended: {}", track_list.first().as_ref().unwrap().1.metadata().source_url.as_ref().unwrap()))
                    .await,
            );
        }

        None
    }
}

#[command]
#[only_in(guilds)]
async fn l(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return leave(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn m(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return mute(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        check_msg(msg.channel_id.say(&ctx.http, "Already muted").await);
    } else {
        if let Err(e) = handler.mute(true).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Now muted").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn p(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return ping(ctx, msg, _args).await;
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play_fade(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match input::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        // This handler object will allow you to, as needed,
        // control the audio track via events and further commands.
        let song = handler.play_source(source);
        let send_http = ctx.http.clone();
        let chan_id = msg.channel_id;

        // This shows how to periodically fire an event, in this case to
        // periodically make a track quieter until it can be no longer heard.
        let _ = song.add_event(
            Event::Periodic(Duration::from_secs(5), Some(Duration::from_secs(7))),
            SongFader {
                chan_id,
                http: send_http,
            },
        );

        let send_http = ctx.http.clone();

        // This shows how to fire an event once an audio track completes,
        // either due to hitting the end of the bytestream or stopped by user code.
        let _ = song.add_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                chan_id,
                http: send_http,
            },
        );

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

struct SongFader {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for SongFader {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(&[(state, track)]) = ctx {
            let _ = track.set_volume(state.volume / 2.0);

            if state.volume < 1e-2 {
                let _ = track.stop();
                check_msg(self.chan_id.say(&self.http, "Stopping song...").await);
                Some(Event::Cancel)
            } else {
                check_msg(self.chan_id.say(&self.http, "Volume reduced.").await);
                None
            }
        } else {
            None
        }
    }
}

struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        check_msg(
            self.chan_id
                .say(&self.http, "Song faded out completely!")
                .await,
        );

        None
    }
}

struct Periodic {
    voice_chan_id: ChannelId,
    chan_id: ChannelId,
    http: Arc<Http>,
    cache: Arc<Cache>,
    ctx: Context,
}

#[async_trait]
impl VoiceEventHandler for Periodic {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let channel = self.http.get_channel(self.voice_chan_id.into()).await;
        match channel.unwrap().guild() {
            Some(guild_channel) => {
                let members = guild_channel.members(&self.cache).await;

                // please modularize this monstrocity
                // what i mean by this is create some functions and call the functions instead
                // we want to utilize DRY (DON'T REPEAT YOURSELF) principles
                if members.unwrap().len() <= 1 {
                    let manager = songbird::get(&self.ctx)
                        .await
                        .expect("Songbird Voice client placed in at initialisation.")
                        .clone();

                    let has_handler = manager.get(guild_channel.guild_id).is_some();

                    if has_handler {
                        if let Err(e) = manager.remove(guild_channel.guild_id).await {
                            check_msg(
                                self.chan_id
                                    .say(&self.http, format!("Failed: {:?}", e))
                                    .await,
                            );
                        }

                        check_msg(self.chan_id.say(&self.http, "Left voice channel").await);
                    } else {
                        check_msg(self.chan_id.say(&self.http, "Not in a voice channel").await);
                    }
                }
            },
            None => {
                println!("{}", "channel was none")
            }
        }

        None
    }
}

#[command]
#[only_in(guilds)]
async fn q(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    return queue(ctx, msg, args).await;
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let og_args = args.clone();
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        handler.enqueue_source(source.into());

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Added song to queue: position {}", handler.queue().len()),  
                )
                .await,
        );

        let database_url = std::env::var("DATABASE_URL").expect("Expected a database url in the environment");
        let wrapped_client = MongoClient::with_uri_str(database_url).await;
        let db = wrapped_client.unwrap().database("rexmit");
        let collection: Collection<Guild> = db.collection("guilds");
        let guild_id = msg.guild_id.unwrap().0;
        let http_guild = ctx.http.get_guild(guild_id).await;
        let partial_guild = http_guild.unwrap();
        let mut guild = Guild::new(guild_id.to_string(), partial_guild.clone().name, true);

        for track_handle in handler.queue().current_queue() {
            guild.queue.push(track_handle.metadata().source_url.clone().unwrap())
        }
        
        let result = collection.find_one_and_update(doc! { "id": &guild_id.to_string() }, doc! { "$set": { "queue": &guild.queue }}, None).await;

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
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
        match join(ctx, msg, og_args.clone()).await {
            Ok(result) => {
                /*check_msg(
                    msg.channel_id
                        .say(&ctx.http, og_args.clone().single::<String>().unwrap())
                        .await,
                );*/
                match queue(ctx, msg, og_args.clone()).await {
                    Ok(result) => result,
                    Err(_why) => {
                        check_msg(
                            msg.channel_id
                                .say(&ctx.http, "unable to queue")
                                .await,
                        );
                    }
                }
                return Ok(result);
            },
            Err(_why) => {
                check_msg(
                    msg.channel_id
                        .say(&ctx.http, "unable to join")
                        .await,
                );
                return Ok(());
            }
        }
    }
    //let databases = wrapped_client.unwrap().list_databases(None, None).await;*/

    /*let collection: Collection<Document> = db.collection("guilds");
    let filter = doc! {  };
    let options = CountOptions::builder().build();
    println!("{:?}", collection.count_documents(filter, options).await);*/

    Ok(())
}


#[command]
#[only_in(guilds)]
async fn s(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return skip(ctx, msg, _args).await;
} 

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Song skipped: {} in queue.", queue.len()),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn c(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return stop(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn clear(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return stop(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.stop();

        check_msg(msg.channel_id.say(&ctx.http, "Queue cleared.").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn ud(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return undeafen(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.deafen(false).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Undeafened").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to undeafen in")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn um(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    return unmute(ctx, msg, _args).await;
}

#[command]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.mute(false).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to unmute in")
                .await,
        );
    }

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}