use strum::IntoEnumIterator;

use crate::{
    commands::suggest::{enums::ContextType, SuggestCmdError, SuggestCommand},
    CliContext,
};

use super::command::SuggestSubCommand;

impl SuggestCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), SuggestCmdError> {
        match &self.subcommand {
            SuggestSubCommand::Alts(args) => {
                println!("name: {}", args.name);
                println!("description: {:?}", args.description);
                let ctx_type = Self::resolve_context_arg(&args.context, ctx)?;
                println!("context: {:?}", ctx_type);
            }
            SuggestSubCommand::Name(args) => {
                println!("description: {}", args.description);
                let ctx_type = Self::resolve_context_arg(&args.context, ctx)?;
                println!("context: {:?}", ctx_type);
            }
        }

        Ok(())
    }

    fn resolve_context_arg(
        context_arg: &Option<Option<ContextType>>,
        ctx: &CliContext,
    ) -> Result<Option<ContextType>, SuggestCmdError> {
        if let Some(context_type) = context_arg {
            match context_type {
                Some(t) => return Ok(Some(t.clone())),
                None => {
                    let options: Vec<ContextType> = ContextType::iter().collect();
                    let input = ctx
                        .service
                        .init_prompt()
                        .select(&options, "Select context type:")?;
                    return Ok(Some(input));
                }
            };
        }

        Ok(None)
    }
}
