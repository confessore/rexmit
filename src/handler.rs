use crate::{
    command::check_msg,
    database::{
        clear_guild_queue, count_free_guilds_joined_to_channel, count_guilds_joined_to_channel,
        count_subscribed_guilds_joined_to_channel, pop_guild_queue, set_joined_to_channel, get_first_free_guild_joined_to_channel,
    }, context::context_boot_guild,
};
use serenity::{
    async_trait,
    client::{Cache, Context, EventHandler},
    http::Http,
    model::{
        gateway::Ready,
        prelude::{Activity, ChannelId, GuildId},
    },
};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler, Songbird};
use std::sync::Arc;
use tracing::{info, debug};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::listening("~q <youtube url>"))
            .await;
        println!("{} is connected!", ready.user.name);

        let free_guilds_option = count_free_guilds_joined_to_channel().await;
        if free_guilds_option.is_some() {
            info!("free guilds: {}", free_guilds_option.unwrap())
        }

        let sub_guilds_option = count_subscribed_guilds_joined_to_channel().await;
        if sub_guilds_option.is_some() {
            info!("sub guilds: {}", sub_guilds_option.unwrap())
        }

        let guilds_option = count_guilds_joined_to_channel().await;
        if guilds_option.is_some() {
            info!("total guilds: {}", guilds_option.unwrap())
        }

        match get_first_free_guild_joined_to_channel().await {
            Some(free_guild) => {
                debug!("free guild option is some");
                info!("{:#?}", free_guild);
                context_boot_guild(&ctx, free_guild).await;
            },
            None => {
                debug!("free guild option is none");
            }
        }

        //checking guild queues, could use better naming
        /*let joined_guilds_option = get_guilds_joined_to_channel().await;
        if joined_guilds_option.is_some() {
            let joined_guilds = joined_guilds_option.unwrap();
            for joined_guild in joined_guilds {
                let subscribed_option = get_guild_is_subscribed(joined_guild.to_string()).await;
                match subscribed_option {
                    Some(subscribed) => {
                        info!("guild is subscribed: {:?}", subscribed);
                    },
                    None => {}
                }
            }
        }*/
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

pub struct TrackEndNotifier {
    pub guild_id: GuildId,
    pub message_channel_id: ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            pop_guild_queue(self.guild_id.to_string()).await;
            check_msg(
                self.message_channel_id
                    .say(
                        &self.http,
                        &format!(
                            "Track ended: {}",
                            track_list
                                .first()
                                .as_ref()
                                .unwrap()
                                .1
                                .metadata()
                                .source_url
                                .as_ref()
                                .unwrap()
                        ),
                    )
                    .await,
            );
        }

        None
    }
}

pub struct SongFader {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
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

pub struct SongEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
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

pub struct Periodic {
    pub voice_channel_id: ChannelId,
    pub message_channel_id: ChannelId,
    pub http: Arc<Http>,
    pub cache: Arc<Cache>,
    pub songbird_arc: Arc<Songbird>,
}

#[async_trait]
impl VoiceEventHandler for Periodic {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let channel = self.http.get_channel(self.voice_channel_id.into()).await;
        match channel.unwrap().guild() {
            Some(guild_channel) => {
                let members = guild_channel.members(&self.cache).await;

                // please modularize this monstrocity
                // what i mean by this is create some functions and call the functions instead
                // we want to utilize DRY (DON'T REPEAT YOURSELF) principles
                if members.unwrap().len() <= 1 {
                    let has_handler = self.songbird_arc.get(guild_channel.guild_id).is_some();

                    if has_handler {
                        if let Err(e) = self.songbird_arc.remove(guild_channel.guild_id).await {
                            check_msg(
                                self.message_channel_id
                                    .say(&self.http, format!("Failed: {:?}", e))
                                    .await,
                            );
                        }

                        check_msg(
                            self.message_channel_id
                                .say(&self.http, "Left voice channel")
                                .await,
                        );
                        match guild_channel.guild(&self.cache) {
                            Some(_guild) => {
                                clear_guild_queue(guild_channel.guild_id.to_string()).await;
                            }
                            None => {}
                        };
                        set_joined_to_channel(guild_channel.guild_id.to_string(), None, None).await;
                    } else {
                        check_msg(
                            self.message_channel_id
                                .say(&self.http, "Not in a voice channel")
                                .await,
                        );
                    }
                }
            }
            None => {
                println!("{}", "channel is none")
            }
        }

        None
    }
}
