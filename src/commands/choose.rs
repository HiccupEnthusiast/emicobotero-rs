use serenity::framework::standard::{macros::command, CommandResult};
use serenity::prelude::*;
use serenity::model::prelude::*;

#[command]
#[aliases(pick, decide)]
#[description("Ask the bot to decide for you between some options")]
#[usage("<a choice>, <multiple choices>")]
#[example("pineapple banana apple lemon\n\nI choose apple!")]
#[help_available]
#[bucket("basic")]
async fn choose () -> CommandResult{
    Ok(())
}