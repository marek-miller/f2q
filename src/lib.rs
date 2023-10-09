use std::{
    fmt::Display,
    hash::Hash,
};

use terms::SumRepr;

pub mod maps;
pub mod qubit;
pub mod sec;
pub mod terms;

/// Representation of Hermitian operators
pub trait Code: Copy + Clone + Eq + Hash + Default {}

/// Convert and serialize sum of terms in various encodings
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
