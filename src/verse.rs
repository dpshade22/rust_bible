use log::{debug, error};
use reqwest;
use serde::Deserialize;
use serde_json;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Verses {
    pub verses: Vec<Verse>,
}

impl Verses {
    pub fn search_by_keyword(&self, word: &str) -> Verses {
        let search_word = format!(" {} ", word.to_lowercase());
        let filtered_verses = self
            .verses
            .iter()
            .filter(|verse| verse.text.to_lowercase().contains(&search_word))
            .cloned()
            .collect();
        Verses {
            verses: filtered_verses,
        }
    }

    pub fn aggregate_by_chapter(&self) -> BTreeMap<String, Verses> {
        let mut chapter_map: BTreeMap<String, Verses> = BTreeMap::new();

        for verse in &self.verses {
            let chapter = verse.get_chapter();
            chapter_map.entry(chapter.clone()).or_insert_with(|| {
                let filtered_verses = self
                    .verses
                    .iter()
                    .filter(|new_verse| new_verse.get_chapter() == chapter)
                    .cloned()
                    .collect();
                Verses {
                    verses: filtered_verses,
                }
            });
        }

        chapter_map
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Verse {
    pub r#ref: String,
    pub book: String,
    pub chapter: String,
    #[serde(rename = "verseNum")]
    pub verse_num: String,
    pub text: String,
    pub events: Vec<String>,
    pub entities: Vec<String>,
}

impl Verse {
    pub fn get_chapter(&self) -> String {
        let parts: Vec<&str> = self.r#ref.split('.').collect();
        if parts.len() >= 2 {
            format!("{}.{}", parts[0], parts[1])
        } else {
            String::new()
        }
    }
}

pub async fn fetch_verses_from_url(url: &str) -> Option<Verses> {
    match reqwest::get(url).await {
        Ok(response) => {
            debug!("Response: {:?}", response);
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => match serde_json::from_str::<Vec<Verse>>(&body) {
                        Ok(verses) => Some(Verses { verses }),
                        Err(err) => {
                            error!("Error deserializing JSON: {}", err);
                            None
                        }
                    },
                    Err(err) => {
                        error!("Error retrieving response body: {}", err);
                        None
                    }
                }
            } else {
                error!("Request failed with status: {}", response.status());
                None
            }
        }
        Err(err) => {
            error!("Request failed with error: {}", err);
            None
        }
    }
}
