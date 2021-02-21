extern crate serenity;

use std::collections::HashMap;

fn get_weapon_property_list() -> HashMap<String, String> {
    let mut list_of_weapon_properties : HashMap<String, String> = HashMap::new();

    list_of_weapon_properties.insert("type".to_string() , "Weapon type:".to_string());
    list_of_weapon_properties.insert("ammo".to_string() , "Weapon ammo:".to_string());
    list_of_weapon_properties.insert("reload".to_string() , "Reload time:".to_string());
    list_of_weapon_properties.insert("damage".to_string() , "Weapon damage:".to_string());
    list_of_weapon_properties.insert("numofsmallies".to_string() , "Number of pellets:".to_string());
    list_of_weapon_properties.insert("maxdamage".to_string() , "Max damage potential:".to_string());
    list_of_weapon_properties.insert("falloffrange".to_string() , "Weapon falloff range:".to_string());
    list_of_weapon_properties.insert("firerate".to_string() , "Firerate:".to_string());
    list_of_weapon_properties.insert("isfalloff".to_string() , "Weapon has falloff:".to_string());
    list_of_weapon_properties.insert("isheadshot".to_string() , "Weapon can headshot:".to_string());
    list_of_weapon_properties.insert("heal".to_string() , "Healing rate:".to_string());
    list_of_weapon_properties.insert("range".to_string() , "Range:".to_string());
    list_of_weapon_properties.insert("radius".to_string() , "Radius:".to_string());
    list_of_weapon_properties.insert("effect".to_string() , "Effect:".to_string());
    list_of_weapon_properties.insert("duration".to_string() , "Duration:".to_string());
    list_of_weapon_properties.insert("cooldown".to_string() , "Cooldown:".to_string());
    list_of_weapon_properties.insert("casttime".to_string() , "Casttime:".to_string());
    list_of_weapon_properties.insert("description".to_string() , "Casttime:".to_string());
    list_of_weapon_properties.insert("ulttype".to_string(), "Type of ultimate:".to_string());
    list_of_weapon_properties.insert("movementspeed".to_string(), "Movement speed while active (m/s):".to_string());
    list_of_weapon_properties.insert("brrhealth".to_string(), "Health of the barrier:".to_string());
    list_of_weapon_properties.insert("projspeed".to_string(), "Speed of the projectile (m/s):".to_string());

    list_of_weapon_properties
} 

pub fn build_new(list : Vec<String>, e : &mut serenity::builder::CreateEmbed) -> &mut serenity::builder::CreateEmbed {
    let list_of_weapon_properties : HashMap<String, String> = get_weapon_property_list();

    for item in list {
        let split : Vec<&str> = item.split('=').collect();
        let property = split[0].trim();
        let value = split[1].trim();

        if property == "name" {
            e.title(value);
            continue
        }
        if property == "description" {
            e.description(value);
            continue
        }
        if property == "ultquote" {
            e.description(format!["\"{}\"", value]);
            continue
        }

        if let Some(property_string) = list_of_weapon_properties.get(property) {
            e.field(property_string, value, true);
        }
    }

    e
}
