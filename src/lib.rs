use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::Add,
};

use num::Float;
use serde::{
    Deserialize,
    Serialize,
};

const PAULI_MASK: u64 = 3;

#[derive(Debug, PartialEq)]
pub enum Error {
    CodeValue,
}

impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::CodeValue => write!(f, "CodeValue"),
        }
    }
}

impl std::error::Error for Error {}

pub trait Code: Clone + Eq + Hash + Default {}

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

pub type PauliSum<T> = SumRepr<T, PauliCode>;

#[derive(Debug)]
pub struct IterRepr<T, K, I>
where
    I: Iterator<Item = (T, K)>,
{
    iter: I,
}

impl<T, K, I> IterRepr<T, K, I>
where
    I: Iterator<Item = (T, K)>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
        }
    }
}

impl<T, K, I> Terms<T, K> for IterRepr<T, K, I>
where
    T: Float,
    K: Code,
    I: Iterator<Item = (T, K)>,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    ) {
        for (coeff, code) in self.iter.by_ref() {
            repr.add(code, coeff);
        }
    }
}

#[derive(Debug)]
pub struct OpRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    f: OP,
}

impl<T, K, OP> OpRepr<T, K, OP>
where
    OP: FnMut() -> Option<(T, K)>,
{
    pub fn new(f: OP) -> Self {
        Self {
            f,
        }
    }
}

impl<T, K, OP> Terms<T, K> for OpRepr<T, K, OP>
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
    /// Allocated memory for the closure on the heap.
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

pub trait Terms<T, K>
where
    K: Code,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, K>,
    );
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
            repr.add(code.clone(), *value);
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Pauli {
    #[default]
    I,
    X,
    Y,
    Z,
}

macro_rules! impl_pauli_int {
    ( $($Typ:ty)* ) => {
        $(

            impl TryFrom<$Typ> for Pauli {
                type Error = Error;

                fn try_from(value: $Typ) -> Result<Self, Self::Error> {
                    use Pauli::*;
                    match value {
                        0 => Ok(I),
                        1 => Ok(X),
                        2 => Ok(Y),
                        3 => Ok(Z),
                        _ => Err(Self::Error::CodeValue),
                    }
                }
            }

            impl From<Pauli> for $Typ {
                fn from(value: Pauli) -> Self {
                    match value {
                        Pauli::I => 0,
                        Pauli::X => 1,
                        Pauli::Y => 2,
                        Pauli::Z => 3,
                    }
                }
            }

        )*
    };
}

impl_pauli_int!(u8 u16 u32 u64 u128 usize);
impl_pauli_int!(i8 i16 i32 i64 i128 isize);

/// Pauli string of up to 64 qubits.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PauliCode {
    pack: (u64, u64),
}

impl Default for PauliCode {
    fn default() -> Self {
        Self::new((0, 0))
    }
}

impl PauliCode {
    /// Create new code.
    ///
    /// The pauli product is specified by providing a tuple `(u64, u64)` with 2
    /// bits for each Pauli operator in the tensor product:
    ///
    /// ```text
    /// Pauli::I = 0b00
    /// Pauli::X = 0b01
    /// Pauli::Y = 0b10
    /// Pauli::Z = 0b11
    /// ```
    ///
    /// The first integer in the tuple represents qubits 0 to 31 (included),
    /// and the second integer represents qubits 32 to 63 (included).
    /// The pairs of bits in each integer follow little-endian convention.
    /// For example, `PauliCode::new((0b1001,0))` represents the following Pauli
    /// product of 64 Pauli operators:
    ///
    /// ```text
    /// [X, Y, I, I, ... , I]
    /// ```
    ///
    /// whereas `PauliCode::new((0, 0b0111))` represents:
    ///
    /// ```text
    /// [I, I, .. I, Z, X, I, ... , I],
    /// ```
    /// with `Z` at site 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil64::{Pauli, PauliCode};
    ///
    /// let code = PauliCode::new((0b0100, 0b1110));
    ///
    /// assert_eq!(code.pauli(0), Some(Pauli::I));
    /// assert_eq!(code.pauli(1), Some(Pauli::X));
    /// assert_eq!(code.pauli(32), Some(Pauli::Y));
    /// assert_eq!(code.pauli(33), Some(Pauli::Z));
    /// ```
    #[must_use]
    pub fn new(pack: (u64, u64)) -> Self {
        Self {
            pack,
        }
    }

