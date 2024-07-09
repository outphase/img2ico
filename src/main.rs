use image::{self, imageops::FilterType};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_name = args.get(1).unwrap_or_else(|| {
        eprint!(" Please provide a .png image\n  -> png2ico <image name>.png");
        process::exit(1);
    });

    // HACK: Implement Failstate
    let image = image::io::Reader::open(image_name)
        .unwrap_or_else(|_| {
            eprintln!(" The provided image '{image_name}' does not exist");
            process::exit(1);
        })
        .decode()
        .unwrap_or_else(|_| {
            eprintln!(" Could not read {image_name}, is it the correct format?");
            // eprintln!(" Could not convert '{image_name}' to i");
            process::exit(1);
        });
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
}
