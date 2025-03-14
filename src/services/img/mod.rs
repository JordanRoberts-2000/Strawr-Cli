use image::GenericImageView;
use std::path::PathBuf;
use webp::{Encoder, PixelLayout};

// struct ImageService {};

// impl ImageService {
//     pub fn webp_convert(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
//         let img = image::open(path)?;
//         let (width, height) = img.dimensions();

//         // Convert the image to RGBA8, which is what the webp encoder expects.
//         let rgba_image = img.to_rgba8();

//         // Create a WebP encoder.
//         let encoder = Encoder::new(&rgba_image, PixelLayout::Rgba, width, height);
//         encoder.encode_simple(lossless: bool, quality)
//         Ok(())
//     }
// }
