use std::time::Duration;

use serenity::{
    model::prelude::{ChannelId, Message, PartialGuild, Guild},
    prelude::Context,
};
use songbird::{error::JoinError, Event, TrackEvent};
use tracing::{debug, error};

use crate::{
    command::check_msg,
    handler::{Periodic, TrackEndNotifier}, database::set_joined_to_channel,
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

pub async fn context_songbird_join(
    ctx: &Context,
    msg: &Message,
    voice_channel_id: ChannelId,
) -> Result<(), JoinError> {
    let songbird_arc_option = songbird::get(ctx).await;
    let songbird_arc = match songbird_arc_option {
        Some(songbird_arc) => songbird_arc.clone(),
        None => {
            return Err(JoinError::Dropped);
        }
    };

    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Err(JoinError::Dropped);
        }
    };

    let (call_mutex_arc, empty_joinerror_result) =
        songbird_arc.join(guild_id, voice_channel_id).await;

    match &empty_joinerror_result {
        Ok(()) => {
            let mut call_mutexguard = call_mutex_arc.lock().await;
            call_mutexguard.add_global_event(
                Event::Track(TrackEvent::End),
                TrackEndNotifier {
                    guild_id,
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
        }
        Err(joinerror) => {}
    }
    return empty_joinerror_result;
}

pub async fn context_join_to_voice_channel(ctx: &Context, msg: &Message, guild: &Guild) -> Result<(), JoinError> {
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
            return Err(JoinError::Dropped);
        }
    };
    match context_songbird_join(ctx, msg, voice_channel_id).await {
        Ok(()) => {
            debug!("context join channel is ok");
            set_joined_to_channel(
                guild.id.to_string(),
                Some(voice_channel_id.to_string()),
                Some(msg.channel_id.to_string()),
            )
            .await;
            check_msg(msg.channel_id.say(&ctx.http, "joined").await);
            return Ok(());
        },
        Err(why) => {
            debug!("context join channel is err");
            error!("{}", why);
            return Err(why);
        }
    }
}