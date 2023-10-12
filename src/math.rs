//! Various mathematical structures

use std::ops::Mul;

use num::One;

/// Iterate over all pairs in a slice.
#[derive(Debug)]
pub struct Pairs<'a, T> {
    data: &'a [T],
    i:    usize,
    j:    usize,
}

impl<'a, T> Pairs<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            i: 0,
            j: 0,
        }
    }
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            return None;
        }

        let out = (&self.data[self.i], &self.data[self.j]);

        self.j += 1;

        if self.j >= self.data.len() {
            self.j = 0;
            self.i += 1;
        }

        Some(out)
    }
}

/// Group structure.
pub trait Group: Mul<Output = Self> + One {
    fn inverse(self) -> Self;
}

/// 4th order roots of unity
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Root4 {
    /// R0 = 1
    R0,
    /// R1 = -1
    R1,
    /// R2 = i
    R2,
    /// R3 = -i
    R3,
}

impl One for Root4 {
    fn one() -> Self {
        Self::R0
    }
}

impl Mul for Root4 {
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self::Output {
        use Root4::*;
        match self {
            R0 => rhs,
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

impl Group for Root4 {
    fn inverse(self) -> Self {
        use Root4::*;
        match self {
            R0 => R0,
            R1 => R1,
            R2 => R3,
            R3 => R2,
        }
    }
}

#[test]
fn root4_identity() {
    let e = Root4::one();

    assert_eq!(e, Root4::R0);
}

#[test]
fn root4_inverse() {
    assert_eq!(Root4::R0.inverse(), Root4::R0);
    assert_eq!(Root4::R1.inverse(), Root4::R1);
    assert_eq!(Root4::R2.inverse(), Root4::R3);
    assert_eq!(Root4::R3.inverse(), Root4::R2);
}


#[test]
fn root4_mul() {
    use Root4::*;

    assert_eq!(R0 * R0, R0);
    assert_eq!(R0 * R1, R1);
    assert_eq!(R0 * R2, R2);
    assert_eq!(R0 * R3, R3);

    assert_eq!(R1 * R0, R1);
    assert_eq!(R1 * R1, R0);
    assert_eq!(R1 * R2, R3);
    assert_eq!(R1 * R3, R2);

    assert_eq!(R2 * R0, R2);
    assert_eq!(R2 * R1, R3);
    assert_eq!(R2 * R2, R1);
    assert_eq!(R2 * R3, R0);

    assert_eq!(R3 * R0, R3);
    assert_eq!(R3 * R1, R2);
    assert_eq!(R3 * R2, R0);
    assert_eq!(R3 * R3, R1);
}