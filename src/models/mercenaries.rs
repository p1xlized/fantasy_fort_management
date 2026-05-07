use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Occupation {
    Mage,
    Knight,
    Cook,
    Archer,
    Blacksmith,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mercenary {
    pub name: String,
    pub occupation: Occupation,
    pub level: u32,
    pub daily_cost: u64,
}
impl Mercenary {
    fn new(name: String, occupation: Occupation, level: u32, daily_cost: u64) -> Self {
        Mercenary {
            name,
            occupation,
            level,
            daily_cost,
        }
    }
}
