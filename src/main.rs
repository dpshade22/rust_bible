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
    const _TAILWIND_URL: &str = manganis::mg!(file("./public/tailwind.css"));

    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        dioxus_web::launch::launch(App, vec![], Default::default());
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // dioxus_desktop::launch::launch(App, vec![], Default::default());
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
    let mut unique_books = use_signal(|| vec![]);

    use_future(move || async move {
        // TODO: Handle error case better if fetch fails
        if let Ok(fetched_bible) =
            fetch_verses_from_url("https://arweave.net/daKtqqHpLRnAWCNEWY8Q92NwSyJxWbm7WFDE3ut_BuM")
                .await
        {
            unique_books.set(fetched_bible.get_unique_books());

            update_bible_state(
                bible,
                fetched_bible,
                current_chapter,
                current_chapter_text,
                entered_chapter_num,
                "Gen.1",
            );
        }
    });

    rsx! {
            if bible().is_none() {
                LoadingScreen {}
            } else {
            div {
                style: include_str!("../public/tailwind.css") ,
                class: "flex w-full bg-gray-100/40",
                display: "flex",
                flex_direction: "row",
                // Focusable input to receive keyboard events
                Sidebar {bible, unique_books, current_chapter, current_chapter_text, entered_chapter_num},
                div {
                    class: "flex-1 max-h-screen overflow-y-auto",
                    div {
                        class: "flex px-4 pt-2",
                        ChapterNav { bible, current_chapter, current_chapter_text, entered_chapter_num, show_jump }
                    }
                    hr {}
                    ChapterText { bible, smart_verses }
                }
                SmartJump { bible, show_jump, current_chapter, current_chapter_text, entered_chapter_num, smart_verses, search_text }
            }
        }
    }
}
