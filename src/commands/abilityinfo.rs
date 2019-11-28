extern crate heck;
extern crate http;

use heck::TitleCase;

use http::{
    Request,
    Response,
    Uri
};

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandError,
    CommandResult,
    macros::command,
};

#[command]
fn abilityinfo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut message_content : String = msg.content.clone();

    message_content = message_process(message_content);

    if !message_content.is_ascii() {
        msg.reply(ctx, "Sorry, that message doesn't appear to be a valid ascii string.")?;
        return Err(CommandError("Abilityinfo: Not ascii string".to_string()))
    }

    println!["{}", message_content];

    let ability_uri = uri_create(message_content);

    println!["{}", ability_uri.uri()];

    Ok(())
}

fn message_process(message: String) -> String {
    let mut message_content = message;
    
    message_content = message_content.replace("~abilityinfo", "");
    message_content = message_content.replace('"', "");
    message_content = message_content.replace('\'', "");
    message_content = message_content.replace('\\', "");
    message_content = message_content.trim().to_string();
    message_content = message_content.to_title_case();
    message_content.replace(" ", "_")
}

fn uri_create(name: String) -> Request<()> {
    let uri_base = "overwatch.wikia.com";
    let arguments = format!("/api.php?action=query&titles={}&prop=revisions&rvprop=content&format=json", name);
    let arguments_slice : &str = &arguments[..]; 
    
    
    let custom_uri = Uri::builder()
        .scheme("http")
        .authority(uri_base)
        .path_and_query(arguments_slice)
        .build()
        .unwrap();

    Request::builder()
        .method("GET")
        .uri(custom_uri)
        .body(())
        .unwrap()
}