//! Qubit representation

use std::fmt::Display;

use crate::Error;

const PAULI_MASK: u64 = 0b11;

/// Pauli operator
///
/// # Examples
///
/// ```rust
/// # use f2q::code::qubits::PauliOp;
/// use f2q::Error::QubitIndex;
///
/// let paulis: Vec<_> = (0..=4).map(|i| PauliOp::try_from(i)).collect();
///
/// assert_eq!(
///     paulis[0..=3],
///     [
///         Ok(PauliOp::I),
///         Ok(PauliOp::X),
///         Ok(PauliOp::Y),
///         Ok(PauliOp::Z),
///     ]
/// );
/// matches!(paulis[4], Err(QubitIndex { .. }));
/// ```
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PauliOp {
    #[default]
    I,
    X,
    Y,
    Z,
}

macro_rules! impl_pauli_int {
    ( $($Typ:ty)* ) => {
        $(

            impl TryFrom<$Typ> for PauliOp {
                type Error = Error;

                fn try_from(value: $Typ) -> Result<Self, Self::Error> {
                    use PauliOp::*;
                    match value {
                        0 => Ok(I),
                        1 => Ok(X),
                        2 => Ok(Y),
                        3 => Ok(Z),
                        _ => Err(Self::Error::QubitIndex{ msg: "Pauli index should be within 0..=3".to_string()}),
                    }
                }
            }

            impl From<PauliOp> for $Typ {
                fn from(value: PauliOp) -> Self {
                    match value {
                        PauliOp::I => 0,
                        PauliOp::X => 1,
                        PauliOp::Y => 2,
                        PauliOp::Z => 3,
                    }
                }
            }

        )*
    };
}

impl_pauli_int!(u8 u16 u32 u64 u128 usize);
impl_pauli_int!(i8 i16 i32 i64 i128 isize);

impl Display for PauliOp {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let str_repr = match self {
            PauliOp::I => "I",
            PauliOp::X => "X",
            PauliOp::Y => "Y",
            PauliOp::Z => "Z",
        };
        write!(f, "{str_repr}")
    }
}

impl From<PauliOp> for String {
    fn from(value: PauliOp) -> Self {
        value.to_string()
    }
}

/// Pauli string of up to 64 qubits.
///
/// # Examples
///
/// ```rust
/// # use f2q::code::qubits::Pauli;
/// let code = Pauli::default();
///
/// assert_eq!(code.index(), 0);
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pauli {
    pack: (u64, u64),
}

impl Default for Pauli {
    fn default() -> Self {
        Self::new((0, 0))
    }
}

impl Pauli {
    /// Create new code.
    ///
    /// The pauli product is specified by providing a tuple `(u64, u64)` with 2
    /// bits for each Pauli operator in the tensor product:
    ///
    /// ```text
    /// PauliOp::I = 0b00
    /// PauliOp::X = 0b01
    /// PauliOp::Y = 0b10
    /// PauliOp::Z = 0b11
    /// ```
    ///
    /// The first integer in the tuple represents qubits 0 to 31 (incl.),
    /// and the second integer represents qubits 32 to 63 (incl.).
    /// The pairs of bits in each integer follow the little-endian convention.
    /// For example, `Pauli::new((0b1001, 0))` represents the following
    /// Pauli product of 64 Pauli operators:
    ///
    /// ```text
    /// [X, Y, I, I, ... , I]
    /// ```
    ///
    /// whereas `Pauli::new((0, 0b0111))` represents:
    ///
    /// ```text
    /// [I, I, .. I, Z, X, I, ... , I],
    /// ```
    ///
    /// with `Z`'s 0-based index 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    ///
    /// let code = Pauli::new((0b0100, 0b1110));
    ///
    /// assert_eq!(code.pauli(0), Some(PauliOp::I));
    /// assert_eq!(code.pauli(1), Some(PauliOp::X));
    /// assert_eq!(code.pauli(32), Some(PauliOp::Y));
    /// assert_eq!(code.pauli(33), Some(PauliOp::Z));
    /// ```
    #[must_use]
    pub fn new(pack: (u64, u64)) -> Self {
        Self {
            pack,
        }
    }

