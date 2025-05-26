mod generate {
    pub mod image_decription;
    pub mod image_urls;
}

pub use {
    generate::image_decription::image_description,
    generate::image_urls::{image, ImageGenBuilder},
};
