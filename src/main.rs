mod models;
mod utils;

use models::fort::Fort;
use models::loot::{ItemType, Loot, Rarity};
use models::raid::Raids;
use utils::fileread::read_file;

use crate::models::mercenaries::Mercenary;

fn main() {
    let mut fort = Fort::new(String::from("Fort Downguard"), 12000);

    let initial_raids = Raids::new();
    initial_raids.raids.iter().for_each(|raid| {
        println!(
            "Location: {} | Level: {} | Reward: {}",
            raid.location, raid.level, raid.reward
        );
    });
    let mercenary1 = Mercenary::new(
        String::from("Alain StrongHeart"),
        models::mercenaries::Occupation::Knight,
        2,
        18,
    );
    fort.addStaff(mercenary1);
    let mercenary2 = Mercenary::new(
        String::from("Greg of BlackStone"),
        models::mercenaries::Occupation::Mage,
        2,
        18,
    );
    fort.addStaff(mercenary2);
    let mut queen = Mercenary::new(
        String::from("Beloved Kristina Barri"),
        models::mercenaries::Occupation::Mage,
        2,
        18,
    );
    queen.make_as_leader();
    fort.addStaff(queen);
    // user_armory.addItem(sword);
    // user_armory.addItem(talisman);
    // user_armory.addItem(cuirass);
    // user_armory.show_inventory();
    // storage::save_json(&user_armory);
}
