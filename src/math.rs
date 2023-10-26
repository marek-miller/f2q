//! Mathematical structures.

use std::ops::{
    Mul,
    Neg,
};

pub fn pairs<'a, T, K>(
    x: &'a [T],
    y: &'a [K],
) -> impl Iterator<Item = (&'a T, &'a K)> {
    x.iter().flat_map(|i| y.iter().map(move |j| (i, j)))
}


/// Group structure.
pub trait Group: Mul<Output = Self> + Sized {
    fn identity() -> Self;
    #[must_use]
    fn inverse(self) -> Self;
}

/// 4th order roots of unity
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Root4 {
    /// 1
    #[default]
    R0,
    /// -1
    R1,
    /// i
    R2,
    /// -i
    R3,
}

impl Root4 {
    /// Identity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::math::Root4;
    /// assert_eq!(Root4::one(), Root4::R0);
    /// assert_eq!(Root4::one(), Root4::default());
    /// ```
    #[must_use]
    pub fn one() -> Self {
        Self::R0
    }

    /// Complex number: `i`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::math::Root4;
    /// assert_eq!(Root4::i(), Root4::R2);
    /// ```
    #[must_use]
    pub fn i() -> Self {
        Self::R2
    }

    /// Complex conjugation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use f2q::math::Root4;
    /// assert_eq!(Root4::R0.conj(), Root4::R0);
    /// assert_eq!(Root4::R1.conj(), Root4::R1);
    /// assert_eq!(Root4::R2.conj(), Root4::R3);
    /// assert_eq!(Root4::R3.conj(), Root4::R2);
    /// ```
    #[must_use]
    pub fn conj(self) -> Self {
        use Root4::{
            R0,
            R1,
            R2,
            R3,
        };
        match self {
            R0 => R0,
            R1 => R1,
            R2 => R3,
            R3 => R2,
        }
    }
}

impl Mul for Root4 {
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        use Root4::{
            R0,
            R1,
            R2,
            R3,
        };
        match self {
            R0 => match rhs {
                R0 => R0,
                R1 => R1,
                R2 => R2,
                R3 => R3,
            },
            R1 => match rhs {
                R0 => R1,
                R1 => R0,
                R2 => R3,
                R3 => R2,
            },
            R2 => match rhs {
                R0 => R2,
                R1 => R3,
                R2 => R1,
                R3 => R0,
            },
            R3 => match rhs {
                R0 => R3,
                R1 => R2,
                R2 => R0,
                R3 => R1,
            },
        }
    }
}

impl Neg for Root4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Root4::{
            R0,
            R1,
            R2,
            R3,
        };
        match self {
            R0 => R1,
            R1 => R0,
            R2 => R3,
            R3 => R2,
        }
    }
}

impl Group for Root4 {
    fn identity() -> Self {
        Self::R0
    }

    fn inverse(self) -> Self {
        use Root4::{
            R0,
            R1,
            R2,
            R3,
        };
        match self {
            R0 => R0,
            R1 => R1,
            R2 => R3,
            R3 => R2,
        }
    }
}
