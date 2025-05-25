mod utils {
    mod open_ai_client;
    pub use open_ai_client::OpenAiClient;
}
mod generate {
    pub mod image_decription;
    pub mod image_urls;
}

pub mod gen {
    pub use super::{
        generate::image_decription::image_description,
        generate::image_urls::{image, ImageGenBuilder},
    };
}
