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
    fn start_raid(&mut self) {
        self.status = Status::InProgress
    }
}

pub struct Raids {
    pub raids: Vec<Raid>,
}
impl Raids {
    // We change the name to 'load' and make it 'async'
    pub async fn load() -> Self {
        // 1. ADD AWAIT HERE
        let names = match read_file("raid_names.txt").await {
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

        for i in 0..10 {
            if let Some(name) = names.get(i) {
                let raid_type = match i % 4 {
                    0 => "Fort",
                    1 => "Tower",
                    2 => "Camp",
                    _ => "Cave",
                };

                let location = format!("{} {}", raid_type, name);
                let level = (i as u32 + 1) * 5;
                let reward = (i as u64 + 1) * 100;

                generated_raids.push(Raid::new(location, reward, level));
            }
        }

        Raids {
            raids: generated_raids,
        }
    }
}
