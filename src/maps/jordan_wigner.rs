use num::Float;

use crate::{
    codes::{
        fermions::FermiCode,
        qubits::{
            Pauli,
            PauliCode,
        },
    },
    Error,
};

pub struct Map(FermiCode);

impl Map {
    pub fn pauli_iter<T>(
        &self,
        coeff: T,
    ) -> impl Iterator<Item = (T, PauliCode)>
    where
        T: Float,
    {
        PauliIter::new(coeff, self.0)
    }
}

impl TryFrom<FermiCode> for Map {
    type Error = Error;

    fn try_from(value: FermiCode) -> Result<Self, Self::Error> {
        match value {
            FermiCode::Offset => Ok(Self(value)),
            FermiCode::One {
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
            FermiCode::Two {
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
pub struct PauliIter<T> {
    coeff: T,
    code:  FermiCode,
    index: u8,
}

impl<T> PauliIter<T> {
    pub fn new(
        coeff: T,
        code: FermiCode,
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
    type Item = (T, PauliCode);

    fn next(&mut self) -> Option<Self::Item> {
        match self.code {
            FermiCode::Offset => {
                if self.index == 0 {
                    self.index += 1;
                    Some((self.coeff, PauliCode::default()))
                } else {
                    None
                }
            }
            FermiCode::One {
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
            FermiCode::Two {
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
) -> Option<(T, PauliCode)> {
    let one_half = T::from(0.5).expect("cannot convert 0.5");

    match index {
        0 => Some((coeff * one_half, PauliCode::identity())),
        1 => {
            let mut code = PauliCode::default();
            code.set(p, Pauli::Z);
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
) -> Option<(T, PauliCode)> {
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
            Some((coeff * one_half, code))
        }
        1 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Y);
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
) -> Option<(T, PauliCode)> {
    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    match index {
        0 => {
            let code = PauliCode::default();
            Some((term, code))
        }
        1 => {
            let mut code = PauliCode::default();
            code.set(p, Pauli::Z);
            Some((-term, code))
        }
        2 => {
            let mut code = PauliCode::default();
            code.set(q, Pauli::Z);
            Some((-term, code))
        }
        3 => {
            let mut code = PauliCode::default();
            code.set(p, Pauli::Z);
            code.set(q, Pauli::Z);
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
) -> Option<(T, PauliCode)> {
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
            Some((term, code))
        }
        1 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::Z);
            code.set(s, Pauli::X);
            Some((-term, code))
        }
        2 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(s, Pauli::Y);
            Some((term, code))
        }
        3 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Z);
            code.set(s, Pauli::Y);
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
) -> Option<(T, PauliCode)> {
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
            Some((term, code))
        }
        1 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::X);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::Y);
            Some((-term, code))
        }
        2 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::X);
            code.set(s, Pauli::Y);
            Some((term, code))
        }
        3 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::X);
            code.set(r, Pauli::X);
            code.set(s, Pauli::Y);
            Some((term, code))
        }
        4 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::X);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::X);
            Some((term, code))
        }
        5 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::X);
            code.set(s, Pauli::X);
            Some((-term, code))
        }
        6 => {
            let mut code = code;
            code.set(p, Pauli::X);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::X);
            Some((term, code))
        }
        7 => {
            let mut code = code;
            code.set(p, Pauli::Y);
            code.set(q, Pauli::Y);
            code.set(r, Pauli::Y);
            code.set(s, Pauli::Y);
            Some((term, code))
        }
        _ => None,
    }
}
