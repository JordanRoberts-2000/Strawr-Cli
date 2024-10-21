use clap::ArgMatches;
use std::{env, path::PathBuf};

use crate::{
    commands::template::enums::{NodeVariants, TemplateCommand},
    utils::run_command,
};

pub fn open_editor_for_template(matches: &ArgMatches) {
    let template_type = matches
        .get_one::<TemplateCommand>("template_type")
        .expect("template_type is required");
    let variant = matches.get_one::<NodeVariants>("variant");
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let command = if let Some(variant) = variant {
        format!(
            "code {}/assets/templates/{}/{}",
            project_root.display(),
            template_type.to_string().to_lowercase(),
            variant.to_string().to_lowercase()
        )
    } else {
        format!(
            "code {}/assets/templates/{}",
            project_root.display(),
            template_type.to_string().to_lowercase()
        )
    };

    run_command(&command).expect("Command failed");
}
