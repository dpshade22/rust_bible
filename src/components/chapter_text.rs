use crate::bible::Bible;
use dioxus::prelude::*;

#[component]
pub fn ChapterText(bible: Signal<Option<Bible>>) -> Element {
    rsx! {
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
                                        class: "w-8 flex-shrink-0 text-right py-2  mr-2 font-bold",
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
}
