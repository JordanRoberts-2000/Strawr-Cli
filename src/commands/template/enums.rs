use std::fmt;

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

impl fmt::Display for TemplateCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TemplateCommand::Node => "node",
            TemplateCommand::React => "react",
            TemplateCommand::Fullstack => "fullstack",
            TemplateCommand::Rust => "rust",
            TemplateCommand::Go => "go",
            TemplateCommand::Next => "next",
        };
        write!(f, "{}", s)
    }
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

impl fmt::Display for NodeVariants {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NodeVariants::Vanilla => "vanilla",
            NodeVariants::Express => "express",
            NodeVariants::Fastify => "fastify",
        };
        write!(f, "{}", s)
    }
}
