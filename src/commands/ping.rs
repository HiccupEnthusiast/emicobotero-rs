use serenity::framework::standard::{macros::command, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, 
        format!("Pong! <@{}>", msg.author.id)).await?;

    Ok(())
}