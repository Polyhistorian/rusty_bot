use std::process;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[owners_only]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Shutting down.").await?;

    println!["Got shutdown command from user, shutting down."];

    println!["Disconnecting from shard"];
    ctx.shard.shutdown_clean();

    println!["Now process exit"];
    process::exit(0);
}
