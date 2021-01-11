use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
    utils::Colour,
};

use crate::typemap_keys::ShardManagerContainer;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // First we need to get shard manager from client data.
    let data = ctx.data.read().await;
    let shard_manager = data
        .get::<ShardManagerContainer>()
        .expect("Unable to get shard manager");

    // After we have shard manager, we must get runners.
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    // Now we can get specific shard runner.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(r) => r,
        None => {
            msg.reply(ctx, "Unable to get guild's shard information.")
                .await?;

            return Ok(());
        }
    };

    // As now we also have shard information, we can get latency.
    let latency = match runner.latency {
        Some(l) => l.as_millis().to_string(),
        None => String::from("?"),
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Pong!");
                e.field("Latency", &format!("{} **ms**", latency), true);
                e.footer(|f| {
                    f.text(&format!("Shard {:?}/{:?}", ctx.shard_id + 1, runners.len()));

                    f
                });
                e.colour(Colour::DARK_ORANGE);

                e
            });

            m
        })
        .await?;

    Ok(())
}

#[group]
#[commands(ping)]
pub struct Utils;
