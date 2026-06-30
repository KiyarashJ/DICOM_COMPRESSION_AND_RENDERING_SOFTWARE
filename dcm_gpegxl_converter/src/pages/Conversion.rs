use dioxus::prelude::*;
use crate::{pages::repeatedcomponents::{homebtn,fileinput}};


#[component]
pub fn Conversion() -> Element {
    let mut compression_type = use_signal(|| "".to_string());
    let options = [
        ("bg-black","none","--select--"),("bg-black","jpeg","jpeg-xl-lossless"),("bg-black","png","png")
    ];
    rsx!(
        div { class: "w-full h-screen bg-linear-to-tr from-emerald-800 to-emerald-950",
            h1 { class: "text-center md:text-2xl lg:text-5xl", "Medical Dicom Co-operation" }
            homebtn::Homebtn { op: "replace", bp: "home", position: "left-4" }
            homebtn::Homebtn { op: "replace", bp: "back", position: "right-4" }
            div { class: "w-full h-9/10 flex flex-col justify-center items-center",
                h2 { class: "mb-5", "which format of dcm file do you prefer ?" }
                select {
                    oninput: move |e| {
                        compression_type.set(e.value());
                    },
                    class: "mb-15 bg-black rounded-full transition-all ease-in-out p-1 hover:bg-gray-800 hover:cursor-pointer hover:scale-105 outline-none",
                    for (clss , value , name) in options.into_iter() {
                        option { class: "{clss}", value: "{value}", "{name}" }
                    }
                }
                span { class: "mb-5", "Then select your file OR files :" }
                fileinput::Fileinput {
                    from_req: "compression",
                    compression_type: compression_type(),
                
                }
            }
        }
    )
}