//! Mappings between various encodings

use num::Float;

use crate::{
    qubit::{
        Pauli,
        PauliCode,
    },
    secnd::Fermions,
    terms::SumRepr,
    Code,
    Error,
    Terms,
};

/// Jordan-Wigner mapping.
///
/// This mapping is initialized with [`SumRepr<T,Fermions>`],
/// but implements [`Terms<T, PauliCode>`].  The standard way
/// of using it is presented in the following example.
///
/// # Examples
///
/// ```rust
/// use f2q::prelude::*;
/// # fn main() -> Result<(), f2q::Error> {
///
/// let idx = 11;
/// let mut fermi_repr = SumRepr::new();
///
/// // Create orbital with qubit index 11
/// let p = Orbital::from_index(idx);
///
/// // Add it as one-electron interaction term to the sum with coefficient: 1.0
/// fermi_repr.add_term(Fermions::one_electron(Cr(p), An(p)).unwrap(), 1.0);
///
/// // Map fermionic hamiltonian to a sum of Pauli strings
/// let mut pauli_repr = SumRepr::new();
/// JordanWigner::new(&fermi_repr).add_to(&mut pauli_repr)?;
///
/// // We should obtain the following two Pauli strings weights 0.5
/// let code_i0 = PauliCode::default();
/// let code_z0 = {
///     let mut code = PauliCode::default();
///     code.set(idx, Pauli::Z);
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

impl<'a, T> Terms<T, PauliCode> for JordanWigner<'a, T>
where
    T: Float,
{
    type Error = Error;

    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, PauliCode>,
    ) -> Result<(), Self::Error> {
        for (&code, &coeff) in self.repr.as_map() {
            for (pauli_code, pauli_coeff) in JWMap::try_from(code)?.map(coeff) {
                repr.add_term(pauli_code, pauli_coeff);
            }
        }

        Ok(())
    }
}

pub struct JWMap<K: Code>(K);

impl JWMap<Fermions> {
    pub fn map<T>(
        &self,
        coeff: T,
    ) -> impl Iterator<Item = (PauliCode, T)>
    where
        T: Float,
    {
        JWIter::new(coeff, self.0)
    }
}

impl TryFrom<Fermions> for JWMap<Fermions> {
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
                    Err(Error::PauliIndex {
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
                    Err(Error::PauliIndex {
                        msg: "orbital index must be less than 64".to_string(),
                    })
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct JWIter<T, K> {
    coeff: T,
    code:  K,
    index: usize,
}

impl<T, K> JWIter<T, K> {
    pub fn new(
        coeff: T,
        code: K,
    ) -> Self {
        Self {
            coeff,
            code,
            index: 0,
        }
    }
}

impl<T> Iterator for JWIter<T, Fermions>
where
    T: Float,
{
    type Item = (PauliCode, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.code {
            Fermions::Offset => {
                if self.index == 0 {
                    self.index += 1;
                    Some((PauliCode::default(), self.coeff))
                } else {
                    None
                }
            }
            Fermions::One {
                cr: p,
                an: q,
            } => {
                let p = p.index();
                let q = q.index();
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
                let p = p.index();
                let q = q.index();
                let r = r.index();
                let s = s.index();

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
    index: usize,
    coeff: T,
    p: usize,
) -> Option<(PauliCode, T)> {
    let one_half = T::from(0.5).expect("cannot convert 0.5");

    match index {
        0 => Some((PauliCode::default(), coeff * one_half)),
        1 => {
            let mut code = PauliCode::default();
            code.set(p, Pauli::Z);
            Some((code, -coeff * one_half))
        }
        _ => None,
    }
}

fn next_item_one_pq<T: Float>(
    index: usize,
    coeff: T,
    p: usize,
    q: usize,
) -> Option<(PauliCode, T)> {
    let one_half = T::from(0.5).expect("cannot convert 0.5");

    let code = {
        let mut code = PauliCode::default();
        for i in p + 1..q {
            code.set(i, Pauli::Z);
        }
        code
    };

    match index {
        0 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::X);
            Some((code, coeff * one_half))
        }
        1 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Y);
            Some((code, coeff * one_half))
        }
        _ => None,
    }
}

fn next_item_two_pq<T: Float>(
    index: usize,
    coeff: T,
    p: usize,
    q: usize,
) -> Option<(PauliCode, T)> {
    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    match index {
        0 => {
            let code = PauliCode::default();
            Some((code, term))
        }
        1 => {
            let mut code = PauliCode::default();
            code.set(p, Pauli::Z);
            Some((code, -term))
        }
        2 => {
            let mut code = PauliCode::default();
            code.set(q, Pauli::Z);
            Some((code, -term))
        }
        3 => {
            let mut code = PauliCode::default();
            code.set(p, Pauli::Z);
            code.set(q, Pauli::Z);
            Some((code, term))
        }
        _ => None,
    }
}

fn next_item_two_pqs<T: Float>(
    index: usize,
    coeff: T,
    p: usize,
    q: usize,
    s: usize,
) -> Option<(PauliCode, T)> {
    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let code = {
        let mut code = PauliCode::default();
        for i in p + 1..s {
            code.set(i, Pauli::Z);
        }
        code
    };

    match index {
        0 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(s, Pauli::X);
            Some((code, term))
        }
        1 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::Z);
            code.set(s, Pauli::X);
            Some((code, -term))
        }
        2 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(s, Pauli::Y);
            Some((code, term))
        }
        3 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Z);
            code.set(s, Pauli::Y);
            Some((code, -term))
        }
        _ => None,
    }
}

fn next_item_two_pqrs<T: Float>(
    index: usize,
    coeff: T,
    p: usize,
    q: usize,
    r: usize,
    s: usize,
) -> Option<(PauliCode, T)> {
    let term = coeff
        * T::from(0.125).expect("cannot obtain floating point fraction: 0.125");

    let code = {
        let mut code = PauliCode::default();
        for i in p + 1..q {
            code.set(i, Pauli::Z);
        }
        for i in s + 1..r {
            code.set(i, Pauli::Z);
        }
        code
    };

    match index {
        0 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::X);
            code.set(r, Pauli::X);
            code.set(s, Pauli::X);
            Some((code, term))
        }
        1 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::X);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::Y);
            Some((code, -term))
        }
        2 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::X);
            code.set(s, Pauli::Y);
            Some((code, term))
        }
        3 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::X);
            code.set(r, Pauli::X);
            code.set(s, Pauli::Y);
            Some((code, term))
        }
        4 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::X);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::X);
            Some((code, term))
        }
        5 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::X);
            code.set(s, Pauli::X);
            Some((code, -term))
        }
        6 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::X);
            Some((code, term))
        }
        7 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::Y);
            Some((code, term))
        }
        _ => None,
    }
}
