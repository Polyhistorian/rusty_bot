extern crate heck;
use heck::TitleCase;

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