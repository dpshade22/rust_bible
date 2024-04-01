use crate::helpers::*;
use crate::models::*;
use dioxus::prelude::*;
use log::{debug, error};

#[component]
pub fn SmartJump(
    bible: Signal<Option<Bible>>,
    show_jump: Signal<bool>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
    smart_verses: Signal<Vec<Verse>>,
) -> Element {
    let mut search_text = use_signal(|| "".to_string());

    rsx! {
        if show_jump() {
            div {
                class: "fixed inset-0 flex items-center justify-center z-50",
                div {
                    class: "fixed inset-0 bg-gray-900 opacity-50",
                    onclick: move |_| show_jump.set(false),
                }
                div {
                    class: "rounded-lg shadow-lg my-4 z-10 w-1/2",
                    input {
                        class: "rounded-lg px-4 py-2 w-full focus:border-sky-500 focus:ring-sky-500 focus:ring-1 outline-bg-gray-50 appearance-none",
                        "type": "text",
                        placeholder: "Enter search text...",
                        autofocus: true,
                        // is_attribute: "autofocus",
                        oninput: move |evt| search_text.set(evt.value()),
                        onchange: move |_| {
                            match bible() {
                                Some(mut curr_bible) => {
                                    match parse_bible_reference(&search_text()) {
                                        Some(found_match) => {
                                            if let Some(chapter_ref) = format_bible_reference(Some(found_match)) {
                                                smart_verses.set(find_smart_verses(&curr_bible, &chapter_ref));
                                                debug!("OG SMART V: {:?}", smart_verses());

                                                let chapter_ref = parse_chapter_ref(&chapter_ref);

                                                curr_bible.go_to_chapter(&chapter_ref);
                                                current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                                current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                                entered_chapter_num.set(curr_bible.get_current_chapter().unwrap().chapter.to_string());

                                                show_jump.set(false);
                                                bible.set(Some(curr_bible));

                                            } else {
                                                error!("Failed to format the reference");
                                            }
                                        }
                                        None => {
                                            error!("No match found");
                                            show_jump.set(false);
                                        },
                                    }
                                },
                                None => error!("No Bible match found during search")
                            }
                        },
                    }
                }
            }
        }
    }
}

fn parse_chapter_ref(chapter_ref: &str) -> String {
    let parts: Vec<&str> = chapter_ref.split('.').collect();

    if parts.len() >= 2 {
        format!("{}.{}", parts[0], parts[1])
    } else {
        chapter_ref.to_string()
    }
}
fn find_smart_verses(bible: &Bible, chapter_ref: &str) -> Vec<Verse> {
    let parts: Vec<&str> = chapter_ref.split('.').collect();
    if parts.len() > 2 {
        let book = parts[0];
        let chapter = parts[1];

        let verse_range: Vec<&str> = parts
            .get(2)
            .map(|s| s.split('-').collect())
            .unwrap_or_else(|| vec![]);

        let start_verse = verse_range.get(0).and_then(|v| v.parse().ok()).unwrap_or(1);
        let end_verse = verse_range
            .get(1)
            .and_then(|v| v.parse().ok())
            .unwrap_or(start_verse);

        bible
            .chapters
            .iter()
            .flat_map(|ch| ch.verses.iter())
            .filter(|v| {
                v.r#ref.contains(book)
                    && v.chapter == chapter
                    && v.verse_num
                        .parse::<usize>()
                        .map(|num| start_verse <= num && num <= end_verse)
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    } else {
        error!("Parts length invalid: {:?}", parts);
        vec![]
    }
}
