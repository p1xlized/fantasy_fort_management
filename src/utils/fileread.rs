use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn read_file(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
