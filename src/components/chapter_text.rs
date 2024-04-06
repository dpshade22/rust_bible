use crate::helpers::*;
use crate::models::*;
use dioxus::prelude::*;
use log::debug;

#[component]
pub fn ChapterText(
    sidebar_hidden: Signal<bool>,
    bible: Signal<Option<Bible>>,
    smart_verses: Signal<Vec<Verse>>,
) -> Element {
    fn verse_cleaning(verse: String) -> String {
        let verse = verse.replace("--", "-");
        let verse = verse.replace("\u{00b6} ", "");
        let verse = verse.replace("[", "");
        verse.replace("]", "")
    }

    rsx! {
        div {
            class: format!("my-4 {}", if sidebar_hidden() { "flex justify-center items-center" } else { "mx-6" }),
            if let Some(curr_bible) = bible() {
                if let Some(chapter) = curr_bible.get_current_chapter() {
                    div {
                        class: "max-w-3xl prose-gray no-scrollbar",
                        {
                            chapter.verses.iter().map(|verse| {
                                let is_smart_verse = smart_verses().iter().any(|v| {
                                    &v.r#ref == &verse.r#ref
                                });
                                let class = if is_smart_verse {
                                    "text-rose-600 font-medium"
                                } else {
                                    ""
                                };
                                rsx! {
                                    div {
                                        class: "flex items-start line",
                                        div {
                                            class: "w-8 flex-shrink-0 text-right py-2  mr-2 font-bold",
                                            "{verse.verse_num}"
                                        }
                                        p {
                                            class: "{class} flex-grow pl-4 py-1 leading-loose",
                                            "{verse_cleaning(verse.text.to_string())}"
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}
