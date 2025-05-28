use crate::{
    commands::img::{
        context::ImgContext,
        enums::ImageInput,
        flow::{ImagesResolved, ImgFlow},
        ImgCmdError,
    },
    CliContext,
};

use super::ImgCommand;

impl ImgCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ImgCmdError> {
        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(ctx);
        }

        let img_ctx = ImgContext::new(&self, &ctx.config);

        if let Some(input) = &self.input {
            Self::resolve_images(input)?
                .apply_transformations(&img_ctx)?
                .save(&self.output)?;
        }

        Ok(())
    }

    fn resolve_images(input: &ImageInput) -> Result<ImgFlow<ImagesResolved>, ImgCmdError> {
        let flow = ImgFlow::new().resolve_images(input)?;
        Ok(flow)
    }
}
