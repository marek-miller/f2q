use std::ops::Mul;

use num::{
    Complex,
    Float,
    One,
};

use crate::{
    code::{
        fermions::{
            An,
            Cr,
            Fermions,
            Orbital,
        },
        qubits::{
            Pauli,
            PauliGroup,
            PauliOp,
        },
    },
    terms::{
        SumRepr,
        Terms,
    },
    Error,
};

enum JWMap {
    Cr(Orbital),
    An(Orbital),
}

impl TryFrom<Cr> for JWMap {
    type Error = Error;

    fn try_from(value: Cr) -> Result<Self, Self::Error> {
        (value.index() < 64)
            .then_some(Self::Cr(value.0))
            .ok_or_else(|| Error::QubitIndex {
                msg: "orbital index must be within 0..=63".to_string(),
            })
    }
}

impl TryFrom<An> for JWMap {
    type Error = Error;

    fn try_from(value: An) -> Result<Self, Self::Error> {
        (value.index() < 64)
            .then_some(Self::An(value.0))
            .ok_or_else(|| Error::QubitIndex {
                msg: "orbital index must be within 0..=63".to_string(),
            })
    }
}

fn pauli_codes_from_index(index: u16) -> (Pauli, Pauli) {
    let code = Pauli::parity_op(index.saturating_sub(1));

    let x = {
        let mut code = code;
        code.set(index, PauliOp::X);
        code
    };
    let y = {
        let mut code = code;
        code.set(index, PauliOp::Y);
        code
    };

    (x, y)
}

impl JWMap {
    fn index(&self) -> u16 {
        u16::try_from(match self {
            Self::An(an) => an.index(),
            Self::Cr(cr) => cr.index(),
        })
        .expect("index within 0..=63")
    }

    fn mul_iter<'a, T, I>(
        &'a self,
        rhs: I,
    ) -> impl Iterator<Item = (Complex<T>, Pauli)> + 'a
    where
        T: Float + 'a,
        I: Iterator<Item = (Complex<T>, Pauli)> + 'a,
    {
        let one_half =
            T::from(0.5_f64).expect("floating point conversion from 0.5");

        let (x, y) = pauli_codes_from_index(self.index());

        rhs.flat_map(move |(rhs_coeff, rhs_pauli)| {
            let (root_x, prod_x) = x * rhs_pauli;
            let (root_y, prod_y) = y * rhs_pauli;

            let term_x =
                rhs_coeff * Complex::from(one_half) * Complex::from(root_x);
            let term_y = rhs_coeff
                * match self {
                    Self::An(_) => Complex::new(T::zero(), one_half),
                    Self::Cr(_) => Complex::new(T::zero(), -one_half),
                }
                * Complex::from(root_y);

            [(term_x, prod_x), (term_y, prod_y)].into_iter()
        })
    }
}

fn jw_map_two_hemitian<'a, T: Float + 'a>(
    op1: &'a JWMap,
    op2: &'a JWMap,
    coeff: T,
) -> Result<impl Iterator<Item = (T, Pauli)> + 'a, Error> {
    let start = [(Complex::from(coeff), Pauli::identity())].into_iter();

    Ok(op1
        .mul_iter(op2.mul_iter(start))
        .filter(|(x, _)| x.re != T::zero())
        .map(|(x, p)| (x.re, p)))
}

pub struct Map(Fermions);

impl Map {
    pub fn pauli_iter<T>(
        &self,
        coeff: T,
    ) -> impl Iterator<Item = (T, Pauli)>
    where
        T: Float,
    {
        PauliIter::new(coeff, self.0)
    }
}

impl TryFrom<Fermions> for Map {
    type Error = Error;

