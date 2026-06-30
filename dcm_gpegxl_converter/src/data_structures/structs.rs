use crate::data_structures::enums::Activetool;

#[derive(Debug, Clone, Default)]
pub struct Zoomtool {
    pub zoom_x: f64,
    pub zoom_y: f64,
    pub scale_amount: f64
}

#[derive(Debug,Clone , Default)]
pub struct Pantool {
    pub translate_x: f64,
    pub translate_y: f64, 
    pub is_dragging: bool, 
    pub last_mouse_x: f64, 
    pub last_mouse_y: f64,
}

#[derive(Debug,Clone , Default)]
pub struct Windowleveltool{
    pub width: f64,
    pub center: f64, 
    pub is_dragging: bool, 
    pub last_mouse_x: f64, 
    pub last_mouse_y: f64,
}


// CONTEXT IS FOR IMAGE RENDERING , PREVIEW IMAGES RENDERING AND ZOOMTOOL SETTING !
#[derive(Debug, Clone, Default)]
pub struct DicomRenderData {
    pub png_bytes: Vec<Vec<u8>>,
    pub selected_img: Option<usize>,
    pub activetool : Activetool,
    pub zoom_tool: Zoomtool,
    pub pan_tool: Pantool,
    pub windowlevel_tool : Windowleveltool
}
