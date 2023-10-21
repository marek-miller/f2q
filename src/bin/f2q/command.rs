use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

use f2q::{
    codes::Code,
    terms::SumRepr,
};
use num::Float;
use serde::Serialize;

use crate::{
    args::{
        Convert,
        Encoding,
        Format,
        Generate,
        Mapping,
    },
    errors::Error,
};

mod convert;
mod generate;

pub fn generate(args: &Generate) -> Result<(), Error> {
    match args.encoding {
        Encoding::Fermions => {
            generate::fermions(args)?;
        }
        Encoding::Qubits => generate::qubits(args)?,
    }
    Ok(())
}

pub fn convert(args: &Convert) -> Result<(), Error> {
    match args.mapping {
        Mapping::JordanWigner => convert::jordan_wigner(args)?,
    }
    Ok(())
}

fn serialize_sumrepr<T, K>(
    repr: &SumRepr<T, K>,
    output_path: Option<&str>,
    format: Format,
    pretty_print: bool,
) -> Result<(), Error>
where
    T: Float,
    K: Code,
    SumRepr<T, K>: Serialize,
{
    if let Some(path) = output_path {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serialize_sumrepr_to_writer(repr, writer, format, pretty_print)
    } else {
        let stdout = std::io::stdout().lock();
        let writer = BufWriter::new(stdout);
        serialize_sumrepr_to_writer(repr, writer, format, pretty_print)
    }
}

fn serialize_sumrepr_to_writer<T, K, W>(
    repr: &SumRepr<T, K>,
    writer: BufWriter<W>,
    format: Format,
    pretty_print: bool,
) -> Result<(), Error>
where
    T: Float,
    K: Code,
    SumRepr<T, K>: Serialize,
    W: Write,
{
    match format {
        Format::Json => {
            if pretty_print {
                serde_json::to_writer_pretty(writer, repr)?;
            } else {
                serde_json::to_writer(writer, repr)?;
            }
        }
        Format::Yaml => serde_yaml::to_writer(writer, repr)?,
        Format::Toml => {
            let mut writer = writer;
            let buf = if pretty_print {
                toml::to_string_pretty(&repr)?
            } else {
                toml::to_string(&repr)?
            };
            write!(writer, "{buf}")?;
        }
    };

    Ok(())
}
