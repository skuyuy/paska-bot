use std::env;

use data::release::Release;
use serenity::{
    async_trait,
    prelude::*,
    model::channel::Message,
    framework::standard::{
        macros::{command, group},
        StandardFramework,
        CommandResult
    },
};

mod commands;
mod data;

use crate::{
    commands::{
        help::*
    },
    data::ReleaseStore
};

#[group]
#[commands(help)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {

}

#[tokio::main]
async fn main() {
    let mut releases = ReleaseStore::new();
    if let Err(err) = releases.read("store.json") {
        panic!("Could not read releases: {}", err)
    }

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("An error occurred while running the client: {:?}", err);
    }
}