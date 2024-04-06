use crate::models::*;
use anyhow::Result;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct UiProps {
    pub sidebar_hidden: Signal<bool>,
}

pub fn update_bible(
    mut bible: Signal<Option<Bible>>,
    mut temp_bible: Bible,
    mut current_chapter: Signal<String>,
    mut current_chapter_text: Signal<String>,
    mut entered_chapter_num: Signal<String>,
    chapter_ref: &str,
) -> Option<()> {
    temp_bible.go_to_chapter(&chapter_ref);
    current_chapter_text.set(
        temp_bible
            .get_current_chapter()
            .map_or("".to_string(), |chapter| chapter.text.clone()),
    );
    current_chapter.set(
        temp_bible
            .get_current_chapter()
            .map_or("".to_string(), |chapter| chapter.get_pretty_chapter()),
    );
    entered_chapter_num.set(temp_bible.get_current_chapter()?.chapter.to_string());
    bible.set(Some(temp_bible.clone()));
    Some(())
}
