//! Second quantization

// Describe canonical ordering of indices in Hamiltonian

use std::ops::{
    Bound,
    RangeBounds,
};

use crate::Code;

/// Spin one-half
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Spin {
    #[default]
    Down,
    Up,
}

impl Spin {
    /// True if spin up.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hamil_repr::sec::Spin;
    ///
    /// let spin = Spin::Up;
    /// assert!(spin.is_up());
    ///
    /// let spin = Spin::Down;
    /// assert!(!spin.is_up());
    /// ```
    pub fn is_up(&self) -> bool {
        match self {
            Self::Down => false,
            Self::Up => true,
        }
    }

    /// Flip the spin to its opposite.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil_repr::sec::Spin;
    ///
    /// let spin = Spin::Down;
    /// assert_eq!(spin.flip(), Spin::Up);
    /// ```
    pub fn flip(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
        }
    }

    /// Iterate over both configurations: down and then up.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hamil_repr::sec::Spin;
    ///
    /// let spins: Vec<_> = Spin::both().collect();
    ///
    /// assert_eq!(spins, &[Spin::Down, Spin::Up]);
    /// ```
    pub fn both() -> impl Iterator<Item = Self> {
        [Spin::Down, Spin::Up].into_iter()
    }
}

impl From<bool> for Spin {
    fn from(value: bool) -> Self {
        match value {
            false => Spin::Down,
            true => Spin::Up,
        }
    }
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

/// Electronic orbital consisting of a principal quantum number and a spin 1/2.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct Orbital {
    pub n: usize,
    pub s: Spin,
}

impl Orbital {
    /// Create a new orbital.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil_repr::sec::{Orbital, Spin};
    ///
    /// let orb = Orbital::new(0, Spin::Down);
    /// assert_eq!(orb.index(), 0);
    /// ```
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

    /// Compute orbital index.
    ///
    /// Orbitals can be enumerated according to the formula: `2*n + s`,
    /// where `n` is the principal quantum number and `s` is the spin number,
    /// i.e. `s = 0` for `Spin::Down` and `s = 1` for `Spin::Up`.
    ///
    /// # Panics
    ///
    /// Panics is the orbitals index cannot fit into `usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil_repr::sec::{Orbital, Spin};
    ///
    /// let orb = Orbital::new(0, Spin::Down);
    /// assert_eq!(orb.index(), 0);
    ///
    /// let orb = Orbital::new(3, Spin::Up);
    /// assert_eq!(orb.index(), 7);
    /// ```
    #[must_use]
    pub fn index(&self) -> usize {
        assert!(
            self.n <= usize::MAX / 2 - usize::from(self.s),
            "orbital index out of bound"
        );
        self.n * 2 + usize::from(self.s)
    }

    /// Return orbital corresponding to the given index.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil_repr::sec::{Orbital, Spin};
    ///
    /// let orbital = Orbital::from_index(3);
    ///
    /// assert_eq!(orbital, Orbital::new(1, Spin::Up));
    /// ```
    pub fn from_index(idx: usize) -> Self {
        Self::new(idx / 2, Spin::from(idx & 1 != 0))
    }

    /// Generate orbitals with indeces in the given range.
    ///
    /// If the start bound is unbounded, the iterator starts from zero.
    /// If the end bound is unbounded, it is taken to be usize::MAx (incl.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil_repr::sec::{Orbital, Spin};
    ///
    /// let orbitals: Vec<_> = Orbital::gen_range((1..=3)).collect();
    ///
    /// assert_eq!(
    ///     orbitals,
    ///     &[
    ///         Orbital::new(0, Spin::Up),
    ///         Orbital::new(1, Spin::Down),
    ///         Orbital::new(1, Spin::Up)
    ///     ]
    /// )
    /// ```
    pub fn gen_range<R>(range: R) -> impl Iterator<Item = Orbital>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) if x < usize::MAX => x + 1,
            Bound::Excluded(_) => usize::MAX,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) if x > 0 => x - 1,
            Bound::Excluded(_) => 0,
            Bound::Unbounded => usize::MAX,
        };

        (start..=end).map(Orbital::from_index)
    }
}

/// Electronic integral with creation (cr) and annihilation (hr)
/// operators indexed by orbitals in canonical order:
///
/// - If Integral is a one-electron integral and `p = cr.index()`, `q =
///   an.index()`, then `p <= q`
/// - If Integral is a two-electron integral and
///
///   ```text
///   p = cr.0.index()
///   q = cr.1.index()
///   r = an.0.index()
///   s = an.0.index()
///   ```
///
///   then `p < q`, `r > s` and `p <= s`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

impl Integral {
    /// Create Integral as constant offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use hamil_repr::sec::Integral;
    ///
    /// let integral = Integral::new();
    ///
    /// assert_eq!(integral, Integral::Constant);
    /// ```
    pub fn new() -> Integral {
        Self::default()
    }

    /// Create a one-electron integral.
    ///
    /// Orbitals must be in canonical order: `cr.index() <= an.index()`,
    /// otherwise return None.
    ///
    /// ```rust
    /// # use hamil_repr::sec::{Integral, Orbital, Spin};
    ///
    /// let integral = Integral::one_electron(
    ///     Orbital::new(0, Spin::Down),
    ///     Orbital::new(1, Spin::Down),
    /// );
    /// assert!(integral.is_some());
    ///
    /// let integral = Integral::one_electron(
    ///     Orbital::new(1, Spin::Down),
    ///     Orbital::new(0, Spin::Down),
    /// );
    /// assert!(integral.is_none());
    /// ```
    pub fn one_electron(
        cr: Orbital,
        an: Orbital,
    ) -> Option<Self> {
        (cr.index() <= an.index()).then_some(Self::OneElectron {
            cr,
            an,
        })
    }

    /// Create a two-electron integral.
    ///
    /// Orbitals must be in canonical order:
    ///
    /// ```text
    /// cr.0.index() < cr.1.index()
    /// an.0.index() > an.1.index()
    /// cr.0.index() <= an.1.index()
    /// ```
    /// otherwise return None.
    ///
    /// ```rust
    /// # use hamil_repr::sec::{Integral, Orbital, Spin};
    ///
    /// let integral = Integral::two_electron(
    ///     (Orbital::new(0, Spin::Down), Orbital::new(0, Spin::Up)),
    ///     (Orbital::new(1, Spin::Down), Orbital::new(0, Spin::Down)),
    /// );
    /// assert!(integral.is_some());
    /// 
    /// let integral = Integral::two_electron(
    ///     (Orbital::new(0, Spin::Down), Orbital::new(0, Spin::Down)),
    ///     (Orbital::new(1, Spin::Down), Orbital::new(0, Spin::Down)),
    /// );
    /// assert!(integral.is_none());
    /// ```
    pub fn two_electron(
        cr: (Orbital, Orbital),
        an: (Orbital, Orbital),
    ) -> Option<Self> {
        (cr.0.index() < cr.1.index()
            && an.0.index() > an.1.index()
            && cr.0.index() <= an.1.index())
        .then_some(Self::TwoElectron {
            cr,
            an,
        })
    }
}

impl Code for Integral {}
