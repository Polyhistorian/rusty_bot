extern crate heck;
extern crate json;
extern crate reqwest;

use std::collections::HashMap;

use heck::ToTitleCase;
use regex::Regex;
use serde_json::Value;
use serenity::framework::standard::{macros::command, CommandError, CommandResult};
use serenity::model::id::ChannelId;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod embedbuilder;

#[command]
async fn abilityinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let mut message_content: String = msg.content.clone();

    message_content = message_to_title(message_content);

    if !message_content.is_ascii() {
        msg.reply(
            &ctx.http,
            "Sorry, that message doesn't appear to be a valid ascii string.",
        )
        .await
        .map_err(|e| e.to_string())?;
        return Err(CommandError::from(
            "Abilityinfo: Not ascii string".to_string(),
        ));
    }

    //println!["{}", message_content];

    let request_url = uri_create(&message_content);

    let response_body = reqwest::get(&request_url).await?.text().await?;

    let page_parse = strip_to_page_data(&response_body);

    let page_data: String;

    match page_parse {
        Ok(x) => page_data = x,
        Err(x) => {
            msg.reply(&ctx.http, "Server returned no options for formatting, check that an ability by that name exists or try again later.").await?;
            return Err(CommandError::from(x.to_string()));
        }
    }

    //println!("{}", page_data);

    let ability_box: &str = page_data.split("</onlyinclude>").collect::<Vec<&str>>()[0];

    let mut split_ability_box: Vec<&str> = ability_box.split("\\n").collect::<Vec<&str>>();

    //Remove ending and starting lines, leaving content
    split_ability_box.remove(0);
    split_ability_box.pop();

    let mut main_list: Vec<String> = Vec::new();
    let mut primary_fire_list: Vec<String> = Vec::new();
    let mut secondary_fire_list: Vec<String> = Vec::new();

    for item in split_ability_box {
        let mutable_item = item;

        let replaced_item = mutable_item
            .replace('|', "")
            .replace('{', "")
            .replace('}', "")
            .replace('[', "")
            .replace(']', "")
            .replace("Texttip", "")
            .replace("texttip", "")
            .replace("/Texttip", "")
            .replace("/texttip", "");

        let colour_code_regex = Regex::new(r"<.*?>").unwrap();

        let regex_replaced_item: String = colour_code_regex
            .replace_all(&replaced_item, "")
            .trim()
            .to_string();

        if regex_replaced_item.contains("key=") {
            continue;
        }

        if regex_replaced_item.starts_with("image")
            || regex_replaced_item.starts_with("name")
            || regex_replaced_item.starts_with("description")
        {
            main_list.push(regex_replaced_item.to_string());
        } else if regex_replaced_item.starts_with("secd") {
            let prefix_removal = regex_replaced_item.replacen("secd", "", 1);
            secondary_fire_list.push(prefix_removal.to_string());
        } else if regex_replaced_item.starts_with("prim") {
            let prefix_removal = regex_replaced_item.replacen("prim", "", 1);
            primary_fire_list.push(prefix_removal.to_string());
        } else {
            primary_fire_list.push(regex_replaced_item.to_string());
        }
    }

    /*
    println!["{}", ability_box];

    println!();
    */

    /*
    println!["Printing main list:"];
    for item in main_list.clone() {
        println!["{}", item];
    }

    println!();
    println!["Printing primary fire list:"];
    for item in primary_fire_list.clone() {
        println!["{}", item];
    }

    if !secondary_fire_list.is_empty() {
        println!();
        println!["Printing secondary fire list:"];
        for item in secondary_fire_list.clone() {
            println!["{}", item];
        }
    }*/

    let channel_id = msg.channel_id;

    //Main message, with weapon name and description
    send_embed(channel_id, main_list, ctx).await;

    //Primary fire message, with info on that mode
    send_embed(channel_id, primary_fire_list, ctx).await;

    //All weapons don't have a secondary fire mode, so no message for them
    if !secondary_fire_list.is_empty() {
        send_embed(channel_id, secondary_fire_list, ctx).await;
    }

    Ok(())
}

fn strip_to_page_data(response_body: &str) -> std::result::Result<String, CommandError> {
    let response_json = json::parse(&response_body)?;

    let filtered_data = response_json["query"]["pages"].clone();

    let filtered_string = json::stringify(filtered_data);

    //Convert JSON object to a HashMap to make accessing the first object easier (The name of which is an ID, so we can't predict it)
    let deserialized: HashMap<String, Value> = serde_json::from_str(&filtered_string).unwrap();

    let value = deserialized.values().nth(0);

    let deserialised_filter: String;

    if let Some(x) = value {
        deserialised_filter = x.to_string()
    } else {
        return Err(CommandError::from(
            "Abilityinfo: Data error, no returned options from server".to_string(),
        ));
    }

    //println!["{}", deserialised_filter];

    if !deserialised_filter.contains("revisions") {
        return Err(CommandError::from(
            "Abilityinfo: Data error, no returned options from server".to_string(),
        ));
    }

    let page_json = json::parse(&deserialised_filter)?;

    let filtered_page_json = page_json["revisions"][0]["*"].clone();

    Ok(json::stringify(filtered_page_json))
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
    let arguments = format!(
        "/api.php?action=query\
                                &prop=revisions\
                                &rvprop=content\
                                &format=json\
                                &titles={}",
        name
    );

    format!("{}{}", uri_base, arguments)
}

async fn send_embed(channel_id: ChannelId, embed_vector: Vec<String>, ctx: &Context) {
    channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|mut e| {
                e = embedbuilder::build_new(embed_vector, e);

                e
            });
            m
        })
        .await
        .map_err(|e| {
            println!["Abilityinfo: Failed to send message, got {}", e];
        })
        .unwrap();
}
