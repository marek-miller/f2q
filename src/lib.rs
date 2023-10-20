use std::{
    fmt::Display,
    hash::Hash,
};

use prelude::{
    Fermions,
    PauliCode,
};
use terms::SumRepr;

pub mod maps;
pub mod math;
pub mod qubit;
pub mod secq;
pub mod terms;

mod serialize;

/// Basic flattened API  
pub mod prelude {
    pub use crate::{
        maps::JordanWigner,
        qubit::{
            Pauli,
            PauliCode,
        },
        secq::{
            An,
            Cr,
            Fermions,
            Orbital,
            Spin,
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
