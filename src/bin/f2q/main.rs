use std::{
    fmt::Display,
    process::ExitCode,
};

use clap::{
    Args,
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
        Code,
    },
    terms::SumRepr,
};
use num::Float;
use rand::Rng;
use serde::Serialize;

#[derive(Debug)]
enum CliError {
    Serde { msg: String },
}

impl Display for CliError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            CliError::Serde {
                msg,
            } => write!(f, "serde: {msg}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<CliError> for ExitCode {
    fn from(value: CliError) -> Self {
        ExitCode::from(match value {
            CliError::Serde {
                ..
            } => 11,
        })
    }
}

/// Fermion to qubit mappings
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "f2q")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generates Hamiltonian
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'G')]
    Generate(GenerateArgs),
    Convert,
}

#[derive(Debug, Args)]
struct GenerateArgs {
    #[arg(short, long, required = true)]
    random:       bool,
    #[arg(short, long)]
    encoding:     Encoding,
    #[arg(short, long, default_value = "json")]
    format:       Format,
    /// Pretty print the output if possible
    #[arg(short, long, default_value = "false")]
    pretty_print: bool,
    num_terms:    u64,
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

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Format {
    Json,
    Yaml,
    Toml,
}

impl std::fmt::Display for Format {
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

    match main_exec(&arg.command) {
        Ok(()) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(err)
        }
    }
}

fn main_exec(command: &Commands) -> Result<(), CliError> {
    match command {
        Commands::Generate(args) => generate_hamiltonian(args),
        Commands::Convert => todo!(),
    }
}

fn generate_hamiltonian(args: &GenerateArgs) -> Result<(), CliError> {
    match args.encoding {
        Encoding::Fermions => {
            generate_hamiltonian_fermions(args)?;
        }
        Encoding::Qubits => generate_hamiltonian_qubits(args)?,
    }
    Ok(())
}

fn generate_hamiltonian_fermions(args: &GenerateArgs) -> Result<(), CliError> {
    if args.random {
        generate_hamiltonian_fermions_random(args)
    } else {
        todo!()
    }
}

fn generate_hamiltonian_fermions_random(
    args: &GenerateArgs
) -> Result<(), CliError> {
    let mut rng = rand::thread_rng();
    let mut repr = SumRepr::new();
    let mut count = 0;
    while count < args.num_terms {
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
                );
            }
            _ => (),
        }
        count += 1;
    }

    let repr_str = serialize_sumrepr_to_string(args, repr)?;
    println!("{repr_str}");

    Ok(())
}

fn generate_hamiltonian_qubits(args: &GenerateArgs) -> Result<(), CliError> {
    if args.random {
        generate_hamiltonian_qubits_random(args)
    } else {
        todo!()
    }
}

fn generate_hamiltonian_qubits_random(
    args: &GenerateArgs
) -> Result<(), CliError> {
    let mut rng = rand::thread_rng();
    let mut repr = SumRepr::new();
    for _ in 0..args.num_terms {
        repr.add_term(
            PauliCode::new((rng.gen(), rng.gen())),
            rng.gen_range(-1.0..1.0),
        );
    }

    let repr_str = serialize_sumrepr_to_string(args, repr)?;

    println!("{repr_str}");

    Ok(())
}

fn serialize_sumrepr_to_string<T, K>(
    args: &GenerateArgs,
    repr: SumRepr<T, K>,
) -> Result<String, CliError>
where
    T: Float,
    K: Code,
    SumRepr<T, K>: Serialize,
{
    Ok(match args.format {
        Format::Json => if args.pretty_print {
            serde_json::to_string_pretty(&repr)
        } else {
            serde_json::to_string(&repr)
        }
        .map_err(|e| CliError::Serde {
            msg: e.to_string()
        })?,
        Format::Yaml => {
            serde_yaml::to_string(&repr).map_err(|e| CliError::Serde {
                msg: e.to_string(),
            })?
        }
        Format::Toml => if args.pretty_print {
            toml::to_string_pretty(&repr)
        } else {
            toml::to_string(&repr)
        }
        .map_err(|e| CliError::Serde {
            msg: e.to_string()
        })?,
    })
}
