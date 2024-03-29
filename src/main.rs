#![allow(non_snake_case)]

mod components;
mod verse;

use dioxus::prelude::*;
use log::debug;
use verse::{fetch_verses_from_url, Bible};

fn main() {
    // Urls are relative to your Cargo.toml file
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch::launch(App, vec![], Default::default());
}

#[component]
fn App() -> Element {
    let mut bible: Signal<Option<Bible>> = use_signal(|| None);
    let mut current_chapter = use_signal(|| "".to_string());
    let mut current_chapter_text = use_signal(|| "".to_string());
    let mut unique_books = use_signal(|| vec![]);

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
            unique_books.set(fetched_bible.get_unique_books());
        }
    });

    rsx! {
    div {
        class: "flex w-full bg-gray-100/40",
        display: "flex",
        flex_direction: "row",
        div {
            class: "flex flex-col  border-r bg-gray-100/40 w-[250px] lg:block max-h-screen overflow-y-auto ",
            nav {
                class: "flex flex-col flex-1",
                // div {
                //     class: "py-2 md:py-4 lg:py-2",
                //     h1 {
                //         class: "flex justify-center items-center space-x-4 text-xl font-semibold",
                //         "Books"
                //     }
                //     hr {}
                // }
                div {
                    class: "flex-1 grid items-start px-4 py-2 text-sm font-medium",
                    if let Some(curr_bible) = bible() {
                        for book in unique_books() {
                            button {
                                class: "block px-4 py-2 rounded-md hover:bg-gray-200",
                                onclick: move |_| {
                                    match bible() {
                                        Some(mut curr_bible) => {
                                            debug!("{}", &book);
                                            let chapter_ref = curr_bible.chapters
                                                .iter()
                                                .find(|chapter| &chapter.book == &book)
                                                .map(|chapter| chapter.r#ref.as_str())
                                                .map(|s| s.to_string());

                                            if let Some(chapter_ref) = chapter_ref {
                                                curr_bible.go_to_chapter(&chapter_ref);
                                                current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                                current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                                bible.set(Some(curr_bible));
                                            }
                                        }
                                        None => {debug!("{}", book);}
                                    }
                                },
                                // "{book}"

                                    if curr_bible.get_current_chapter().map_or(false, |chapter| chapter.book == book) {

                                        strong {
                                            class: "block text-3xl px-4 py-2 rounded-md bg-slate-700	",
                                            "{book.to_uppercase()}"
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
            div {
                class: "flex-1 max-h-screen overflow-y-auto",
                div {
                    class: "px-4 py-6 md:px-6 md:py-12 lg:py-16",
                    div {
                        class: "space-y-6 prose prose-gray max-w-6xl mx-auto",
                        h1 {
                            class: "text-4xl font-extrabold tracking-tight lg:text-5xl mb-4 inline-block",
                            "{current_chapter}"
                        }
                        div {
                            class: "flex space-x-4 overflow-x-auto pb-4 scrollbar-hide",
                            if let Some(curr_bible) = bible() {
                                for chapter in curr_bible.chapters {
                                    button {
                                        class: "flex-none rounded-md px-4 py-2 text-sm font-medium hover:bg-gray-200 focus:outline-none",
                                        class: if chapter.get_pretty_chapter() == current_chapter() { "bg-slate-700 text-white" } else { "bg-white" },
                                        onclick: move |_| {
                                            current_chapter_text.set(chapter.text.clone());
                                            current_chapter.set(chapter.get_pretty_chapter());
                                        },
                                        "{chapter.get_pretty_chapter()}"
                                    }
                                }
                            }
                            }
                        hr {}
                        div {
                            class: "space-y-4 prose prose-gray max-w-prose",

                            if let Some(curr_bible) = bible() {
                                if let Some(chapter) = curr_bible.get_current_chapter() {
                                    {
                                        chapter.verses.iter().map(|verse| {
                                            rsx! {
                                                p {
                                                    class: "text-lg leading-loose",
                                                    dangerous_inner_html: format_args!("<b>{}</b> {} <br />", verse.verse_num, verse.text),
                                                    " "
                                                }
                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "fixed bottom-0 left-250 m-4",
                    button {
                        class: "px-4 py-2 text-black bg-slate-300 rounded",
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
                        class: "px-4 py-2 text-black bg-slate-300 rounded",
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
