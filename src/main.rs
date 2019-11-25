#![warn(clippy::all, /*clippy::cargo,*/ clippy::pedantic)]
 
use serenity::{
    client::Client,
    model::{
        channel::Message,
        gateway::Ready
    },
    prelude::{
        EventHandler, 
        Context
    },
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }   
    }
};

use std::{
    fs,
    process,
    collections::HashSet
};

group!({
    name: "general",
    options: {},
    commands: [ping, shutdown],
});

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

fn main() {
    let token = fs::read_to_string("token.txt").expect("Something went wrong reading the token file, ensure that you have a file named token.txt");

    if token.len() != 59 {
        println!["The token is not the correct length, please check that the token is correctly inputted."];
        process::exit(1);
    }

    // Login with a bot token from the environment
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("~")
        )
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

#[command]
#[owners_only]
fn shutdown(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Shutting down.")?;

    println!["Got shutdown command from user, shutting down."];

    process::exit(0);
}

