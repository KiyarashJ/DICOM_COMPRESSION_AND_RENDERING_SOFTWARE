use dioxus::signals::Signal;

use crate::data_structures::structs::{Pantool, Windowleveltool};




#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub enum Activetool {
    #[default]
    None,
    Zoom,
    Pan,
    Windowlevel
}



#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub enum Setdown {
    #[default]
    None,
    Pan(Signal<Pantool>),
    Windowlevel(Signal<Windowleveltool>)
}