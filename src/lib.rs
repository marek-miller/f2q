use std::{
    fmt::Display,
    hash::Hash,
};

use terms::SumRepr;

pub mod maps;
pub mod qubit;
pub mod secnd;
pub mod terms;

pub mod prelude {
    pub use crate::{
        maps::JordanWigner,
        qubit::{
            Pauli,
            PauliCode,
            PauliIter,
        },
        secnd::{
            Integral,
            Orbital,
            Spin,
        },
        terms::{
            Hamil,
            HeapRepr,
            StackRepr,
            SumRepr,
        },
        Code,
        Pairs,
        Terms,
    };
}

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
            } => write!(f, "PauliIndex: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Iterate over all pairs in a slice.
#[derive(Debug)]
pub struct Pairs<'a, T> {
    data: &'a [T],
    i:    usize,
    j:    usize,
}

impl<'a, T> Pairs<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            i: 0,
            j: 0,
        }
    }
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            return None;
        }

        let out = (&self.data[self.i], &self.data[self.j]);

        self.j += 1;

        if self.j >= self.data.len() {
            self.j = 0;
            self.i += 1;
        }

        Some(out)
    }
}

