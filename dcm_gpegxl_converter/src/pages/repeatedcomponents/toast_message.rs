use dioxus::prelude::*;


#[component]
pub fn ToastMsg(msg: String, color: String) -> Element {
    rsx!(
        div { class: "w-60 h-15 bg-emerald-950/65 absolute absolute bottom-[5%] md:left-[35%] lg:left-[44%] rounded-full flex justify-center items-center pr-3 pl-3",
            h1 { class: "{color}", "{msg}" }
        }
    )
}
