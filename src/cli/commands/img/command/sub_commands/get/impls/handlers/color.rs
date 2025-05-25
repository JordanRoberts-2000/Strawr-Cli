use crate::{
    commands::img::{command::sub_commands::get::GetSubcommmand, enums::ColorOutput, ImgError},
    img::Img,
    CliContext,
};

impl GetSubcommmand {
    pub fn handle_color(&self, ctx: &CliContext, img: &mut Img) -> Result<(), ImgError> {
        match &ctx.config.img.get.default_color_output {
            ColorOutput::Rgb => {
                let rgb = img.color()?.rgb();
                ctx.service.init_clipboard().save_to_clipboard(&rgb)?;
                println!("Copied Rgb to clipboard");
            }
            ColorOutput::Hex => {
                let hex = img.color()?.hex();
                ctx.service.init_clipboard().save_to_clipboard(&hex)?;
                println!("Copied hex to clipboard");
            }
        }

        Ok(())
    }
}
