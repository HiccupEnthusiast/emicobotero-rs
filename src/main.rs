use std::{collections::HashSet, env};

use serenity::{
    prelude::*,
    async_trait,
    model::{
        gateway::{
            Ready, 
            GatewayIntents
        }, 
        id::UserId, 
        channel::Message
    }, 
    framework::{
        StandardFramework, 
        standard::macros::{
            help, group
        }
    },
    framework::standard::{
        help_commands,
        HelpOptions, 
        CommandResult,
        CommandGroup, 
        Args
    },
};

mod commands;
use crate::commands::admin::*;
use crate::commands::ping::*;
use crate::commands::choose::*;

#[help]
#[individual_command_tip(
    "Oh hello, hi! If you want to get more information about a command you can do so by typing e!help <the command you want to see more info>. \n\n")]
// In practice these two appear to be switched, stay tunned to next update
#[no_help_available_text("Error: I don't know that command!")]
#[command_not_found_text("It seems like this command doesn't have any help written yet :(")]
#[suggestion_text("Bottom text")]
#[lacking_role("strike")]
#[lacking_ownership("hide")]
#[lacking_permissions("strike")]
#[wrong_channel("strike")]
async fn my_help (
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(
        ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(ping, choose, ask)]
struct General;

#[group]
#[owners_only]
#[commands(check)]
struct Owner;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, bot: Ready) {
        println!(
            "{}#{} has been connected and is ready for operation",
            bot.user.name, bot.user.discriminator);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Couldn't load .env file");
    let token = env::var("TOKEN").expect(
        "Couldn't find TOKEN in .env, make sure there isn't any spaces between the key, the equal, and the value");
    

    // IMPORTANT: The ids in the .env MUST not have spaces in between them and
    // have to be separated using commas, ex; 
    // OWNERS=00000000,11111111,123456789,55661122
    let owners_ids = env::var("OWNERS").expect("
        Couldn't load OWNERS in .env");

    let mut owners = HashSet::new() as HashSet<UserId>;
    for id in owners_ids.split(","){
        owners.insert(UserId(id.parse().unwrap()));
    }
    

    let framework = StandardFramework::new()
    .configure(
        |c| c
            .prefix("h!")
            .delimiters(vec![" ", ", ", ","])
            .owners(owners)
    )
    .bucket("basic", |b| b.delay(1)).await
    .help(&MY_HELP)
    .group(&GENERAL_GROUP)
    .group(&OWNER_GROUP);

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error while creating client");

    if let Err(err) = client.start().await {
        println!("Error starting the client: {:#?}", err);
    }
}