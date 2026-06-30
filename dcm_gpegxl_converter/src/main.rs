pub mod converting_fn;
pub mod write_folder;
pub mod data_structures;
mod router;
mod pages;

use router::Router;
use dioxus::prelude::*;
use crate::data_structures::structs::DicomRenderData;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");







fn main() {
    dioxus::launch(|| {
        let mut context = use_context_provider(|| Signal::new(DicomRenderData::default()));
        rsx!{
            Router::<Router> {}
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        }
    });
}

