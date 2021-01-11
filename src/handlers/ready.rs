use std::env;

use sentry::capture_error;
use serenity::{model::id::ChannelId, prelude::Context};

pub async fn cache_ready(ctx: Context) {
    let channel_id = match env::var_os("CHANNEL_DEV_LOG") {
        Some(val) => val
            .into_string()
            .unwrap()
            .parse::<u64>()
            .unwrap_or(798065874236866560),
        None => 798065874236866560,
    };
    if let Err(why) = ChannelId(channel_id)
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("All shards is ready!");
                e
            })
        })
        .await
    {
        capture_error(&why);
    }
}
