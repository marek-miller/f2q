//! Second quantization
use crate::{
    Code,
    Hamil,
};

pub type FermiHamil<T> = Hamil<T, Integral>;
pub type FermiSum<T> = crate::SumRepr<T, Integral>;

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
/// [doc](https://learn.microsoft.com/en-us/azure/quantum/user-guide/libraries/chemistry/concepts/second-quantization)
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum Integral {
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

impl Code for Integral {}
