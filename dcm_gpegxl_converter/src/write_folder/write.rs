use std::{fs::{self, File}, io::BufWriter, path::{Path, PathBuf}};

pub fn write
    (
    type_write: &str, path: &str, file_name: &str
    )
    -> (BufWriter<File>, PathBuf)
    {
    let output_dir = Path::new(path);
    let _ = fs::create_dir_all(output_dir);
    let full_path = if type_write == "png" {
        let png_dir = output_dir.join("PNGS");
        fs::create_dir_all(&png_dir).unwrap();
        png_dir.join(format!("{}.png", file_name))
    } else {
        fs::create_dir_all(output_dir).unwrap();
        output_dir.join(file_name)
    };
    let mut writer = BufWriter::new(
        File::create(&full_path).unwrap()
    );
    
    (writer , full_path)

}