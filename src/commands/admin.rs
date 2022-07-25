use serenity::framework::standard::{macros::command, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;

#[command]
async fn check(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, 
        "I'm alive and working!").await?;
    Ok(())
}