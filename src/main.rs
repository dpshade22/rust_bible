#![allow(non_snake_case)]
mod verse;
// use dioxus::prelude::*;
use dioxus_lib::prelude::*;
use log::debug;
use manganis::*;
use verse::{fetch_verses_from_url, Bible};

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = mg!(file("./public/tailwind.css"));

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch::launch(App, vec![], Default::default());
}

#[component]
fn App() -> Element {
    let mut bible = use_signal(|| None);
    let mut current_chapter = use_signal(|| "".to_string());
    let mut current_chapter_text = use_signal(|| "".to_string());

    use_future(move || async move {
        if let Ok(fetched_bible) =
            fetch_verses_from_url("https://arweave.net/daKtqqHpLRnAWCNEWY8Q92NwSyJxWbm7WFDE3ut_BuM")
                .await
        {
            bible.set(Some(fetched_bible.clone()));
            current_chapter.set(
                fetched_bible
                    .get_current_chapter()
                    .map_or("".to_string(), |chapter| chapter.get_pretty_chapter()),
            );
            current_chapter_text.set(
                fetched_bible
                    .get_current_chapter()
                    .map_or("".to_string(), |chapter| chapter.text.clone()),
            );
        }
    });

    rsx! {
        div {
            class: "flex min-h-screen w-full bg-gray-100/40 dark:bg-gray-800/40",
            // div {
            //     class: "hidden border-r bg-gray-100/40 w-[300px] dark:bg-gray-800/40 lg:block",
            //     nav {
            //         class: "flex flex-col h-full overflow-auto",
            //         div {
            //             class: "px-4 border-b",
            //             h1 {
            //                 class: "text-xl font-semibold",
            //                 "Books of the Bible"
            //             }
            //         }
            //         div {
            //             class: "flex-1 grid items-start px-4 py-2 text-sm font-medium",
            //             // Add book links here
            //         }
            //     }
            // }
            div {
                class: "flex-1 px-4 py-6 md:px-6 md:py-12 lg:py-16",
                div {
                    class: "space-y-6 prose prose-gray max-w-6xl mx-auto dark:prose-invert",
                    h1 {
                        class: "text-4xl font-extrabold tracking-tight lg:text-5xl",
                        "{current_chapter}"
                    }
                    div {
                        class: "space-y-2 not-prose",
                        p {
                            class: "text-gray-500 dark:text-gray-400",
                            "The Creation of the World"
                        }
                    }
                    div {
                        class: "space-y-2 prose prose-gray max-w-none",
                        p {
                            class: "drop-initial",
                            "{current_chapter_text}"
                        }
                    }
                }
                div {
                    class: "fixed bottom-0 left-0 m-4",
                    button {
                        class: "px-4 py-2 text-black rounded",
                        onclick: move |_| {
                            match bible() {
                                Some(mut curr_bible) => {
                                    curr_bible.previous_chapter();
                                    debug!("{:?}", current_chapter);
                                    current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                    current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                    bible.set(Some(curr_bible));
                                },
                                None => debug!("Bible match failed")
                            }
                        },
                        "Previous"
                    }
                }
                div {
                    class: "fixed bottom-0 right-0 m-4",
                    button {
                        class: "px-4 py-2 text-black rounded",
                        onclick: move |_| {
                            match bible() {
                                Some(mut curr_bible) => {
                                    curr_bible.next_chapter();
                                    current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                    current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                    debug!("{:?}", current_chapter);
                                    bible.set(Some(curr_bible));
                                },
                                None => debug!("Bible match failed")
                            }
                        },
                        "Next"
                    }
                }
            }
        }
    }
}
