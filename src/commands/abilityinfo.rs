extern crate heck;
extern crate json;
extern crate reqwest;

use heck::TitleCase;

use regex::Regex;

use serde_json::Value;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandError,
    CommandResult,
    macros::command,
};

use std::collections::HashMap;

mod embedbuilder;

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

    let page_parse = strip_to_page_data(&response_body);

    let page_data : String;

    match page_parse {
        Ok(x) => page_data = x,
        Err(x) => {
            msg.reply(ctx, "Server returned no options for formatting, check that an ability by that name exists or try again later.")?;
            return Err(x)
        }
    }

    //println!("{}", page_data);

    let ability_box : &str = page_data.split("</onlyinclude>").collect::<Vec<&str>>()[0];

    let mut split_ability_box : Vec<&str> = ability_box.split("\\n").collect::<Vec<&str>>();

    //Remove ending and starting lines, leaving content 
    split_ability_box.remove(0);
    split_ability_box.pop();

    let mut main_list : Vec<String> = Vec::new();
    let mut primary_fire_list : Vec<String> = Vec::new();
    let mut secondary_fire_list : Vec<String> = Vec::new();


    for item in split_ability_box {
        let mutable_item = item;

        let replaced_item = mutable_item.replace('|', "")
                    .replace('{', "")
                    .replace('}', "")
                    .replace('[', "")
                    .replace(']', "")
                    .replace("Texttip", "")
                    .replace("texttip", "")
                    .replace("/Texttip", "")
                    .replace("/texttip", "");

        let colour_code_regex = Regex::new(r"<.*?>").unwrap();

        let regex_replaced_item : String = colour_code_regex.replace_all(&replaced_item, "").trim().to_string();

        if regex_replaced_item.starts_with("image") || regex_replaced_item.starts_with("name") || regex_replaced_item.starts_with("description") {
            main_list.push(regex_replaced_item.to_string());
        }
        else if regex_replaced_item.starts_with("secd") {
            secondary_fire_list.push(regex_replaced_item.to_string());
        }
        else {
            primary_fire_list.push(regex_replaced_item.to_string());
        }
    }

    /*
    println!["{}", ability_box];

    println!();
    */

    println!["Printing main list:"];
    for item in main_list {
        println!["{}", item];
    }

    println!();
    println!["Printing primary fire list:"];
    for item in primary_fire_list {
        println!["{}", item];
    }

    if !secondary_fire_list.is_empty() {
        println!();
        println!["Printing secondary fire list:"];
        for item in secondary_fire_list {
            println!["{}", item];
        }
    }

    let channel_id = msg.channel_id;

    let _ = channel_id.send_message(&ctx.http, |m| {    
        m.embed(|e| {
            e.title("This is an embed");
            e.description("With a description");
    
            e
        });
        m
    });

    Ok(())
}

fn strip_to_page_data(response_body : &str ) -> Result<String, CommandError> {
    let response_json = json::parse(&response_body)?;

    let filtered_data = response_json["query"]["pages"].clone();

    let filtered_string = json::stringify(filtered_data);

    //Convert JSON object to a HashMap to make accessing the first object easier (The name of which is an ID, so we can't predict it)
    let deserialized : HashMap<String, Value> = serde_json::from_str(&filtered_string).unwrap();

    let value = deserialized.values().nth(0);

    let deserialised_filter : String;

    if let Some(x) = value { 
        deserialised_filter = x.to_string() 
    } 
    else {
        return Err(CommandError("Abilityinfo: Data error, no returned options from server".to_string()))
    }

    //println!["{}", deserialised_filter];

    if !deserialised_filter.contains("revisions") {
        return Err(CommandError("Abilityinfo: Data error, no returned options from server".to_string()))
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
    let arguments = format!("/api.php?action=query\
                                &prop=revisions\
                                &rvprop=content\
                                &format=json\
                                &titles={}", name
                            );
                            

    format!("{}{}", uri_base, arguments)
}