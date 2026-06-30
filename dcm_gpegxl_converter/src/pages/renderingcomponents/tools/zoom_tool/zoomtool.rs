use dioxus::prelude::*;
use crate::{data_structures::structs::Zoomtool, pages::repeatedcomponents::tools_btn};

#[component]
pub fn Zoomtoolcomp() -> Element {
    rsx!(
        div {
            tools_btn::ToolsButtons {
                btn_name: "Zoom",
                tool_name: crate::data_structures::enums::Activetool::Zoom,
            }
        }
    )
}



pub fn zoom_set(event: Event<WheelData>, mut zoom_scale_signal: Signal<Zoomtool>){
    let wheel = event.delta().strip_units().y;
    let coords = event.element_coordinates();
    zoom_scale_signal.with_mut(|zoom| {
        zoom.zoom_x = coords.x;
        zoom.zoom_y = coords.y;
        if wheel < 0.0 { 
            if zoom.scale_amount < 8.0 {
                zoom.scale_amount += 0.1;
            }
        } else {
            if zoom.scale_amount > 0.5 {
                zoom.scale_amount -= 0.1;
            }
        }
    });
}