    fn try_from(value: Fermions) -> Result<Self, Self::Error> {
        match value {
            Fermions::Offset => Ok(Self(value)),
            Fermions::One {
                cr,
                an,
            } => {
                if cr.index() < 64 && an.index() < 64 {
                    Ok(Self(value))
                } else {
                    Err(Error::QubitIndex {
                        msg: "orbital index must be less than 64".to_string(),
                    })
                }
            }
            Fermions::Two {
                cr,
                an,
            } => {
                if cr.0.index() < 64
                    && cr.1.index() < 64
                    && an.0.index() < 64
                    && an.1.index() < 64
                {
                    Ok(Self(value))
                } else {
                    Err(Error::QubitIndex {
                        msg: "orbital index must be less than 64".to_string(),
                    })
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PauliIter<T> {
    coeff: T,
    code:  Fermions,
    index: u8,
}

impl<T> PauliIter<T> {
    pub fn new(
        coeff: T,
        code: Fermions,
    ) -> Self {
        Self {
            coeff,
            code,
            index: 0,
        }
    }
}

impl<T> Iterator for PauliIter<T>
where
    T: Float,
{
    type Item = (T, Pauli);

    fn next(&mut self) -> Option<Self::Item> {
        match self.code {
            Fermions::Offset => {
                if self.index == 0 {
                    self.index += 1;
                    Some((self.coeff, Pauli::default()))
                } else {
                    None
                }
            }
            Fermions::One {
                cr: p,
                an: q,
            } => {
                let p = u16::try_from(p.index())
                    .expect("orbital index out of bounds for type u16");
                let q = u16::try_from(q.index())
                    .expect("orbital index out of bounds for type u16");
                let item = if p == q {
                    next_item_one_pp(self.index, self.coeff, p)
                } else {
                    next_item_one_pq(self.index, self.coeff, p, q)
                };
                self.index = (self.index + 1).min(2);
                item
            }
            Fermions::Two {
                cr: (p, q),
                an: (r, s),
            } => {
                let p = u16::try_from(p.index())
                    .expect("orbital index out of bounds for type u16");
                let q = u16::try_from(q.index())
                    .expect("orbital index out of bounds for type u16");
                let r = u16::try_from(r.index())
                    .expect("orbital index out of bounds for type u16");
                let s = u16::try_from(s.index())
                    .expect("orbital index out of bounds for type u16");

                let item = if p == s && q == r {
                    next_item_two_pq(self.index, self.coeff, p, q)
                } else if q == r {
                    next_item_two_pqs(self.index, self.coeff, p, q, s)
                } else {
                    next_item_two_pqrs(self.index, self.coeff, p, q, r, s)
                };
                self.index = (self.index + 1).min(8);
                item
            }
        }
    }
}

fn next_item_one_pp<T: Float>(
    index: u8,
    coeff: T,
    p: u16,
) -> Option<(T, Pauli)> {
    let one_half = T::from(0.5).expect("cannot convert 0.5");

    match index {
        0 => Some((coeff * one_half, Pauli::identity())),
        1 => {
            let mut code = Pauli::default();
            code.set(p, PauliOp::Z);
            Some((-coeff * one_half, code))
        }
        _ => None,
    }
}

fn next_item_one_pq<T: Float>(
    index: u8,
    coeff: T,
    p: u16,
    q: u16,
) -> Option<(T, Pauli)> {
    let one_half = T::from(0.5).expect("cannot convert 0.5");

    let code = {
        let mut code = Pauli::default();
        for i in p + 1..q {
            code.set(i, PauliOp::Z);
        }
        code
    };

    match index {
        0 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(q, PauliOp::X);
            Some((coeff * one_half, code))
        }
        1 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(q, PauliOp::Y);
            Some((coeff * one_half, code))
        }
        _ => None,
    }
}

fn next_item_two_pq<T: Float>(
    index: u8,
    coeff: T,
    p: u16,
    q: u16,
) -> Option<(T, Pauli)> {
    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    match index {
        0 => {
            let code = Pauli::default();
            Some((term, code))
        }
        1 => {
            let mut code = Pauli::default();
            code.set(p, PauliOp::Z);
            Some((-term, code))
        }
        2 => {
            let mut code = Pauli::default();
            code.set(q, PauliOp::Z);
            Some((-term, code))
        }
        3 => {
            let mut code = Pauli::default();
            code.set(p, PauliOp::Z);
            code.set(q, PauliOp::Z);
            Some((term, code))
        }
        _ => None,
    }
}

fn next_item_two_pqs<T: Float>(
    index: u8,
    coeff: T,
    p: u16,
    q: u16,
    s: u16,
) -> Option<(T, Pauli)> {
    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let code = {
        let mut code = Pauli::default();
        for i in p + 1..s {
            code.set(i, PauliOp::Z);
        }
        code
    };

    match index {
        0 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(s, PauliOp::X);
            Some((term, code))
        }
        1 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(q, PauliOp::Z);
            code.set(s, PauliOp::X);
            Some((-term, code))
        }
        2 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(s, PauliOp::Y);
            Some((term, code))
        }
        3 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(q, PauliOp::Z);
            code.set(s, PauliOp::Y);
            Some((-term, code))
        }
        _ => None,
    }
}

