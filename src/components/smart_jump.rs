use dioxus::prelude::*;
use crate::helpers::*;
use crate::models::*;
use log::debug;

#[component]
pub fn SmartJump(
    bible: Signal<Option<Bible>>, 
    show_jump: Signal<bool>, 
    current_chapter: Signal<String>,
    current_chapter_text: Signal<String>,
    entered_chapter_num: Signal<String>) -> Element {

    let mut search_text = use_signal(|| "".to_string());
    

    rsx! {
        if show_jump() {
            div {
                class: "fixed inset-0 flex items-center justify-center z-50",
                div {
                    class: "fixed inset-0 bg-gray-900 opacity-50",
                    onclick: move |_| show_jump.set(false),
                }
                div {
                    class: "rounded-lg shadow-lg my-4 z-10 w-1/2",
                    input {
                        class: "rounded-lg px-4 py-2 w-full focus:border-sky-500 focus:ring-sky-500 focus:ring-1 outline-bg-gray-50 appearance-none",
                        "type": "text",
                        placeholder: "Enter search text...",
                        autofocus: "{show_jump()}",
                        // is_attribute: "autofocus",
                        oninput: move |evt| search_text.set(evt.value()),
                        onchange: move |_| {
                            match bible() {
                                Some(mut curr_bible) => {
                                    match parse_bible_reference(&search_text()) {
                                        Some(found_match) => {
                                            if let Some(chapter_ref) = format_bible_reference(Some(found_match)) {
                                                debug!("{}", &chapter_ref);
                                                let chapter_ref = parse_chapter_ref(&chapter_ref);
                                                debug!("Parsed chapter ref: {}", &chapter_ref);
                                                curr_bible.go_to_chapter(&chapter_ref);
                                                current_chapter_text.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.text.clone()));
                                                current_chapter.set(curr_bible.get_current_chapter().map_or("".to_string(), |chapter| chapter.get_pretty_chapter()));
                                                entered_chapter_num.set(curr_bible.get_current_chapter().unwrap().chapter.to_string());
                                                show_jump.set(false);
                                                bible.set(Some(curr_bible));
                                                
                                            } else {
                                                debug!("Failed to format the reference");
                                            }
                                        }
                                        None => {
                                            debug!("No match found");
                                            show_jump.set(false);
                                        },
                                    }
                                },
                                None => debug!("No Bible match found during search")
                            }
                        },
                    }
                }
            }
        }
    }
}

fn parse_chapter_ref(chapter_ref: &str) -> String {
    let parts: Vec<&str> = chapter_ref.split('.').collect();
    
    if parts.len() >= 2 {
        format!("{}.{}", parts[0], parts[1])
    } else {
        chapter_ref.to_string()
    }
}