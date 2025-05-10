use base64::{engine::general_purpose::STANDARD, Engine};
use std::io::Cursor;

use crate::img::{
    error::{ImgError, Result},
    Img,
};

impl Img {
    pub fn data_url(&mut self) -> Result<String> {
        let mut buffer = Cursor::new(Vec::new());
        self.img
            .write_to(&mut buffer, self.format)
            .map_err(|e| ImgError::Conversion {
                source: e,
                id: self.id(),
                format: self.format,
            })?;

        let encoded = STANDARD.encode(buffer.get_ref());
        let data_url = format!("data:{};base64,{}", self.format.to_mime_type(), encoded);

        Ok(data_url)
    }
}
