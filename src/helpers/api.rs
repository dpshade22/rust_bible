use crate::models::{Bible, Verse};
use anyhow::{Context, Result};

use serde_json;

pub async fn fetch_verses_from_url(url: &str) -> Result<Bible> {
    let response = reqwest::get(url).await.context("Failed to send request")?;

    anyhow::ensure!(
        response.status().is_success(),
        "Request failed with status: {}",
        response.status()
    );

    let body = response.text().await?;
    let verses =
        serde_json::from_str::<Vec<Verse>>(&body).context("Failed to parse JSON response")?;

    let chapters = Bible::to_chapters(verses).context("Failed to convert verses to chapters")?;

    Ok(Bible {
        translation: "ESV".to_string(),
        chapters,
    })
}
