use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
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

use crate::{
    args::{
        Format,
        Generate,
    },
    errors::Error,
};

pub fn fermions(args: &Generate) -> Result<(), Error> {
    if args.random {
        fermions_random(args)
    } else {
        todo!()
    }
}

fn fermions_random(args: &Generate) -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let capacity = if let Ok(cap) = usize::try_from(args.num_terms) {
        cap
    } else {
        usize::MAX
    };
    let mut repr = SumRepr::with_capacity(capacity);
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
            _ => unimplemented!(),
        }
        count += 1;
    }
    serialize_sumrepr(repr, args)?;

    Ok(())
}

pub fn qubits(args: &Generate) -> Result<(), Error> {
    if args.random {
        qubits_random(args)
    } else {
        todo!()
    }
}

fn qubits_random(args: &Generate) -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let capacity = if let Ok(cap) = usize::try_from(args.num_terms) {
        cap
    } else {
        usize::MAX
    };
    let mut repr = SumRepr::with_capacity(capacity);
    for _ in 0..args.num_terms {
        repr.add_term(
            PauliCode::new((rng.gen(), rng.gen())),
            rng.gen_range(-1.0..1.0),
        );
    }
    serialize_sumrepr(repr, args)?;

    Ok(())
}

fn serialize_sumrepr<T, K>(
    repr: SumRepr<T, K>,
    args: &Generate,
) -> Result<(), Error>
where
    T: Float,
    K: Code,
    SumRepr<T, K>: Serialize,
{
    if let Some(path) = &args.output_file {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serialize_sumrepr_to_writer(&repr, writer, args)?;
    } else {
        let stdout = std::io::stdout().lock();
        let writer = BufWriter::new(stdout);
        serialize_sumrepr_to_writer(&repr, writer, args)?;
    };
    Ok(())
}

fn serialize_sumrepr_to_writer<T, K, W>(
    repr: &SumRepr<T, K>,
    writer: BufWriter<W>,
    args: &Generate,
) -> Result<(), Error>
where
    T: Float,
    K: Code,
    SumRepr<T, K>: Serialize,
    W: Write,
{
    match args.format {
        Format::Json => {
            if args.pretty_print {
                serde_json::to_writer_pretty(writer, repr)?;
            } else {
                serde_json::to_writer(writer, repr)?;
            }
        }
        Format::Yaml => serde_yaml::to_writer(writer, repr)?,
        Format::Toml => {
            let mut writer = writer;
            let buf = if args.pretty_print {
                toml::to_string_pretty(&repr)?
            } else {
                toml::to_string(&repr)?
            };
            write!(writer, "{buf}")?;
        }
    };

    Ok(())
}
