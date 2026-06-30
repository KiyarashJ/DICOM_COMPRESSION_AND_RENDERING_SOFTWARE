use dioxus::prelude::*;

use crate::{data_structures::enums::Activetool, DicomRenderData};


#[component]
pub fn ToolsButtons(btn_name: &'static str, tool_name: Activetool) -> Element {
    let mut tools_activity = use_context::<Signal<DicomRenderData>>();
    rsx!(
        button {
            class: "md:w-20 md:h-1/5 lg:w-35 lg:h-1/7 bg-white/30 flex justify-center items-center hover:bg-white/80 transition-all ease-out cursor-pointer hover:rounded-xl hover:scale-105 hover:shadow-sm hover:text-black z-10 m-2",
            onclick: move |_| { tools_activity.write().activetool = tool_name },
            "{btn_name}"
        }
    )
}