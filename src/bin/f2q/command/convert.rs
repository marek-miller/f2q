use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        BufWriter,
        Write,
    },
};

use f2q::{
    codes::{
        fermions::FermiSum,
        qubits::PauliSum,
    },
    maps::JordanWigner,
    terms::Terms,
};

use crate::{
    args::{
        Cli,
        Convert,
        Encoding,
        Format,
    },
    errors::Error,
};

pub fn jordan_wigner(
    args: &Convert,
    cli: &Cli,
) -> Result<(), Error> {
    if !(args.input_encoding == Encoding::Fermions
        && args.output_encoding == Encoding::Qubits)
    {
        return Err(Error::CmdArgs {
            msg: "Jordan-Wigner mapping must be from fermions to qubits \
                  encoding"
                .to_string(),
        });
    }

    let in_repr = jordan_wigner_parse_input(args, cli)?;
    let mut out_repr = PauliSum::new();
    JordanWigner::new(&in_repr).add_to(&mut out_repr)?;
    jordan_wigner_gen_ouput(&out_repr, args, cli)
}

fn jordan_wigner_parse_input(
    args: &Convert,
    cli: &Cli,
) -> Result<FermiSum<f64>, Error> {
    if let Some(path) = &args.input_file {
        let reader = BufReader::new(File::open(path)?);
        jordan_wigner_parse_input_reader(reader, args, cli)
    } else {
        jordan_wigner_parse_input_reader(std::io::stdin().lock(), args, cli)
    }
}

fn jordan_wigner_parse_input_reader<R: BufRead>(
    reader: R,
    args: &Convert,
    _cli: &Cli,
) -> Result<FermiSum<f64>, Error> {
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

fn jordan_wigner_gen_ouput(
    out_repr: &PauliSum<f64>,
    args: &Convert,
    cli: &Cli,
) -> Result<(), Error> {
    if let Some(path) = &args.output_file {
        let writer =
            BufWriter::new(File::create(path).map_err(|e| Error::File {
                msg: format!("{e}"),
            })?);
        jordan_wigner_parse_gen_output_writer(out_repr, writer, args, cli)
    } else {
        jordan_wigner_parse_gen_output_writer(
            out_repr,
            BufWriter::new(std::io::stdout().lock()),
            args,
            cli,
        )
    }
}

fn jordan_wigner_parse_gen_output_writer<W: Write>(
    out_repr: &PauliSum<f64>,
    writer: BufWriter<W>,
    args: &Convert,
    _cli: &Cli,
) -> Result<(), Error> {
    match args.input_format {
        Format::Json => if args.pretty_print {
            serde_json::to_writer_pretty(writer, &out_repr)
        } else {
            serde_json::to_writer(writer, &out_repr)
        }
        .map_err(Error::from),
        Format::Toml => {
            let mut writer = writer;
            let repr = if args.pretty_print {
                toml::to_string_pretty(&out_repr)
            } else {
                toml::to_string(&out_repr)
            }
            .map_err(Error::from)?;

            write!(writer, "{repr}").map_err(Error::from)
        }
        Format::Yaml => {
            serde_yaml::to_writer(writer, &out_repr).map_err(Error::from)
        }
    }
}
