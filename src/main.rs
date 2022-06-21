#![warn(clippy::all, /*clippy::cargo,*/ clippy::pedantic)]

extern crate tokio;

use std::{collections::HashSet, fs, process};

use serenity::http::Http;
use serenity::prelude::GatewayIntents;
use serenity::{
    async_trait,
    client::Client,
    framework::standard::{macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::{Context, EventHandler},
};

use commands::{abilityinfo::*, common::*};

mod commands;

#[group("general")]
#[commands(ping, shutdown)]
struct General;

#[group("abilityinfo")]
#[default_command(abilityinfo)]
#[commands(abilityinfo)]
struct AbilityInfo;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("token.txt").expect(
        "Something went wrong reading the token file, ensure that you have a file named token.txt",
    );

    if token.len() != 59 {
        println!["The token is not the correct length, please check that the token is correctly inputted."];
        process::exit(1);
    }

    let http = Http::new(&token);

    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .group(&ABILITYINFO_GROUP)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    // Login with a bot token from the environment
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
