mod core;

mod executers {
    mod command;
    mod sub_commands {
        mod create;
        mod delete;
        mod edit;
        mod rename;
    }
}

mod helpers {
    mod handle_subcommands;
    mod no_input;
    mod resolve_inputs;
    mod stack_flags;
}

mod operations {
    mod inject_files;
    mod template {
        mod create;
        mod delete;
        mod select;
    }
    mod variant {
        mod select;
    }
}

pub(crate) use core::TemplateController;
