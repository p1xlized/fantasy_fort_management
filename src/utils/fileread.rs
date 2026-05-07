use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader}; // Provides the 'lines()' method for async

pub async fn read_file(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path).await?; // .await the open
    let reader = BufReader::new(file);

    let mut lines = reader.lines(); // Create an async stream of lines
    let mut names = Vec::new();

    // In async, we usually iterate with a while loop
    while let Some(line) = lines.next_line().await? {
        names.push(line);
    }

    Ok(names)
}
