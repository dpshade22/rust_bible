use crate::models::*;
use anyhow::Result;
use dioxus::prelude::*;

#[derive(Debug, Clone)]
// theme.rs
pub struct Theme {
    pub prim_50: &'static str,
    pub prim_100: &'static str,
    pub prim_200: &'static str,
    pub prim_300: &'static str,
    pub prim_400: &'static str,
    pub prim_500: &'static str,
    pub prim_600: &'static str,
    pub prim_700: &'static str,
    pub prim_800: &'static str,
    pub prim_900: &'static str,
    pub highlight_600: &'static str,
    // pub page_background: &'static str,
}

impl Theme {
    pub fn light() -> Self {
        Theme {
            prim_50: "stone-50",
            prim_100: "stone-100",
            prim_200: "stone-200",
            prim_300: "stone-300",
            prim_400: "stone-400",
            prim_500: "stone-500",
            prim_600: "stone-600",
            prim_700: "stone-700",
            prim_800: "stone-800",
            prim_900: "stone-900",
            highlight_600: "orange-600",
        }
    }
    pub fn rose() -> Self {
        Theme {
            prim_50: "rose-900",
            prim_100: "rose-800",
            prim_200: "rose-700",
            prim_300: "rose-600",
            prim_400: "rose-500",
            prim_500: "rose-400",
            prim_600: "rose-300",
            prim_700: "rose-200",
            prim_800: "rose-100",
            prim_900: "stone-50",
            highlight_600: "orange-600",
        }
    }
}
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
