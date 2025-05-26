use strum::IntoEnumIterator;

use crate::{
    commands::suggest::{enums::ContextType, SuggestCmdError, SuggestCommand},
    utils::spinner,
    CliContext,
};

use super::command::SuggestSubCommand;

const MAX_TOKENS: u16 = 60;
const RESPONSE_ERROR: &str = "ERROR: Unable to generate 8 valid names";

impl SuggestCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), SuggestCmdError> {
        match &self.subcommand {
            SuggestSubCommand::Alts(args) => {
                let ctx_type = Self::resolve_context_arg(&args.context, ctx)?;
                let prompt = Self::create_alts_prompt(&args.name, &args.description, &ctx_type);

                let response: String =
                    spinner("Thinking…", || -> Result<String, SuggestCmdError> {
                        let res = ctx.service.init_ai()?.prompt(&prompt, MAX_TOKENS)?;
                        Ok(res)
                    })?;

                if response == RESPONSE_ERROR {
                    return Err(SuggestCmdError::GenerationUnsuccessful);
                }
                println!("{response}");
            }
            SuggestSubCommand::Name(args) => {
                let ctx_type = Self::resolve_context_arg(&args.context, ctx)?;
                let prompt = Self::create_name_prompt(&args.description, &ctx_type);

                let response: String =
                    spinner("Thinking…", || -> Result<String, SuggestCmdError> {
                        let res = ctx.service.init_ai()?.prompt(&prompt, MAX_TOKENS)?;
                        Ok(res)
                    })?;

                if response == RESPONSE_ERROR {
                    return Err(SuggestCmdError::GenerationUnsuccessful);
                }
                println!("{response}");
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

        prompt.push_str(". Each on its own line and numbered 1-8, with no extra text.");

        if let Some(d) = desc {
            prompt.push_str(&format!(" Additional context: {}", d));
        }

        prompt.push_str(&format!(
            "If you cannot produce 8 meaningful, valid one-word names, respond with exactly:{RESPONSE_ERROR}"
        ));

        prompt
    }

    fn create_name_prompt(desc: &str, ctx_type: &Option<ContextType>) -> String {
        let context_label = ctx_type
            .as_ref()
            .map(|ct| format!("{} name", ct.to_string()))
            .unwrap_or_else(|| "name".into());

        let prompt = format!(
            "Please generate 8 unique {}s for the following description: \"{}\". \
            each on its own line and numbered 1-8, with no extra text. If you cannot produce 8 meaningful, valid one-word names, respond with exactly:{}",
            context_label, desc, RESPONSE_ERROR
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
