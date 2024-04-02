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
                    class: "rounded-lg shadow-lg bg-gray-900 p-6 my-4 z-10 w-1/2",
                    input {
                        class: "rounded-lg px-4 py-2 w-full focus:ring-0 outline-bg-gray-50 appearance-none",
                        "type": "text",
                        placeholder: "Enter search text...",
                        tabindex: 0,
                        autofocus: true,
                        oninput: move |evt| search_text.set(evt.value()),
                        onchange: move |_| {
                            let mut exact: bool = false;
                            match bible() {
                                Some(curr_bible) => {
                                    match parse_bible_reference(&search_text()) {
                                        Some(found_match) => {
                                            if let Some(chapter_ref) = format_bible_reference(Some(found_match)) {
                                                let temp_bible = curr_bible.clone();

                                                let found_smart_verses = find_smart_verses(&temp_bible, &chapter_ref);
                                                if !found_smart_verses.is_empty() {
                                                    smart_verses.set(found_smart_verses);
                                                }

                                                let chapter_ref = parse_chapter_ref(&chapter_ref);

                                                update_bible_state(bible, temp_bible, current_chapter_text, current_chapter, entered_chapter_num, &chapter_ref);
                                                show_jump.set(false);

                                                exact = true;
                                            } else {
                                                error!("Failed to format the reference");
                                            }
                                        }
                                        None => {
                                            debug!("No match found");
                                            // show_jump.set(false);
                                        },
                                    }
                                    if !exact {
                                        debug!("Trying keyword search");
                                        let temp_bible = curr_bible.clone();
                                        smart_verses.set(keyword_search(&temp_bible.clone(), &search_text()));

                                        debug!("{:?}", smart_verses());

                                        if !smart_verses.is_empty() {
                                            let chapter_ref = &smart_verses.first().unwrap().get_chapter();
                                            update_bible_state(bible, temp_bible, current_chapter_text, current_chapter, entered_chapter_num, &chapter_ref);

                                            // show_jump.set(false);
                                        }
                                    } else {
                                        debug!("No keywords found.");
                                    }
                                },
                                None => error!("No Bible match found during search")
                            }
                        },
                    }
                    div {
                        class: "mt-2 overflow-y-auto max-h-64",
                        {
                            smart_verses().iter().map(|verse| rsx! {
                                div {
                                    class: "px-4 py-2 max-h-fit bg-gray-100 rounded-md mb-2 ",
                                    button {
                                        class: "flex-1",
                                        onclick: move |_| {

                                        },
                                        p {
                                            class: "font-medium",
                                            "{verse.get_pretty_verse()}",
                                        }
                                        p {
                                            class: "italic",
                                            "{verse.text}"
                                        }
                                    }
                                }
                            })
                        }
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
