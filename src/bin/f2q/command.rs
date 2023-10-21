use crate::{
    args::{
        Cli,
        Convert,
        Encoding,
        Generate,
        Mapping,
    },
    errors::Error,
};

mod convert;
mod generate;

pub fn generate(
    args: &Generate,
    _cli: &Cli,
) -> Result<(), Error> {
    match args.encoding {
        Encoding::Fermions => {
            generate::fermions(args)?;
        }
        Encoding::Qubits => generate::qubits(args)?,
    }
    Ok(())
}

pub fn convert(
    args: &Convert,
    cli: &Cli,
) -> Result<(), Error> {
    match args.mapping {
        Mapping::JordanWigner => convert::jordan_wigner(args, cli)?,
    }

    Ok(())
}
