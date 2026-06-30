use dioxus::prelude::*;
use crate::data_structures::{enums::Setdown};

pub fn tool_set_down(e: MouseEvent, tool_signal : Setdown) {
    let coordinates = e.page_coordinates();
    match tool_signal {
        Setdown::None => {},
        Setdown::Pan( mut signal) => {
            signal.with_mut(|c| {
                c.is_dragging = true;
                c.last_mouse_x = coordinates.x;
                c.last_mouse_y = coordinates.y;
            });
        },
        Setdown::Windowlevel( mut signal) => {
            signal.with_mut(|c| {
                c.is_dragging = true;
                c.last_mouse_x = coordinates.x;
                c.last_mouse_y = coordinates.y;
            });
        }
    }
}