use std::fmt::Display;

use codes::{
    fermions::FermiCode,
    qubits::PauliCode,
    Code,
};
use terms::SumRepr;

pub type PauliSum<T> = SumRepr<T, PauliCode>;
pub type FermiSum<T> = SumRepr<T, FermiCode>;

pub mod maps;
pub mod math;
pub mod terms;

/// Basic flattened API  
pub mod prelude {
    pub use crate::{
        codes::{
            fermions::{
                An,
                Cr,
                FermiCode,
                Orbital,
                Spin,
            },
            qubits::{
                Pauli,
                PauliCode,
            },
            Code,
        },
        maps::JordanWigner,
        terms::{
            Hamil,
            SumRepr,
        },
        Terms,
    };
}

pub mod codes;

/// Convert and serialize sum of terms in various encodings
pub trait Terms<T, K>
where
    K: Code,
{
    type Error;

    /// Add terms to the supplied [`SumRepr`].
    ///
    /// # Errors
    ///
    /// Return error on failure.
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) -> Result<(), Error>;
}

#[derive(Debug, PartialEq)]
pub enum Error {
    /// Invalid index of a Pauli operator
    PauliIndex { msg: String },
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::PauliIndex {
                msg,
            } => write!(f, "PauliIndex: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

mod serialize;
pub use serialize::Encoding;
