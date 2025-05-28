use crate::{
    commands::img::{command::sub_commands::get::GetSubcommmand, ImgCmdError},
    img::Img,
    CliContext,
};

impl GetSubcommmand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgCmdError> {
        let mut img = Img::open(&self.path)?;

        match true {
            _ if self.color => self.handle_color(ctx, &mut img)?,
            _ if self.data_url => self.handle_data_url(ctx, &mut img)?,
            _ if self.blur_data_url => self.handle_blur_data_url(ctx, &mut img)?,
            _ if self.alt => self.handle_alt(ctx, &mut img)?,
            _ if self.hash => self.handle_blurhash(ctx, &mut img)?,
            _ => self.handle_default(&mut img),
        }

        Ok(())
    }
}
