//! Mappings between various encodings

use num::Float;

use crate::{
    qubit::{
        Pauli,
        PauliCode,
    },
    secnd::{
        Integral,
        Orbital,
    },
    terms::SumRepr,
    Error,
    Terms,
};

pub struct JordanWigner<'a, T> {
    repr: &'a SumRepr<T, Integral>,
}

impl<'a, T> JordanWigner<'a, T> {
    #[must_use]
    pub fn new(repr: &'a SumRepr<T, Integral>) -> Self {
        Self {
            repr,
        }
    }
}

impl<'a, T> Terms<T, PauliCode> for JordanWigner<'a, T>
where
    T: Float,
{
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, PauliCode>,
    ) -> Result<(), Error> {
        for (&code, &coeff) in self.repr.as_map() {
            match code {
                Integral::Constant => {
                    repr.add(PauliCode::default(), coeff);
                }
                Integral::OneElectron {
                    cr,
                    an,
                } => one_electron(cr, an, coeff, repr)?,
                Integral::TwoElectron {
                    cr,
                    an,
                } => two_electron(cr, an, coeff, repr)?,
            }
        }

        Ok(())
    }
}

fn one_electron<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    if cr == an {
        one_electron_pp(cr, an, coeff, pauli_repr)?;
    } else {
        one_electron_pq(cr, an, coeff, pauli_repr)?;
    }

    Ok(())
}

fn one_electron_pp<T: Float>(
    cr: Orbital,
    _an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    if cr.index() >= 64 {
        return Err(Error::PauliIndex {
            msg: "cr index out of bound".to_string(),
        });
    }

    let term = coeff
        * T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();
    pauli_repr.add(code, term);

    // SAFETY: We checked bounds above
    unsafe {
        code.set_unchecked(cr.index(), Pauli::Z);
    }
    pauli_repr.add(code, -term);

    Ok(())
}

fn one_electron_pq<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    let term = coeff
        * T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();

    if cr.index() >= 64 {
        return Err(Error::PauliIndex {
            msg: "cr index out of bound".to_string(),
        });
    }

    if an.index() >= 64 {
        return Err(Error::PauliIndex {
            msg: "an index out of bound".to_string(),
        });
    }

    // SAFETY:
    // We just checked if indices are within bound
    for i in cr.index() + 1..an.index() {
        unsafe {
            code.set_unchecked(i, Pauli::Z);
        }
    }
    unsafe {
        code.set_unchecked(cr.index(), Pauli::X);
        code.set_unchecked(an.index(), Pauli::X);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(cr.index(), Pauli::Y);
        code.set_unchecked(an.index(), Pauli::Y);
    }
    pauli_repr.add(code, term);

    Ok(())
}

fn two_electron<T: Float>(
    cr: (Orbital, Orbital),
    an: (Orbital, Orbital),
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    let (p, q, r, s) = (cr.0.index(), cr.1.index(), an.0.index(), an.1.index());

    if p == s && q == r {
        two_electron_pq(p, q, coeff, pauli_repr)?;
    } else if q == r {
        two_electron_pqs(p, q, s, coeff, pauli_repr)?;
    } else {
        two_electron_pqrs(p, q, r, s, coeff, pauli_repr)?;
    }

    Ok(())
}

fn two_electron_pq<T: Float>(
    p: usize,
    q: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    if p >= 64 {
        return Err(Error::PauliIndex {
            msg: "p index out of bound".to_string(),
        });
    }

    if q >= 64 {
        return Err(Error::PauliIndex {
            msg: "q index out of bound".to_string(),
        });
    }

    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    // I
    pauli_repr.add(code, term);

    // SAFETY: We just checked if indices are within bound
    unsafe {
        code.set_unchecked(p, Pauli::Z);
    }
    // Z_p
    pauli_repr.add(code, -term);
    unsafe {
        code.set_unchecked(p, Pauli::I);
        code.set_unchecked(q, Pauli::Z);
    }
    // Z_q
    pauli_repr.add(code, -term);
    unsafe {
        code.set_unchecked(p, Pauli::Z);
    }
    // Z_p Z_q
    pauli_repr.add(code, term);

    Ok(())
}

fn two_electron_pqs<T: Float>(
    p: usize,
    q: usize,
    s: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    if p >= 64 {
        return Err(Error::PauliIndex {
            msg: "p index out of bound".to_string(),
        });
    }

    if q >= 64 {
        return Err(Error::PauliIndex {
            msg: "q index out of bound".to_string(),
        });
    }

    if s >= 64 {
        return Err(Error::PauliIndex {
            msg: "s index out of bound".to_string(),
        });
    }

    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    // SAFETY: We just checked if indices are within bound
    for i in p + 1..s {
        unsafe {
            code.set_unchecked(i, Pauli::Z);
        }
    }
    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(q, Pauli::Z);
    }
    pauli_repr.add(code, -term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add(code, -term);

    unsafe {
        code.set_unchecked(q, Pauli::I);
    }
    pauli_repr.add(code, term);

    Ok(())
}

fn two_electron_pqrs<T: Float>(
    p: usize,
    q: usize,
    r: usize,
    s: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) -> Result<(), Error> {
    if p >= 64 {
        return Err(Error::PauliIndex {
            msg: "p index out of bound".to_string(),
        });
    }

    if q >= 64 {
        return Err(Error::PauliIndex {
            msg: "q index out of bound".to_string(),
        });
    }

    if r >= 64 {
        return Err(Error::PauliIndex {
            msg: "q index out of bound".to_string(),
        });
    }

    if s >= 64 {
        return Err(Error::PauliIndex {
            msg: "s index out of bound".to_string(),
        });
    }

    let term = coeff
        * T::from(0.125).expect("cannot obtain floating point fraction: 0.125");

    let mut code = PauliCode::default();

    // SAFETY: We just checked if indices are within bound
    for i in p + 1..q {
        unsafe {
            code.set_unchecked(i, Pauli::Z);
        }
    }
    for i in s + 1..r {
        unsafe {
            code.set_unchecked(i, Pauli::Z);
        }
    }

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add(code, -term);

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add(code, -term);

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add(code, term);

    Ok(())
}