    #[must_use]
    pub fn enumerate(&self) -> u128 {
        u128::from(self.pack.0) + (u128::from(self.pack.1) << 64)
    }

    /// # Safety
    ///
    /// Make sure index is within 0..64
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub unsafe fn pauli_unchecked(
        &self,
        index: usize,
    ) -> Pauli {
        let pauli_int = if index < 32 {
            (self.pack.0 >> (index * 2)) & PAULI_MASK
        } else {
            (self.pack.1 >> ((index - 32) * 2)) & PAULI_MASK
        };
        Pauli::try_from(pauli_int).expect("incorrect encoding. This is a bug")
    }

    #[must_use]
    pub fn pauli(
        &self,
        index: usize,
    ) -> Option<Pauli> {
        if index >= 64 {
            None
        } else {
            // SAFETY: We just checked if index is within bounds
            Some(unsafe { self.pauli_unchecked(index) })
        }
    }

    /// # Safety
    ///
    /// Make sure index is within 0..64
    pub unsafe fn pauli_mut_unchecked<OP>(
        &mut self,
        index: usize,
        f: OP,
    ) where
        OP: FnOnce(&mut Pauli),
    {
        let mut pauli = self.pauli_unchecked(index);
        f(&mut pauli);
        if index < 32 {
            self.pack.0 &= !(PAULI_MASK << (index * 2));
            self.pack.0 |= u64::from(pauli) << (index * 2);
        } else {
            self.pack.1 &= !(PAULI_MASK << ((index - 32) * 2));
            self.pack.1 |= u64::from(pauli) << ((index - 32) * 2);
        }
    }

    pub fn pauli_mut<OP>(
        &mut self,
        index: usize,
        f: OP,
    ) where
        OP: FnOnce(Option<&mut Pauli>),
    {
        if index >= 64 {
            f(None);
        } else {
            // SAFETY: We just checked if index is within bounds
            unsafe {
                self.pauli_mut_unchecked(index, |x: &mut Pauli| f(Some(x)));
            }
        }
    }

    /// # Panics
    ///
    /// Panics if index outside of 0..64
    pub fn set(
        &mut self,
        index: usize,
        pauli: Pauli,
    ) {
        self.pauli_mut(index, |x| {
            if let Some(p) = x {
                *p = pauli;
            } else {
                panic!("index should be within 0..64");
            }
        });
    }

    #[must_use]
    pub fn iter(&self) -> PauliIter<'_> {
        PauliIter::new(self)
    }

    pub fn from_paulis<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Pauli>,
    {
        let mut code = Self::default();
        for (i, pauli) in iter.into_iter().take(64).enumerate() {
            code.set(i, pauli);
        }
        code
    }
}

impl<'a> IntoIterator for &'a PauliCode {
    type IntoIter = PauliIter<'a>;
    type Item = Pauli;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Iterate over Paulis in PauliCode
#[derive(Debug)]
pub struct PauliIter<'a> {
    code:  &'a PauliCode,
    index: usize,
}

impl<'a> PauliIter<'a> {
    fn new(code: &'a PauliCode) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl<'a> Iterator for PauliIter<'a> {
    type Item = Pauli;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            return None;
        }

        let pauli = self.code.pauli(self.index);
        self.index += 1;
        pauli
    }
}
impl Code for PauliCode {}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Spin {
    #[default]
    Down,
    Up,
}

