use image::{self, imageops::FilterType};
use std::{error::Error, path::PathBuf};

pub fn convert_image(image_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let image_name = image_path
        .to_str()
        .unwrap_or_else(|| {
            println!(" Could not parse file name");
            "\\icon"
        })
        .split('\\')
        .last()
        .unwrap_or("icon");
    let image = image::io::Reader::open(&image_path)?.decode()?;
    let server_icon = image::DynamicImage::resize_exact(&image, 256, 256, FilterType::CatmullRom);
    let mut icon_name = image_name
        .split('.')
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap_or(&"icon")
        .to_string();
    icon_name.push_str(".ico");
    server_icon
        .clone()
        .save(icon_name)
        .expect("could not save icon");
    Ok(())
}
