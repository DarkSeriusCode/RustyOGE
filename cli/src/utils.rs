use std::error::Error;
use std::process;

use clap::Command;
use colored::Colorize;

pub(crate) fn exit_with_any_error(error: Box<dyn Error>) -> ! {
    eprintln!("{}", error.to_string().red().bold());
    process::exit(1);
}

// ------------------------------------------------------------------------------------------------

/// Примесь, как либо изменяющая `Command`
pub trait CommandArgMixin {
    /// Подмешивает что-либо в `Command`
    fn mix_to_command(cmd: Command) -> Command;
}
