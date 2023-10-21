use std::{
    process::ExitCode,
    sync::OnceLock,
};

use clap::Parser;

static CLI: OnceLock<Cli> = OnceLock::new();

fn log_or_eprintln_if_verbose(msg: &str) {
    if let Some(cli) = CLI.get() {
        if cli.verbose {
            eprintln!("{msg}");
        } else {
            log::info!("{msg}");
        }
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

fn main() -> ExitCode {
    env_logger::init();

    log::debug!("Parsing command line arguments");
    let cli = CLI.get_or_init(Cli::parse);

    if cli.verbose {
        log::debug!("--verbose flag set");
    }

    if let Err(e) = main_exec(cli) {
        log::error!("{e}");
        let exit_code = u8::from(&e);
        log::error!("Exit ({})", exit_code);
        ExitCode::from(exit_code)
    } else {
        log_or_eprintln_if_verbose("Done.");
        log::info!("Exit (0)");
        ExitCode::from(0)
    }
}

fn main_exec(cli: &Cli) -> Result<(), Error> {
    match &cli.command {
        Commands::Generate(args) => command::generate(args),
        Commands::Convert(args) => command::convert(args),
    }
}
