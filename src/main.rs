use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

group!({
    name: "general",
    options: {},
    commands: [ping],
});

use std::env;
use std::fs;
use std::process;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let token = fs::read_to_string("token.txt").expect("Something went wrong reading the token file, ensure that you have a file named token.txt");

    if token.len() != 59 {
        println!["{}", token.len()];
        println!["The token is not the correct length, please check that the token is correctly inputted."];
        process::exit(1);
    }

    // Login with a bot token from the environment
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}