use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use dioxus::prelude::*;
use crate::{DicomRenderData};

#[component]
pub fn Preview() -> Element {
    let mut files = use_context::<Signal<DicomRenderData>>();
    let base64_images: Vec<String> = files.read().png_bytes.iter()
        .map(|bytes| BASE64_STANDARD.encode(bytes))
        .collect();

    rsx!(
        section { class: "w-1/4 h-full",
            div { class: "w-19/20 h-37/40 bg-white/20 rounded-4xl overflow-y-scroll flex flex-col justify-center items-center no-scrollbar",
                for (index , base64_img) in base64_images.iter().enumerate() {

                    div { class: "w-9/10 h-auto bg-[#80EF80]/70 mb-2 rounded-2xl pt-1 pl-1 transition-all ease-out hover:cursor-pointer hover:scale-105",
                        img {
                            class: "w-1/2 md:h-[80px] lg:h-[170px] rounded-3xl mb-2",
                            src: "data:image/png;base64,{base64_img}",
                            onclick: move |_| {
                                files.write().selected_img = Some(index);
                            },
                        
                        }
                    }
                }
            }
        }
    )
}


