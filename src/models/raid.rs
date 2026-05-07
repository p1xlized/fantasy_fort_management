use crate::utils::fileread::read_file;

pub enum Status {
    Planned,
    InProgress,
    Successful,
    Defeat,
}
pub enum Type {
    Fort,
    Tower,
    Camp,
    Cave,
}

pub struct Raid {
    pub location: String,
    pub reward: u64,
    pub level: u32,
    pub status: Status,
}
impl Raid {
    pub fn new(location: String, reward: u64, level: u32) -> Self {
        Raid {
            location,
            reward,
            level,
            status: Status::Planned,
        }
    }
}

pub struct Raids {
    pub raids: Vec<Raid>,
}
impl Raids {
    pub fn new() -> Self {
        let names = match read_file("raid_names.txt") {
            Ok(data) => {
                println!("Loaded {} names successfully.", data.len());
                data
            }
            Err(e) => {
                println!("Error reading file: {}. Starting with empty raids.", e);
                Vec::new()
            }
        };

        let mut generated_raids = Vec::new();

        // Loop 10 times or until we run out of names in the file
        for i in 0..10 {
            if let Some(name) = names.get(i) {
                // 1. Determine a Type (cycling through the enum)
                let raid_type = match i % 4 {
                    0 => "Fort",
                    1 => "Tower",
                    2 => "Camp",
                    _ => "Cave",
                };

                // 2. Combine Type and Name
                let location = format!("{} {}", raid_type, name);

                // 3. Generate some stats based on index
                let level = (i as u32 + 1) * 5;
                let reward = (i as u64 + 1) * 100;

                // 4. Create the Raid and push to our list
                generated_raids.push(Raid::new(location, reward, level));
            }
        }

        Raids {
            raids: generated_raids,
        }
    }
}
