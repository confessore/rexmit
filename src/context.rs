use std::{sync::Arc, time::Duration};

use serenity::{
    model::prelude::{ChannelId, Guild, GuildId, Message, PartialGuild},
    prelude::Context,
};
use songbird::{input::Restartable, Event, Songbird, TrackEvent};
use tracing::{debug, error};

use crate::{
    command::check_msg,
    database::{
        count_free_guilds_joined_to_channel, count_guilds_joined_to_channel,
        get_first_free_guild_joined_to_channel, get_guild_document, set_joined_to_channel,
    },
    functions::{get_max_slots, parse_u64_from_string},
    handler::{Periodic, TrackEndNotifier},
};

pub async fn context_get_guild(ctx: &Context, guild_id: u64) -> Option<PartialGuild> {
    let partial_guild_result = ctx.http.get_guild(guild_id).await;
    if partial_guild_result.is_ok() {
        let partial_guild = partial_guild_result.unwrap();
        return Some(partial_guild);
    }
    println!("context guild not found");
    return None;
}

pub async fn context_join_to_voice_channel(
    ctx: &Context,
    msg: &Message,
    guild: &Guild,
) -> Option<bool> {
    let songbird_arc_option = songbird::get(&ctx).await;
    let songbird_arc = match songbird_arc_option {
        Some(songbird_arc) => {
            debug!("songbird arc is some");
            songbird_arc
        }
        None => {
            debug!("songbird arc is none");
            return None;
        }
    };
    let voice_channel_id_option = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);
    let voice_channel_id = match voice_channel_id_option {
        Some(voice_channel_id) => {
            debug!("voice channel id option is some");
            voice_channel_id
        }
        None => {
            debug!("voice channel id option is none");
            return None;
        }
    };
    let (call_mutex_arc, empty_joinerror_result) =
        songbird_arc.join(guild.id, voice_channel_id).await;
    match empty_joinerror_result {
        Ok(()) => {
            debug!("empty joinerror result is ok");
            let mut call_mutexguard = call_mutex_arc.lock().await;
            call_mutexguard.add_global_event(
                Event::Track(TrackEvent::End),
                TrackEndNotifier {
                    guild_id: guild.id,
                    message_channel_id: msg.channel_id,
                    http: ctx.http.clone(),
                },
            );
            call_mutexguard.add_global_event(
                Event::Periodic(Duration::from_secs(1800), None),
                Periodic {
                    voice_channel_id,
                    message_channel_id: msg.channel_id,
                    http: ctx.http.clone(),
                    cache: ctx.cache.clone(),
                    songbird_arc,
                },
            );

            set_joined_to_channel(
                guild.id.to_string(),
                Some(voice_channel_id.to_string()),
                Some(msg.channel_id.to_string()),
            )
            .await;
            return Some(true);
        }
        Err(why) => {
            debug!("context join channel is err");
            error!("{}", why);
            return Some(false);
        }
    }
}

pub async fn context_boot_guild(ctx: &Context, guild_id: GuildId) {
    let songbird_arc_option = songbird::get(&ctx).await;
    match songbird_arc_option {
        Some(songbird_arc) => {
            debug!("songbird arc is some");
            match songbird_arc.remove(guild_id).await {
                Ok(()) => {
                    debug!("songbird arc remove is ok");
                }
                Err(why) => {
                    debug!("songbird arc remove is err");
                    error!("{}", why);
                }
            };
        }
        None => {
            debug!("songbird arc is none");
        }
    };
    set_joined_to_channel(guild_id.to_string(), None, None).await;
}

pub async fn context_rejoin_to_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel_id: ChannelId,
    message_channel_id: ChannelId,
) -> Option<Arc<Songbird>> {
    let songbird_arc_option = songbird::get(&ctx).await;
    match songbird_arc_option {
        Some(songbird_arc) => {
            debug!("songbird arc option is some");
            let (call_mutex_arc, empty_joinerror_result) =
                songbird_arc.join(guild_id, voice_channel_id).await;
            match empty_joinerror_result {
                Ok(()) => {
                    debug!("empty joinerror result is ok");
                    let mut call_mutexguard = call_mutex_arc.lock().await;
                    call_mutexguard.add_global_event(
                        Event::Track(TrackEvent::End),
                        TrackEndNotifier {
                            guild_id,
                            message_channel_id,
                            http: ctx.http.clone(),
                        },
                    );
                    call_mutexguard.add_global_event(
                        Event::Periodic(Duration::from_secs(1800), None),
                        Periodic {
                            voice_channel_id,
                            message_channel_id,
                            http: ctx.http.clone(),
                            cache: ctx.cache.clone(),
                            songbird_arc: songbird_arc.clone(),
                        },
                    );
                    return Some(songbird_arc);
                }
                Err(why) => {
                    debug!("context join channel is err");
                    error!("{}", why);
                    return None;
                }
            };
        }
        None => {
            debug!("songbird arc option is none");
            return None;
        }
    }
}

