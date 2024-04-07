#![allow(non_snake_case)]

mod components;
mod helpers;
mod models;

use crate::components::*;
use crate::helpers::*;
use crate::models::*;
use dioxus::prelude::*;
use log::debug;

fn main() {
    // Urls are relative to your Cargo.toml file
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
    }

    launch(App);
}

#[component]
fn App() -> Element {
    let bible: Signal<Option<Bible>> = use_signal(|| None);
    let current_chapter = use_signal(|| "".to_string());
    let current_chapter_text = use_signal(|| "".to_string());
    let entered_chapter_num = use_signal(|| "1".to_string());
    let smart_verses: Signal<Vec<Verse>> = use_signal(|| vec![]);
    let show_jump = use_signal(|| true);
    let search_text = use_signal(|| "".to_string());
    let selected_translation = use_signal(|| "ESV".to_string());
    let sidebar_hidden: Signal<bool> = use_signal(|| false);
    let mut unique_books = use_signal(|| vec![]);

    const STYLE: &str = manganis::mg!(file("public/tailwind.css"));

    use_future(move || async move {
        // TODO: Handle error case better if fetch fails

        if let Some(bible_url) = TRANSLATIONS.get(&selected_translation()) {
            if let Ok(fetched_bible) = fetch_verses_from_url(&bible_url).await {
                unique_books.set(fetched_bible.get_unique_books());

                update_bible(
                    bible,
                    fetched_bible,
                    current_chapter,
                    current_chapter_text,
                    entered_chapter_num,
                    "Gen.1",
                );
            }
        }
    });

    rsx! {
        link { href: "{STYLE}", rel: "stylesheet", r#type: "text/css" }

        if bible().is_none() {
            LoadingScreen {}
        } else {
            div {
                class: "flex w-full bg-stone-100/40",
                display: "flex",
                flex_direction: "row",
                // Focusable input to receive keyboard events
                Sidebar {
                    sidebar_hidden,
                    bible,
                    unique_books,
                    current_chapter,
                    current_chapter_text,
                    entered_chapter_num
                }
                div { class: "flex-1 max-h-screen overflow-y-auto",
                    div { class: "flex px-4 pt-2",
                        ChapterNav {
                            sidebar_hidden,
                            bible,
                            current_chapter,
                            current_chapter_text,
                            entered_chapter_num,
                            show_jump
                        }
                    }
                    hr {}
                    ChapterText { sidebar_hidden, bible, smart_verses }
                }
                SmartJump {
                    bible,
                    show_jump,
                    current_chapter,
                    current_chapter_text,
                    entered_chapter_num,
                    smart_verses,
                    unique_books,
                    search_text,
                    selected_translation
                }
            }
        }
    }
}
