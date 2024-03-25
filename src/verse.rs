use itertools::Itertools;
use log::{debug, error};
use reqwest;
use serde::Deserialize;
use serde_json;

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

#[derive(Debug, Clone)]
pub struct Chapter {
    pub r#ref: String,
    pub book: String,
    pub chapter: String,
    pub verses: Vec<Verse>,
    pub text: String,
    pub events: Vec<String>,
    pub entities: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Bible {
    pub translation: String,
    pub chapters: Vec<Chapter>,
}

impl Bible {
    pub fn new(translation: String, verses: Vec<Verse>) -> Self {
        let chapters = Bible::to_chapters(verses);

        Bible {
            translation,
            chapters,
        }
    }

    pub fn get_current_chapter(&self) -> Option<&Chapter> {
        self.chapters.first()
    }

    pub fn get_chapter(&self, chapter_ref: &str) -> Option<&Chapter> {
        match self
            .chapters
            .iter()
            .find(|&chapter| chapter.r#ref == chapter_ref)
        {
            Some(chapter) => Some(chapter),
            None => None,
        }
    }

    pub fn search_by_keyword(&self, word: &str) -> Vec<Verse> {
        let search_word = format!(" {} ", word.to_lowercase());
        let filtered_verses = self
            .chapters
            .iter()
            .flat_map(|chapter| chapter.verses.iter())
            .filter(|verse| verse.text.to_lowercase().contains(&search_word))
            .cloned()
            .collect();

        filtered_verses
    }

    pub fn to_chapters(verses: Vec<Verse>) -> Vec<Chapter> {
        let mut chapters = Vec::new();
        let mut verses_by_chapter = verses.into_iter().group_by(|verse| verse.get_chapter());

        for (chapter_ref, verses) in &verses_by_chapter {
            let verses: Vec<_> = verses.collect();
            let first_verse = verses.first().unwrap();

            let chapter = Chapter {
                r#ref: chapter_ref.to_string(),
                book: first_verse.book.clone(),
                chapter: first_verse.chapter.clone(),
                verses: verses.clone(),
                text: verses
                    .iter()
                    .map(|v| format!("{} {}", v.verse_num, v.text))
                    .collect::<Vec<_>>()
                    .join(" "),
                events: verses
                    .iter()
                    .flat_map(|v| v.events.iter().cloned())
                    .unique()
                    .collect(),
                entities: verses
                    .iter()
                    .flat_map(|v| v.entities.iter().cloned())
                    .unique()
                    .collect(),
            };

            chapters.push(chapter);
        }

        chapters
    }
    pub fn current_chapter(&self) -> Option<&Chapter> {
        self.chapters.first()
    }

    pub fn next_chapter(&mut self) {
        if let Some(current_chapter) = self.current_chapter() {
            if let Some(index) = self
                .chapters
                .iter()
                .position(|chapter| chapter.r#ref == current_chapter.r#ref)
            {
                if index < self.chapters.len() - 1 {
                    self.chapters.rotate_left(1);
                }
            }
        }
    }

    pub fn previous_chapter(&mut self) {
        if let Some(current_chapter) = self.current_chapter() {
            if let Some(index) = self
                .chapters
                .iter()
                .position(|chapter| chapter.r#ref == current_chapter.r#ref)
            {
                if index > 0 {
                    self.chapters.rotate_right(1);
                }
            }
        }
    }

    pub fn go_to_chapter(&mut self, chapter_ref: &str) {
        if let Some(index) = self
            .chapters
            .iter()
            .position(|chapter| chapter.r#ref == chapter_ref)
        {
            self.chapters.rotate_left(index);
        }
    }
}

pub async fn fetch_verses_from_url(url: &str) -> Option<Bible> {
    match reqwest::get(url).await {
        Ok(response) => {
            debug!("Response: {:?}", response);
            if response.status().is_success() {
                match response.text().await {
                    Ok(body) => match serde_json::from_str::<Vec<Verse>>(&body) {
                        Ok(verses) => {
                            let chapters = Bible::to_chapters(verses);
                            debug!("Chapters: {:?}", chapters);
                            Some(Bible {
                                translation: "ESV".to_string(),
                                chapters,
                            })
                        }
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
