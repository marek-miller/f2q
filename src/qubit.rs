//! Qubit representation

use crate::Code;
pub use crate::Error;

const PAULI_MASK: u64 = 0b11;

/// Pauli operator
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
                        _ => Err(Self::Error::CodeIndex),
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
