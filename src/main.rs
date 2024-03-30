#![allow(non_snake_case)]

mod components;
mod verse;

use dioxus::prelude::*;
use log::debug;
use verse::*;

fn main() {
    // Urls are relative to your Cargo.toml file
    const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

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
    let mut bible: Signal<Option<Bible>> = use_signal(|| None);
    let mut current_chapter = use_signal(|| "".to_string());
    let mut current_chapter_text = use_signal(|| "".to_string());
    let mut unique_books = use_signal(|| vec![]);
    let mut chapter_tuples = use_signal(|| Vec::new());
    let mut entered_chapter_num = use_signal(|| "1".to_string());

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
        // TODO: Handle error case better if fetch fails

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

            let chapter_tuples_vec: Vec<(String, Chapter)> = fetched_bible
                .chapters
                .iter()
                .map(|chapter| (chapter.r#ref.clone(), chapter.clone()))
                .collect();
            chapter_tuples.set(chapter_tuples_vec);
        }
    });

    rsx! {
        div {
            class: "flex w-full bg-gray-100/40",
            display: "flex",
            flex_direction: "row",
            div {
                class: "flex bg-gray-100/10 w-[250px] lg:block max-h-screen overflow-y-auto",
                nav {
                    div {
                        class: "flex-1 grid items-start py-2 text-sm font-medium",
                        if let Some(curr_bible) = bible() {
                            for book in unique_books() {
                                button {
                                    class: "py-2",
                                    disabled: curr_bible.get_current_chapter().map_or(false, |chapter| chapter.book == book),
                                    onclick: move |_| {
                                        match bible() {
                                            Some(mut curr_bible) => {
                                                // debug!("{}", &book);
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
                                        {}
                                        div {
                                            class: "flex justify-between align-middle text-base pl-2 py-2 text-white bg-gray-700",
                                            strong {
                                                class: "flex items-center",
                                                "{book.to_uppercase()}"
                                            }
                                            input {
                                                class: "ml-auto pl-2 pr-2 py-1 w-20 text-right bg-gray-700 appearance-none focus:outline-none hover:bg-gray-600",
                                                r#type: "number",
                                                maxlength: "3",
                                                value: "{entered_chapter_num}",
                                                key: "{book}",
                                                autofocus: true,
                                                oninput: move |evt| {
                                                    if let Some(mut curr_bible) = bible() {
                                                        let chapter_num = evt.value().parse().unwrap_or(1);
                                                        let num_chapters_in_book = curr_bible.num_chapters_in_current_book();
                                                        // TODO: Handle "no current chapter" case more explicitly

                                                        let chapter_num = match chapter_num {
                                                            // TODO: Validate chapter_num input more strictly
                                                            num if num < 1 => {entered_chapter_num.set(1.to_string()); 1},
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
            div {
                class: "flex-1 max-h-screen overflow-y-auto",
                div {
                    class: "flex px-4 pt-2",
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

                hr {}
                div {
                    class: "ml-6 my-4 prose-gray max-w-prose",
                    if let Some(curr_bible) = bible() {
                        if let Some(chapter) = curr_bible.get_current_chapter() {
                            {
                                chapter.verses.iter().map(|verse| {
                                    rsx! {
                                        div {
                                            class: "flex items-start line",
                                            div {
                                                class: "w-8 flex-shrink-0 text-right py-1  mr-2 font-bold",
                                                "{verse.verse_num}"
                                            }
                                            div {
                                                class: "flex-grow pl-4 py-1 leading-loose",
                                                "{verse.text}"
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
            // div {
            //     class: "fixed bottom-0 left-250 m-4",
            //     button {
            //         class: "px-4 py-2 text-white bg-gray-700 rounded",
            //         onclick: move |_| {
            //             match bible() {
            //                 Some(mut curr_bible) => {
            //                     curr_bible.previous_chapter();
            //                     debug!("{:?}", current_chapter);
            //                     current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
            //                     current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
            //                     bible.set(Some(curr_bible));
            //                 },
            //                 None => debug!("Bible match failed")
            //             }
            //         },
            //         "Previous"
            //     }
            // }
            // div {
            //     class: "fixed bottom-0 right-0 m-4",
            //     button {
            //         class: "px-4 py-2 text-white bg-gray-700 rounded",
            //         onclick: move |_| {
            //             match bible() {
            //                 Some(mut curr_bible) => {
            //                     curr_bible.next_chapter();
            //                     current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
            //                     current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
            //                     debug!("{:?}", current_chapter);
            //                     bible.set(Some(curr_bible));
            //                 },
            //                 None => debug!("Bible match failed")
            //             }
            //         },
            //         "Next"
            //     }
            // }
        }
    }
}
