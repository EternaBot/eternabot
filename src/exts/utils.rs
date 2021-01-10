use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong.").await?;

    Ok(())
}

#[group]
#[commands(ping)]
pub struct Utils;
