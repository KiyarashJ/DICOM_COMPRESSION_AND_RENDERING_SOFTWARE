use dioxus::prelude::*;
use crate::router::Router;
use crate::pages::repeatedcomponents::homebtn;

#[component]
pub fn Requestuser() -> Element {
    let link_class = vec![
        ("md:w-1/5 md:h-1/11 lg:w-1/10 lg:h-1/16 bg-white/30 flex justify-center items-center hover:bg-white/80 transition-all ease-out cursor-pointer hover:rounded-xl hover:scale-105 hover:shadow-sm hover:text-black z-10 mr-10", Router::Conversion {}, "compress"),
        ("md:w-1/5 md:h-1/11 lg:w-1/10 lg:h-1/16 bg-white/30 flex justify-center items-center hover:bg-white/80 transition-all ease-out cursor-pointer hover:rounded-xl hover:scale-105 hover:shadow-sm hover:text-black z-10", Router::Render {}, "render")
    ];
    
    rsx!(
        div { class: "w-full h-screen bg-linear-to-tr from-emerald-800 to-emerald-950 relative text-white",
            h1 { class: "text-center text-[200%] pt-5", "Dicom Medical Rendering Page" }
            homebtn::Homebtn { op: "replace", bp: "home", position: "left-4" }

            div { class: "w-full h-9/10 flex flex-col justify-center items-center relative",
                h1 { class: "absolute top-[30%] md:text-[150%] lg:text-[250%]",
                    "Choose your Work: 👇"
                }
                div { class: "w-full h-full flex justify-center items-center p-10",

                    for (clss , route , name) in link_class.iter() {
                        Link { class: "{clss}", to: route.clone(), "{name}" }
                    }
                }
            }
        }
    )
}