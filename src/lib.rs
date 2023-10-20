use std::{
    fmt::Display,
    hash::Hash,
};

use fermions::Fermions;
use qubits::PauliCode;
use terms::SumRepr;

pub type PauliSum<T> = SumRepr<T, PauliCode>;
pub type FermiSum<T> = SumRepr<T, Fermions>;

pub mod fermions;
pub mod maps;
pub mod math;
pub mod qubits;
pub mod terms;

/// Basic flattened API  
pub mod prelude {
    pub use crate::{
        fermions::{
            An,
            Cr,
            Fermions,
            Orbital,
            Spin,
        },
        maps::JordanWigner,
        qubits::{
            Pauli,
            PauliCode,
        },
        terms::{
            Hamil,
            SumRepr,
        },
        Code,
        Terms,
    };
}

/// Sum terms of a Hamiltonian
pub trait Code: Copy + Clone + Eq + Hash + Default {}

impl Code for u64 {}
impl Code for PauliCode {}
impl Code for Fermions {}

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
