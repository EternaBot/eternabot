mod exts;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::ffi::OsString;
use serenity::{
    prelude::Client,
    framework::standard::StandardFramework,
};

use exts::*;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenv::dotenv().expect("Failed to load .env file.");

    pretty_env_logger::init();
    info!("Logger setup complete...");

    // Setup Sentry
    let _guard = sentry::init(env::var_os("SENTRY_DNS")
        .unwrap_or_else(OsString::new).into_string().unwrap());
    info!("Sentry setup complete...");
    
    // Gather token from environment
    let token: String = env::var("BOT_TOKEN")
        .expect("Discord bot token required");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&utils::UTILS_GROUP);
    
    let mut client = Client::builder(token)
        .framework(framework)
        .await
        .expect("Error while creating client");
    
    info!("Starting EternaBot...");
    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
