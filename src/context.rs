use std::{error::Error, sync::Arc, time::Duration};

use serenity::{
    model::prelude::{ChannelId, Guild, GuildId, Message, PartialGuild},
    prelude::Context,
};
use songbird::{input::Restartable, Event, Songbird, TrackEvent};
use tracing::{debug, error};

use crate::{
    command::check_msg,
    database::{get_guild_document, get_guild_queue, set_joined_to_channel},
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
    let songbird_arc_option = songbird::get(ctx).await;
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
    let songbird_arc_option = songbird::get(ctx).await;
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
    let songbird_arc_option = songbird::get(ctx).await;
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

pub async fn context_repopulate_guild_queue(
    ctx: &Context,
    guild_id: GuildId,
) -> Option<Arc<Songbird>> {
    let guild_option = get_guild_document(guild_id.to_string()).await;
    match guild_option {
        Some(guild) => {
            debug!("guild option is some");
            let voice_channel_id = ChannelId(guild.voice_channel_id.parse::<u64>().unwrap());
            let message_channel_id = ChannelId(guild.message_channel_id.parse::<u64>().unwrap());
            match context_rejoin_to_voice_channel(
                ctx,
                guild_id,
                voice_channel_id,
                message_channel_id,
            )
            .await
            {
                Some(songbird_arc) => {
                    debug!("songbird arc is some");
                    if let Some(handler_lock) = songbird_arc.get(guild_id) {
                        let mut handler = handler_lock.lock().await;
                        for url in guild.queue {
                            // Here, we use lazy restartable sources to make sure that we don't pay
                            // for decoding, playback on tracks which aren't actually live yet.
                            match Restartable::ytdl(url, true).await {
                                Ok(source) => {
                                    debug!("ytdl is ok");
                                    handler.enqueue_source(source.into());
                                }
                                Err(why) => {
                                    debug!("ytdl is err");
                                    error!("{}", why);
                                    println!("Err starting source: {:?}", why);
                                }
                            };
                        }
                    }
                    return Some(songbird_arc);
                }
                None => {
                    debug!("songbird arc is none");
                    return None;
                }
            }
        }
        None => {
            debug!("guild option is none");
            return None;
        }
    };
}