macro_rules! impl_spin_int {
    ($($Typ:ty)* ) => {
        $(
            impl From<Spin> for $Typ {
                fn from(value: Spin) -> Self {
                    match value {
                        Spin::Down => 0,
                        Spin::Up => 1,
                    }
                }
            }
        )*
    };
}

impl_spin_int!(u8 u16 u32 u64 u128 usize);
impl_spin_int!(i8 i16 i32 i64 i128 isize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct Orbital {
    n: usize,
    s: Spin,
}

impl Orbital {
    #[must_use]
    pub fn new(
        n: usize,
        s: Spin,
    ) -> Self {
        Self {
            n,
            s,
        }
    }

    /// # Panics
    ///
    /// Panics is the orbitals index cannot fit into `usize`,
    #[must_use]
    pub fn enumerate(&self) -> usize {
        assert!(
            self.n <= usize::MAX / 2 - usize::from(self.s),
            "orbital index out of bound"
        );
        self.n * 2 + usize::from(self.s)
    }
}

/// Electronic integral with orbitals in canonical order:
///
/// https://learn.microsoft.com/en-us/azure/quantum/user-guide/libraries/chemistry/concepts/second-quantization
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum FermiIntegral {
    #[default]
    Constant,
    OneElectron {
        cr: Orbital,
        an: Orbital,
    },
    TwoElectron {
        cr: (Orbital, Orbital),
        an: (Orbital, Orbital),
    },
}

impl Code for FermiIntegral {}

pub type FermiSum<T> = SumRepr<T, FermiIntegral>;

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

pub type PauliHamil<T> = Hamil<T, PauliCode>;
pub type FermiHamil<T> = Hamil<T, FermiIntegral>;

impl<T> From<SumRepr<T, FermiIntegral>> for SumRepr<T, PauliCode>
where
    T: Float,
{
    fn from(value: SumRepr<T, FermiIntegral>) -> Self {
        let mut pauli_repr = SumRepr::new();
        for (&code, &coeff) in value.as_map() {
            match code {
                FermiIntegral::Constant => {
                    pauli_repr.add(PauliCode::default(), coeff);
                }
                FermiIntegral::OneElectron {
                    cr,
                    an,
                } => pauli_add_one_electron_integral(
                    cr,
                    an,
                    coeff,
                    &mut pauli_repr,
                ),
                FermiIntegral::TwoElectron {
                    cr,
                    an,
                } => pauli_add_two_electron_integral(
                    cr,
                    an,
                    coeff,
                    &mut pauli_repr,
                ),
            }
        }
        pauli_repr
    }
}

fn pauli_add_one_electron_integral<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    if cr == an {
        pauli_add_one_electron_integral_equal(cr, an, coeff, pauli_repr);
    } else {
        pauli_add_one_electron_integral_nonequal(cr, an, coeff, pauli_repr);
    }
}

fn pauli_add_one_electron_integral_equal<T: Float>(
    cr: Orbital,
    _an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let one_half =
        T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let code = PauliCode::default();
    pauli_repr.add(code, coeff * one_half);

    let mut code = PauliCode::default();
    code.set(cr.enumerate(), Pauli::Z);
    pauli_repr.add(code, -coeff * one_half);
}

fn pauli_add_one_electron_integral_nonequal<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let one_half =
        T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();
    // we know that orbitals are ordered: cr <= an
    for i in cr.enumerate() + 1..an.enumerate() {
        code.set(i, Pauli::Z);
    }
    code.set(cr.enumerate(), Pauli::X);
    code.set(an.enumerate(), Pauli::X);
    pauli_repr.add(code, coeff * one_half);

    code.set(cr.enumerate(), Pauli::Y);
    code.set(an.enumerate(), Pauli::Y);
    pauli_repr.add(code, -coeff * one_half);
}

