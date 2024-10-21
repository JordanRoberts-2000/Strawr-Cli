use args::{
    edit::open_editor_for_template, go::handle_go, next::handle_next, node::handle_node,
    react::handle_react, rust::handle_rust,
};
use enums::TemplateCommand;

mod args;
pub mod command;
mod enums;

pub fn handle_template(matches: &clap::ArgMatches) {
    if matches.get_flag("edit") {
        open_editor_for_template(matches);
    } else {
        if let Some(template_type) = matches.get_one::<TemplateCommand>("template_type") {
            match template_type {
                TemplateCommand::React => handle_react(),
                TemplateCommand::Node => handle_node(matches),
                TemplateCommand::Fullstack => println!("Generating Fullstack template..."),
                TemplateCommand::Rust => handle_rust(),
                TemplateCommand::Go => handle_go(),
                TemplateCommand::Next => handle_next(),
            }
        }
    }
}
