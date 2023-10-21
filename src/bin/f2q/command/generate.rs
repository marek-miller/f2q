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

pub fn fermions_random(args: &Generate) -> Result<(), Error> {
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

pub fn qubits(args: &Generate) -> Result<(), Error> {
    if args.random {
        qubits_random(args)
    } else {
        todo!()
    }
}

pub fn qubits_random(args: &Generate) -> Result<(), Error> {
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
    args: &Generate,
) -> Result<String, Error>
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
