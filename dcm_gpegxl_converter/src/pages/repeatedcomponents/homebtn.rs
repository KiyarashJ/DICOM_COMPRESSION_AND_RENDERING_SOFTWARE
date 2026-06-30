use dioxus::prelude::*;
use crate::router::Router;

#[component]
pub fn Homebtn(op: &'static str, bp: &'static str, position: &'static str) -> Element {
    let nav = use_navigator();
        rsx!(
            img {
                class: format!(
                    "w-16 h-15 transition-all ease-in hover:cursor-pointer hover:scale-105 absolute top-4 {} z-20",
                    position,
                ),
                src: if bp == "back" { asset!("/assets/back.png") } else { asset!("/assets/government-building.png") },
                onclick: move |_| {
                    match op {
                        "replace" => {
                            match bp {
                                "back" => nav.go_back(),
                                "home" => {
                                    nav.replace(Router::Home {});
                                }

                                _ => {
                                    eprintln!(
                                        "there is something wrong with home or back btn : wrong input",
                                    )
                                }
                            }
                        }
                        _ => eprintln!("there is something wrong with get back or home btn"),
                    }
                },
            }
        )
}
