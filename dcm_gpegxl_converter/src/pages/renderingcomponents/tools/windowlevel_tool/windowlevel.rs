use dioxus::prelude::*;

use crate::{data_structures::enums::Activetool, pages::repeatedcomponents::tools_btn::ToolsButtons};

#[component]
pub fn Windowlevelcomp() -> Element {
    rsx!(
        div {
            ToolsButtons { btn_name: "windowlevel", tool_name: Activetool::Windowlevel }
        }
    )
    
}