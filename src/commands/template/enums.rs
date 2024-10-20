use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TemplateCommand {
    React,
    Node,
    Rust,
    Go,
    Fullstack,
    Next,
}

pub enum rustVariants {
    Async,
    Webserver,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum NodeVariants {
    Vanilla,
    Express,
    Fastify,
}
