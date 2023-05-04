use rexmit::{command::GENERAL_GROUP, handler::Handler};
use serenity::{client::Client, framework::StandardFramework, prelude::GatewayIntents};
use songbird::SerenityInit;
use std::env;
use tracing::Level;

#[tokio::main]
async fn main() {
    let debug = env::var("DEBUG").expect("Expected a DEBUG == to 1 or 0 in the environment");
    let mut log_level = Level::INFO;
    if debug == "1" {
        log_level = Level::DEBUG;
    }
    tracing_subscriber::fmt().with_max_level(log_level).init();

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

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

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