fn next_item_two_pqrs<T: Float>(
    index: u8,
    coeff: T,
    p: u16,
    q: u16,
    r: u16,
    s: u16,
) -> Option<(T, Pauli)> {
    let term = coeff
        * T::from(0.125).expect("cannot obtain floating point fraction: 0.125");

    let code = {
        let mut code = Pauli::default();
        for i in p + 1..q {
            code.set(i, PauliOp::Z);
        }
        for i in s + 1..r {
            code.set(i, PauliOp::Z);
        }
        code
    };

    match index {
        0 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(q, PauliOp::X);
            code.set(r, PauliOp::X);
            code.set(s, PauliOp::X);
            Some((term, code))
        }
        1 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(q, PauliOp::X);
            code.set(r, PauliOp::Y);
            code.set(s, PauliOp::Y);
            Some((-term, code))
        }
        2 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(q, PauliOp::Y);
            code.set(r, PauliOp::X);
            code.set(s, PauliOp::Y);
            Some((term, code))
        }
        3 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(q, PauliOp::X);
            code.set(r, PauliOp::X);
            code.set(s, PauliOp::Y);
            Some((term, code))
        }
        4 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(q, PauliOp::X);
            code.set(r, PauliOp::Y);
            code.set(s, PauliOp::X);
            Some((term, code))
        }
        5 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(q, PauliOp::Y);
            code.set(r, PauliOp::X);
            code.set(s, PauliOp::X);
            Some((-term, code))
        }
        6 => {
            let mut code = code;
            code.set(p, PauliOp::X);
            code.set(q, PauliOp::Y);
            code.set(r, PauliOp::Y);
            code.set(s, PauliOp::X);
            Some((term, code))
        }
        7 => {
            let mut code = code;
            code.set(p, PauliOp::Y);
            code.set(q, PauliOp::Y);
            code.set(r, PauliOp::Y);
            code.set(s, PauliOp::Y);
            Some((term, code))
        }
        _ => None,
    }
}

/// Jordan-Wigner mapping.
///
/// This mapping is initialized with [`SumRepr<T,Fermions>`],
/// but implements [`Terms<T, Pauli>`].  The standard way
/// of using it is presented in the following example.
///
/// # Examples
///
/// ```rust
/// # use f2q::{
/// #     code::{
/// #         fermions::{
/// #             An,
/// #             Cr,
/// #             Fermions,
/// #             Orbital,
/// #         },
/// #         qubits::{
/// #             Pauli,
/// #             PauliOp,
/// #         },
/// #     },
/// #     map::JordanWigner,
/// #     terms::{
/// #         PauliSum,
/// #         SumRepr,
/// #         Terms,
/// #     },
/// # };
/// # fn main() -> Result<(), f2q::Error> {
/// let idx = 11;
/// let mut fermi_repr = SumRepr::new();
///
/// // Create orbital with qubit index 11
/// let p = Orbital::with_index(idx);
///
/// // Add it as one-electron interaction term to the sum with coefficient: 1.0
/// fermi_repr.add_term(Fermions::one_electron(Cr(p), An(p)).unwrap(), 1.0);
///
/// // Map fermionic hamiltonian to a sum of Pauli strings
/// let mut pauli_repr = PauliSum::new();
/// JordanWigner::new(&fermi_repr).add_to(&mut pauli_repr)?;
///
/// // We should obtain the following two Pauli strings weights 0.5
/// let code_i0 = Pauli::default();
/// let code_z0 = {
///     let mut code = Pauli::default();
///     code.set(idx.try_into().unwrap(), PauliOp::Z);
///     code
/// };
///
/// assert_eq!(pauli_repr.coeff(code_i0), 0.5);
/// assert_eq!(pauli_repr.coeff(code_z0), -0.5);
/// #   Ok(())
/// # }
/// ```
pub struct JordanWigner<'a, T> {
    repr: &'a SumRepr<T, Fermions>,
}

impl<'a, T> JordanWigner<'a, T> {
    #[must_use]
    pub fn new(repr: &'a SumRepr<T, Fermions>) -> Self {
        Self {
            repr,
        }
    }
}

impl<'a, T> Terms<(T, Pauli)> for JordanWigner<'a, T>
where
    T: Float,
{
    type Error = Error;

    fn add_to(
        &mut self,
        repr: &mut impl Extend<(T, Pauli)>,
    ) -> Result<(), Error> {
        for (&coeff, &code) in self.repr.iter() {
            repr.extend(Map::try_from(code)?.pauli_iter(coeff));
        }

        Ok(())
    }
}
