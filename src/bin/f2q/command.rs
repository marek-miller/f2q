use crate::{
    args::{
        Convert,
        Encoding,
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
