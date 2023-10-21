use std::fmt::Display;

use codes::Code;
use terms::SumRepr;

pub type IndexedSum<T> = SumRepr<T, u64>;

pub mod codes;
pub mod maps;
pub mod math;
pub mod terms;

mod serialize;
pub use serialize::Encoding;

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
