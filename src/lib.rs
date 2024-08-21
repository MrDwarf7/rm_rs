pub mod cli;
pub mod error;
pub mod logic;
pub mod prelude;

pub use self::prelude::{Error, Result, W};

pub use logic::{ActionType, Logic};
