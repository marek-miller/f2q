use std::fmt::Display;

use serde::{
    de::Visitor,
    Deserialize,
    Serialize,
};

/// Possible encodings of Hamiltonian terms
#[derive(Debug, PartialEq)]
pub enum Encoding {
    /// Second quantization fermion interaction
    FermiCode,
    /// Pauli strings (codes)
    Qubits,
    /// Indexed
    U64,
}

impl Display for Encoding {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Encoding::FermiCode => write!(f, "fermions"),
            Encoding::Qubits => write!(f, "qubits"),
            Encoding::U64 => write!(f, "u64"),
        }
    }
}

impl Serialize for Encoding {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct EncodingVisitor;

impl<'de> Visitor<'de> for EncodingVisitor {
    type Value = Encoding;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "string denoting correct encoding")
    }

    fn visit_str<E>(
        self,
        v: &str,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "fermions" => Ok(Encoding::FermiCode),
            "qubits" => Ok(Encoding::Qubits),
            "u64" => Ok(Encoding::U64),
            _ => Err(E::custom("wrong encoding")),
        }
    }
}

impl<'de> Deserialize<'de> for Encoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EncodingVisitor)
    }
}

mod fermions;
mod qubits;
