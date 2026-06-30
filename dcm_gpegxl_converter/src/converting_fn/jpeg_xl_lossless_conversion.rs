use std::io::Write;
use std::error::Error;
use dicom::object::{FileDicomObject, InMemDicomObject, open_file};
use dicom_pixeldata::{Transcode};
use dicom_transfer_syntax_registry::{TransferSyntaxRegistry, entries::JPEG_XL_LOSSLESS, TransferSyntaxIndex};

use crate::{write_folder::write::write};



pub fn converting_dcm_to_gpegxl_lossless(file: String, file_name: &str) -> Result<&'static str, Box<dyn Error>> {
    let mut file = open_file(file).unwrap();
    let registry = TransferSyntaxRegistry::default();
    let get_jpegxl_transfer_uid = registry.get(JPEG_XL_LOSSLESS.uid()).unwrap();
    let _ = file.transcode(get_jpegxl_transfer_uid)?;
    let _ = write_on_new_file(file, file_name);
    Ok("Conversion of dcm to jpeg was successful")
}

fn write_on_new_file(file: FileDicomObject<InMemDicomObject>, file_name: &str) {
    
    let (mut writer , _) = write("jpeg", "./CONVERTED_JPEG_FILES", file_name);
    
    file.write_all(&mut writer)
        .expect("Error while writing file");

    writer.flush().expect("Error while flushing");
    
}