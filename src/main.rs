use std::env;

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
use crate::commands::{help::*};

#[group]
#[commands(help)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {

}

#[tokio::main]
async fn main() {
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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}