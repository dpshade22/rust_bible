#![allow(non_snake_case)]
mod verse;
use dioxus::prelude::*;
use log::debug;
use verse::{fetch_verses_from_url, Bible, Verse};

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let bible: &UseState<Option<Bible>> = use_state(cx, || None);
    // let all_verses: &UseState<Vec<Verse>> = use_state(cx, || vec![]);
    let current_chapter: &UseState<String> = use_state(cx, || "Rom.1".to_string());
    let current_chapter_text: &UseState<String> = use_state(cx, || "".to_string());

    debug!("Hi");
    let _fetch_verses = use_future(cx, (), |_| {
        let bible = bible.to_owned();
        // let current_chapter = current_chapter.to_owned();
        let current_chapter_text = current_chapter_text.to_owned();
        debug!("Trying to fetch chapter");

        async move {
            if let Some(fetched_bible) = fetch_verses_from_url(
                "https://arweave.net/daKtqqHpLRnAWCNEWY8Q92NwSyJxWbm7WFDE3ut_BuM",
            )
            .await
            {
                debug!("Beginning the wait");
                debug!("{:?}", fetched_bible.chapters);
                bible.set(Some(fetched_bible));
                match bible.get() {
                    Some(bible) => match bible.get_chapter("Gen.1") {
                        Some(chapter) => current_chapter_text.set(chapter.text.clone()),
                        None => current_chapter_text.set("Failed".to_string()),
                    },
                    None => debug!("Failed to get dioxus Bible"),
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Bible Verses" }
            ul {
                "{current_chapter_text}"
            }
        }
    })
}
