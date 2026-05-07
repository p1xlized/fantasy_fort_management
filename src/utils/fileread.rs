use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_file(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let names: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(names)
}
