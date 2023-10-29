//! Mathematical structures.

use std::ops::{
    Mul,
    Neg,
};

use num::One;

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
    #[must_use]
    pub fn one() -> Self {
        Self::R0
    }

    /// Complex number: `i`.
    #[must_use]
    pub fn i() -> Self {
        Self::R2
    }

    /// Complex conjugation.
    #[must_use]
    pub fn conj(self) -> Self {
        match self {
            Root4::R0 => Root4::R0,
            Root4::R1 => Root4::R1,
            Root4::R2 => Root4::R3,
            Root4::R3 => Root4::R2,
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
        match self {
            Root4::R0 => Root4::R1,
            Root4::R1 => Root4::R0,
            Root4::R2 => Root4::R3,
            Root4::R3 => Root4::R2,
        }
    }
}

impl Group for Root4 {
    fn identity() -> Self {
        Self::R0
    }

    fn inverse(self) -> Self {
        match self {
            Root4::R0 => Root4::R0,
            Root4::R1 => Root4::R1,
            Root4::R2 => Root4::R3,
            Root4::R3 => Root4::R2,
        }
    }
}

/// A complex number that can only be either real or imaginary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReIm<T> {
    Zero,
    Re(T),
    Im(T),
}

impl<T> Mul for ReIm<T>
where
    T: Mul<Output = T> + Neg<Output = T>,
{
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        match self {
            Self::Zero => Self::Zero,
            Self::Re(x) => match rhs {
                Self::Zero => Self::Zero,
                Self::Re(y) => Self::Re(x * y),
                Self::Im(y) => Self::Im(x * y),
            },
            Self::Im(x) => match rhs {
                Self::Zero => Self::Zero,
                Self::Re(y) => Self::Im(x * y),
                Self::Im(y) => Self::Re(-x * y),
            },
        }
    }
}

impl<T> From<Root4> for ReIm<T>
where
    T: Neg<Output = T> + One,
{
    fn from(value: Root4) -> Self {
        use Root4::{
            R0,
            R1,
            R2,
            R3,
        };
        match value {
            R0 => ReIm::Re(T::one()),
            R1 => ReIm::Re(-T::one()),
            R2 => ReIm::Im(T::one()),
            R3 => ReIm::Im(-T::one()),
        }
    }
}
