use dioxus::prelude::*;
use crate::data_structures::enums::Setdown;

pub fn tool_set_up(signal: Setdown) {
    match signal {
        Setdown::None => {}
        Setdown::Pan(mut tool_signal) => {
            tool_signal.with_mut(|f| {
                f.is_dragging = false;
            });
        }
        Setdown::Windowlevel(mut tool_signal) => {
            tool_signal.with_mut(|f| {
                f.is_dragging = false;
            });
        }
    }
}