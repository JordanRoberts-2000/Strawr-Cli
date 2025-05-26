use strum::IntoEnumIterator;

use crate::{
    commands::suggest::{enums::ContextType, SuggestCmdError, SuggestCommand},
    CliContext,
};

use super::command::SuggestSubCommand;

const MAX_TOKENS: u16 = 60;

impl SuggestCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), SuggestCmdError> {
        match &self.subcommand {
            SuggestSubCommand::Alts(args) => {
                let ctx_type = Self::resolve_context_arg(&args.context, ctx)?;
                let prompt = Self::create_alts_prompt(&args.name, &args.description, &ctx_type);
                let response = ctx.service.init_ai()?.prompt(&prompt, MAX_TOKENS)?;
                println!("{response}");
                // let suggestions = Self::parse_prompt_response?;
                // Self::display_suggestions(&suggestions);
            }
            SuggestSubCommand::Name(args) => {
                let ctx_type = Self::resolve_context_arg(&args.context, ctx)?;
                let prompt = Self::create_name_prompt(&args.description, &ctx_type);
                let response = ctx.service.init_ai()?.prompt(&prompt, MAX_TOKENS)?;
                println!("{response}");
                // let suggestions = Self::parse_prompt_response?;
                // Self::display_suggestions(&suggestions);
            }
        }

        Ok(())
    }

    fn create_alts_prompt(
        name: &str,
        desc: &Option<String>,
        ctx_type: &Option<ContextType>,
    ) -> String {
        let context_label = ctx_type
            .as_ref()
            .map(|ct| format!("{} name", ct.to_string()))
            .unwrap_or_else(|| "name".into());

        let mut prompt = format!(
            "Please provide exactly 8 one-word alternative {}s for \"{}\"",
            context_label, name
        );

        prompt.push_str(". Each on its own line, with no extra text.");

        if let Some(d) = desc {
            prompt.push_str(&format!(" Additional context: {}", d));
        }

        prompt
    }

    fn create_name_prompt(desc: &str, ctx_type: &Option<ContextType>) -> String {
        let context_label = ctx_type
            .as_ref()
            .map(|ct| format!("{} name", ct.to_string()))
            .unwrap_or_else(|| "name".into());

        let prompt = format!(
            "Please generate 8 unique {}s for the following description: \"{}\". \
          each on its own line, with no extra text.",
            context_label, desc
        );

        prompt
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
