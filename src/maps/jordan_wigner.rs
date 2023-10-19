use num::Float;

use crate::{
    qubit::{
        Pauli,
        PauliCode,
    },
    secq::Fermions,
    Code,
    Error,
};

pub struct Map<K: Code>(K);

impl Map<Fermions> {
    pub fn map<T>(
        &self,
        coeff: T,
    ) -> impl Iterator<Item = (PauliCode, T)>
    where
        T: Float,
    {
        Iter::new(coeff, self.0)
    }
}

impl TryFrom<Fermions> for Map<Fermions> {
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
pub struct Iter<T, K> {
    coeff: T,
    code:  K,
    index: usize,
}

impl<T, K> Iter<T, K> {
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

impl<T> Iterator for Iter<T, Fermions>
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
