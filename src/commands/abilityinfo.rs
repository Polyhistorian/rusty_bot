extern crate heck;
extern crate reqwest;
extern crate json;

use heck::TitleCase;

use serde_json::Value;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandError,
    CommandResult,
    macros::command,
};

use std::collections::HashMap;

#[command]
fn abilityinfo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut message_content : String = msg.content.clone();

    message_content = message_to_title(message_content);

    if !message_content.is_ascii() {
        msg.reply(ctx, "Sorry, that message doesn't appear to be a valid ascii string.")?;
        return Err(CommandError("Abilityinfo: Not ascii string".to_string()))
    }

    //println!["{}", message_content];

    let request_url = uri_create(&message_content);

    let response_body = reqwest::get(&request_url)?.text()?;

    let response_json = json::parse(&response_body)?;

    let filtered_data = response_json["query"]["pages"].clone();

    let filtered_string = json::stringify(filtered_data);

    let deserialized : HashMap<String, Value> = serde_json::from_str(&filtered_string).unwrap();

    if deserialized.len() != 1{
        msg.reply(ctx, "Server returned too many options for formatting, try again later.")?;
        return Err(CommandError("Abilityinfo: Data error, too many returned options from server".to_string()))
    }

    let mut deserialised_filter : String = "".to_string();

    for (_, value) in deserialized {
        deserialised_filter = value.to_string()
    };

    if deserialised_filter.len() == 0{
        msg.reply(ctx, "Server returned too many options for formatting, try again later.")?;
        return Err(CommandError("Abilityinfo: Data error, too many returned options from server".to_string()))
    }

    let ability_json = json::parse(&deserialised_filter)?;

    let filtered_ability_json = ability_json["revisions"][0]["*"].clone();

    let ability_string : String = json::stringify(filtered_ability_json);

    println!("{}", ability_string);

    Ok(())
}

fn message_to_title(message: String) -> String {
    let mut message_content = message;
    
    message_content = message_content.replace("~abilityinfo", "");
    message_content = message_content.replace('"', "");
    message_content = message_content.replace('\'', "");
    message_content = message_content.replace('\\', "");
    message_content = message_content.trim().to_string();
    message_content = message_content.to_title_case();
    message_content.replace(" ", "_")
}

fn uri_create(name: &str) -> String {
    let uri_base = "http://overwatch.wikia.com";
    let arguments = format!("/api.php?action=query\
                                &prop=revisions\
                                &rvprop=content\
                                &format=json\
                                &titles={}", name
                            );
                            

    format!("{}{}", uri_base, arguments)
}