use serenity::{
    async_trait,
    model::{guild::Guild, id::GuildId},
    prelude::{Context, EventHandler},
};

use crate::utils::guild::insert_guild;

use super::ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        ready::cache_ready(ctx).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        insert_guild(ctx, guild).await;
    }
}
