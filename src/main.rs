#![allow(non_snake_case)]
mod verse;
use dioxus::prelude::*;
use log::{debug, error};
use verse::{fetch_verses_from_url, Verses};

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let all_verses: &UseState<Verses> = use_state(cx, || Verses { verses: Vec::new() });
    let current_chapter: &UseState<String> = use_state(cx, || "Gen.1".to_string());
    let current_chapter_text: &UseState<String> = use_state(cx, || "".to_string());

    let _fetch_verses = use_future(cx, (), |_| {
        let all_verses = all_verses.to_owned();
        let current_chapter = current_chapter.to_owned();
        let current_chapter_text = current_chapter_text.to_owned();

        async move {
            if let Some(fetched_verses) = fetch_verses_from_url(
                "https://arweave.net/daKtqqHpLRnAWCNEWY8Q92NwSyJxWbm7WFDE3ut_BuM",
            )
            .await
            {
                all_verses.set(fetched_verses.clone());
                current_chapter_text.set({
                    fetched_verses
                        .verses
                        .iter()
                        .filter(|verse| verse.get_chapter() == current_chapter.get().to_string())
                        .fold("".to_string(), |acc, v| {
                            debug!("Hi");
                            format!("{} {}", acc, v.text)
                        })
                });
            }
        }
    });

    // let chapters = all_verses.aggregate_by_chapter().iter().map(|(x, y)| x);

    cx.render(rsx! {
        div {
            h1 { "Bible Verses" }
            ul {
                "{current_chapter_text}"
            }
        }
    })
}
