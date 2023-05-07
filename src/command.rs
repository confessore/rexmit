use std::time::Duration;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandError, CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
    Result as SerenityResult,
};
use songbird::{
    input::{self, Restartable},
    Event, TrackEvent,
};
use tracing::debug;

use crate::{
    context::context_join_to_voice_channel,
    database::{
        clear_guild_queue, get_guild_is_subscribed, set_guild_queue, set_joined_to_channel,
    },
    handler::{SongEndNotifier, SongFader},
};

#[group]
#[commands(
    d, deafen, j, join, l, leave, m, mute, q, queue, s, skip, c, clear, stop, p, ping, ud,
    undeafen, um, unmute
)]
struct General;

pub async fn rejoin_channels() {}

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
        }
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

#[command]
#[only_in(guilds)]
async fn j(ctx: &Context, msg: &Message) -> CommandResult {
    return join(ctx, msg, _args).await;
}

/// joins the command issuer's voice channel
/// without a database url, the join simply occurs
/// with a database url, a subscription check is performed
///
/// ### arguments
///
/// * `ctx` - a reference to the context
/// * `msg` - a reference to the message
///
/// ### returns
///
/// command result
///
#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let subscribed_option = get_guild_is_subscribed(guild.id.to_string()).await;
    match subscribed_option {
        Some(subscribed) => {
            debug!("subscribed option is some");
            if subscribed {
                let log = "guild is subscribed";
                debug!(log);
                match context_join_to_voice_channel(ctx, msg, &guild).await {
                    Some(_success) => {
                        debug!("context join to voice channel is some");
                        return Ok(());
                    }
                    None => {
                        let log = "context join to voice channel is none";
                        debug!(log);
                        check_msg(msg.channel_id.say(&ctx.http, log).await);
                        return Err(CommandError::from(log));
                    }
                }
            } else {
                let log = "guild is not subscribed";
                debug!(log);
                check_msg(msg.channel_id.say(&ctx.http, log).await);
                return Err(CommandError::from(log));
            }
        }
        None => {
            debug!("subscribed option is none");
            match context_join_to_voice_channel(ctx, msg, &guild).await {
                Some(_success) => {
                    debug!("context join to voice channel is some");
                    return Ok(());
                }
                None => {
                    debug!("context join to voice channel is none");
                    let log = "context join to voice channel is none";
                    debug!(log);
                    return Err(CommandError::from(log));
                }
            }
        }
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
    if msg.guild_id.is_some() {
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
            clear_guild_queue(guild_id.to_string()).await;
            set_joined_to_channel(guild_id.to_string(), None, None).await;
        } else {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);
        }
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
        }
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
        }
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
            }
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

#[command]
#[only_in(guilds)]
async fn q(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    return queue(ctx, msg, args).await;
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // capture args before mutating
    let og_args = &args.clone();

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
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
            }
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

        let mut queue = vec![];
        for track_handle in handler.queue().current_queue() {
            queue.push(track_handle.metadata().source_url.clone().unwrap())
        }

        set_guild_queue(guild_id.to_string(), queue).await;
    } else {
        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    "not in a voice channel to play in. attempting to join",
                )
                .await,
        );
        match join(ctx, msg, og_args.to_owned()).await {
            Ok(result) => {
                match queue(ctx, msg, og_args.to_owned()).await {
                    Ok(result) => result,
                    Err(why) => {
                        check_msg(msg.channel_id.say(&ctx.http, "unable to queue").await);
                        return Err(why);
                    }
                };
                result
            }
            Err(why) => {
                check_msg(msg.channel_id.say(&ctx.http, "unable to join").await);
                return Err(why);
            }
        };
    }

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
        let track_queue = handler.queue();
        let _ = track_queue.skip();

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Song skipped: {} in queue.", track_queue.len()),
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
        let track_queue = handler.queue();
        let _ = track_queue.stop();

        check_msg(msg.channel_id.say(&ctx.http, "Queue cleared.").await);

        let mut queue = vec![];
        for track_handle in handler.queue().current_queue() {
            queue.push(track_handle.metadata().source_url.clone().unwrap())
        }

        clear_guild_queue(guild_id.to_string()).await;
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
pub async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
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
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
