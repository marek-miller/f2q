//! Qubit representation

use crate::{
    Code,
    Error,
};

const PAULI_MASK: u64 = 0b11;

/// Pauli operator
///
/// # Examples
///
/// ```rust
/// # use f2q::qubit::Pauli;
/// use f2q::Error::PauliIndex;
///
/// let paulis: Vec<_> = (0..=4).map(|i| Pauli::try_from(i)).collect();
///
/// assert_eq!(
///     paulis[0..=3],
///     [Ok(Pauli::I), Ok(Pauli::X), Ok(Pauli::Y), Ok(Pauli::Z),]
/// );
/// matches!(paulis[4], Err(PauliIndex { .. }));
/// ```
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
                        _ => Err(Self::Error::PauliIndex{ msg: "PauliCode index should be within 0..=3".to_string()}),
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
///
/// # Examples
///
/// ```rust
/// # use f2q::qubit::PauliCode;
/// let code = PauliCode::default();
///
/// assert_eq!(code.enumerate(), 0);
/// ```
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
    /// The first integer in the tuple represents qubits 0 to 31 (incl.),
    /// and the second integer represents qubits 32 to 63 (incl.).
    /// The pairs of bits in each integer follow the little-endian convention.
    /// For example, `PauliCode::new((0b1001, 0))` represents the following
    /// Pauli product of 64 Pauli operators:
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
    ///
    /// with `Z`'s 0-based index 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::qubit::{Pauli, PauliCode};
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

    /// Enumerate Pauli code.
    ///
    /// This convert the code to a 128-wide integer.
    /// The code consisting of only `Pauli:I` has index zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::qubit::PauliCode;
    /// let code = PauliCode::new((3, 4));
    ///
    /// assert_eq!(code.enumerate(), 3 + (4 << 64));
    /// ```
    #[must_use]
    pub fn enumerate(&self) -> u128 {
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
    /// # use f2q::qubit::{PauliCode, Pauli};
    /// let code = PauliCode::new((0b1000, 0));
    /// let pauli = unsafe { code.pauli_unchecked(1) };
    ///
    /// assert_eq!(pauli, Pauli::Y);
    /// ```
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

    /// Read out the Pauli operator at site `i`.
    ///
    /// # Returns
    ///
    /// Returns None if index `i` is larger or equal `64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::qubit::{PauliCode, Pauli};
    /// let code = PauliCode::new((0b1000, 0));
    ///
    /// let pauli = code.pauli(1);
    /// assert_eq!(pauli, Some(Pauli::Y));
    ///
    /// let pauli = code.pauli(64);
    /// assert_eq!(pauli, None);
    /// ```
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
    /// # use f2q::qubit::{Pauli, PauliCode};
    /// let mut code = PauliCode::new((0, 0b01));
    /// assert_eq!(code.pauli(32), Some(Pauli::X));
    ///
    /// unsafe {
    ///     code.pauli_mut_unchecked(32, |pauli| *pauli = Pauli::Y);
    /// }
    ///
    /// assert_eq!(code.pauli(32), Some(Pauli::Y));
    /// ```
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

    /// Modify the Pauli operator in the code at site `i`.
    ///
    /// If index `i` is less then `64`, the supplied closure will receive a
    /// mutable reference to the relevant Pauli.  Otherwise it will receive
    /// None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::qubit::{Pauli, PauliCode};
    /// let mut code = PauliCode::new((0, 0b01));
    /// assert_eq!(code.pauli(32), Some(Pauli::X));
    ///
    /// code.pauli_mut(32, |x| {
    ///     if let Some(pauli) = x {
    ///         *pauli = Pauli::Y
    ///     }
    /// });
    /// assert_eq!(code.pauli(32), Some(Pauli::Y));
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

    /// Set Pauli operator at `index`.
    ///
    /// # Panics
    ///
    /// Panics if index outside of `0..64` (excl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::qubit::{Pauli, PauliCode};
    /// let mut code = PauliCode::default();
    /// assert_eq!(code.pauli(17), Some(Pauli::I));
    ///
    /// code.set(17, Pauli::Z);
    ///
    /// assert_eq!(code.pauli(17), Some(Pauli::Z));
    /// ```
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

    /// Build the code from an iterator over Paulis.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::qubit::{Pauli, PauliCode};
    /// use f2q::qubit::Pauli::{
    ///     X,
    ///     Y,
    ///     Z,
    /// };
    ///
    /// let code = PauliCode::from_paulis([X, Y, Z]);
    ///
    /// assert_eq!(code.pauli(0), Some(X));
    /// assert_eq!(code.pauli(1), Some(Y));
    /// assert_eq!(code.pauli(2), Some(Z));
    /// ```
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

impl IntoIterator for PauliCode {
    type IntoIter = PauliIter;
    type Item = Pauli;

    fn into_iter(self) -> Self::IntoIter {
        PauliIter::new(self)
    }
}

/// Iterate over Paulis in `PauliCode`
#[derive(Debug)]
pub struct PauliIter {
    code:  PauliCode,
    index: usize,
}

impl PauliIter {
    fn new(code: PauliCode) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl Iterator for PauliIter {
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
