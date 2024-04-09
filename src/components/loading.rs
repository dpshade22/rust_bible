use crate::helpers::*;
use crate::models::*;
use dioxus::prelude::*;

#[component]
pub fn LoadingScreen() -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        div { class: format!("flex items-center justify-center h-screen {}", theme.background),
            div { class: "loading-dots",
                div { class: "dot dot-1" }
                div { class: "dot dot-2" }
                div { class: "dot dot-3" }
            }
        }
    }
}
