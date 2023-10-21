use std::{
    process::ExitCode,
    sync::OnceLock,
};

use clap::Parser;

static CLI: OnceLock<Cli> = OnceLock::new();

fn main() -> ExitCode {
    env_logger::init();

    log::debug!("Parsing command line arguments");
    let cli = CLI.get_or_init(Cli::parse);

    if cli.verbose {
        log::debug!("--verbose flag set");
    }

    match main_exec(cli) {
        Ok(()) => {
            log_or_eprintln_if_verbose("Done.");
            log::info!("Exit (0)");
            ExitCode::from(0)
        }
        Err(err) => {
            log::error!("{err}");
            let exit_code = u8::from(&err);
            log::error!("Exit ({})", exit_code);
            ExitCode::from(exit_code)
        }
    }
}

fn main_exec(cli: &Cli) -> Result<(), Error> {
    match &cli.command {
        Commands::Generate(args) => command::generate(args),
        Commands::Convert(args) => command::convert(args),
    }
}

mod errors;
use errors::Error;

mod args;
use args::{
    Cli,
    Commands,
};

mod command;

fn log_or_eprintln_if_verbose(msg: &str) {
    if let Some(cli) = CLI.get() {
        if cli.verbose {
            eprintln!("{msg}");
        } else {
            log::info!("{msg}");
        }
    }
}
