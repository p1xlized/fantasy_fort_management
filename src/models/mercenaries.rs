use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Occupation {
    Mage,
    Knight,
    Cook,
    Archer,
    Blacksmith,
    Queen,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mercenary {
    pub name: String,
    pub occupation: Occupation,
    pub level: u32,
    pub daily_cost: u64,
    pub is_ruler: bool,
}
impl Mercenary {
    pub fn new(name: String, occupation: Occupation, level: u32, daily_cost: u64) -> Self {
        Mercenary {
            name,
            occupation,
            level,
            daily_cost,
            is_ruler: false,
        }
    }
    pub fn make_as_leader(&mut self) {
        self.is_ruler = true;
    }
}
