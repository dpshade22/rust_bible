use crate::models::*;

pub fn keyword_search(bible: &Bible, keywords: &str, book_filter: Option<&str>) -> Vec<Verse> {
    let keywords_with_spaces = format!(" {} ", keywords.to_lowercase());

    bible
        .chapters
        .iter()
        .flat_map(|ch| ch.verses.iter())
        .filter(|v| {
            v.text.to_lowercase().contains(&keywords_with_spaces)
                && book_filter.map_or(true, |filter| v.r#ref.contains(filter))
        })
        .cloned()
        .collect()
}
