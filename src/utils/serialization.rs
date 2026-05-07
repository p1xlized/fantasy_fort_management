use crate::models::fort::Fort;
use std::fs;

pub fn save_json(armory: &Fort) {
    // We use a reference (&) so we don't "kill" (drop) the armory after saving it
    let save_file = serde_json::to_string_pretty(armory).expect("Failed to serialize");
    fs::write("armory.json", &save_file).expect("Unable to write file");
    println!("Output: {}", &save_file);
}
