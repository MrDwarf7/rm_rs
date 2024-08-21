use crate::{prelude::*, ActionType};
use clap::{FromArgMatches, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use walkdir::DirEntry;

#[derive(Debug, ValueEnum, Clone)]
pub enum TypeOfPath {
    File,
    Directory,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum TypeOfAction {
    Force,
    Recursive,
    Interactive,
    Directories,
}

impl From<&str> for TypeOfPath {
    fn from(path: &str) -> Self {
        Self::from_path(path)
    }
}

impl From<ActionType> for String {
    fn from(action: ActionType) -> Self {
        match action {
            ActionType::Force(action) => action,
            ActionType::Recursive(action) => action,
            ActionType::Interactive(action) => action,
            ActionType::Directories(action) => action,
        }
    }
}

impl TypeOfPath {
    fn from_path(path: &str) -> Self {
        if PathBuf::from(path).is_dir() {
            Self::Directory
        } else {
            Self::File
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "rm but it's for all platforms!",
    about = "It's *nix rm for all platforms",
    long_about = None,
    author = "Blake B./MrDwarf7@github.com",
    version,
    arg_required_else_help(true),
    allow_missing_positional(true)
)]
pub struct Cli {
    #[arg(
        value_enum,
        name = "path",
        required = true,
        value_name = "GIVEN_PATH",
        help = "The path to the thing to delete/remove"
    )]
    pub path: TypeOfPath,

    #[arg(
        name = "actions",
        required = false,
        value_name = "ACTIONS",
        help = "The action to take on the path"
    )]
    pub actions: Vec<TypeOfAction>,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
