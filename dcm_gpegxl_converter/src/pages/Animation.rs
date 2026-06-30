use dioxus::prelude::*;

#[component]
pub fn Animation() -> Element {
    let mut opacity_state = use_signal(|| "opacity-0");
    use_effect(move || {
        opacity_state.set("opacity-100");
    });

    rsx! {
        div { class: "w-full h-screen bg-emerald-400 flex justify-center items-center transition-all duration-5000 ease-in-out {opacity_state}",
            h1 { class: "text-5xl lg:text-8xl text-neutral-50 ", "Medical Dicom Co-operation" }
        }
    }
}