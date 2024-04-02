use crate::helpers::*;
use crate::models::*;
use dioxus::prelude::*;
use log::debug;

#[component]
pub fn ChapterNav(
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
    show_jump: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "flex py-6 items-center w-full",
            button {
                class: "text-gray-500 hover:text-gray-700 order-1",
                onclick: move |_| {
                    match bible() {
                        Some(mut temp_bible) => {
                            temp_bible.previous_chapter();
                            let chapter_ref = temp_bible.get_current_chapter().unwrap().r#ref.clone();
                            update_bible_state(bible, temp_bible, current_chapter, current_chapter_text, entered_chapter_num, &chapter_ref)
                        },
                        None => debug!("Bible match failed")
                    }
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
                        d: "M15 19l-7-7 7-7",
                    }
                }
            }
            h1 {
                class: "text-4xl font-extrabold tracking-tight lg:text-5xl mx-4 w-50% order-2 py-2",
                "{current_chapter}"
            }
            button {
                class: "text-gray-500 hover:text-gray-700 order-3",
                onclick: move |_| {
                    match bible() {
                        Some(mut temp_bible) => {
                            temp_bible.next_chapter();
                            let chapter_ref = temp_bible.get_current_chapter().unwrap().r#ref.clone();
                            update_bible_state(bible, temp_bible, current_chapter, current_chapter_text, entered_chapter_num, &chapter_ref)
                        },
                        None => debug!("Bible match failed")
                    }
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
                        d: "M9 5l7 7-7 7",
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
                path {
                    d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                }
            }
        }
    }
}
