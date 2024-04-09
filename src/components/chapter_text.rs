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
    let theme = use_context::<Theme>();

    fn verse_cleaning(verse: String) -> String {
        let verse = verse.replace("--", "-");
        let verse = verse.replace("\u{00b6} ", "");
        let verse = verse.replace("[", "");
        verse.replace("]", "")
    }

    rsx! {
        div {
            class: format!("my-4 {}", if sidebar_hidden() { "flex justify-center items-center mr-4" } else { "md:mx-3 sm:mx-4" }),
            if let Some(curr_bible) = bible() {
                if let Some(chapter) = curr_bible.get_current_chapter() {
                    div {
                        class: format!("max-w-3xl prose-stone text-{}", theme.prim_50),
                        {
                            chapter.verses.iter().map(|verse| {
                                let is_smart_verse = smart_verses().iter().any(|v| {
                                    &v.r#ref == &verse.r#ref
                                });
                                let class = if is_smart_verse {
                                    "text-orange-600 font-medium"
                                } else {
                                    "text-stone-800"
                                };
                                rsx! {
                                    div {
                                        class: "flex items-start line",
                                        div {
                                            class: format!("w-8 flex-shrink-0 text-{} text-right py-2  mr-2 font-bold", theme.prim_400),
                                            "{verse.verse_num}"
                                        }
                                        p {
                                            class: "{class} flex-grow pl-4 pt-1 leading-loose",
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
