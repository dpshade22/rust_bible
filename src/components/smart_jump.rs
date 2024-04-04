// smart_jump.rs
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
    unique_books: Signal<Vec<String>>,
    search_text: Signal<String>,
    selected_translation: Signal<String>,
) -> Element {
    let show_dropdown = use_signal(|| false);

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
                        onchange: move |_| handle_input(
                            show_jump,
                            smart_verses,
                            bible,
                            current_chapter,
                            current_chapter_text,
                            entered_chapter_num,
                            unique_books,
                            search_text,
                            selected_translation
                        ),
                    }

                    SearchResults { smart_verses, bible, show_jump, current_chapter, current_chapter_text, entered_chapter_num }

                    TranslationDropdown {selected_translation, show_dropdown }
                }
            }
        }
    }
}

#[component]
fn SearchResults(
    smart_verses: Signal<Vec<Verse>>,
    bible: Signal<Option<Bible>>,
    show_jump: Signal<bool>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) -> Element {
    rsx! {
        div {
            class: "rounded-lg mt-2 overflow-y-auto max-h-64",

            for verse in smart_verses() {
                div {
                    class: "flex flex-col justify-center w-full bg-white px-4 py-2 max-h-fit hover:bg-gray-50",

                    button {
                        class: "rounded-lg",
                        onclick: move |_| {
                            if let Some(temp_bible) = bible() {
                                update_bible(
                                    bible,
                                    temp_bible,
                                    current_chapter,
                                    current_chapter_text,
                                    entered_chapter_num,
                                    None,
                                    &verse.get_chapter()
                                );
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
    }
}

#[component]
fn TranslationDropdown(
    selected_translation: Signal<String>,
    show_dropdown: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "relative",

            button {
                class: "px-4 py-1 my-2 bg-gray-700 rounded-b-lg text-white",
                onclick: move |_| show_dropdown.set(!show_dropdown()),
                strong {
                    "{selected_translation()}"
                }
            }

            if show_dropdown() {
                div {
                    class: "border absolute bg-gray-700 text-white shadow-md py-2 rounded-md mt-1",

                    for translation_key in TRANSLATIONS.keys() {
                        if translation_key.to_string() != selected_translation() {
                            button {
                                class: "rounded-md px-4 py-2 hover:bg-gray-500",
                                onclick: move |_| {
                                    selected_translation.set(translation_key.to_string());
                                    show_dropdown.set(false);
                                },
                                strong {
                                    "{translation_key}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_input(
    show_jump: Signal<bool>,
    smart_verses: Signal<Vec<Verse>>,
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
    unique_books: Signal<Vec<String>>,
    search_text: Signal<String>,
    selected_translation: Signal<String>,
) {
    if let Some(mut curr_bible) = bible() {
        handle_translation_change(
            &mut curr_bible,
            selected_translation,
            unique_books,
            bible,
            current_chapter,
            current_chapter_text,
            entered_chapter_num,
        );

        match parse_bible_reference(&search_text()) {
            Some((Some(book), Some(chapter), verse_start, verse_end, _)) => {
                handle_book_chapter_verse(
                    &book,
                    chapter,
                    verse_start,
                    verse_end,
                    curr_bible,
                    smart_verses,
                    bible,
                    current_chapter,
                    current_chapter_text,
                    entered_chapter_num,
                    show_jump,
                )
            }
            Some((Some(book), _, _, _, remaining_query)) => handle_book_keyword(
                &book,
                &remaining_query,
                curr_bible,
                smart_verses,
                bible,
                current_chapter,
                current_chapter_text,
                entered_chapter_num,
            ),
            Some((_, _, _, _, remaining_query)) => handle_keyword(
                &remaining_query,
                curr_bible,
                smart_verses,
                bible,
                current_chapter,
                current_chapter_text,
                entered_chapter_num,
            ),
            None => {
                debug!("No match found");
            }
        }
    } else {
        error!("No Bible match found during search");
    }
}

fn handle_translation_change(
    curr_bible: &mut Bible,
    selected_translation: Signal<String>,
    mut unique_books: Signal<Vec<String>>,
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) {
    if curr_bible.translation != selected_translation() {
        spawn(async move {
            if let Some(bible_url) = TRANSLATIONS.get(&selected_translation()) {
                if let Ok(fetched_bible) = fetch_verses_from_url(&bible_url).await {
                    unique_books.set(fetched_bible.get_unique_books());

                    update_bible(
                        bible,
                        fetched_bible,
                        current_chapter,
                        current_chapter_text,
                        entered_chapter_num,
                        None,
                        "Gen.1",
                    );
                }
            }
        });
        curr_bible.translation = selected_translation();
    }
}

fn handle_book_chapter_verse(
    book: &str,
    chapter: u32,
    verse_start: Option<u32>,
    verse_end: Option<u32>,
    curr_bible: Bible,
    mut smart_verses: Signal<Vec<Verse>>,
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
    mut show_jump: Signal<bool>,
) {
    if let Some(chapter_ref) = format_bible_reference(Some((book, chapter, verse_start, verse_end)))
    {
        let found_smart_verses = find_smart_verses(&curr_bible, &chapter_ref);
        if !found_smart_verses.is_empty() {
            smart_verses.set(found_smart_verses);
        }

        let chapter_ref = parse_chapter_ref(&chapter_ref);
        update_bible(
            bible,
            curr_bible,
            current_chapter,
            current_chapter_text,
            entered_chapter_num,
            None,
            &chapter_ref,
        );
        show_jump.set(false);
    } else {
        error!("Failed to format the reference");
    }
}

fn handle_book_keyword(
    book: &str,
    remaining_query: &str,
    curr_bible: Bible,
    mut smart_verses: Signal<Vec<Verse>>,
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) {
    debug!("Trying keyword search");
    if !remaining_query.trim().is_empty() {
        debug!("Remaining query: {}", remaining_query.trim());

        smart_verses.set(keyword_search(&curr_bible, &remaining_query, Some(&book)));

        debug!(
            "Found book and NO chapter... {}... Filtering by keyword: {:?}",
            book, remaining_query
        );

        if !smart_verses.is_empty() {
            let chapter_ref = &smart_verses.first().unwrap().get_chapter();
            update_bible(
                bible,
                curr_bible,
                current_chapter,
                current_chapter_text,
                entered_chapter_num,
                None,
                &chapter_ref,
            );
        }
    }
}

fn handle_keyword(
    remaining_query: &str,
    curr_bible: Bible,
    mut smart_verses: Signal<Vec<Verse>>,
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) {
    debug!("Trying keyword search");
    if !remaining_query.trim().is_empty() {
        smart_verses.set(keyword_search(&curr_bible, &remaining_query, None));

        if !smart_verses.is_empty() {
            let chapter_ref = &smart_verses.first().unwrap().get_chapter();
            update_bible(
                bible,
                curr_bible,
                current_chapter,
                current_chapter_text,
                entered_chapter_num,
                None,
                &chapter_ref,
            );
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
