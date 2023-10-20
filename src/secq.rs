//! Second quantization

// Describe canonical ordering of indices in Hamiltonian

use std::{
    fmt::Display,
    ops::{
        Bound,
        RangeBounds,
    },
};

use serde::{
    de::Visitor,
    ser::SerializeSeq,
    Deserialize,
    Serialize,
};

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
    /// # use f2q::secq::Spin;
    ///
    /// let spin = Spin::Up;
    /// assert!(spin.is_up());
    ///
    /// let spin = Spin::Down;
    /// assert!(!spin.is_up());
    /// ```
    #[must_use]
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
    /// # use f2q::secq::Spin;
    ///
    /// let spin = Spin::Down;
    /// assert_eq!(spin.flip(), Spin::Up);
    /// ```
    #[must_use]
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
    /// # use f2q::secq::Spin;
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
        if value {
            Spin::Up
        } else {
            Spin::Down
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
    /// # use f2q::secq::{Orbital, Spin};
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
    /// # use f2q::secq::{Orbital, Spin};
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
    /// # use f2q::secq::{Orbital, Spin};
    ///
    /// let orbital = Orbital::from_index(3);
    ///
    /// assert_eq!(orbital, Orbital::new(1, Spin::Up));
    /// ```
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        Self::new(index / 2, Spin::from(index & 1 != 0))
    }

    /// Generate orbitals with indeces in the given range.
    ///
    /// If the start bound is unbounded, the iterator starts from zero.
    /// If the end bound is unbounded, it is taken to be `usize::MAX` (incl.)
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::secq::{Orbital, Spin};
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
        OrbitalRange::new(range)
    }
}

struct OrbitalRange {
    end:   Option<usize>,
    index: Option<usize>,
}

impl OrbitalRange {
    fn new<R>(range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let index = match range.start_bound() {
            Bound::Included(&x) => Some(x),
            Bound::Excluded(&x) if x < usize::MAX => Some(x + 1),
            Bound::Excluded(_) => None,
            Bound::Unbounded => Some(0),
        };

        let end = match range.end_bound() {
            Bound::Included(&y) => Some(y),
            Bound::Excluded(&y) if y > 0 => Some(y - 1),
            Bound::Excluded(_) => None,
            Bound::Unbounded => Some(usize::MAX),
        };

        Self {
            end,
            index,
        }
    }
}

impl Iterator for OrbitalRange {
    type Item = Orbital;

    fn next(&mut self) -> Option<Self::Item> {
        match self.end {
            Some(end) => match &mut self.index {
                Some(i) => {
                    if *i > end {
                        None
                    } else {
                        let orbital = Orbital::from_index(*i);
                        if *i < end {
                            *i += 1;
                        } else {
                            self.end = None;
                        }
                        Some(orbital)
                    }
                }
                None => None,
            },
            None => None,
        }
    }
}

/// Electronic integral with creation (cr) and annihilation (an)
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
pub enum Fermions {
    #[default]
    Offset,
    One {
        cr: Cr,
        an: An,
    },
    Two {
        cr: (Cr, Cr),
        an: (An, An),
    },
}

impl Fermions {
    /// Create Integral as constant offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::secq::Fermions;
    ///
    /// let integral = Fermions::new();
    ///
    /// assert_eq!(integral, Fermions::Offset);
    /// ```
    #[must_use]
    pub fn new() -> Fermions {
        Self::default()
    }

