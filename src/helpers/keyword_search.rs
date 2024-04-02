use crate::models::*;

pub fn keyword_search(bible: &Bible, keywords: &str) -> Vec<Verse> {
    bible
        .chapters
        .iter()
        .flat_map(|ch| ch.verses.iter())
        .filter(|v| v.text.to_lowercase().contains(&keywords.to_lowercase()))
        .cloned()
        .collect()
}
