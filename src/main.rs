use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_name = args.get(1).unwrap_or_else(|| {
        eprintln!(" Please provide a .png image\n  -> png2ico <image name>.png");
        process::exit(1);
    });
    if let Err(e) = img2ico::convert_image(image_name) {
        eprintln!("{e}")
    }
}