fn pauli_add_two_electron_integral<T: Float>(
    cr: (Orbital, Orbital),
    an: (Orbital, Orbital),
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let (p, q, r, s) = (
        cr.0.enumerate(),
        cr.1.enumerate(),
        an.0.enumerate(),
        an.1.enumerate(),
    );

    if p == s && q == r {
        pauli_add_two_electron_integral_pq(p, q, coeff, pauli_repr);
    } else if q == r {
        pauli_add_two_electron_integral_pqs(p, q, s, coeff, pauli_repr);
    } else {
        todo!()
    }
}

fn pauli_add_two_electron_integral_pq<T: Float>(
    p: usize,
    q: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let frac =
        T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    // I
    pauli_repr.add(code, coeff * frac);
    code.set(p, Pauli::Z);
    // Z_p
    pauli_repr.add(code, -coeff * frac);
    code.set(p, Pauli::I);
    code.set(q, Pauli::Z);
    // Z_q
    pauli_repr.add(code, -coeff * frac);
    code.set(p, Pauli::Z);
    // Z_p Z_q
    pauli_repr.add(code, coeff * frac);
}

fn pauli_add_two_electron_integral_pqs<T: Float>(
    _p: usize,
    _q: usize,
    _s: usize,
    _coeff: T,
    _pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    todo!()
}

