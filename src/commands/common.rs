use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use std::process;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
#[owners_only]
fn shutdown(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Shutting down.")?;

    println!["Got shutdown command from user, shutting down."];

    process::exit(0);
}