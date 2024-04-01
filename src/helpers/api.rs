use crate::models::{Bible, Verse};
use anyhow::{Context, Result};
use serde_json;

pub async fn fetch_verses_from_url(url: &str) -> Result<Bible> {
    let response = reqwest::get(url).await.context("Failed to send request")?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .context("Failed to read response body")?;

        let verses = serde_json::from_str::<Vec<Verse>>(&body).context("Failed to parse JSON")?;

        let chapters = Bible::to_chapters(verses);

        Ok(Bible {
            translation: "ESV".to_string(),
            chapters,
            keyword_search_chapters: None,
        })
    } else {
        Err(anyhow::anyhow!(
            "Request failed with status: {}",
            response.status()
        ))
    }
}
