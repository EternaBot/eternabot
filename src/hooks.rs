use serenity::{
    framework::standard::{macros::hook, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[hook]
pub async fn after(
    _ctx: &Context,
    msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => {
            info!(
                "Processed command '{}' by {} ({})",
                command_name,
                msg.author.tag(),
                msg.author.id
            );
        }
        Err(why) => {
            error!(
                "Command '{}' by {} ({}) returned with error: {}",
                command_name,
                msg.author.tag(),
                msg.author.id,
                why
            );
        }
    }
}