impl<T> Terms<T, PauliCode> for Hamil<T, FermiIntegral>
where
    T: Float,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, PauliCode>,
    ) {
        let mut fermi_repr = SumRepr::new();
        <Self as Terms<T, FermiIntegral>>::add_to(self, &mut fermi_repr);
        let mut pauli_repr = SumRepr::<T, PauliCode>::from(fermi_repr);
        pauli_repr.add_to(repr);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pauli_01() {
        assert_eq!(Pauli::try_from(0u32).unwrap(), Pauli::I);
        assert_eq!(Pauli::try_from(1u32).unwrap(), Pauli::X);
        assert_eq!(Pauli::try_from(2u32).unwrap(), Pauli::Y);
        assert_eq!(Pauli::try_from(3u32).unwrap(), Pauli::Z);
    }

    #[test]
    fn test_pauli_02() {
        let err = Pauli::try_from(4u16).unwrap_err();
        assert_eq!(err, Error::CodeValue);
    }

    #[test]
    fn test_pauli_03() {
        assert_eq!(u8::from(Pauli::I), 0);
        assert_eq!(u8::from(Pauli::X), 1);
        assert_eq!(u8::from(Pauli::Y), 2);
        assert_eq!(u8::from(Pauli::Z), 3);
    }

    #[test]
    fn test_paulicode_init() {
        let code = PauliCode::new((0b01, 0b00));
        assert_eq!(code.enumerate(), 0b01);
    }

    #[test]
    fn test_paulicode_default() {
        let code = PauliCode::default();
        assert_eq!(code, PauliCode::new((0, 0)));
    }

    #[test]
    fn test_paulicode_pauli_02() {
        let code = PauliCode::new((0b0101, 0b00));

        assert_eq!(code.pauli(0), Some(Pauli::X));
        assert_eq!(code.pauli(1), Some(Pauli::X));
        assert_eq!(code.pauli(2), Some(Pauli::I));
        assert_eq!(code.pauli(63), Some(Pauli::I));

        assert_eq!(code.pauli(64), None);
        assert_eq!(code.pauli(123), None);
    }

    #[test]
    fn test_paulicode_pauli_mut_01() {
        let mut code = PauliCode::default();
        assert_eq!(code.pauli(7).unwrap(), Pauli::I);

        code.pauli_mut(7, |x| {
            if let Some(pauli) = x {
                *pauli = Pauli::Z;
            }
        });
        assert_eq!(code.pauli(7).unwrap(), Pauli::Z);
    }

    #[test]
    fn test_paulicode_set_pauli_01() {
        let mut code = PauliCode::new((29_332_281_938, 0b00));
        assert_eq!(code.pauli(7).unwrap(), Pauli::I);

        code.set(7, Pauli::Y);
        assert_eq!(code.pauli(7).unwrap(), Pauli::Y);
    }

    #[test]
    #[should_panic(expected = "index should be within 0..64")]
    fn test_paulicode_set_pauli_02() {
        let mut code = PauliCode::default();
        assert_eq!(code.pauli(7).unwrap(), Pauli::I);

        code.set(65, Pauli::Y);
        assert_eq!(code.pauli(7).unwrap(), Pauli::Y);
    }

    #[test]
    fn test_paulicode_set_pauli_03() {
        let mut code = PauliCode::default();

        for i in 0..13 {
            code.set(i, Pauli::X);
        }
        for i in 13..29 {
            code.set(i, Pauli::Y);
        }
        for i in 29..61 {
            code.set(i, Pauli::Z);
        }

        for i in 0..13 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::X, "{i}");
        }
        for i in 13..29 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::Y, "{i}");
        }
        for i in 29..61 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::Z, "{i}");
        }
        for i in 61..64 {
            assert_eq!(code.pauli(i).unwrap(), Pauli::I, "{i}");
        }
    }

    #[test]
    fn test_paulicode_codes_iter_01() {
        use Pauli::*;
        let result = PauliCode::new((0b01, 0b00))
            .iter()
            .take(3)
            .collect::<Vec<_>>();

        assert_eq!(result, &[X, I, I]);
    }

    #[test]
    fn test_paulicode_codes_iter_02() {
        use Pauli::*;
        let result = PauliCode::new((0b11_1001, 0b00))
            .iter()
            .take(5)
            .collect::<Vec<_>>();

        assert_eq!(result, &[X, Y, Z, I, I]);
    }

    #[test]
    fn test_paulicode_codes_iter_03() {
        use Pauli::*;
        let result = PauliCode::new((0b0101_0000, 0b1111_1010))
            .iter()
            .take(36)
            .collect::<Vec<_>>();

        assert_eq!(
            result,
            &[
                I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
                I, I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
            ]
        );
    }

    #[test]
    fn test_paulicode_from_paulis_01() {
        use Pauli::*;

        assert_eq!(
            PauliCode::from_paulis([I, X, Y, Z]),
            PauliCode::new((0b1110_0100, 0b00))
        );
    }

    #[test]
    fn test_paulicode_from_paulis_02() {
        use Pauli::*;

        assert_eq!(
            PauliCode::from_paulis([
                I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
                I, I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
            ]),
            PauliCode::new((0b0101_0000, 0b1111_1010))
        );
    }

    #[test]
    fn test_paulisum_init_01() {
        let code = PauliCode::new((1234, 0));
        let mut hamil = SumRepr::new();

        hamil.as_map_mut().insert(code, 4321.);
        let coeff = hamil.coeff(&code);
        assert!(f64::abs(coeff - 4321.) < f64::EPSILON);
    }

    #[test]
    fn test_spin_init_01() {
        let spin = Spin::Down;
        assert_eq!(u8::from(spin), 0);
        let spin = Spin::Up;
        assert_eq!(u8::from(spin), 1);

        let spin = Spin::default();
        assert_eq!(u8::from(spin), 0);
    }

    #[test]
    fn test_orbital_enumerate_01() {
        let orb = Orbital::default();
        assert_eq!(orb.enumerate(), 0);

        let orb = Orbital::new(3, Spin::Down);
        assert_eq!(orb.enumerate(), 6);

        let orb = Orbital::new(8, Spin::Up);
        assert_eq!(orb.enumerate(), 17);
    }

    #[test]
    #[should_panic(expected = "orbital index out of bound")]
    fn test_orbital_enumerate_02() {
        let orb = Orbital::new(usize::MAX / 2, Spin::Up);
        assert_eq!(orb.enumerate(), usize::MAX);
    }
}