    /// Tensor product of identity operators.
    ///
    /// This is the same as `Pauli::default()` or `Pauli::new((0,0))`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    ///
    /// let code = Pauli::identity();
    ///
    /// assert_eq!(code, Pauli::new((0, 0)));
    /// assert_eq!(code, Pauli::default());
    /// assert_eq!(code, Pauli::from_paulis([PauliOp::I]));
    /// ```
    #[must_use]
    pub fn identity() -> Self {
        Self::default()
    }

    /// Enumerate Pauli code.
    ///
    /// This convert the code to a 128-wide integer.
    /// The code consisting of only `Pauli:I` has index zero.
    ///
    /// You can also use implementation of [`From<Pauli>`] for `u128`
    /// (and vice versa).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::Pauli;
    /// let code = Pauli::new((3, 4));
    ///
    /// assert_eq!(code.index(), 3 + (4 << 64));
    /// assert_eq!(u128::from(code), 3 + (4 << 64));
    /// ```
    #[must_use]
    pub fn index(&self) -> u128 {
        u128::from(self.pack.0) + (u128::from(self.pack.1) << 64)
    }

    /// Read out the Pauli operator at site `i`.
    ///
    ///
    /// # Safety
    ///
    /// The user must ensure that that `i` is within 0..64 (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Pauli, PauliOp};
    /// let code = Pauli::new((0b1000, 0));
    /// let pauli = unsafe { code.pauli_unchecked(1) };
    ///
    /// assert_eq!(pauli, PauliOp::Y);
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub unsafe fn pauli_unchecked(
        &self,
        index: u16,
    ) -> PauliOp {
        let pauli_int = if index < 32 {
            (self.pack.0 >> (index * 2)) & PAULI_MASK
        } else {
            (self.pack.1 >> ((index - 32) * 2)) & PAULI_MASK
        };
        PauliOp::try_from(pauli_int).expect("incorrect encoding. This is a bug")
    }

    /// Read out the Pauli operator at site `i`.
    ///
    /// # Returns
    ///
    /// Returns None if index `i` is larger or equal `64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{Pauli, PauliOp};
    /// let code = Pauli::new((0b1000, 0));
    ///
    /// let pauli = code.pauli(1);
    /// assert_eq!(pauli, Some(PauliOp::Y));
    ///
    /// let pauli = code.pauli(64);
    /// assert_eq!(pauli, None);
    /// ```
    #[must_use]
    pub fn pauli(
        &self,
        index: u16,
    ) -> Option<PauliOp> {
        if index >= 64 {
            None
        } else {
            // SAFETY: We just checked if index is within bounds
            Some(unsafe { self.pauli_unchecked(index) })
        }
    }

    /// Modify the Pauli operator in the code at site `i`.
    ///
    /// The supplied closure will receive a mutable reference to the relevant
    /// Pauli.
    ///
    /// # Safety
    ///
    /// The user must ensure that that `i` is within `0..64` (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    /// let mut code = Pauli::new((0, 0b01));
    /// assert_eq!(code.pauli(32), Some(PauliOp::X));
    ///
    /// unsafe {
    ///     code.pauli_mut_unchecked(32, |pauli| *pauli = PauliOp::Y);
    /// }
    ///
    /// assert_eq!(code.pauli(32), Some(PauliOp::Y));
    /// ```
    pub unsafe fn pauli_mut_unchecked<OP>(
        &mut self,
        index: u16,
        f: OP,
    ) where
        OP: FnOnce(&mut PauliOp),
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

    /// Set Pauli operator at `index`.
    ///
    /// # Safety
    ///
    /// The user must ensure that that `i` is within `0..64` (excl.)
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    /// let mut code = Pauli::default();
    /// assert_eq!(code.pauli(17), Some(PauliOp::I));
    ///
    /// unsafe {
    ///     code.set_unchecked(17, PauliOp::Z);
    /// }
    ///
    /// assert_eq!(code.pauli(17), Some(PauliOp::Z));
    /// ```
    pub unsafe fn set_unchecked(
        &mut self,
        index: u16,
        pauli: PauliOp,
    ) {
        self.pauli_mut_unchecked(index, |p| {
            *p = pauli;
        });
    }

