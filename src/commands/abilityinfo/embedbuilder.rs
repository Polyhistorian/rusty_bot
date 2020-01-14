extern crate serenity;

use std::collections::HashMap;

fn build(list : Vec<String>) -> serenity::model::channel::Embed {
    let mut list_of_weapon_properties : HashMap<String, String> = HashMap::new();

}

fn build_main(list : Vec<String>) -> serenity::model::channel::Embed  {
    let e : serenity::model::channel::Embed;
    
    for item in list {
        if item == "name".to_string() {
            e.title(item);
        }

    };

    e
}