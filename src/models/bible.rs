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
    pub keyword_search_chapters: Option<Vec<Chapter>>,
}

impl Bible {
    // pub fn new(translation: String, verses: Vec<Verse>) -> Self {
    //     let chapters = Bible::to_chapters(verses);

    //     Bible {
    //         translation,
    //         chapters,
    //         keyword_search_chapters: None,
    //     }
    // }

    // pub fn get_chapter_by_ref(&self, chapter_ref: &str) -> Option<&Chapter> {
    //     match self
    //         .chapters
    //         .iter()
    //         .find(|&chapter| chapter.r#ref == chapter_ref)
    //     {
    //         Some(chapter) => Some(chapter),
    //         None => None,
    //     }
    // }

    // pub fn search_by_keyword(&self, word: &str) -> Vec<Verse> {
    //     let search_word = format!(" {} ", word.to_lowercase());
    //     self.chapters
    //         .iter()
    //         .flat_map(|chapter| chapter.verses.iter())
    //         .filter(|verse| verse.text.to_lowercase().contains(&search_word))
    //         .cloned()
    //         .collect()
    // }

    // pub fn chapters_by_keyword(&mut self, word: &str) {
    //     let search_word = format!(" {} ", word.to_lowercase());

    //     let verses = self
    //         .chapters
    //         .iter()
    //         .flat_map(|chapter| chapter.verses.iter())
    //         .filter(|verse| verse.text.to_lowercase().contains(&search_word))
    //         .cloned()
    //         .collect();

    //     self.keyword_search_chapters = Some(Bible::to_chapters(verses))
    // }

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
