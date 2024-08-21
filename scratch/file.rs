// #[derive(Debug, ValueEnum, Clone)]
// #[clap(name = "TypeOfPath", rename_all = "upper")]
// pub enum TypeOfPath {
//     File,
//     Directory,
// }

// impl From<&str> for TypeOfPath {
//     fn from(path: &str) -> Self {
//         Self::from_path(path)
//     }
// }

// impl TypeOfPath {
//     fn from_path(path: &str) -> Self {
//         if PathBuf::from(path).is_dir() {
//             Self::Directory
//         } else {
//             Self::File
//         }
//     }
// }

// #[derive(Debug, Clone, Subcommand)]
// pub enum ActionType {
//     Force(String),
//     Recursive(String),
//     Interactive(String),
//     Directories(String),
// }

// #[derive(Parser, Debug)]
// #[command(
//     name = "rm but it's for all platforms!",
//     about = "It's *nix rm for all platforms",
//     long_about = None,
//     author = "Blake B./MrDwarf7@github.com",
//     version,
//     arg_required_else_help(true),
//     allow_missing_positional(true)
// )]
// pub struct Cli {
//     // Path To thing
//     #[arg(
//         value_enum,
//         name = "path",
//         required = true,
//         value_name = "GIVEN_PATH",
//         help = "The path to the thing to delete/remove"
//     )]
//     pub path: TypeOfPath,

//     ///  Action to take
//     #[arg(
//         name = "actions",
//         required = false,
//         value_name = "ACTIONS",
//         help = "The set of actions to take on the provided path"
//     )]
//     pub actions: Vec<ActionType>,
// }
