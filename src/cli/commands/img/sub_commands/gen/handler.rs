use reqwest::blocking::Client;
use serde_json::json;

use super::args::Gen;
use crate::{
    cli::commands::img::{sub_commands::gen::manager::GenManager, ImgError},
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    error::Result,
    services::keychain::keychain,
    state::AppContext,
};

impl Gen {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        let mut manager = GenManager::new(ctx, &self);
        manager.handle_args();

        //  let api_key = keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;
        //   let client = Client::new();

        //   let image_url = manager.generate_image(&client, &api_key);

        //   let img = manager.download_image(&client, &image_url);

        //   img.save_to();

        // let request_body = json!({
        //     "prompt": self.description,
        //     "n": 1,                  // Number of images to generate
        //     "size": "1024x1024"
        // });

        // let client = Client::new();
        // let response = client
        //     .post("https://api.openai.com/v1/images/generations")
        //     .bearer_auth(&api_key)
        //     .json(&request_body)
        //     .send()
        //     .map_err(ImgError::Request)?;

        // let response_json: serde_json::Value = response.json().map_err(ImgError::Request)?;

        // if let Some(data) = response_json.get("data") {
        //     for image in data.as_array().unwrap() {
        //         if let Some(url) = image.get("url") {
        //             println!("Generated image URL: {}", url.as_str().unwrap());
        //             // let response = client.get(image_url).send()
        //         }
        //     }
        // } else {
        //     eprintln!("No image data returned: {:?}", response_json);
        // }

        Ok(())
    }
}