    /// Modify the Pauli operator in the code at site `i`.
    ///
    /// If index `i` is less then `64`, the supplied closure will receive a
    /// mutable reference to the relevant Pauli.  Otherwise it will receive
    /// None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    /// let mut code = Pauli::new((0, 0b01));
    /// assert_eq!(code.pauli(32), Some(PauliOp::X));
    ///
    /// code.pauli_mut(32, |x| {
    ///     if let Some(pauli) = x {
    ///         *pauli = PauliOp::Y
    ///     }
    /// });
    /// assert_eq!(code.pauli(32), Some(PauliOp::Y));
    ///
    /// let mut was_it_none = false;
    /// code.pauli_mut(64, |x| {
    ///     if x.is_none() {
    ///         was_it_none = true
    ///     }
    /// });
    /// assert!(was_it_none);
    /// ```
    pub fn pauli_mut<OP>(
        &mut self,
        index: u16,
        f: OP,
    ) where
        OP: FnOnce(Option<&mut PauliOp>),
    {
        if index >= 64 {
            f(None);
        } else {
            // SAFETY: We just checked if index is within bounds
            unsafe {
                self.pauli_mut_unchecked(index, |x: &mut PauliOp| f(Some(x)));
            }
        }
    }

    /// Set Pauli operator at `index`.
    ///
    /// # Panics
    ///
    /// Panics if index outside of `0..64` (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    /// let mut code = Pauli::default();
    /// assert_eq!(code.pauli(17), Some(PauliOp::I));
    ///
    /// code.set(17, PauliOp::Z);
    ///
    /// assert_eq!(code.pauli(17), Some(PauliOp::Z));
    /// ```
    pub fn set(
        &mut self,
        index: u16,
        pauli: PauliOp,
    ) {
        self.pauli_mut(index, |x| {
            if let Some(p) = x {
                *p = pauli;
            } else {
                panic!("index should be within 0..64");
            }
        });
    }

    /// Build the code from an iterator over Paulis.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    /// use f2q::code::qubits::PauliOp::{
    ///     X,
    ///     Y,
    ///     Z,
    /// };
    ///
    /// let code = Pauli::from_paulis([X, Y, Z]);
    ///
    /// assert_eq!(code.pauli(0), Some(X));
    /// assert_eq!(code.pauli(1), Some(Y));
    /// assert_eq!(code.pauli(2), Some(Z));
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn from_paulis<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = PauliOp>,
    {
        let mut code = Self::default();
        for (i, pauli) in iter.into_iter().take(64).enumerate() {
            // SAFETY: we take only 64 elements, so the index must be within
            // bounds
            let i = u16::try_from(i)
                .expect("index out of bounds for type u16. This is a bug");
            unsafe {
                code.set_unchecked(i, pauli);
            }
        }
        code
    }

    /// Parity operator.
    ///
    /// Returns code that consists of a consecutive string of `num_qubits`
    /// [`PauliOp::Z`].
    ///
    /// # Panics
    ///
    /// Panics if `num_qubits > 64`
    ///
    /// # Examples
    ///
    ///
    /// ```rust
    /// # use f2q::code::qubits::{PauliOp, Pauli};
    ///
    /// let par_op = Pauli::parity_op(2);
    ///
    /// assert_eq!(par_op.pauli(0), Some(PauliOp::Z));
    /// assert_eq!(par_op.pauli(1), Some(PauliOp::Z));
    /// assert_eq!(par_op.pauli(2), Some(PauliOp::I));
    ///
    /// assert_eq!(Pauli::parity_op(0), Pauli::default());
    /// ```
    #[must_use]
    pub fn parity_op(num_qubits: u16) -> Self {
        assert!(num_qubits <= 64, "number of qubits must be within 1..=64");

        Pauli::from_paulis((0..num_qubits).map(|_| PauliOp::Z))
    }
}

/// Iterate over Paulis in `Pauli`
#[derive(Debug)]
pub struct PauliIter {
    code:  Pauli,
    index: u16,
}

impl PauliIter {
    fn new(code: Pauli) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl Iterator for PauliIter {
    type Item = PauliOp;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 64 {
            return None;
        }

        let pauli = self.code.pauli(self.index);
        self.index += 1;
        pauli
    }
}

impl IntoIterator for Pauli {
    type IntoIter = PauliIter;
    type Item = PauliOp;

    fn into_iter(self) -> Self::IntoIter {
        PauliIter::new(self)
    }
}

