#![allow(non_snake_case)]
mod verse;
// use dioxus::prelude::*;
use dioxus_lib::prelude::*;
use log::debug;
use verse::{fetch_verses_from_url, Bible};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch::launch(App, vec![], Default::default());
}

#[component]
fn App() -> Element {
    let mut bible: Signal<Option<Bible>> = use_signal(|| None);
    let mut current_chapter = use_signal(|| "".to_string());
    let mut current_chapter_text = use_signal(|| "".to_string());
    // let mut search_keyword = use_signal(|| "".to_string());

    // let handle_search = move |_| {
    //     if let Some(mut curr_bible) = bible() {
    //         let keyword = search_keyword();
    //         curr_bible.chapters_by_keyword(&keyword);

    //         if let Some(chapters) = curr_bible.keyword_search_chapters {
    //             if chapters.len() != 0 {
    //                 let first_chapter = chapters.first().unwrap();
    //                 current_chapter_text.set(first_chapter.text.clone());
    //                 current_chapter.set(first_chapter.get_pretty_chapter());
    //             }
    //         }
    //     }
    // };

    use_future(move || async move {
        if let Ok(fetched_bible) =
            fetch_verses_from_url("https://arweave.net/daKtqqHpLRnAWCNEWY8Q92NwSyJxWbm7WFDE3ut_BuM")
                .await
        {
            bible.set(Some(fetched_bible.clone()));
            current_chapter.set(
                fetched_bible
                    .get_current_chapter()
                    .map_or("Chapter title not found...".to_string(), |chapter| {
                        chapter.get_pretty_chapter()
                    }),
            );
            current_chapter_text.set(
                fetched_bible
                    .get_current_chapter()
                    .map_or("Chapter text not found...".to_string(), |chapter| {
                        chapter.text.clone()
                    }),
            );
        }
    });

    rsx! {
        div {
            class: "flex min-h-screen w-full bg-gray-100/40 dark:bg-gray-800/40",
            div {
                class: "hidden border-r bg-gray-100/40 w-[300px] dark:bg-gray-800/40 lg:block",
                nav {
                    class: "flex flex-col h-full overflow-auto",
                    div {
                        class: "py-2 md:py-4 lg:py-2",
                        h1 {
                            class: "flex justify-center items-center space-x-4 text-xl font-semibold",
                            "Books"
                        }
                    }
                    div {
                        class: "flex-1 grid items-start px-4 py-2 text-sm font-medium",
                        // Add book links here
                    }
                }
            }
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
                            // "The Creation of the World"
                        }
                        hr {}
                    }
                    div {
                        class: "space-y-2 prose prose-gray max-w-none",
                        p {
                            class: "drop-initial",
                            dangerous_inner_html: current_chapter_text().chars().map(|c| {
                                if c.is_numeric() && !current_chapter_text().matches(&format!("{}(?!\\d{{1,2}}years)", c)).next().is_some() {
                                    format!("<b>{}</b>", c)
                                } else {
                                    c.to_string()
                                }
                            }).collect::<String>()
                        }
                    }
                }
                div {
                    // class: "fixed bottom-0 left-0 right-0 m-4 flex justify-center items-center space-x-4",
                    // div {
                    //     class: "relative",
                    //     input {
                    //         class: "block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-black-500 focus:border-black-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-black-500 dark:focus:border-black-500",
                    //         placeholder: "Search...",
                    //         value: "{search_keyword}",
                    //         oninput: move |evt| search_keyword.set(evt.value()),
                    //     }
                    //     button {
                    //         class: "text-black absolute right-2.5 bottom-2.5 focus:ring-4 focus:outline-none focus:ring-black-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-black-600 dark:hover:bg-black-700 dark:focus:ring-black-800",
                    //         onclick: handle_search,
                    //         "Search"
                    //     }
                    // }
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
}
