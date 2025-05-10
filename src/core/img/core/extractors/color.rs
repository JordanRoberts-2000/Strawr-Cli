use crate::img::{
    error::{ImgError, Result},
    Img,
};
use color_thief::{get_palette, ColorFormat};

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Img {
    pub fn color(&mut self) -> Result<Rgb> {
        let rgb_pixels = self.img.to_rgb8().into_raw();

        let palette =
            get_palette(&rgb_pixels, ColorFormat::Rgb, 5, 5).map_err(ImgError::GetColors)?;

        let dominant_color = palette.get(0).ok_or_else(|| ImgError::EmptyPalette)?;

        Ok(Rgb {
            r: dominant_color.r,
            g: dominant_color.g,
            b: dominant_color.b,
        })
    }
}
