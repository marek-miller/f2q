use std::marker::PhantomData;

use num::Float;
use serde::{
    de::Visitor,
    ser::SerializeSeq,
    Deserialize,
    Serialize,
};

use crate::{
    codes::fermions::{
        An,
        Cr,
        FermiCode,
        Orbital,
    },
    serialize::Encoding,
    terms::SumRepr,
};

impl Serialize for FermiCode {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FermiCode::Offset => {
                let seq = serializer.serialize_seq(Some(0))?;
                seq.end()
            }
            FermiCode::One {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&cr.index())?;
                seq.serialize_element(&an.index())?;
                seq.end()
            }
            FermiCode::Two {
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

struct FermiCodeVisitor;

impl<'de> Visitor<'de> for FermiCodeVisitor {
    type Value = FermiCode;

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
            (None, None, None, None) => Ok(FermiCode::Offset),
            (Some(p), Some(q), None, None) => FermiCode::one_electron(
                Cr(Orbital::from_index(p)),
                An(Orbital::from_index(q)),
            )
            .ok_or(A::Error::custom("cannot parse one-electron term")),
            (Some(p), Some(q), Some(r), Some(s)) => FermiCode::two_electron(
                (Cr(Orbital::from_index(p)), Cr(Orbital::from_index(q))),
                (An(Orbital::from_index(r)), An(Orbital::from_index(s))),
            )
            .ok_or(A::Error::custom("cannot parse two-electron term")),
            _ => Err(A::Error::custom("cannot parse sequence")),
        }
    }
}

impl<'de> Deserialize<'de> for FermiCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FermiCodeVisitor)
    }
}

#[derive(Serialize, Deserialize)]
struct FermiSumTerm<T> {
    code:  FermiCode,
    value: T,
}

struct FermiSumSerSequence<'a, T>(&'a SumRepr<T, FermiCode>);

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
    r#type:   &'a str,
    encoding: Encoding,
    terms:    FermiSumSerSequence<'a, T>,
}

impl<T> Serialize for SumRepr<T, FermiCode>
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
            r#type:   "sumrepr",
            encoding: Encoding::FermiCode,
            terms:    FermiSumSerSequence(self),
        })
        .serialize(serializer)
    }
}

struct FermiSumDeSequence<T>(SumRepr<T, FermiCode>);

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
    type Value = FermiSumDeSequence<T>;

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
        let mut repr = SumRepr::new();

        while let Some(FermiSumTerm {
            code,
            value,
        }) = seq.next_element()?
        {
            repr.add_term(code, value);
        }

        Ok(FermiSumDeSequence(repr))
    }
}

impl<'de, T> Deserialize<'de> for FermiSumDeSequence<T>
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
    r#type:   String,
    encoding: Encoding,
    terms:    FermiSumDeSequence<T>,
}

impl<'de, T> Deserialize<'de> for SumRepr<T, FermiCode>
where
    T: Float + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let sumde = FermiSumDe::deserialize(deserializer)?;

        if sumde.r#type != "sumrepr" {
            return Err(D::Error::custom("type should be: 'sumrepr'"));
        }

        if sumde.encoding != Encoding::FermiCode {
            return Err(D::Error::custom("encoding should be: 'fermions'"));
        }

        Ok(sumde.terms.0)
    }
}