/// gets a guild document from mongo given a guild id
/// and then enqueues each url found in the vector of string queue field
///
/// ### arguments
///
/// * `ctx` - serenity client context reference
/// * `guild_id` - serenity model id guild_id
///
/// ### returns
///
/// some atomically reference counted songbird or none
///
pub async fn context_repopulate_guild_queue(
    ctx: &Context,
    guild_id: GuildId,
) -> Option<Arc<Songbird>> {
    match get_guild_document(guild_id.to_string()).await {
        Some(guild) => {
            debug!("guild option is some");
            match guild.voice_channel_id.parse::<u64>() {
                Ok(voice_channel_id) => {
                    debug!("voice channel id result is ok");
                    match guild.message_channel_id.parse::<u64>() {
                        Ok(message_channel_id) => {
                            debug!("message channel id result is ok");
                            match context_rejoin_to_voice_channel(
                                ctx,
                                guild_id,
                                ChannelId(voice_channel_id),
                                ChannelId(message_channel_id),
                            )
                            .await
                            {
                                Some(songbird_arc) => {
                                    debug!("songbird arc is some");
                                    match songbird_arc.get(guild_id) {
                                        Some(handle_lock) => {
                                            debug!("handle lock option is some");
                                            let mut handle = handle_lock.lock().await;
                                            for url in guild.queue {
                                                // Here, we use lazy restartable sources to make sure that we don't pay
                                                // for decoding, playback on tracks which aren't actually live yet.
                                                match Restartable::ytdl(url, true).await {
                                                    Ok(source) => {
                                                        debug!("ytdl is ok");
                                                        handle.enqueue_source(source.into());
                                                    }
                                                    Err(why) => {
                                                        debug!("ytdl is err");
                                                        error!("{}", why);
                                                    }
                                                };
                                            }
                                            return Some(songbird_arc);
                                        }
                                        None => {
                                            debug!("handle lock option is none");
                                            return None;
                                        }
                                    }
                                }
                                None => {
                                    debug!("songbird arc is none");
                                    return None;
                                }
                            }
                        }
                        Err(why) => {
                            debug!("message channel id result is err");
                            error!("{}", why);
                            return None;
                        }
                    };
                }
                Err(why) => {
                    debug!("voice channel id option is err");
                    error!("{}", why);
                    return None;
                }
            };
        }
        None => {
            debug!("guild option is none");
            return None;
        }
    };
}

pub async fn context_slot_is_available(
    ctx: &Context,
    guild_id: String,
    message_channel_id: ChannelId,
) -> Option<bool> {
    if let Some(guild) = get_guild_document(guild_id).await {
        if let Some(used_slots) = count_guilds_joined_to_channel().await {
            if let Some(free_slots) = count_free_guilds_joined_to_channel().await {
                if let Some(max_slots) = get_max_slots() {
                    if used_slots < max_slots {
                        if guild.has_reservation() {
                            check_msg(message_channel_id.say(&ctx.http, "good news! this guild has a reservation. please bring your friends and family with you to our discord server https://discord.gg/95eUjKqT7e 🐣").await);
                            return Some(true);
                        } else {
                            if free_slots < max_slots / 2 {
                                check_msg(message_channel_id.say(&ctx.http, "good news! there is a free slot available. please bring your friends and family with you to our discord server https://discord.gg/95eUjKqT7e 🐣").await);
                                return Some(true);
                            } else {
                                check_msg(message_channel_id.say(&ctx.http, format!("it looks like you do not have an active reservation and all free slots are currently full. please visit https://balasolu.com/rexmit/{} to reserve rexmit for your guild 🐣", guild.id)).await);
                                return Some(false);
                            }
                        }
                    } else {
                        if guild.has_reservation() {
                            if free_slots > 0 {
                                check_msg(message_channel_id.say(&ctx.http, "good news! this guild has a reservation. please bring your friends and family with you to our discord server https://discord.gg/95eUjKqT7e 🐣").await);
                                return context_boot_first_free_guild(&ctx).await;
                            } else {
                                check_msg(message_channel_id.say(&ctx.http, "it looks like you have a reservation but are unable to join. please bring your torch and pitchfork with you to our discord server https://discord.gg/95eUjKqT7e and let us know what is going on so we can look in to it 🐣").await);
                                return Some(false);
                            }
                        } else {
                            check_msg(message_channel_id.say(&ctx.http, format!("it looks like you do not have an active reservation and all free slots are currently full. please visit https://balasolu.com/rexmit/{} to reserve rexmit for your guild 🐣", guild.id)).await);
                            return Some(false);
                        }
                    }
                }
            }
        }
    }
    return None;
}

async fn context_boot_first_free_guild(ctx: &Context) -> Option<bool> {
    if let Some(free_guild) = get_first_free_guild_joined_to_channel().await {
        if let Some(free_guild_id) = parse_u64_from_string(free_guild.id) {
            if let Some(free_guild_message_channel_id) =
                parse_u64_from_string(free_guild.message_channel_id)
            {
                context_boot_guild(&ctx, GuildId(free_guild_id)).await;
                check_msg(
                    ChannelId(free_guild_message_channel_id)
                        .say(&ctx.http, "retreated to attend to a reserved guild 🐣")
                        .await,
                );
                return Some(true);
            }
        }
    }
    return None;
}
