use std::process::ExitCode;

use clap::Parser;

mod errors;
use errors::Error;

mod args;
use args::{
    Cli,
    Commands,
};

mod command;

fn main() -> ExitCode {
    env_logger::init();

    log::debug!("Parsing command line arguments");
    let cli = Cli::parse();

    if cli.verbose {
        log::info!("--verbose flag set");
    }

    match main_exec(&cli) {
        Ok(()) => {
            log::info!("Exit (0)");
            ExitCode::from(0)
        }
        Err(err) => {
            log::error!("{err}");
            log::error!("Exit ({})", u8::from(&err));
            ExitCode::from(u8::from(&err))
        }
    }
}

fn main_exec(cli: &Cli) -> Result<(), Error> {
    match &cli.command {
        Commands::Generate(args) => command::generate(args, cli),
        Commands::Convert(args) => command::convert(args, cli),
    }
}
