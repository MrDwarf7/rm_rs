use rm_rs::*;

use rm_rs::{Error, Result, W};

fn main() -> Result<()> {
    let cli = cli::Cli::new();
    // let action_type = logic::ActionType::from(cli.as_ref().into());

    // let logic = logic::Logic::new(action_type, cli.directory).take_action();

    // if let Err(e) = logic {
    //     return Err(Error::from(e));
    // } else {
    //     println!("Success!");
    // }

    println!("{:?}", cli);

    Ok(())
}
