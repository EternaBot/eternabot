use serenity::{client::bridge::gateway::ShardManager, prelude::TypeMapKey};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct PostgresPool;

impl TypeMapKey for PostgresPool {
    type Value = PgPool;
}
