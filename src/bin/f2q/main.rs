use std::{
    fmt::Display,
    fs::File,
    io::{
        BufRead,
        BufReader,
        BufWriter,
        Write,
    },
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
            FermiSum,
            Orbital,
        },
        qubits::{
            PauliCode,
            PauliSum,
        },
        Code,
    },
    maps::JordanWigner,
    terms::{
        SumRepr,
        Terms,
    },
};
use num::Float;
use rand::Rng;
use serde::Serialize;

#[derive(Debug)]
enum CliError {
    CmdArgs { msg: String },
    File { msg: String },
    Serde { msg: String },
    F2q(f2q::Error),
}

impl Display for CliError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            CliError::CmdArgs {
                msg,
            } => write!(f, "[command line] {msg}"),
            CliError::File {
                msg,
            } => write!(f, "[file] {msg}"),
            CliError::F2q(e) => write!(f, "f2q: {e}"),
            CliError::Serde {
                msg,
            } => write!(f, "[serde] {msg}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<&CliError> for u8 {
    fn from(value: &CliError) -> Self {
        match value {
            CliError::CmdArgs {
                ..
            } => 1,
            CliError::File {
                ..
            } => 2,
            CliError::F2q(_) => 3,
            CliError::Serde {
                ..
            } => 11,
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(value: std::io::Error) -> Self {
        Self::File {
            msg: format!("{value}"),
        }
    }
}

impl From<f2q::Error> for CliError {
    fn from(value: f2q::Error) -> Self {
        Self::F2q(value)
    }
}

macro_rules! impl_serde_error {
    ($($Typ:ty)* ) => {
        $(
            impl From<$Typ> for CliError {
                fn from(value: $Typ) -> Self {
                    Self::Serde { msg: format!("{value}") }
                }
            }
        )*
    };
}

impl_serde_error!(serde_json::Error);
impl_serde_error!(serde_yaml::Error);
impl_serde_error!(toml::ser::Error toml::de::Error);

/// Fermion to qubit mappings
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "f2q")]
struct Cli {
    #[arg(long, short, default_value = "false")]
    verbose: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generates Hamiltonian
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'G')]
    Generate(ArgsGenerate),
    #[command(short_flag = 'C')]
    Convert(ArgsConvert),
}

#[derive(Debug, Args)]
struct ArgsGenerate {
    #[arg(short, long, required = true)]
    random:            bool,
    #[arg(short, long, default_value = "qubits")]
    encoding:          Encoding,
    #[arg(short, long, default_value = "json")]
    format:            Format,
    /// Pretty print the output if possible
    #[arg(short, long, default_value = "false")]
    pretty_print:      bool,
    num_terms:         u64,
    #[arg(long, default_value = "63")]
    max_orbital_index: u32,
}

