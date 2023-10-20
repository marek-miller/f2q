use std::marker::PhantomData;

use num::Float;
use serde::{
    de::Visitor,
    Deserialize,
    Serialize,
};

use crate::{
    codes::qubits::{
        Pauli,
        PauliCode,
    },
    prelude::SumRepr,
};

impl Serialize for Pauli {
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

struct PauliVisitor;

impl<'de> Visitor<'de> for PauliVisitor {
    type Value = Pauli;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "one-character string: I, X, Y, or Z")
    }

    fn visit_char<E>(
        self,
        v: char,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            'I' => Ok(Pauli::I),
            'X' => Ok(Pauli::X),
            'Y' => Ok(Pauli::Y),
            'Z' => Ok(Pauli::Z),
            _ => Err(E::custom("unknown symbol")),
        }
    }

    fn visit_str<E>(
        self,
        v: &str,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "I" => Ok(Pauli::I),
            "X" => Ok(Pauli::X),
            "Y" => Ok(Pauli::Y),
            "Z" => Ok(Pauli::Z),
            _ => Err(E::custom("unknown symbol")),
        }
    }
}

impl<'de> Deserialize<'de> for Pauli {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PauliVisitor)
    }
}

impl Serialize for PauliCode {
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

struct PauliCodeVisitor;

impl<'de> Visitor<'de> for PauliCodeVisitor {
    type Value = PauliCode;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str(
            "string of 64 Pauli operators (trailing identities truncated)",
        )
    }

    fn visit_str<E>(
        self,
        v: &str,
    ) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() > 64 || v.is_empty() {
            return Err(E::custom("str len out of range: 1..=64".to_string()));
        }

        let mut code = PauliCode::default();

        for (i, ch) in v.chars().enumerate() {
            let pauli = match ch {
                'I' => Ok(Pauli::I),
                'X' => Ok(Pauli::X),
                'Y' => Ok(Pauli::Y),
                'Z' => Ok(Pauli::Z),
                _ => Err(E::custom(
                    "character must be one of: I, X, Y, Z".to_string(),
                )),
            }?;
            let idx = u16::try_from(i)
                .expect("index out of range for u16. This is a bug.");
            code.set(idx, pauli);
        }

        Ok(code)
    }
}

impl<'de> Deserialize<'de> for PauliCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PauliCodeVisitor)
    }
}

impl<T> Serialize for SumRepr<T, PauliCode>
where
    T: Float + Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut terms = serializer.serialize_map(Some(self.len()))?;
        for (coeff, code) in self {
            terms.serialize_entry(code, coeff)?;
        }
        terms.end()
    }
}

struct PauliSumVisitor<T> {
    _marker: PhantomData<T>,
}

impl<T> PauliSumVisitor<T> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for PauliSumVisitor<T>
where
    T: Float + Deserialize<'de>,
{
    type Value = SumRepr<T, PauliCode>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter
            .write_str("object with Pauli string as key and float as value")
    }

    fn visit_map<A>(
        self,
        map: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = map;
        let mut repr = SumRepr::new();
        while let Some((code, coeff)) = map.next_entry()? {
            repr.add_term(code, coeff);
        }

        Ok(repr)
    }
}

impl<'de, T> Deserialize<'de> for SumRepr<T, PauliCode>
where
    T: Float + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PauliSumVisitor::new())
    }
}
