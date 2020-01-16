use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use std::{process};

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
#[owners_only]
fn shutdown(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx.clone(), "Shutting down.")?;

    println!["Got shutdown command from user, shutting down."];

    println!["Disconnecting from shard"];
    ctx.shard.shutdown_clean();

    println!["Now process exit"];
    process::exit(0);
}