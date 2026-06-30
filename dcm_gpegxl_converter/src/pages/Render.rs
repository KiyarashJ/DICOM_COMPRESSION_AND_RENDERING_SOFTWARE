use dioxus::prelude::*;
use crate::pages::repeatedcomponents::{fileinput, homebtn};

#[component]
pub fn Render() -> Element {
    rsx!(
        div { class: "w-full h-screen bg-linear-to-tr from-emerald-800 to-emerald-950",
            h1 { class: "text-center md:text-2xl lg:text-5xl", "Medical Dicom Co-operation" }
            homebtn::Homebtn { op: "replace", bp: "home", position: "left-4" }
            homebtn::Homebtn { op: "replace", bp: "back", position: "right-4" }
            div { class: "w-full h-9/10 flex flex-col justify-center items-center",
                article { class: "w-7/10 text-justify",
                    "The Next Page you're gonna face needs some details about how it works and what are the options ? \n you need to upload your dicom files here maybe 1 or more files afterwards these file are gonna be rendered by the engine and you can see the first file in main section and others in preview section ! also there'll be some tools you need to be aware of them ! make sure when you use them ! ⚠️ CUATION : THESE RENDERED FILE ARE NOT GONNA BE SAVED ON YOUR LOCAL MACHINE ! If you want to save them you need to refer the other section of software(compression section) and select the png format type to save on the local machine ! "
                }
                fileinput::Fileinput { from_req: "rendering", compression_type: "" }
            }
        }
    )
}




                