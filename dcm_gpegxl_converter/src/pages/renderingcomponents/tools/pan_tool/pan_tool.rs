use dioxus::prelude::*;

use crate::{pages::repeatedcomponents::tools_btn};

#[component]
pub fn Pantoolcomp() -> Element {
    rsx!(
        div {
            tools_btn::ToolsButtons {
                btn_name: "Pan",
                tool_name: crate::data_structures::enums::Activetool::Pan,
            }
        }
    )
}



// pub fn pan_set_down(e: MouseEvent, mut pan_signal : Signal<Pantool>) {
//     let coordinates = e.page_coordinates();
//     pan_signal.with_mut(|c| {
//         c.is_dragging = true;
//         c.last_mouse_x = coordinates.x;
//         c.last_mouse_y = coordinates.y;
//     });
// }

// pub fn pan_set_move(e: MouseEvent, mut pan_signal: Signal<Pantool>){
//     let is_dragging = pan_signal.read().is_dragging;
    
//     if is_dragging {
//         let coordinates = e.page_coordinates();
//         let last_x = pan_signal.read().last_mouse_x;
//         let last_y = pan_signal.read().last_mouse_y;
        
//         let delta_x = coordinates.x - last_x;
//         let delta_y = coordinates.y - last_y;

//         pan_signal.with_mut(|c| {
//             c.translate_x += delta_x;
//             c.translate_y += delta_y;
//             c.last_mouse_x = coordinates.x; 
//             c.last_mouse_y = coordinates.y;
//         });
//     }
// }

// pub fn pan_set_up(mut pan_signal: Signal<Pantool>){
//     pan_signal.with_mut(|c|{
//         c.is_dragging = false;
//     });
// }