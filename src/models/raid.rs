pub enum Status {
    Planned,
    InProgress,
    Successful,
    Defeat,
}

pub struct Raid {
    location: String,
    reward: u64,
    level: u32,
    status: Status,
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
    raids: Vec<Raid>,
}
impl Raids {
    fn new() -> Self {
        Raids { raids: Vec::new() }
    }
}
