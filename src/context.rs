use std::time::Duration;

use serenity::{
    model::prelude::{Guild, Message, PartialGuild, GuildId, ChannelId},
    prelude::Context,
};
use songbird::{Event, TrackEvent};
use tracing::{debug, error};

use crate::{
    database::set_joined_to_channel,
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
                    set_joined_to_channel(
                        guild_id.to_string(),
                        None,
                        None,
                    )
                    .await;
                } ,
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
}