#[derive(Debug, Args)]
struct ArgsConvert {
    /// STDIN, if not specified
    #[arg(long, short)]
    input_file:      Option<String>,
    #[arg(long, default_value = "fermions")]
    input_encoding:  Encoding,
    #[arg(long, default_value = "json")]
    input_format:    Format,
    /// STDOUT, if not specified
    #[arg(long, short)]
    output_file:     Option<String>,
    #[arg(long, default_value = "qubits")]
    output_encoding: Encoding,
    #[arg(long, default_value = "json")]
    output_format:   Format,
    /// Pretty print the output if possible
    #[arg(short, long, default_value = "false")]
    pretty_print:    bool,
    #[arg(short, long)]
    mapping:         Mapping,
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
enum Mapping {
    JordanWigner,
}

impl std::fmt::Display for Mapping {
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

fn main_exec(cli: &Cli) -> Result<(), CliError> {
    match &cli.command {
        Commands::Generate(args) => cmd_generate(args, cli),
        Commands::Convert(args) => cmd_convert(args, cli),
    }
}

fn cmd_generate(
    args: &ArgsGenerate,
    _cli: &Cli,
) -> Result<(), CliError> {
    match args.encoding {
        Encoding::Fermions => {
            cmd_generate_fermions(args)?;
        }
        Encoding::Qubits => cmd_generate_qubits(args)?,
    }
    Ok(())
}

fn cmd_generate_fermions(args: &ArgsGenerate) -> Result<(), CliError> {
    if args.random {
        cmd_generate_fermions_random(args)
    } else {
        todo!()
    }
}

fn cmd_generate_fermions_random(args: &ArgsGenerate) -> Result<(), CliError> {
    let mut rng = rand::thread_rng();
    let mut repr = SumRepr::new();
    let mut count = 0;
    while count < args.num_terms {
        let category = rng.gen_range(0..=2);
        match category {
            0 => repr.add_term(FermiCode::Offset, rng.gen_range(-1.0..1.0)),
            1 => {
                let max_val = args.max_orbital_index;
                let p = rng.gen_range(0..max_val - 1);
                let q = rng.gen_range(p + 1..=max_val);
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
                let max_val = args.max_orbital_index;
                let p = rng.gen_range(0..max_val - 2);
                let q = rng.gen_range(p + 1..=max_val);
                let s = rng.gen_range(p..max_val - 1);
                let r = rng.gen_range(s + 1..=max_val);

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

    let repr_str = serialize_sumrepr_to_string(repr, args)?;
    println!("{repr_str}");

    Ok(())
}

fn cmd_generate_qubits(args: &ArgsGenerate) -> Result<(), CliError> {
    if args.random {
        cmd_generate_qubits_random(args)
    } else {
        todo!()
    }
}

fn cmd_generate_qubits_random(args: &ArgsGenerate) -> Result<(), CliError> {
    let mut rng = rand::thread_rng();
    let mut repr = SumRepr::new();
    for _ in 0..args.num_terms {
        repr.add_term(
            PauliCode::new((rng.gen(), rng.gen())),
            rng.gen_range(-1.0..1.0),
        );
    }

    let repr_str = serialize_sumrepr_to_string(repr, args)?;

    println!("{repr_str}");

    Ok(())
}

fn serialize_sumrepr_to_string<T, K>(
    repr: SumRepr<T, K>,
    args: &ArgsGenerate,
) -> Result<String, CliError>
where
    T: Float,
    K: Code,
    SumRepr<T, K>: Serialize,
{
    Ok(match args.format {
        Format::Json => {
            if args.pretty_print {
                serde_json::to_string_pretty(&repr)
            } else {
                serde_json::to_string(&repr)
            }?
        }
        Format::Yaml => serde_yaml::to_string(&repr)?,
        Format::Toml => {
            if args.pretty_print {
                toml::to_string_pretty(&repr)
            } else {
                toml::to_string(&repr)
            }?
        }
    })
}

fn cmd_convert(
    args: &ArgsConvert,
    cli: &Cli,
) -> Result<(), CliError> {
    match args.mapping {
        Mapping::JordanWigner => cmd_convert_jordan_wigner(args, cli)?,
    }

    Ok(())
}

fn cmd_convert_jordan_wigner(
    args: &ArgsConvert,
    cli: &Cli,
) -> Result<(), CliError> {
    if !(args.input_encoding == Encoding::Fermions
        && args.output_encoding == Encoding::Qubits)
    {
        return Err(CliError::CmdArgs {
            msg: "Jordan-Wigner mapping must be from fermions to qubits \
                  encoding"
                .to_string(),
        });
    }

    let in_repr = cmd_convert_jordan_wigner_parse_input(args, cli)?;
    let mut out_repr = PauliSum::new();
    JordanWigner::new(&in_repr).add_to(&mut out_repr)?;
    cmd_convert_jordan_wigner_gen_ouput(&out_repr, args, cli)
}

fn cmd_convert_jordan_wigner_parse_input(
    args: &ArgsConvert,
    cli: &Cli,
) -> Result<FermiSum<f64>, CliError> {
    if let Some(path) = &args.input_file {
        let reader = BufReader::new(File::open(path)?);
        cmd_convert_jordan_wigner_parse_input_reader(reader, args, cli)
    } else {
        cmd_convert_jordan_wigner_parse_input_reader(
            std::io::stdin().lock(),
            args,
            cli,
        )
    }
}

fn cmd_convert_jordan_wigner_parse_input_reader<R: BufRead>(
    reader: R,
    args: &ArgsConvert,
    _cli: &Cli,
) -> Result<FermiSum<f64>, CliError> {
    Ok(match args.input_format {
        Format::Json => serde_json::from_reader(reader)?,
        Format::Toml => {
            let mut reader = reader;
            let mut buf = String::new();
            reader.read_to_string(&mut buf).unwrap();
            toml::from_str(&buf)?
        }
        Format::Yaml => serde_yaml::from_reader(reader)?,
    })
}

fn cmd_convert_jordan_wigner_gen_ouput(
    out_repr: &PauliSum<f64>,
    args: &ArgsConvert,
    cli: &Cli,
) -> Result<(), CliError> {
    if let Some(path) = &args.output_file {
        let writer =
            BufWriter::new(File::create(path).map_err(|e| CliError::File {
                msg: format!("{e}"),
            })?);
        cmd_convert_jordan_wigner_parse_gen_output_writer(
            out_repr, writer, args, cli,
        )
    } else {
        cmd_convert_jordan_wigner_parse_gen_output_writer(
            out_repr,
            BufWriter::new(std::io::stdout().lock()),
            args,
            cli,
        )
    }
}

fn cmd_convert_jordan_wigner_parse_gen_output_writer<W: Write>(
    out_repr: &PauliSum<f64>,
    writer: BufWriter<W>,
    args: &ArgsConvert,
    _cli: &Cli,
) -> Result<(), CliError> {
    match args.input_format {
        Format::Json => if args.pretty_print {
            serde_json::to_writer_pretty(writer, &out_repr)
        } else {
            serde_json::to_writer(writer, &out_repr)
        }
        .map_err(CliError::from),
        Format::Toml => {
            let mut writer = writer;
            let repr = if args.pretty_print {
                toml::to_string_pretty(&out_repr)
            } else {
                toml::to_string(&out_repr)
            }
            .map_err(CliError::from)?;

            write!(writer, "{repr}").map_err(CliError::from)
        }
        Format::Yaml => {
            serde_yaml::to_writer(writer, &out_repr).map_err(CliError::from)
        }
    }
}
