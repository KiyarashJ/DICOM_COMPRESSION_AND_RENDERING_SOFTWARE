use dioxus::prelude::*;
use crate::converting_fn::conversion_folder;
use crate::{DicomRenderData, router::Router};
use rayon::prelude::*;
use crate::{pages::repeatedcomponents::{toast_message}};


#[component]
pub fn Fileinput(from_req: &'static str, compression_type: String) -> Element {
    let mut status = use_signal(|| None::<(String, String)>);
    let nav = use_navigator();
    let mut context = use_context::<Signal<DicomRenderData>>();
    rsx!(
        input {
            class: "w-1/3 h-1/10 text-center bg-white/40 rounded-full transition-all ease-out cursor-pointer hover:bg-white/80 hover:scale-105",
            r#type: "file",
            accept: ".dcm",
            multiple: true,
            oninput: move |e| {
                let files = e.files();
                let filess: Vec<std::path::PathBuf> = files
                    .into_par_iter()
                    .map(|f| f.path())
                    .collect();
                if from_req == "rendering" {
                    let photos = conversion_folder::convert_files_into(
                        filess,
                        "png",
                        "rendering",
                    );
                    context.write().png_bytes = photos;
                    nav.push(Router::Renderuserfile {});
                } else {
                    println!("compression_type: {}", compression_type);
                    let _ = match compression_type.as_str() {
                        "jpeg" => {
                            let _ = conversion_folder::convert_files_into(
                                filess,
                                compression_type.as_str(),
                                "compression",
                            );
                            status
                                .set(
                                    Some((
                                        "conversion successful ended!".to_string(),
                                        "text-green-300".to_owned(),
                                    )),
                                );
                        }
                        "png" => {
                            let _ = conversion_folder::convert_files_into(
                                filess,
                                compression_type.as_str(),
                                "compression",
                            );
                            status
                                .set(
                                    Some((
                                        "conversion successful ended!".to_string(),
                                        "text-green-300".to_owned(),
                                    )),
                                );
                        }
                        "none" | "" | _ => {
                            eprintln!("you need to select the format of compression");
                            status
                                .set(
                                    Some((
                                        "you need to select the format of compression".to_owned(),
                                        "text-red-500".to_owned(),
                                    )),
                                );
                        }
                    };
                }
            },
        }
        if let Some((res, color)) = status() {
            toast_message::ToastMsg { msg: res, color }
        }
    )
}