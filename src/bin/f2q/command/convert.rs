use f2q::{
    codes::qubits::PauliSum,
    map::JordanWigner,
    terms::Terms,
};

use super::serialize_sumrepr;
use crate::{
    cli::{
        Convert,
        Encoding,
    },
    errors::Error,
};

pub fn jordan_wigner(args: &Convert) -> Result<(), Error> {
    if !(args.input_encoding == Encoding::Fermions
        && args.output_encoding == Encoding::Qubits)
    {
        return Err(Error::CmdArgs {
            msg: "Jordan-Wigner mapping must be from fermions to qubits \
                  encoding"
                .to_string(),
        });
    }

    let in_repr = jordan_wigner::parse_input(args)?;
    let mut out_repr = PauliSum::with_capacity(in_repr.len() * 4);
    JordanWigner::new(&in_repr).add_to(&mut out_repr)?;
    serialize_sumrepr(
        &out_repr,
        args.output_file.as_deref(),
        args.output_format,
        args.pretty_print,
    )
}

mod jordan_wigner {

    use std::{
        fs::File,
        io::{
            BufRead,
            BufReader,
        },
    };

    use f2q::codes::fermions::FermiSum;

    use crate::{
        cli::{
            Convert,
            Format,
        },
        errors::Error,
    };

    pub fn parse_input(args: &Convert) -> Result<FermiSum, Error> {
        if let Some(path) = &args.input_file {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            parse_input_reader(reader, args)
        } else {
            let stdin = std::io::stdin().lock();
            let reader = BufReader::new(stdin);
            parse_input_reader(reader, args)
        }
    }

    fn parse_input_reader<R: BufRead>(
        reader: R,
        args: &Convert,
    ) -> Result<FermiSum, Error> {
        Ok(match args.input_format {
            Format::Json => serde_json::from_reader(reader)?,
            Format::Toml => {
                let mut reader = reader;
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;
                toml::from_str(&buf)?
            }
            Format::Yaml => serde_yaml::from_reader(reader)?,
        })
    }
}
