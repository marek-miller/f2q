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
use crate::Error;

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
    /// Create new, empty sum
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::{terms::SumRepr, qubit::PauliCode};
    /// let repr = SumRepr::<f64, PauliCode>::new();
    ///
    /// assert!(repr.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Number of terms in the sum.
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn iter(&self) -> SumIter<'_, T, K> {
        SumIter::new(self)
    }
}

/// Iterator over terms in [`SumRepr`].
#[derive(Debug)]
pub struct SumIter<'a, T, K>
where
    K: Code,
{
    iter: std::collections::hash_map::Iter<'a, K, T>,
}

impl<'a, T, K> SumIter<'a, T, K>
where
    K: Code,
{
    pub fn new(repr: &'a SumRepr<T, K>) -> Self {
        Self {
            iter: repr.map.iter(),
        }
    }
}

impl<'a, T, K> Iterator for SumIter<'a, T, K>
where
    K: Code,
{
    type Item = (&'a T, &'a K);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(code, coeff)| (coeff, code))
    }
}

impl<'a, T, K> IntoIterator for &'a SumRepr<T, K>
where
    K: Code,
{
    type IntoIter = SumIter<'a, T, K>;
    type Item = (&'a T, &'a K);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, K> SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    /// Returns coefficient in the sum for a given code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::terms::SumRepr;
    /// let mut repr = SumRepr::new();
    /// repr.update(1, 0.5);
    ///
    /// assert_eq!(repr.coeff(1), 0.5);
    /// assert_eq!(repr.coeff(2), 0.0);
    /// ```
    #[must_use]
    pub fn coeff(
        &self,
        code: K,
    ) -> T {
        match self.map.get(&code) {
            Some(coeff) => *coeff,
            None => T::zero(),
        }
    }

    /// Replace coefficient for the given code.
    ///
    /// Returns the previous coefficient, if present, or `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::terms::SumRepr;
    /// let mut repr = SumRepr::new();
    /// let old_coeff = repr.update(1, 0.5);
    /// assert_eq!(old_coeff, None);
    ///
    /// let old_coeff = repr.update(1, 0.7);
    /// assert_eq!(old_coeff, Some(0.5));
    /// assert_eq!(repr.coeff(1), 0.7);
    /// ```
    pub fn update(
        &mut self,
        code: K,
        coeff: T,
    ) -> Option<T> {
        self.map.insert(code, coeff)
    }

    /// Add coefficient to the given code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::terms::SumRepr;
    /// let mut repr = SumRepr::new();
    /// assert_eq!(repr.coeff(1), 0.0);
    /// repr.add_term(1, 0.5);
    /// assert_eq!(repr.coeff(1), 0.5);
    /// repr.add_term(1, 0.5);
    /// assert_eq!(repr.coeff(1), 1.0);
    /// ```
    pub fn add_term(
        &mut self,
        code: K,
        coeff: T,
    ) {
        let prev_coeff = self.coeff(code);
        let _ = self.update(code, coeff + prev_coeff);
    }

    pub(crate) fn add_tuple(
        &mut self,
        (code, coeff): (K, T),
    ) {
        self.add_term(code, coeff);
    }
}

impl<T, K> Terms<T, K> for SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    type Error = Error;

    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) -> Result<(), Self::Error> {
        for (&coeff, &code) in self.iter() {
            repr.add_term(code, coeff);
        }
        Ok(())
    }
}

impl<T, K> Extend<(T, K)> for SumRepr<T, K>
where
    K: Code,
    T: Float,
{
    fn extend<I>(
        &mut self,
        iter: I,
    ) where
        I: IntoIterator<Item = (T, K)>,
    {
        for (coeff, code) in iter {
            self.add_term(code, coeff);
        }
    }
}

/// Dynamic representation of a Hamiltonian
pub enum Hamil<T, K> {
    Offset(T),
    Sum(Box<Self>, Box<Self>),
    Terms(Box<dyn Terms<T, K, Error = Error> + Send + Sync>),
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

impl<T, K> Hamil<T, K>
where
    T: Float,
    K: Code,
{
    #[must_use]
    pub fn add_offset(
        self,
        value: T,
    ) -> Self {
        self + Self::Offset(value)
    }

    /// Add terms to the Hamiltonian.
    #[must_use]
    pub fn add_terms(
        self,
        terms: Box<dyn Terms<T, K, Error = Error> + Send + Sync>,
    ) -> Self {
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
    T: Float + Send,
    K: Code + Send,
{
    type Error = Error;

    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Offset(t) => {
                repr.add_term(K::default(), *t);
            }
            Self::Terms(terms) => terms.add_to(repr)?,
            Self::Sum(h1, h2) => {
                let mut repr_left = SumRepr::new();
                let mut repr_right = SumRepr::new();
                let (res_left, res_right) = rayon::join(
                    || h1.add_to(&mut repr_left),
                    || h2.add_to(&mut repr_right),
                );
                res_left?;
                res_right?;
                repr_left.add_to(repr)?;
                repr_right.add_to(repr)?;
            }
        }

        Ok(())
    }
}

impl<T, K> TryFrom<Hamil<T, K>> for SumRepr<T, K>
where
    T: Float + Send,
    K: Code + Send,
{
    type Error = Error;

    fn try_from(value: Hamil<T, K>) -> Result<Self, Self::Error> {
        let mut hamil = value;
        let mut repr = SumRepr::new();
        hamil.add_to(&mut repr)?;

        Ok(repr)
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
    type Error = Error;

    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) -> Result<(), Self::Error> {
        while let Some((coeff, code)) = (self.f)() {
            repr.add_term(code, coeff);
        }

        Ok(())
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
    type Error = Error;

    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) -> Result<(), Self::Error> {
        while let Some((coeff, code)) = (self.f)() {
            repr.add_term(code, coeff);
        }

        Ok(())
    }
}
