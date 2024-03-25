#![allow(non_snake_case)]
mod verse;
use dioxus::prelude::*;
use log::debug;
use verse::{fetch_verses_from_url, Bible};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let bible: &UseState<Option<Bible>> = use_state(cx, || None);
    // let all_verses: &UseState<Vec<Verse>> = use_state(cx, || vec![]);
    let current_chapter: &UseState<String> = use_state(cx, || "".to_string());
    let current_chapter_text: &UseState<String> = use_state(cx, || "".to_string());

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
                debug!("This is fetched Bible: {:?}", fetched_bible);
                bible.set(Some(fetched_bible.clone()));
                debug!("Beginning the wait");
                match fetched_bible.current_chapter() {
                    Some(chapter) => current_chapter_text.set(chapter.text.clone()),
                    None => current_chapter_text.set("Failed ".to_string()),
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Bible" }
            ul {
                "{current_chapter_text}"
            }
            button { onclick: move |event| {
                if let Some(mut bible_value) = bible.get().clone() {
                    bible_value.next_chapter();
                    current_chapter_text.set(bible_value.chapters.first().unwrap().text.clone());
                    bible.set(Some(bible_value));
                }
                log::info!("Clicked! Event: {event:?}")}, "Next Chapter" }
        }
    })
}
