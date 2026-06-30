use std::{path::{PathBuf}};
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use dicom::object::{open_file};
use crate::converting_fn::{jpeg_xl_lossless_conversion::converting_dcm_to_gpegxl_lossless, png_generator::png_generator};


pub fn convert_files_into(folder: Vec<PathBuf>, r#filekind: &str, from_path:&str) -> Vec<Vec<u8>> {
    let start: DateTime<Utc> = Utc::now();
    let results: Vec<Vec<u8>> = folder.into_par_iter().map(|full_path| {
        let file = open_file(&full_path).unwrap();
        let file_name = full_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown.dcm").to_string();
        let input_path_str = full_path.to_string_lossy().to_string();
        if filekind == "jpeg" {
            let _ = converting_dcm_to_gpegxl_lossless(input_path_str, &file_name);
            Vec::new()
        } else {
            match png_generator(&file, &file_name , from_path) {
                Ok(res) => res,
                Err(e) =>{
                    println!("An error occured in png_generator: {}", e);
                    Vec::new()
                }
            }
        }
    }).collect();
    // folder.par_iter().for_each(|full_path| {
    //     let file = open_file(full_path).unwrap();
    //     let file_name = full_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown.dcm").to_string();
    //     let input_path_str = full_path.to_string_lossy().to_string();
        
    //     if filekind == "jpeg" {
    //         match converting_dcm_to_gpegxl_lossless(input_path_str, &file_name) {
    //             Ok(res) => res,
    //             Err(e) => {
    //                 println!("An error occured in converting_dcm_to_gpegxl_lossless : {}", e);
    //                 return;
    //             }
    //         };
    //     } else {
    //         match png_generator(&file, &file_name) {
    //             Ok(res) => res,
    //             Err(e) =>{
    //                 println!("An error occured in png_generator: {}", e);
    //                 return;
    //             }
    //         };
    //     }
    // });
    let end: DateTime<Utc> = Utc::now();
    let duration = end - start;
    println!("duration : {}", duration.num_milliseconds());
    results
}