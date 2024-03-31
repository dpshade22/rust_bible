use crate::bible::*;
use dioxus::prelude::*;
use log::debug;

#[component]
pub fn ChapterNav(
    bible: Signal<Option<Bible>>,
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>,
) -> Element {
    rsx! {

    div {
        class: "flex py-6 items-center w-full",

        button {
            class: "text-gray-500 hover:text-gray-700 order-1",
            onclick: move |_| {
                match bible() {
                    Some(mut curr_bible) => {
                        curr_bible.previous_chapter();
                        current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                        current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                        entered_chapter_num.set(curr_bible.get_current_chapter().unwrap().chapter.to_string());
                        bible.set(Some(curr_bible));
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
                    Some(mut curr_bible) => {
                        curr_bible.next_chapter();
                        current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                        current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                        entered_chapter_num.set(curr_bible.get_current_chapter().unwrap().chapter.to_string());
                        bible.set(Some(curr_bible));
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
        }
}
