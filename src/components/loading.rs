use dioxus::prelude::*;

#[component]
pub fn LoadingScreen() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen",
            div {
                class: "loading-dots",
                div {
                    class: "dot dot-1"
                }
                div {
                    class: "dot dot-2"
                }
                div {
                    class: "dot dot-3"
                }
            }
        }
    }
}