impl Display for Pauli {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.index() == 0 {
            write!(f, "I")
        } else {
            let mut pauli_str = String::with_capacity(64);
            for pauli in self.into_iter() {
                let ch = match pauli {
                    PauliOp::I => 'I',
                    PauliOp::X => 'X',
                    PauliOp::Y => 'Y',
                    PauliOp::Z => 'Z',
                };
                pauli_str.push(ch);
            }

            write!(f, "{}", pauli_str.trim_end_matches('I'))
        }
    }
}

impl From<Pauli> for u128 {
    fn from(value: Pauli) -> Self {
        value.index()
    }
}

impl From<u128> for Pauli {
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: u128) -> Self {
        Self::new((value as u64, (value >> 64) as u64))
    }
}

mod pauli_group {
    use std::ops::Mul;

    use crate::{
        code::qubits::{
            Pauli,
            PauliOp,
        },
        math::{
            Group,
            Root4,
        },
    };

    struct PGrp(Root4, PauliOp);

    impl Mul for PGrp {
        type Output = Self;

        fn mul(
            self,
            rhs: Self,
        ) -> Self::Output {
            use Root4::{
                R0,
                R2,
                R3,
            };
            let (omega, pauli) = match self.1 {
                PauliOp::I => (R0, rhs.1),
                PauliOp::X => match rhs.1 {
                    PauliOp::I => (R0, PauliOp::X),
                    PauliOp::X => (R0, PauliOp::I),
                    PauliOp::Y => (R2, PauliOp::Z),
                    PauliOp::Z => (R2, PauliOp::Y),
                },
                PauliOp::Y => match rhs.1 {
                    PauliOp::I => (R0, PauliOp::Y),
                    PauliOp::X => (R3, PauliOp::Z),
                    PauliOp::Y => (R0, PauliOp::I),
                    PauliOp::Z => (R2, PauliOp::X),
                },
                PauliOp::Z => match rhs.1 {
                    PauliOp::I => (R0, PauliOp::Z),
                    PauliOp::X => (R3, PauliOp::Y),
                    PauliOp::Y => (R3, PauliOp::X),
                    PauliOp::Z => (R0, PauliOp::I),
                },
            };

            PGrp(self.0 * rhs.0 * omega, pauli)
        }
    }

    impl Group for PGrp {
        fn identity() -> Self {
            Self(Root4::R0, PauliOp::I)
        }

        fn inverse(self) -> Self {
            Self(self.0.inverse(), self.1)
        }
    }

    /// Cross-product Root4 x `Pauli`
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PauliGroup(Root4, Pauli);

    impl PauliGroup {
        #[must_use]
        pub fn new(
            omega: Root4,
            code: Pauli,
        ) -> Self {
            Self(omega, code)
        }

        #[must_use]
        pub fn is_hermitian(&self) -> bool {
            self.0 == Root4::R0 || self.0 == Root4::R1
        }
    }

    impl From<Pauli> for PauliGroup {
        fn from(value: Pauli) -> Self {
            Self::new(Root4::identity(), value)
        }
    }

    impl From<Root4> for PauliGroup {
        fn from(value: Root4) -> Self {
            Self::new(value, Pauli::default())
        }
    }

    impl From<PauliGroup> for (Root4, Pauli) {
        fn from(value: PauliGroup) -> Self {
            (value.0, value.1)
        }
    }

    impl Mul for PauliGroup {
        type Output = Self;

        fn mul(
            self,
            rhs: Self,
        ) -> Self::Output {
            self.1.into_iter().enumerate().fold(
                PauliGroup::identity(),
                |acc, (i, pauli_lhs)| {
                    let mut code = acc.1;
                    let lhs = PGrp(Root4::R0, pauli_lhs);
                    let i = u16::try_from(i).expect(
                        "index out of bounds for type u16. This is a bug",
                    );
                    // SAFETY: index i is within bound
                    // since it enumerates a valid Pauli
                    let rhs =
                        PGrp(Root4::R0, unsafe { rhs.1.pauli_unchecked(i) });

                    let prod = lhs * rhs;
                    // SAFETY: index i is within bound
                    // since it enumerates a valid Pauli
                    unsafe {
                        code.set_unchecked(i, prod.1);
                    }
                    PauliGroup(acc.0 * prod.0, code)
                },
            )
        }
    }

    impl Group for PauliGroup {
        fn identity() -> Self {
            Self(Root4::identity(), Pauli::default())
        }

        fn inverse(self) -> Self {
            Self(self.0.inverse(), self.1)
        }
    }
}

pub use pauli_group::PauliGroup;
