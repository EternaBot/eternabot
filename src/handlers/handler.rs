use serenity::{
    async_trait,
    model::id::GuildId,
    prelude::{Context, EventHandler},
};

use super::ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        ready::cache_ready(ctx).await;
    }
}
