use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemType {
    Weapon,
    Armor,
    Artifact,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Legendary,
    Arcane,
    Godlike,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Loot {
    pub name: String,
    pub rarity: Rarity,
    pub value: u32,
    pub item_type: ItemType,
}
