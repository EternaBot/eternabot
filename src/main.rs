mod exts;
mod handlers;
mod typemap_keys;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use serenity::{framework::standard::StandardFramework, prelude::Client};
use std::env;
use std::ffi::OsString;
use std::path::Path;

use exts::*;
use handlers::handler::Handler;
use typemap_keys::*;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    if Path::new("./.env").exists() {
        dotenv::dotenv().expect("Failed to load .env file.");
    }

    pretty_env_logger::init();
    info!("Logger setup complete...");

    // Setup Sentry
    let _guard = sentry::init(
        env::var_os("SENTRY_DNS")
            .unwrap_or_else(OsString::new)
            .into_string()
            .unwrap(),
    );
    info!("Sentry setup complete...");

    // Gather token from environment
    let token: String = env::var("BOT_TOKEN").expect("Discord bot token required");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&utils::UTILS_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error while creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    info!("Starting EternaBot...");
    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
