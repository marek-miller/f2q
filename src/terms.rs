//! Representation of Hamiltonian sum terms.

use std::collections::HashMap;

use num::Float;

use crate::{
    code::Code,
    Error,
};

/// Convert and serialize sum of terms in various encodings
pub trait Terms<T> {
    type Error;

    /// Add terms to the supplied representation.
    ///
    /// # Errors
    ///
    /// Return error on failure.
    fn add_to(
        &mut self,
        repr: impl Extend<T>,
    ) -> Result<(), Error>;
}

/// Weighted sum of codes
#[derive(Debug)]
pub struct SumRepr<T, K> {
    terms: HashMap<K, T>,
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
    /// # use f2q::{terms::SumRepr, code::qubits::Pauli};
    /// let repr = SumRepr::<f64, Pauli>::new();
    ///
    /// assert!(repr.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

    /// Creates an empty `SumRepr` with at least the specified capacity.
    ///
    /// The struct will be able to hold at least `capacity` elements without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::{terms::SumRepr, code::qubits::Pauli};
    /// let repr = SumRepr::<f64, Pauli>::with_capacity(8);
    ///
    /// assert!(repr.capacity() >= 8);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            terms: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the struct might be able to hold more, but
    /// is guaranteed to be able to hold at least this many.
    ///
    ///  /// # Examples
    ///
    /// ```rust
    /// # use f2q::{terms::SumRepr, code::qubits::Pauli};
    /// let repr = SumRepr::<f64, Pauli>::with_capacity(8);
    ///
    /// assert!(repr.capacity() >= 8);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.terms.capacity()
    }

    /// Number of terms in the sum.
    #[must_use]
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over terms in the sum.
    ///
    /// The returned iterator runs over tuples of shared references of type:
    /// `(&T, &K)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::{code::qubits::Pauli, terms::SumRepr};
    ///
    /// let mut repr = SumRepr::new();
    ///
    /// repr.update(Pauli::default(), 0.5);
    /// repr.update(Pauli::new((1, 0)), 0.5);
    ///
    /// let sum = repr.iter().fold(0.0, |acc, (&coeff, _)| acc + coeff);
    ///
    /// assert_eq!(sum, 1.0);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&T, &K)> {
        self.terms.iter().map(|(code, coeff)| (coeff, code))
    }

    /// Iterate over terms in the sum, allow mutable access to coefficients.
    ///
    /// The returned iterator runs over tuples of references of type:
    /// `(&mut T, &K)`.
    ///
    ///  /// # Examples
    ///
    /// ```rust
    /// # use f2q::{code::qubits::Pauli, terms::SumRepr};
    ///
    /// let mut repr = SumRepr::new();
    ///
    /// repr.update(Pauli::default(), 0.5);
    /// repr.update(Pauli::new((1, 0)), 0.5);
    /// for (coeff, _) in repr.iter_mut() {
    ///     *coeff += 0.1;
    /// }
    ///
    /// assert_eq!(repr.coeff(Pauli::default()), 0.6);
    /// assert_eq!(repr.coeff(Pauli::new((1, 0))), 0.6);
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, &K)> {
        self.terms.iter_mut().map(|(code, coeff)| (coeff, code))
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
        match self.terms.get(&code) {
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
        self.terms.insert(code, coeff)
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
}

impl<T, K> Terms<(T, K)> for SumRepr<T, K>
where
    T: Float,
    K: Code,
{
    type Error = Error;

    fn add_to(
        &mut self,
        mut repr: impl Extend<(T, K)>,
    ) -> Result<(), Error> {
        self.iter().try_for_each(|(&coeff, &code)| {
            repr.extend(Some((coeff, code)));
            Ok(())
        })
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

impl<T, K> Extend<(T, K)> for &mut SumRepr<T, K>
where
    K: Code,
    T: Float,
{
    fn extend<I: IntoIterator<Item = (T, K)>>(
        &mut self,
        iter: I,
    ) {
        (*self).extend(iter);
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

impl<T, K, OP> Terms<(T, K)> for StackRepr<T, K, OP>
where
    T: Float,
    K: Code,
    OP: FnMut() -> Option<(T, K)>,
{
    type Error = Error;

    fn add_to(
        &mut self,
        mut repr: impl Extend<(T, K)>,
    ) -> Result<(), Error> {
        while let Some((coeff, code)) = (self.f)() {
            repr.extend(Some((coeff, code)));
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

impl<'a, T, K> Terms<(T, K)> for HeapRepr<'a, T, K>
where
    T: Float,
    K: Code,
{
    type Error = Error;

    fn add_to(
        &mut self,
        mut repr: impl Extend<(T, K)>,
    ) -> Result<(), Error> {
        while let Some((coeff, code)) = (self.f)() {
            repr.extend(Some((coeff, code)));
        }

        Ok(())
    }
}
