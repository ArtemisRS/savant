use std::{error::Error, io, process};
use serde::Deserialize;

///Design ideas
///1. Read in the CSV and turn it into the basic item below that still has all
///   its fields as options.
///2. Separate out the giant list into separate lists for each item type. We
///   know that between each item type is an empty item followed by an item with
///   the name None
///3. Convert the items into the list into the equivalent 'real' item struct
///   that drops some fields and no longer uses Option


#[derive(Debug, Deserialize, Default)]
struct CsvItem {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Stab attack")]
    stab_attack: Option<i16>,
    #[serde(rename = "Slash attack")]
    slash_attack: Option<i16>,
    #[serde(rename = "Crush attack")]
    crush_attack: Option<i16>,
    #[serde(rename = "Magic attack")]
    magic_attack: Option<i16>,
    #[serde(rename = "Ranged attack")]
    ranged_attack: Option<i16>,
    #[serde(rename = "Stab defence")]
    stab_defence: Option<i16>,
    #[serde(rename = "Slash defence")]
    slash_defence: Option<i16>,
    #[serde(rename = "Crush defence")]
    crush_defence: Option<i16>,
    #[serde(rename = "Magic defence")]
    magic_defence: Option<i16>,
    #[serde(rename = "Ranged defence")]
    ranged_defence: Option<i16>,
    #[serde(rename = "Melee strength")]
    melee_strength: Option<i16>,
    #[serde(rename = "Ranged strength")]
    ranged_strength: Option<i16>,
    #[serde(rename = "Magic damage")]
    magic_damage: Option<f32>,
    #[serde(rename = "Prayer")]
    prayer: Option<i16>,
    #[serde(rename = "Special accuracy")]
    special_accuracy: Option<f32>,
    #[serde(rename = "Special damage 1")]
    special_damage_1: Option<f32>,
    #[serde(rename = "Special damage 2")]
    special_damage_2: Option<f32>,
    #[serde(rename = "Special defence roll")]
    special_defence_roll: Option<String>,
    #[serde(rename = "Spell max hit")]
    spell_max_hit: Option<i16>, //set to string for magic dart
    #[serde(rename = "Attack speed")]
    attack_speed: Option<i16>, //set to f32 for heka
    #[serde(rename = "Combat options")]
    combat_options: Option<String>,
    #[serde(rename = "Mining level req")]
    mining_level_req: Option<i16>,
}

enum CombatStyle {
    Stab,
    Slash,
    Crush,
    Ranged,
    Magic,
}

#[derive(Debug, Default)]
struct Item {
    name: String,
    stab_attack: i16,
    slash_attack: i16,
    crush_attack: i16,
    magic_attack: i16,
    ranged_attack: i16,
    stab_defence: i16,
    slash_defence: i16,
    crush_defence: i16,
    magic_defence: i16,
    ranged_defence: i16,
    melee_strength: i16,
    ranged_strength: i16,
    magic_damage: f32,
    prayer: i16,
    weapon: Option<WeaponOptions>,
}

#[derive(Debug, Default)]
struct WeaponOptions {
    attack_speed: i16,
    combat_options: i16,
    special_accuracy: f32,
    special_damage_1: f32,
    special_damage_2: f32,
    special_defence_roll: String,
}

impl From<CsvItem> for Item {
    fn from(item: CsvItem) -> Self {
        Item{
            name: item.name.unwrap_or_default(),
            stab_attack: item.stab_attack.unwrap_or_default(),
            slash_attack: item.slash_attack.unwrap_or_default(),
            crush_attack: item.crush_attack.unwrap_or_default(),
            magic_attack: item.magic_attack.unwrap_or_default(),
            ranged_attack: item.ranged_attack.unwrap_or_default(),
            stab_defence: item.stab_defence.unwrap_or_default(),
            slash_defence: item.slash_defence.unwrap_or_default(),
            crush_defence: item.crush_defence.unwrap_or_default(),
            magic_defence: item.magic_defence.unwrap_or_default(),
            ranged_defence: item.magic_defence.unwrap_or_default(),
            melee_strength: item.melee_strength.unwrap_or_default(),
            ranged_strength: item.ranged_strength.unwrap_or_default(),
            magic_damage: item.magic_damage.unwrap_or(1.0),
            prayer: item.prayer.unwrap_or_default(),
            attack_speed: item.attack_speed.unwrap_or(4),
        }
    }
}

fn example() -> Result<Vec<CsvItem>, Box<dyn Error>> {
    let mut items = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let item: CsvItem = result.unwrap_or_default();
        items.push(item);
        //println!("{:?}", item);
    }
    Ok(items)
}

fn main() {
    println!("Hello, world!");
    match example() {
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
        Ok(vec) => {
            for csvitem in vec {
                let item: Item = csvitem.into();
                println!("{:#?}", item)
            }
        }
    }
}
