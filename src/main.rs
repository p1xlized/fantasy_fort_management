mod models;
mod utils;

use models::fort::Fort;
use models::loot::{ItemType, Loot, Rarity};
use utils::fileread::read_file;

fn main() {
    // let mut user_armory = Armory::new();
    let sword = Loot {
        name: String::from("Rusted Sword"),
        rarity: Rarity::Common,
        value: 5,
        item_type: ItemType::Weapon,
    };
    let talisman = Loot {
        name: String::from("Null Talisman"),
        rarity: Rarity::Arcane,
        value: 5,
        item_type: ItemType::Artifact,
    };
    let cuirass = Loot {
        name: String::from("Mage Knight Cuirass"),
        rarity: Rarity::Legendary,
        value: 5,
        item_type: ItemType::Armor,
    };
    match read_file("raid_names.txt") {
        Ok(_) => println!("File read successfully."),
        Err(e) => println!("Error reading file: {}", e),
    }
    // user_armory.addItem(sword);
    // user_armory.addItem(talisman);
    // user_armory.addItem(cuirass);
    // user_armory.show_inventory();
    // storage::save_json(&user_armory);
}
