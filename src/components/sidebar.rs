use crate::models::Bible;
use dioxus::prelude::*;
use log::debug;

#[component]
pub fn Sidebar(
    bible: Signal<Option<Bible>>,
    unique_books: Signal<Vec<String>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) -> Element {
    rsx! {
        div {
            class: "flex bg-gray-100 w-60 lg:block max-h-screen overflow-y-auto",
            nav {
                div {
                    class: "flex-1 grid items-start py-2 text-sm font-medium no-scrollbar",
                    if let Some(curr_bible) = bible() {
                        for book in unique_books() {
                            button {
                                class: "py-2",
                                disabled: curr_bible.get_current_chapter().map_or(false, |chapter| chapter.book == book),
                                onclick: move |_| {
                                    match bible() {
                                        Some(mut curr_bible) => {

                                            let chapter_ref = curr_bible.chapters
                                                .iter()
                                                .find(|chapter| &chapter.book == &book)
                                                .map(|chapter| chapter.r#ref.as_str())
                                                .map(|s| s.to_string());

                                            if let Some(chapter_ref) = chapter_ref {
                                                // TODO: Validate chapter_ref exists before calling go_to_chapter
                                                curr_bible.go_to_chapter(&chapter_ref);
                                                current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                                current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                                entered_chapter_num.set("1".to_string());
                                                bible.set(Some(curr_bible));
                                            }
                                        }
                                        None => {debug!("Failed to load Bible book: {}", book);}
                                    }
                                },
                                if curr_bible.get_current_chapter().map_or(false, |chapter| chapter.book == book) {
                                    div {
                                        class: "rounded-r-lg flex justify-between align-middle text-base pl-3 text-white bg-gray-700",
                                        strong {
                                            class: "flex items-center",
                                            "{book.to_uppercase()}"
                                        }
                                        input {
                                            class: "rounded-lg w-16 ml-4 px-2 py-2 cursor-pointer text-right bg-gray-500 appearance-none outline-bg-gray-600",
                                            r#type: "number",
                                            maxlength: "3",
                                            value: "{entered_chapter_num}",
                                            onchange: move |evt| {
                                                if let Some(mut curr_bible) = bible() {
                                                    let chapter_num = evt.value().parse().unwrap_or(0);
                                                    let num_chapters_in_book = curr_bible.num_chapters_in_current_book();
                                                    // TODO: Handle "no current chapter" case more explicitly

                                                    let chapter_num = match chapter_num {
                                                        // TODO: Validate chapter_num input more strictly
                                                        num if num < 1 => {entered_chapter_num.set(1.to_string());  1},
                                                        num if num > num_chapters_in_book => {entered_chapter_num.set(1.to_string()); num_chapters_in_book},
                                                        num => {entered_chapter_num.set(1.to_string()); num},
                                                    };

                                                    entered_chapter_num.set(chapter_num.to_string());

                                                    let current_chapter_ref = curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.r#ref.clone());
                                                    let brev = current_chapter_ref.split('.').next().unwrap_or("");

                                                    let new_chapter_ref = format!("{}.{}", brev, chapter_num);

                                                    curr_bible.go_to_chapter(&new_chapter_ref);
                                                    current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                                    current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                                    bible.set(Some(curr_bible));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    "{book}"
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
