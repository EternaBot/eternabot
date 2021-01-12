mod exts;
mod handlers;
mod typemap_keys;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use serenity::{framework::standard::StandardFramework, prelude::Client};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::ffi::OsString;
use std::path::Path;
use tokio::signal::unix::{signal, SignalKind};

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
    let token = env::var("BOT_TOKEN").expect("Discord bot token required");
    let database_url = env::var("DATABASE_URL").expect("Database URL required");

    let pool = PgPoolOptions::new()
        .max_connections(96)
        .min_connections(50)
        .connect(database_url.as_str())
        .await
        .expect("Can't connect to PostgreSQL DB");

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
        data.insert::<PostgresPool>(pool.clone());
    }

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        let mut stream = signal(SignalKind::interrupt()).expect("Unable to get SIGINT signal");
        stream.recv().await;
        info!("Got SIGINT signal...");
        info!("Shutting down...");
        shard_manager.lock().await.shutdown_all().await;
        pool.close().await;
    });

    info!("Starting EternaBot...");
    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
