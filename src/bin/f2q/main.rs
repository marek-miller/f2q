use std::{
    fmt::Display,
    process::ExitCode,
};

use clap::{
    Parser,
    Subcommand,
    ValueEnum,
};
use f2q::{
    codes::{
        fermions::{
            An,
            Cr,
            FermiCode,
            Orbital,
        },
        qubits::PauliCode,
    },
    terms::SumRepr,
};
use rand::Rng;

#[derive(Debug)]
enum CliError {
    CmdLineArg { msg: String },
    Unsupported { msg: String },
}

impl Display for CliError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            CliError::CmdLineArg {
                msg,
            } => write!(f, "error: {}", msg),
            CliError::Unsupported {
                msg,
            } => write!(f, "error: unsupported {}", msg),
        }
    }
}

impl From<CliError> for ExitCode {
    fn from(value: CliError) -> Self {
        ExitCode::from(match value {
            CliError::CmdLineArg {
                ..
            } => 1,
            CliError::Unsupported {
                ..
            } => 2,
        })
    }
}

impl std::error::Error for CliError {}

/// Fermion to qubit mappings
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "f2q")]
#[command(about = "Fermion to qubit mappings", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generates Hamiltonian
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'G')]
    Generate {
        #[arg(short, long)]
        random:    bool,
        #[arg(short, long)]
        encoding:  Encoding,
        num_terms: u64,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Encoding {
    Qubits,
    Fermions,
}

impl std::fmt::Display for Encoding {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

fn main() -> ExitCode {
    let arg = Cli::parse();

    match main_exec(arg.command) {
        Ok(()) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(err)
        }
    }
}

fn main_exec(command: Commands) -> Result<(), CliError> {
    match command {
        Commands::Generate {
            random,
            encoding: _,
            num_terms: _,
        } => {
            if !random {
                Err(CliError::Unsupported {
                    msg: "For now, only random generation is supported. \
                          Provide the relevant flag: --random."
                        .to_string(),
                })?;
            }
            generate_hamiltonian(command)
        }
    }
}

fn generate_hamiltonian(command: Commands) -> Result<(), CliError> {
    let Commands::Generate {
        random: _,
        encoding,
        num_terms: _,
    } = command
    else {
        panic!("wrong command passed. This is a bug.");
    };

    match encoding {
        Encoding::Fermions => {
            generate_hamiltonian_fermions(command)?;
        }
        Encoding::Qubits => generate_hamiltonian_qubits(command)?,
        _ => panic!("no encoding"),
    }
    Ok(())
}

fn generate_hamiltonian_fermions(command: Commands) -> Result<(), CliError> {
    let Commands::Generate {
        random: _,
        encoding: _,
        num_terms,
    } = command
    else {
        panic!("wrong command passed. This is a bug.");
    };

    let mut rng = rand::thread_rng();
    let mut repr = SumRepr::new();
    let mut count = 0;
    while count < num_terms {
        let category = rng.gen_range(0..=2);
        match category {
            0 => repr.add_term(FermiCode::Offset, rng.gen_range(-1.0..1.0)),
            1 => {
                let p = rng.gen_range(0..u32::MAX - 1);
                let q = rng.gen_range(p + 1..=u32::MAX);
                repr.add_term(
                    FermiCode::one_electron(
                        Cr(Orbital::from_index(p)),
                        An(Orbital::from_index(q)),
                    )
                    .unwrap(),
                    rng.gen_range(-1.0..1.0),
                );
            }
            2 => {
                let p = rng.gen_range(0..u32::MAX - 2);
                let q = rng.gen_range(p + 1..=u32::MAX);
                let s = rng.gen_range(p..u32::MAX - 1);
                let r = rng.gen_range(s + 1..=u32::MAX);

                repr.add_term(
                    FermiCode::two_electron(
                        (
                            Cr(Orbital::from_index(p)),
                            Cr(Orbital::from_index(q)),
                        ),
                        (
                            An(Orbital::from_index(r)),
                            An(Orbital::from_index(s)),
                        ),
                    )
                    .unwrap(),
                    rng.gen_range(-1.0..1.0),
                )
            }
            _ => (),
        }
        count += 1;
    }

    let json = serde_json::to_string_pretty(&repr).unwrap();
    println!("{json}");

    Ok(())
}

fn generate_hamiltonian_qubits(command: Commands) -> Result<(), CliError> {
    let Commands::Generate {
        random: _,
        encoding: _,
        num_terms,
    } = command
    else {
        panic!("wrong command passed. This is a bug.");
    };

    let mut rng = rand::thread_rng();
    let mut repr = SumRepr::new();
    for _ in 0..num_terms {
        repr.add_term(
            PauliCode::new((rng.gen(), rng.gen())),
            rng.gen_range(-1.0..1.0),
        )
    }

    let json = serde_json::to_string_pretty(&repr).unwrap();
    println!("{json}");

    Ok(())
}
