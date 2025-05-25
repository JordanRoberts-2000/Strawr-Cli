use crate::{
    commands::img::{command::sub_commands::get::GetSubcommmand, ImgError},
    img::Img,
    CliContext,
};

impl GetSubcommmand {
    pub fn handle_blur_data_url(&self, ctx: &CliContext, img: &mut Img) -> Result<(), ImgError> {
        let data_url = img
            .max_size(ctx.config.img.placeholder_size)
            .blur(ctx.config.img.placeholder_blur_intensity)
            .data_url()?;

        ctx.service.init_clipboard().save_to_clipboard(&data_url)?;
        println!("Blurred data URL successfully copied to clipboard");
        Ok(())
    }
}
