use std::ops::Add;

use num::Float;

use super::{
    Code,
    Terms,
};
use crate::SumRepr;

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
