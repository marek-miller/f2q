use std::{
    fmt::Display,
    hash::Hash,
};

mod hamil;
mod mapping;
mod pauli;

#[cfg(test)]
mod tests;

pub mod q2;
pub mod repr;

pub use hamil::Hamil;
pub use pauli::{
    Pauli,
    PauliCode,
};
#[doc(inline)]
pub use repr::SumRepr;

pub type PauliHamil<T> = Hamil<T, PauliCode>;
pub type PauliSum<T> = SumRepr<T, PauliCode>;

/// Representation of hermitian operators
pub trait Code: Clone + Eq + Hash + Default {}

/// Convert and serialize sum terms in various encodings
pub trait Terms<T, K>
where
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    );
}

#[derive(Debug, PartialEq)]
pub enum Error {
    CodeIndex,
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::CodeIndex => write!(f, "CodeValue"),
        }
    }
}

impl std::error::Error for Error {}
