use serde::{Deserialize, Serialize};

use crate::models::loot::Loot;
use crate::models::mercenaries::Mercenary;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fort {
    pub inventory: Vec<Loot>,
    pub name: String,
    pub budget: u64,
    pub staff: Vec<Mercenary>,
}

impl Fort {
    pub fn new(name: String, budget: u64) -> Self {
        Fort {
            inventory: Vec::new(),
            name: name, // use String::from
            budget: budget,
            staff: Vec::new(),
        }
    }
    pub fn addLoot(&mut self, item: Loot) {
        self.inventory.push(item);
    }
    pub fn addStaff(&mut self, item: Mercenary) {
        self.staff.push(item);
    }
    pub fn show_inventory(&self) {
        println!("--- Armory Report for: {}) ---", self.name);

        for item in &self.inventory {
            // Using :? because we have #[derive(Debug)] on our Loot struct
            println!("{:?}", item);
        }
    }
}
