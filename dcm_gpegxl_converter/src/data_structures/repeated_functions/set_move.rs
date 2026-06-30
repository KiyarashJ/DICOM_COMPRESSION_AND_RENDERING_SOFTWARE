use dioxus::prelude::*;

use crate::data_structures::enums::Setdown;

pub fn tool_set_move(e: MouseEvent, tool_signal: Setdown){
        let coordinates = e.page_coordinates();
        
        match tool_signal {
            Setdown::None => {}
            Setdown::Pan(mut signal) => {
                let is_dragging = signal.read().is_dragging;
                if is_dragging {
                    let last_x = signal.read().last_mouse_x;
                let last_y = signal.read().last_mouse_y;
                let delta_y = coordinates.y - last_y;
                let delta_x = coordinates.x - last_x;
                signal.with_mut(|c| {
                    c.translate_x += delta_x;
                    c.translate_y += delta_y;
                    c.last_mouse_x = coordinates.x; 
                    c.last_mouse_y = coordinates.y;
                });
            }
                }
                
            Setdown::Windowlevel(mut signal) => {                
                let is_dragging = signal.read().is_dragging;
                if is_dragging {
                    let last_x = signal.read().last_mouse_x;
                let last_y = signal.read().last_mouse_y;
                let delta_y = coordinates.y - last_y;
                let delta_x = coordinates.x - last_x;
                signal.with_mut(|c| {
                    c.width += delta_x * 2.0; 
                    if c.width < 1.0 { c.width = 1.0; }
                    c.center -= delta_y * 2.0; 
                    c.last_mouse_x = coordinates.x;
                    c.last_mouse_y = coordinates.y;
                });
                }
                
            }
        }
    
}