    /// Create a one-electron integral.
    ///
    /// Orbitals must be in canonical order: `cr.index() <= an.index()`,
    /// otherwise return None.
    ///
    /// ```rust
    /// # use f2q::secq::{Fermions, Orbital, Spin, Cr, An};
    ///
    /// let integral = Fermions::one_electron(
    ///     Cr(Orbital::new(0, Spin::Down)),
    ///     An(Orbital::new(1, Spin::Down)),
    /// );
    /// assert!(integral.is_some());
    ///
    /// let integral = Fermions::one_electron(
    ///     Cr(Orbital::new(1, Spin::Down)),
    ///     An(Orbital::new(0, Spin::Down)),
    /// );
    /// assert!(integral.is_none());
    /// ```
    #[must_use]
    pub fn one_electron(
        cr: Cr,
        an: An,
    ) -> Option<Self> {
        (cr.index() <= an.index()).then_some(Self::One {
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
    /// # use f2q::secq::{Fermions, Orbital, Spin, Cr, An};
    ///
    /// let integral = Fermions::two_electron(
    ///     (
    ///         Cr(Orbital::new(0, Spin::Down)),
    ///         Cr(Orbital::new(0, Spin::Up)),
    ///     ),
    ///     (
    ///         An(Orbital::new(1, Spin::Down)),
    ///         An(Orbital::new(0, Spin::Down)),
    ///     ),
    /// );
    /// assert!(integral.is_some());
    ///
    /// let integral = Fermions::two_electron(
    ///     (
    ///         Cr(Orbital::new(0, Spin::Down)),
    ///         Cr(Orbital::new(0, Spin::Down)),
    ///     ),
    ///     (
    ///         An(Orbital::new(1, Spin::Down)),
    ///         An(Orbital::new(0, Spin::Down)),
    ///     ),
    /// );
    /// assert!(integral.is_none());
    /// ```
    #[must_use]
    pub fn two_electron(
        cr: (Cr, Cr),
        an: (An, An),
    ) -> Option<Self> {
        (cr.0.index() < cr.1.index()
            && an.0.index() > an.1.index()
            && cr.0.index() <= an.1.index())
        .then_some(Self::Two {
            cr,
            an,
        })
    }
}

impl Display for Fermions {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Fermions::Offset => write!(f, "[]"),
            Fermions::One {
                cr,
                an,
            } => write!(f, "[{}, {}]", cr.index(), an.index()),
            Fermions::Two {
                cr,
                an,
            } => write!(
                f,
                "[{}, {}, {}, {}]",
                cr.0.index(),
                cr.1.index(),
                an.0.index(),
                an.1.index()
            ),
        }
    }
}

impl Serialize for Fermions {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Fermions::Offset => {
                let seq = serializer.serialize_seq(Some(0))?;
                seq.end()
            }
            Fermions::One {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&cr.index())?;
                seq.serialize_element(&an.index())?;
                seq.end()
            }
            Fermions::Two {
                cr,
                an,
            } => {
                let mut seq = serializer.serialize_seq(Some(4))?;
                seq.serialize_element(&cr.0.index())?;
                seq.serialize_element(&cr.1.index())?;
                seq.serialize_element(&an.0.index())?;
                seq.serialize_element(&an.1.index())?;
                seq.end()
            }
        }
    }
}

struct FermionsVisitor;

impl<'de> Visitor<'de> for FermionsVisitor {
    type Value = Fermions;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("sequence of 0, 2 or 4 orbital indices")
    }

    fn visit_seq<A>(
        self,
        seq: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        use serde::de::Error;

        let mut seq = seq;
        let idx_tup: (
            Option<usize>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
        ) = (
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
            seq.next_element()?,
        );

        match idx_tup {
            (None, None, None, None) => Ok(Fermions::Offset),
            (Some(p), Some(q), None, None) => Fermions::one_electron(
                Cr(Orbital::from_index(p)),
                An(Orbital::from_index(q)),
            )
            .ok_or(A::Error::custom("cannot parse one-electron term")),
            (Some(p), Some(q), Some(r), Some(s)) => Fermions::two_electron(
                (Cr(Orbital::from_index(p)), Cr(Orbital::from_index(q))),
                (An(Orbital::from_index(r)), An(Orbital::from_index(s))),
            )
            .ok_or(A::Error::custom("cannot parse two-electron term")),
            _ => Err(A::Error::custom("cannot parse sequence")),
        }
    }
}

impl<'de> Deserialize<'de> for Fermions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FermionsVisitor)
    }
}

/// Creation operator
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cr(pub Orbital);

impl Cr {
    #[must_use]
    pub fn index(&self) -> usize {
        self.0.index()
    }
}

/// Annihilation operator
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct An(pub Orbital);

impl An {
    #[must_use]
    pub fn index(&self) -> usize {
        self.0.index()
    }
}
