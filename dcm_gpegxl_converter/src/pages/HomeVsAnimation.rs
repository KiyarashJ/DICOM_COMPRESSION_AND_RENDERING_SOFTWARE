use dioxus::prelude::*;
use crate::pages::{Animation, Home};
use std::time::Duration;

#[component]
pub fn HomeVsAnimation() -> Element {
    let mut show_home = use_signal(|| false);
    use_future(move || async move {
        tokio::time::sleep(Duration::from_secs(7)).await;
        show_home.set(true);
    });
    rsx! {
        if show_home() {
            Home::Home {}
        } else {
            Animation::Animation {}
        }
    }
}