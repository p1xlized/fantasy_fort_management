mod api;
mod models;
mod utils;

use models::fort::Fort;
use models::raid::Raids;

use crate::models::mercenaries::Mercenary;

#[tokio::main]
async fn main() {
    let mut fort = Fort::new(String::from("Fort Downguard"), 12000);

    let my_raids = Raids::load().await;
    my_raids.raids.iter().for_each(|raid| {
        println!(
            "Location: {} | Level: {} | Reward: {}",
            raid.location, raid.level, raid.reward
        );
    });
    let name1 = api::requests::fetch_mercenary_name()
        .await
        .expect("Failed to get name");
    let mercenary1 = Mercenary::new(name1, models::mercenaries::Occupation::Knight, 2, 18);
    fort.add_staff(mercenary1);
    let name2 = api::requests::fetch_mercenary_name()
        .await
        .expect("Failed to get name");
    let mercenary2 = Mercenary::new(name2, models::mercenaries::Occupation::Mage, 2, 18);
    fort.add_staff(mercenary2);
    let mut queen = Mercenary::new(
        String::from("Beloved Kristina Barri"),
        models::mercenaries::Occupation::Mage,
        2,
        18,
    );
    queen.make_as_leader();
    fort.add_staff(queen);
    fort.staff.iter().for_each(|person| {
        println!(
            "name: {} | level: {} | cost: {} |",
            person.name, person.level, person.daily_cost
        );
    });
    // user_armory.addItem(sword);
    // user_armory.addItem(talisman);
    // user_armory.addItem(cuirass);
    // user_armory.show_inventory();
    // storage::save_json(&user_armory);
}
