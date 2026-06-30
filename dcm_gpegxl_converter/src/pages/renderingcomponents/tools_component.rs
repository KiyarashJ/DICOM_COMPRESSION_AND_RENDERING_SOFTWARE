use dioxus::prelude::*;

use crate::pages::renderingcomponents::tools::{pan_tool::pan_tool::Pantoolcomp, windowlevel_tool::windowlevel::Windowlevelcomp, zoom_tool::zoomtool::Zoomtoolcomp};

#[component]
pub fn Tools() -> Element{
    rsx!(
        aside { class: "w-full h-1/4 mt-2 flex ",
            Zoomtoolcomp {}
            Pantoolcomp {}
            Windowlevelcomp {}
        }
    )
}