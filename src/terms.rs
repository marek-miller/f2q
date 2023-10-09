//! Representation of Hamiltonian sum terms

use std::{
    collections::HashMap,
    ops::Add,
};

use num::Float;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    Code,
    Terms,
};

/// Weighted sum of codes
#[derive(Debug, Serialize, Deserialize)]
pub struct SumRepr<T, K>
where
    K: Code,
{
    map: HashMap<K, T>,
}

impl<T, K> Default for SumRepr<T, K>
where
    K: Code,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, K> SumRepr<T, K>
where
    K: Code,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn as_map(&self) -> &HashMap<K, T> {
        &self.map
    }

    pub fn as_map_mut(&mut self) -> &mut HashMap<K, T> {
        &mut self.map
    }
}

impl<T, K> SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    #[must_use]
    pub fn coeff(
        &self,
        code: &K,
    ) -> T {
        match self.map.get(code) {
            Some(coeff) => *coeff,
            None => T::zero(),
        }
    }

    pub fn update(
        &mut self,
        code: K,
        coeff: T,
    ) -> Option<T> {
        self.map.insert(code, coeff)
    }

    pub fn add(
        &mut self,
        code: K,
        coeff: T,
    ) {
        let prev_coeff = self.coeff(&code);
        let _ = self.update(code, coeff + prev_coeff);
    }
}

impl<T, K> Terms<T, K> for SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        for (code, value) in self.as_map() {
            repr.add(*code, *value);
        }
    }
}

/// Dynamic, heterogenous representation of a Hamiltonian
pub enum Hamil<T, K> {
    Offset(T),
    Sum(Box<Self>, Box<Self>),
    Terms(Box<dyn Terms<T, K>>),
}

impl<T, K> Default for Hamil<T, K>
where
    T: Default,
{
    fn default() -> Self {
        Self::Offset(T::default())
    }
}

impl<T, K> Add for Hamil<T, K> {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        Self::Sum(Box::new(self), Box::new(rhs))
    }
}

impl<T, K> Hamil<T, K> {
    #[must_use]
    pub fn add_offset(
        self,
        value: T,
    ) -> Self {
        self + Self::Offset(value)
    }

    #[must_use]
    pub fn add_terms(
        self,
        terms: Box<dyn Terms<T, K>>,
    ) -> Self
    where
        T: Float,
    {
        self + Self::Terms(terms)
    }

    #[must_use]
    pub fn add_hamil(
        self,
        other: Self,
    ) -> Self {
        self + other
    }
}

impl<T, K> Terms<T, K> for Hamil<T, K>
where
    T: Float,
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        match self {
            Self::Offset(t) => {
                repr.add(K::default(), *t);
            }
            Self::Terms(terms) => terms.add_to(repr),
            Self::Sum(h1, h2) => {
                h1.add_to(repr);
                h2.add_to(repr);
            }
        }
    }
}

impl<T, K> From<Hamil<T, K>> for SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    fn from(value: Hamil<T, K>) -> Self {
        let mut hamil = value;
        let mut repr = SumRepr::new();
        hamil.add_to(&mut repr);
        repr
    }
}

#[derive(Debug)]
pub struct StackRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    f: OP,
}

impl<T, K, OP> StackRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    pub fn new(f: OP) -> Self {
        Self {
            f,
        }
    }
}

impl<T, K, OP> Terms<T, K> for StackRepr<T, K, OP>
where
    T: Float,
    K: Code,
    OP: FnMut() -> Option<(T, K)>,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        while let Some((coeff, code)) = (self.f)() {
            repr.add(code, coeff);
        }
    }
}

pub struct HeapRepr<'a, T, K> {
    f: Box<dyn FnMut() -> Option<(T, K)> + 'a>,
}

impl<'a, T, K> HeapRepr<'a, T, K> {
    /// Allocate memory for the closure on the heap.
    pub fn new<OP>(f: OP) -> Self
    where
        OP: FnMut() -> Option<(T, K)> + 'a,
    {
        Self {
            f: Box::new(f)
        }
    }
}

impl<'a, T, K> Terms<T, K> for HeapRepr<'a, T, K>
where
    T: Float,
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        while let Some((coeff, code)) = (self.f)() {
            repr.add(code, coeff);
        }
    }
}
