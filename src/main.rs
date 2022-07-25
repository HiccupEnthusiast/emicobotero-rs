use std::{collections::HashSet, env};

use serenity::{
    prelude::*,
    async_trait,
    model::{gateway::{Ready, GatewayIntents}, id::UserId}, 
    framework::{StandardFramework, standard::macros::group},
};

mod commands;
use crate::commands::admin::*;
use crate::commands::ping::*;

#[group]
#[commands(ping)]
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
            .prefix("e!")
            .delimiters(vec![" ", ", ", ","])
            .owners(owners)
    ).group(&GENERAL_GROUP).group(&OWNER_GROUP);

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