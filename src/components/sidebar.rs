use crate::helpers::*;
use crate::models::Bible;
use dioxus::prelude::*;
use log::debug;

#[component]
pub fn Sidebar(
    sidebar_hidden: Signal<bool>,
    bible: Signal<Option<Bible>>,
    unique_books: Signal<Vec<String>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        button {
            class: format!("sidebar-toggle bg-{} hover:bg-{} hover:transition-all sm:hidden md:flex {}", theme.prim_400, theme.prim_300, if sidebar_hidden() { "collapsed" } else { "" }),
            style: format!("position: absolute; right: {}rem;", if sidebar_hidden() {"0"} else {"15"}),
            onclick: move |_| {
                sidebar_hidden.set(!sidebar_hidden());
            },
        }
        div {
            class: format!("bg-{} snap-end max-h-screen overflow-y-auto no-scrollbar {}", theme.prim_100, if sidebar_hidden() { "hidden" } else { "sm:w-full md:w-60"}),

            nav {
                div {
                    class: "flex-1 grid items-start py-2 text-sm font-medium no-scrollbar",
                    if let Some(curr_bible) = bible() {
                        for book in unique_books() {
                            button {
                                disabled: curr_bible.get_current_chapter().map_or(false, |chapter| chapter.book == book),
                                onclick: move |_| {
                                    match bible() {
                                        Some(temp_bible) => {
                                            let chapter_ref = temp_bible.chapters
                                                .iter()
                                                .find(|chapter| &chapter.book == &book)
                                                .map(|chapter| chapter.r#ref.as_str())
                                                .map(|s| s.to_string());

                                            if let Some(chapter_ref) = chapter_ref {
                                                update_bible(bible, temp_bible, current_chapter, current_chapter_text, entered_chapter_num, &chapter_ref);
                                            }
                                        }
                                        None => {debug!("Failed to load Bible book: {}", book);}
                                    }
                                },
                                if curr_bible.get_current_chapter().map_or(false, |chapter| chapter.book == book) {
                                    div {
                                        class: format!("rounded-r-lg flex justify-between align-middle text-base pl-3 text-white bg-{}", theme.prim_700),
                                        strong {
                                            class: "flex items-center",
                                            "{book.to_uppercase()}"
                                        }
                                        input {
                                            class: format!("rounded-l-lg w-14 ml-4 px-2 py-2 cursor-pointer text-{} text-right bg-{} appearance-none outline-bg-{}", theme.prim_800, theme.prim_300, theme.prim_600),
                                            r#type: "number",
                                            maxlength: "3",
                                            value: entered_chapter_num,
                                            onchange: move |evt| {
                                                if let Some(temp_bible) = bible() {
                                                    let chapter_num = evt.value().parse().unwrap_or(0);
                                                    let num_chapters_in_book = temp_bible.num_chapters_in_current_book();
                                                    // TODO: Handle "no current chapter" case more explicitly

                                                    let chapter_num = match chapter_num {
                                                        // TODO: Validate chapter_num input more strictly
                                                        num if num < 1 => {entered_chapter_num.set(1.to_string());  1},
                                                        num if num > num_chapters_in_book => {entered_chapter_num.set(1.to_string()); num_chapters_in_book},
                                                        num => {entered_chapter_num.set(1.to_string()); num},
                                                    };

                                                    let current_chapter_ref = temp_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.r#ref.clone());
                                                    let brev = current_chapter_ref.split('.').next().unwrap_or("");

                                                    let new_chapter_ref = format!("{}.{}", brev, chapter_num);

                                                    update_bible(bible, temp_bible, current_chapter, current_chapter_text, entered_chapter_num, &new_chapter_ref);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    div {
                                        class: format!("py-2 hover:bg-{}", theme.prim_300),
                                        "{book}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
