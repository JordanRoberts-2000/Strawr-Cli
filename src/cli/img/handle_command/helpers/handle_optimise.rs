use std::fs::{self, File};
use std::path::Path;

use image::codecs::avif::AvifEncoder;
use image::imageops::FilterType;
use image::{GenericImageView, ImageEncoder, ImageFormat};

pub fn handle_optimization(path: &String, output: &Option<String>) {
    let path = Path::new(&path);

    if path.is_absolute() {
        println!("Absolute path: {}", path.display());
    } else if path.is_relative() {
        println!("Relative path: {}", path.display());
    }

    if let Some(file_path) = find_file_with_any_extension(path) {
        // todo: reducing quality requires encoding
        // toDO: encoding with webp not supported in image crate
        // toDo: progressive jpeg / p-png options need exploring
        // avif
        // let img = image::open(file_path).expect("aaa");

        // // Get dimensions and convert to RGB8
        // let resized_img = img.resize(1024, 1024, FilterType::Lanczos3);
        // let (width, height) = resized_img.dimensions();
        // let rgb_img = resized_img.into_rgb8();

        // // Get raw buffer of RGB values
        // let buf = rgb_img.as_raw();

        // // Create output file
        // let output = File::create("./imgTestFolder/genDog.avif").expect("bbb");

        // // Create encoder with:
        // // - speed: 1 (slower but better compression, range 1-10)
        // // - quality: 75 (0-100)
        // let encoder = AvifEncoder::new_with_speed_quality(output, 1, 75);

        // // Write the image using RGB color type
        // encoder
        //     .write_image(buf, width, height, image::ColorType::Rgb8.into())
        //     .expect("ccc");

        // webp
        // let img = image::open(file_path).expect("OPEN FAIL");
        // let resized_img = img.resize(1024, 1024, FilterType::Lanczos3);

        // resized_img
        //     .save_with_format(&Path::new("./imgTestFolder/genDog.webp"), ImageFormat::WebP)
        //     .expect("SAVE FAIL");
    } else if path.is_dir() {
        println!("Path leads to a folder: {}", path.display());
        // optimize_folder(path, output);
    } else {
        println!("Invalid path or file not found: {}", path.display());
    }
}

fn find_file_with_any_extension(base_path: &Path) -> Option<std::path::PathBuf> {
    if base_path.is_file() {
        return Some(base_path.to_path_buf());
    }

    let dir = base_path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = base_path.file_name()?.to_str()?;

    // Read directory entries and find matching file
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if name == file_name {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}
