use crate::{
    commands::img::{command::sub_commands::get::GetSubcommmand, ImgCmdError},
    img::Img,
    CliContext,
};

impl GetSubcommmand {
    pub fn handle_blurhash(&self, ctx: &CliContext, img: &mut Img) -> Result<(), ImgCmdError> {
        let hash = img.max_size(64).blurhash()?;
        ctx.service.init_clipboard().save_to_clipboard(&hash)?;

        println!("Copied blur-hash to clipboard");
        Ok(())
    }
}
