// src/recruiter.rs

pub async fn fetch_mercenary_name() -> Result<String, reqwest::Error> {
    let url = "https://fantasyname.lukewh.com/?gender=f&family=t";

    let response = reqwest::get(url).await?.text().await?;
    Ok(response.trim().to_string())
}
