use crate::converting_fn::jpeg_xl_lossless_conversion::converting_dcm_to_gpegxl_lossless;

pub fn convert_single_file_into(file: &str) {
    let _ = converting_dcm_to_gpegxl_lossless(file.to_owned(), file);
    println!("Convert File ended !");
}