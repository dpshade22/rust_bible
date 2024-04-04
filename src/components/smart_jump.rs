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
    search_text: Signal<String>,
) -> Element {
    let mut show_dropdown = use_signal(|| false);
    // let mut input_error = use_signal(|| false);

    // Declared here due to being an extracted UI helper function
    fn handle_input(
        mut show_jump: Signal<bool>,
        mut smart_verses: Signal<Vec<Verse>>,
        bible: Signal<Option<Bible>>,
        current_chapter: Signal<String>,
        current_chapter_text: Signal<String>,
        entered_chapter_num: Signal<String>,
        search_text: Signal<String>,
    ) {
        match bible() {
            Some(curr_bible) => {
                match parse_bible_reference(&search_text()) {
                    Some((Some(book), Some(chapter), verse_start, verse_end, _)) => {
                        if let Some(chapter_ref) =
                            format_bible_reference(Some((book, chapter, verse_start, verse_end)))
                        {
                            let temp_bible = curr_bible.clone();

                            let found_smart_verses = find_smart_verses(&temp_bible, &chapter_ref);
                            if !found_smart_verses.is_empty() {
                                smart_verses.set(found_smart_verses);
                            }

                            let chapter_ref = parse_chapter_ref(&chapter_ref);

                            update_bible_state(
                                bible,
                                temp_bible,
                                current_chapter,
                                current_chapter_text,
                                entered_chapter_num,
                                &chapter_ref,
                            );
                            show_jump.set(false);
                        } else {
                            error!("Failed to format the reference");
                        }
                    }
                    Some((Some(book), _, _, _, remaining_query)) => {
                        debug!("Trying keyword search");
                        if !remaining_query.trim().is_empty() {
                            debug!("Remaining query: {}", remaining_query.trim());

                            let temp_bible = curr_bible.clone();

                            smart_verses.set(keyword_search(
                                &temp_bible.clone(),
                                &remaining_query,
                                Some(&book),
                            ));

                            debug!(
                                "Found book and NO chapter... {}... Filtering by keyword: {:?}",
                                book, remaining_query
                            );

                            if !smart_verses.is_empty() {
                                let chapter_ref = &smart_verses.first().unwrap().get_chapter();
                                update_bible_state(
                                    bible,
                                    temp_bible,
                                    current_chapter,
                                    current_chapter_text,
                                    entered_chapter_num,
                                    &chapter_ref,
                                );

                                // show_jump.set(false);
                            }
                        }
                    }
                    Some((_, _, _, _, remaining_query)) => {
                        debug!("Trying keyword search");
                        if !remaining_query.trim().is_empty() {
                            let temp_bible = curr_bible.clone();
                            smart_verses.set(keyword_search(
                                &temp_bible.clone(),
                                &remaining_query,
                                None,
                            ));

                            if !smart_verses.is_empty() {
                                let chapter_ref = &smart_verses.first().unwrap().get_chapter();
                                update_bible_state(
                                    bible,
                                    temp_bible,
                                    current_chapter,
                                    current_chapter_text,
                                    entered_chapter_num,
                                    &chapter_ref,
                                );

                                // show_jump.set(false);
                            }
                        }
                    }
                    None => {
                        debug!("No match found");
                        // show_jump.set(false);
                    }
                }
            }
            None => error!("No Bible match found during search"),
        }
    }

    rsx! {
        if show_jump() {
            div {
                class: "fixed inset-0 flex items-center justify-center z-50",
                div {
                    class: "fixed inset-0 bg-gray-900 opacity-50",
                    onclick: move |_| show_jump.set(false),
                }
                div {
                    class: "rounded-lg bg-white shadow-lg p-4 my-4 z-10 w-1/2",
                    input {
                        class: "px-4 py-2 w-full focus:outline-none border-b appearance-none",
                        "type": "text",
                        placeholder: "Enter search text...",
                        tabindex: 0,
                        autofocus: true,
                        value: search_text,
                        oninput: move |evt| search_text.set(evt.value()),
                        onchange: move |_| handle_input(show_jump, smart_verses, bible, current_chapter, current_chapter_text, entered_chapter_num, search_text),
                    }
                    div {
                        class: "rounded-lg mt-2 overflow-y-auto max-h-64",
                        for verse in smart_verses() {
                            div {
                                class: "flex flex-col justify-center w-full bg-white px-4 py-2 max-h-fit hover:bg-gray-50",
                                button {
                                    class: "rounded-lg",
                                    onclick: move |_| {
                                        if let Some(temp_bible) = bible() {
                                        update_bible_state(bible, temp_bible, current_chapter, current_chapter_text, entered_chapter_num, &verse.get_chapter())
                                        }
                                        show_jump.set(false);
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
                            hr {}
                        }
                    }
                    div {
                        class: "relative",
                        button {
                            class: "px-4 py-1 my-2 bg-gray-700 rounded-b-lg text-white",
                            onclick: move |_| show_dropdown.set(!show_dropdown()),
                            strong {
                                "{bible().unwrap().translation}"
                            }
                        }
                        if show_dropdown() {
                            div {
                                class: "border absolute bg-gray-700 text-white shadow-md py-2 rounded-md mt-2",
                                button {
                                    class: "rounded-md px-4 py-2 hover:bg-gray-100",
                                    strong {
                                        "CSB"
                                    }
                                }
                                // Add more options as needed
                            }
                        }
                    }
                }
            }
        }
    }
}

// Declared here due to being non-UI helper function
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
    match parts.len() {
        len if len > 2 => {
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
        }
        len if len == 2 => {
            debug!("Only book and chapter found");
            vec![]
        }
        _ => {
            error!("Parts length invalid: {:?}", parts);
            vec![]
        }
    }
}
