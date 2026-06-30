// use std::{io::{Write}};
// use dicom::object::{FileDicomObject, InMemDicomObject};
// use dicom_pixeldata::{PixelDecoder, Error};
// use crate::write_folder::write::write;


// pub fn png_generator(file: &FileDicomObject<InMemDicomObject>, file_name: &str)-> Result<&'static str, Error> {
//     let file_n = "./CONVERTED_JPEG_FILES/";
//     let (mut writer, full_path) = write("png", file_n, file_name);
//     let pixels = file.decode_pixel_data()?;
//     let png = pixels.to_dynamic_image(0)?;
//     let _ = png.save(full_path);
//     let _ = writer.flush();
//     Ok("Success in converting to png!")
// }




use std::{io::{Write, Cursor}};
use dicom::object::{FileDicomObject, InMemDicomObject};
use dicom_pixeldata::{PixelDecoder, Error};
use crate::write_folder::write::write;
use image;

pub enum GenResult {
    Inmemory(image::DynamicImage),
    SaveToDisk
}


pub fn png_generator(file: &FileDicomObject<InMemDicomObject>, file_name: &str, from_path:&str) -> Result<Vec<u8>, Error> {
    let pixels = file.decode_pixel_data()?;
    let png = pixels.to_dynamic_image(0)?;
    if from_path == "rendering" {
        let mut image_buffer = Vec::new();
        let mut cursor = Cursor::new(&mut image_buffer);
        let _ = png.write_to(&mut cursor , image::ImageFormat::Png).map_err(|e| println!("error occured while writing png to cursor : {}", e));
        Ok(image_buffer)
    } else {
        let file_n = "./CONVERTED_JPEG_FILES/";
        let (mut writer, full_path) = write("png", file_n, file_name);
        let _ = png.save(full_path);
        let _ = writer.flush();
        Ok(Vec::new())
    }
}