use image::{self, imageops::FilterType};
use std::error::Error;

pub fn convert_image(image_name: &str) -> Result<(), Box<dyn Error>> {
    let image = image::io::Reader::open(image_name)?.decode()?;
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
