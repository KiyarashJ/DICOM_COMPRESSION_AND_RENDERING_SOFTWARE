use dioxus::prelude::*;
use crate::{DicomRenderData, data_structures::{enums::{Activetool, Setdown}, repeated_functions::{set_down::tool_set_down, set_move::tool_set_move, set_up::tool_set_up}, structs::{Pantool, Windowleveltool, Zoomtool}}, pages::renderingcomponents::{preview, tools::{zoom_tool::zoomtool}, tools_component}};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

#[component]
pub fn Renderuserfile() -> Element {
    let files = use_context::<Signal<DicomRenderData>>();
    let zoom_scale_signal = use_signal(|| Zoomtool {
        zoom_x: 0.0,
        zoom_y: 0.0,
        scale_amount: 1.0
    });
    let mut pan_signal = use_signal(|| Pantool {
        translate_x: 0.0,
        translate_y: 0.0,
        is_dragging: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0
    });
    let mut window_signal = use_signal(|| Windowleveltool {
        width: 400.0,
        center: 40.0, 
        is_dragging: false, 
        last_mouse_x: 0.0, 
        last_mouse_y: 0.0,
    });
    
    rsx!(
        div { class: "w-full h-screen bg-linear-to-tl from-[#023047] to-[#03045e]",
            h1 { class: "text-center md:text-2xl lg:text-5xl", "Medical Dicom Co-operation" }
            div { class: "w-full h-full flex ",
                preview::Preview {}
                section { class: "w-3/4 h-37/40 rounded-[5vw]",
                    tools_component::Tools {}
                    main { class: "w-full h-3/4 flex flex-col justify-center items-center",
                        match files.read().selected_img {
                            Some(idx) => {
                                if let Some(bytes) = files.read().png_bytes.get(idx) {
                                    let current_base64 = base64::prelude::BASE64_STANDARD.encode(bytes);
                                    let w_val = window_signal.read().width;
                                    let c_val = window_signal.read().center;
                                    let css_contrast = (400.0 / w_val) * 100.0;
                                    let css_brightness = (c_val / 40.0) * 100.0;

                                    rsx! {
                                        div {
                                            class: "md:w-9/10 md:h-9/10 lg:w-6/10 lg:h-17/20 rounded-3xl shadow-2xl overflow-hidden bg-black flex justify-center items-center",
                                            onmouseup: move |_| {
                                                tool_set_up(Setdown::Pan(pan_signal));
                                                tool_set_up(Setdown::Windowlevel(window_signal));
                                            },
                                            onmouseleave: move |_| {
                                                tool_set_up(Setdown::Pan(pan_signal));
                                                tool_set_up(Setdown::Windowlevel(window_signal));
                                            },
                                            img {
                                                draggable: false,
                                                class: "w-full h-full object-contain cursor-move",
                                                src: "data:image/png;base64,{current_base64}",
                                                style: "transform-origin: {zoom_scale_signal.read().zoom_x}px {zoom_scale_signal.read().zoom_y}px; \
                                                                                                                                                                                                                                                                                                                                                                                                                                        transform: translate({pan_signal.read().translate_x}px, {pan_signal.read().translate_y}px) scale({zoom_scale_signal.read().scale_amount}); \
                                                                                                                                                                                                                                                                                                                                                                                                                                        filter: contrast({css_contrast}%) brightness({css_brightness}%);",

                                                onwheel: move |e| {
                                                    if files.read().activetool == Activetool::Zoom {
                                                        zoomtool::zoom_set(e, zoom_scale_signal);
                                                    }
                                                },
                                                onmousedown: move |e| {
                                                    e.prevent_default();
                                                    if files.read().activetool == Activetool::Pan {
                                                        tool_set_down(e, Setdown::Pan(pan_signal));
                                                    } else {
                                                        tool_set_down(e, Setdown::Windowlevel(window_signal));
                                                    }
                                                },
                                                onmousemove: move |e| {
                                                    if files.read().activetool == Activetool::Pan {
                                                        tool_set_move(e, Setdown::Pan(pan_signal));
                                                    } else {
                                                        tool_set_move(e, Setdown::Windowlevel(window_signal));
                                                    }
                                                },
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        p { class: "text-white", "Image bytes not found in storage" }
                                    }
                                }
                            }
                            None => rsx! {
                                p { class: "text-white/60 text-xl animate-pulse", "Please select an Image from the left-side preview" }
                            },
                        }
                    }
                }
            }
        }
    )
}