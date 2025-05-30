use crate::{
    commands::img::{command::sub_commands::get::GetSubcommmand, ImgCmdError},
    img::Img,
    CliContext,
};

impl GetSubcommmand {
    pub fn handle_data_url(&self, ctx: &CliContext, img: &mut Img) -> Result<(), ImgCmdError> {
        let data_url = img.data_url()?;
        ctx.service.init_clipboard().save_to_clipboard(&data_url)?;

        println!("Data URL successfully copied to clipboard");
        Ok(())
    }
}
