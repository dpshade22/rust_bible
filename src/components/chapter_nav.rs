use crate::helpers::*;
use crate::models::*;
use dioxus::prelude::*;
use log::{debug, error};

enum ChapterNavigationDirection {
    Next,
    Previous,
    One,
}

#[component]
pub fn ChapterNav(
    sidebar_hidden: Signal<bool>,
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
    show_jump: Signal<bool>,
) -> Element {
    let theme = use_context::<Theme>();

    fn handle_chapter_navigation(
        direction: ChapterNavigationDirection,
        bible: Signal<Option<Bible>>,
        current_chapter: Signal<String>,
        current_chapter_text: Signal<String>,
        entered_chapter_num: Signal<String>,
    ) -> Option<()> {
        match bible() {
            Some(mut temp_bible) => {
                match direction {
                    ChapterNavigationDirection::Next => {
                        temp_bible.next_chapter();
                        let chapter_ref = temp_bible.get_current_chapter()?.r#ref.clone();
                        update_bible(
                            bible,
                            temp_bible,
                            current_chapter,
                            current_chapter_text,
                            entered_chapter_num,
                            &chapter_ref,
                        );
                    }
                    ChapterNavigationDirection::Previous => {
                        temp_bible.previous_chapter();
                        let chapter_ref = temp_bible.get_current_chapter()?.r#ref.clone();
                        update_bible(
                            bible,
                            temp_bible,
                            current_chapter,
                            current_chapter_text,
                            entered_chapter_num,
                            &chapter_ref,
                        );
                    }
                    ChapterNavigationDirection::One => match bible() {
                        Some(temp_bible) => {
                            let chapter_ref: String;
                            if let Some(current_osis) =
                                get_osis_by_book(&temp_bible.get_current_chapter()?.book)
                            {
                                chapter_ref = format!("{}.{}", current_osis, "1");
                            } else {
                                chapter_ref = "Gen.1".to_string();
                            }
                            update_bible(
                                bible,
                                temp_bible,
                                current_chapter,
                                current_chapter_text,
                                entered_chapter_num,
                                &chapter_ref,
                            );
                        }
                        None => error!("Didn't get ChapterNavigationDirection enum"),
                    },
                }
                Some(())
            }
            None => {
                debug!("Bible match failed");
                Some(())
            }
        }
    }

    rsx! {
        div { class: format!(
                "flex py-6 items-center w-full {}",
                if sidebar_hidden() { "justify-center" } else { "" },
            ),
            button {
                class: format!("text-{} hover:text-{} order-1", theme.prim_500, theme.prim_700),
                onclick: move |_| {
                    handle_chapter_navigation(
                        ChapterNavigationDirection::Previous,
                        bible,
                        current_chapter,
                        current_chapter_text,
                        entered_chapter_num,
                    );
                },
                svg {
                    class: "h-6 w-6",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M15 19l-7-7 7-7"
                    }
                }
            }
            button {
                class: format!(
                    "flex text-justify text-4xl font-extrabold tracking-tight sm:text-xl lg:text-5xl mx-4 w-50% order-2 py-2 {}",
                    if sidebar_hidden() { "justify-center transition-all" } else { "pl-4" },
                ),
                onclick: move |_| {
                    handle_chapter_navigation(
                        ChapterNavigationDirection::One,
                        bible,
                        current_chapter,
                        current_chapter_text,
                        entered_chapter_num,
                    );
                },
                h1 { class: format!("text-{}", theme.prim_800), "{current_chapter}" }
            }
            button {
                class: format!("text-{} hover:text-{} order-3", theme.prim_500, theme.prim_700),
                onclick: move |_| {
                    handle_chapter_navigation(
                        ChapterNavigationDirection::Next,
                        bible,
                        current_chapter,
                        current_chapter_text,
                        entered_chapter_num,
                    );
                },
                svg {
                    class: "h-6 w-6",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M9 5l7 7-7 7"
                    }
                }
            }
        }
        button {
            class: "sticky top-4 float-right p-2 rounded-full focus:outline-none",
            onclick: move |_| show_jump.set(!show_jump()),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
            }
        }
    }
}
