use dioxus::prelude::*;
use crate::router::Router;

#[component]
pub fn Home() -> Element {
    let elements = vec![
        ("top-[0]","animate-firstcircle"),
        ("top-[0] right-[0]","animate-secondcircle"),
        ("bottom-[0] right-[0]","animate-fourthcircle"),
        ("bottom-[0]","animate-thirdcircle"),
    ];
    rsx!{
        div { class: "w-full h-screen bg-linear-to-tr from-emerald-950 to-emerald-800 relative overflow-hidden",
            h1 { class: "md:text-2xl lg:text-5xl text-center", "Dicom Medical Rendering Page" }
            div { class: "w-full h-full flex",
                for (position , animation) in elements.iter() {
                    div {
                        class: format!(
                            "w-1/10 h-2/11 z-1 bg-white/50 rounded-full absolute backdrop-blur-sm shadow-[0_35px_35px_rgba(0,0,0,0.25)] {} hover:bg-white/80 transition-all ease-in cursor-pointer",
                            format!("{} {}", position, animation),
                        ),
                    }
                }
                div { class: "w-full h-full flex flex-col justify-center items-center ",
                    span { class: "md:mb-8 lg:mb-20 md:text-[120%] lg:text-[200%] z-100",
                        "Click Here to dive deep into ❤️‍🔥❤️‍🔥"
                    }
                    Link {
                        class: "w-1/10 h-1/16 bg-white/30 flex justify-center items-center hover:bg-white/80 transition-all ease-out cursor-pointer hover:rounded-xl hover:scale-105 hover:shadow-sm hover:text-black z-10",
                        to: Router::Requestuser {},
                        "Start"
                    }
                }
            }
        
        }
    }
}