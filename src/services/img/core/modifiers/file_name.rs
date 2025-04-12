use crate::services::img::Img;

impl Img {
    // pub fn with_file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
    //     let new_file_name = file_name.as_ref().to_string();
    //     self.file_name = new_file_name.clone();

    //     let parent = self
    //         .target_path
    //         .parent()
    //         .unwrap_or_else(|| std::path::Path::new(""));
    //     self.target_path = parent.join(&new_file_name);

    //     self
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::services::img::core::ImgSrc;

//     use super::*;
//     use image::{DynamicImage, ImageFormat, RgbaImage};
//     use std::path::PathBuf;

//     fn dummy_img() -> Img {
//         Img {
//             img: DynamicImage::ImageRgba8(RgbaImage::new(100, 100)),
//             src: ImgSrc::Local {
//                 path: PathBuf::from("original.png"),
//             },
//             target_path: PathBuf::from("some/folder/original.png"),
//             file_name: "original.png".to_string(),
//             height: 100,
//             width: 100,
//             aspect_ratio: 1.0,
//             format: ImageFormat::Png,
//             size_bytes: 1024,
//         }
//     }

//     #[test]
//     fn with_file_name() {
//         let mut img = dummy_img();

//         img.with_file_name("new_name");

//         assert_eq!(img.file_name, "new_name.png");
//         assert_eq!(img.target_path, PathBuf::from("some/folder/new_name.png"));
//     }
// }
