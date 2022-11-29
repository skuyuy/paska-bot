use std::env;
use serenity::{
    prelude::*,
    model::channel::Message,
    framework::standard::{
        Args,
        CommandResult,
        macros::{command}
    }
};

#[command]
pub async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let base_url = env::var("BASE_URL").expect("");
    let full_url = match args.single::<String>() {
        Ok(v) => format!("{}#{}", base_url, v),
        _ => format!("{}", base_url)
    };

    msg.channel_id.say(&ctx.http, full_url).await?;
    Ok(())
}