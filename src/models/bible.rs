use itertools::Itertools;
use log::debug;
use serde::Deserialize;

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
    pub fn get_pretty_verse(&self) -> String {
        format!("{} {}:{}", self.book, self.chapter, self.verse_num)
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

impl Chapter {
    pub fn get_pretty_chapter(&self) -> String {
        format!("{} {}", self.book, self.chapter)
    }
}

#[derive(Debug, Clone)]
pub struct Bible {
    pub translation: String,
    pub chapters: Vec<Chapter>,
}

impl Bible {
    pub fn to_chapters(verses: Vec<Verse>) -> Vec<Chapter> {
        let mut chapters = Vec::new();
        let verses_by_chapter = verses.into_iter().group_by(|verse| verse.get_chapter());

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
                    .map(|v| format!("{}", v.text))
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

    pub fn get_current_chapter(&self) -> Option<&Chapter> {
        self.chapters.first()
    }

    pub fn next_chapter(&mut self) {
        if let Some(current_chapter) = self.get_current_chapter() {
            if let Some(index) = self
                .chapters
                .iter()
                .position(|chapter| chapter.r#ref == current_chapter.r#ref)
            {
                debug!("Next chapter index: {index}");
                self.chapters.rotate_left(1);
            }
        }
    }

    pub fn previous_chapter(&mut self) {
        if let Some(current_chapter) = self.get_current_chapter() {
            if let Some(index) = self
                .chapters
                .iter()
                .position(|chapter| chapter.r#ref == current_chapter.r#ref)
            {
                debug!("Previous chapter index: {index}");
                self.chapters.rotate_right(1);
            }
        }
    }

    pub fn get_unique_books(&self) -> Vec<String> {
        self.chapters
            .iter()
            .map(|chapter| chapter.book.clone())
            .unique()
            .collect()
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

    pub fn num_chapters_in_current_book(&self) -> usize {
        // TODO: Handle "no current chapter" case more explicitly
        if let Some(current_chapter) = self.get_current_chapter() {
            self.chapters
                .iter()
                .filter(|chapter| chapter.book == current_chapter.book)
                .count()
        } else {
            0
        }
    }
}
