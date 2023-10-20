use std::marker::PhantomData;

use num::Float;
use serde::{
    de::Visitor,
    ser::SerializeSeq,
    Deserialize,
    Serialize,
};

use crate::{
    fermions::{
        An,
        Cr,
        Fermions,
        Orbital,
    },
    prelude::SumRepr,
    qubits::{
        Pauli,
        PauliCode,
    },
    FermiSum,
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

impl Serialize for Fermions {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Fermions::Offset => {
                let seq = serializer.serialize_seq(Some(0))?;
                seq.end()
            }
            Fermions::One {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&cr.index())?;
                seq.serialize_element(&an.index())?;
                seq.end()
            }
            Fermions::Two {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(4))?;
                seq.serialize_element(&cr.0.index())?;
                seq.serialize_element(&cr.1.index())?;
                seq.serialize_element(&an.0.index())?;
                seq.serialize_element(&an.1.index())?;
                seq.end()
            }
        }
    }
}

struct FermionsVisitor;

impl<'de> Visitor<'de> for FermionsVisitor {
    type Value = Fermions;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("sequence of 0, 2 or 4 orbital indices")
    }

    fn visit_seq<A>(
        self,
        seq: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let mut seq = seq;
        let idx_tup: (Option<u32>, Option<u32>, Option<u32>, Option<u32>) = (
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
        );

        match idx_tup {
            (None, None, None, None) => Ok(Fermions::Offset),
            (Some(p), Some(q), None, None) => Fermions::one_electron(
                Cr(Orbital::from_index(p)),
                An(Orbital::from_index(q)),
            )
            .ok_or(A::Error::custom("cannot parse one-electron term")),
            (Some(p), Some(q), Some(r), Some(s)) => Fermions::two_electron(
                (Cr(Orbital::from_index(p)), Cr(Orbital::from_index(q))),
                (An(Orbital::from_index(r)), An(Orbital::from_index(s))),
            )
            .ok_or(A::Error::custom("cannot parse two-electron term")),
            _ => Err(A::Error::custom("cannot parse sequence")),
        }
    }
}

impl<'de> Deserialize<'de> for Fermions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FermionsVisitor)
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

#[derive(Serialize, Deserialize)]
struct FermiSumTerm<T> {
    code:  Fermions,
    value: T,
}

struct FermiSumSerSequence<'a, T>(&'a FermiSum<T>);

impl<'a, T> Serialize for FermiSumSerSequence<'a, T>
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
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (&coeff, &code) in self.0 {
            seq.serialize_element(&FermiSumTerm {
                code,
                value: coeff,
            })?;
        }

        seq.end()
    }
}

#[derive(Serialize)]
struct FermiSumSer<'a, T>
where
    T: Float,
{
    encoding: &'a str,
    terms:    FermiSumSerSequence<'a, T>,
}

impl<T> Serialize for FermiSum<T>
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
        (FermiSumSer {
            encoding: "fermions",
            terms:    FermiSumSerSequence(self),
        })
        .serialize(serializer)
    }
}

struct FermiSumSequence<T>(FermiSum<T>);

struct FermiSumVisitor<T> {
    _marker: PhantomData<T>,
}

impl<T> FermiSumVisitor<T> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FermiSumVisitor<T>
where
    T: Float + Deserialize<'de>,
{
    type Value = FermiSumSequence<T>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "sequence of objects with keys: 'code', 'value'")
    }

    fn visit_seq<A>(
        self,
        seq: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut seq = seq;
        let mut repr = FermiSum::new();

        while let Some(FermiSumTerm {
            code,
            value,
        }) = seq.next_element()?
        {
            repr.add_term(code, value)
        }

        Ok(FermiSumSequence(repr))
    }
}

impl<'de, T> Deserialize<'de> for FermiSumSequence<T>
where
    T: Float + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FermiSumVisitor::new())
    }
}

#[derive(Deserialize)]
struct FermiSumDe<T>
where
    T: Float,
{
    encoding: String,
    terms:    FermiSumSequence<T>,
}

impl<'de, T> Deserialize<'de> for FermiSum<T>
where
    T: Float + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let fermisumde = FermiSumDe::deserialize(deserializer)?;

        if fermisumde.encoding == "fermions" {
            Ok(fermisumde.terms.0)
        } else {
            Err(D::Error::custom("wrong encoding"))
        }
    }
}
