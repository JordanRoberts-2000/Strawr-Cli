use crate::{
    commands::img::{command::sub_commands::get::GetSubcommmand, ImgError},
    img::Img,
    CliContext,
};

impl GetSubcommmand {
    pub fn handle_alt(&self, ctx: &CliContext, img: &mut Img) -> Result<(), ImgError> {
        let data_url = img.max_size(400).webp()?.data_url()?;
        let description = ctx.service.init_ai()?.get_image_description(&data_url)?;

        ctx.service
            .init_clipboard()
            .save_to_clipboard(&description)?;

        println!("Alt Text: '{}'", description);
        Ok(())
    }
}
