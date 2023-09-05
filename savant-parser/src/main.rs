use std::{error::Error, io, process};
use serde::Deserialize;
use std::cmp::Ordering;

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
    combat_options: Option<i16>,
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
struct WeaponOptions {
    attack_speed: i16,
    combat_options: i16,
    special_accuracy: f32,
    special_damage_1: f32,
    special_damage_2: f32,
    special_defence_roll: String,
}

#[derive(Debug, Default)]
pub struct Item {
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


impl PartialEq for Item {
    fn eq(&self, other: &utmpx) -> bool {
        self.ut_type == other.ut_type
            && self.ut_pid == other.ut_pid
            && self.ut_name == other.ut_name
            && self.ut_line == other.ut_line
            && self.ut_id == other.ut_id
            && self.ut_exit == other.ut_exit
            && self.ut_session == other.ut_session
            && self.ut_tv == other.ut_tv
            && self.ut_ss == other.ut_ss
            && self
            .ut_pad
            .iter()
            .zip(other.ut_pad.iter())
            .all(|(a,b)| a == b)
            && self
            .ut_host
            .iter()
            .zip(other.ut_host.iter())
            .all(|(a,b)| a == b)
    }
}


//impl Ord for Item{
//    fn cmp(&self, other: &Item) -> Ordering {
//        (self.raw.time, self.raw.offset).cmp(&(other.raw.time, other.raw.offset))
//    }
//}

impl Item {
    fn empty() -> Item {
        Item::default()
    }

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
            magic_damage: item.magic_damage.unwrap_or(0.0),
            prayer: item.prayer.unwrap_or_default(),
            weapon: match item.attack_speed {
                Some(attack_speed) => Some(WeaponOptions {
                    attack_speed,
                    combat_options: item.combat_options.unwrap_or_default(),
                    special_accuracy: item.special_accuracy.unwrap_or_default(),
                    special_damage_1: item.special_damage_1.unwrap_or_default(),
                    special_damage_2: item.special_damage_2.unwrap_or_default(),
                    special_defence_roll: item.special_defence_roll.unwrap_or_default(),
                }),
                None => None,
            }
        }
    }
}

fn read_in_csv() -> Result<Vec<CsvItem>, Box<dyn Error>> {
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

pub fn generate_items() -> Vec<Item> {
    //let's switch this to take a reader as input
    //

    let mut items = Vec::new();
    match read_in_csv() {
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
        Ok(vec) => {
            for csvitem in vec {
                if let Some(i16) = csvitem.spell_max_hit {
                    break;
                }
                let item: Item = csvitem.into();
                println!("{:#?}", item);
                items.push(item);
            }
        }
    }

    items

}

pub struct SortedItems {
    head: Vec<Item>,
    cape: Vec<Item>,
    neck: Vec<Item>,
    ammo: Vec<Item>,
    weapon: Vec<Item>,
    body: Vec<Item>,
    shield: Vec<Item>,
    legs: Vec<Item>,
    hands: Vec<Item>,
    feet: Vec<Item>,
    ring: Vec<Item>,
}

impl SortedItems {
    fn new(full: Vec<Item>) -> SortedItems {
        let mut none_counter = 0;
        let empty_item = Item::empty();
        let mut full = full;

        let mut head: Vec<Item> = Vec::new();
        {
        let first = full.pop().unwrap();
        if first.name != " " {
            panic!("format changed - first item is not blank with name ' '");
        }
        let second = full.pop().unwrap();
        if second != empty_item {
            panic!("format changed - second item is not empty");
        }


        loop {

        }
        }

        for item in full {

        }

        todo!()
    }
}

pub struct Equipment {
    head: Item,
    cape: Item,
    neck: Item,
    ammo: Item,
    weapon: Item,
    body: Item,
    shield: Item,
    legs: Item,
    hands: Item,
    feet: Item,
    ring: Item,
}

impl Equipment {

    fn add_item (mut self, item_list: &[Item], item_name: String) -> Equipment {
        todo!()
    }

    fn remove_item(mut self, item_name: String) -> Equipment {
        todo!()
    }

}


fn main() {
    generate_items();
}
