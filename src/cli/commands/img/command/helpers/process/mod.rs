use crate::{
    cli::commands::img::{ImgCommand, ImgError},
    context::AppContext,
    services::img::Img,
};

pub mod blur;
pub mod conversion;
pub mod file_name;
pub mod output;
pub mod resize;

impl ImgCommand {
    pub fn process_image(
        &self,
        img: &mut Img,
        ctx: &AppContext,
    ) -> Result<Option<String>, ImgError> {
        self.apply_blur(img, ctx);

        self.apply_resize(img, ctx);

        self.apply_conversion(img, ctx)?;

        self.apply_file_name(img)?;

        let output = self.handle_output(img)?;

        Ok(output)
    }
}
