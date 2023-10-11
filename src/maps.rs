//! Mappings between various encodings

use num::Float;

use crate::{
    qubit::{
        Pauli,
        PauliCode,
    },
    secnd::{
        Fermions,
        Orbital,
    },
    terms::SumRepr,
    Terms,
};

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
    fn add_to(
        &mut self,
        repr: &mut SumRepr<T, PauliCode>,
    ) {
        for (&code, &coeff) in self.repr.as_map() {
            match code {
                Fermions::Offset => {
                    repr.add_term(PauliCode::default(), coeff);
                }
                Fermions::One {
                    cr,
                    an,
                } => pauli_add_one_electron_integral(cr, an, coeff, repr),
                Fermions::Two {
                    cr,
                    an,
                } => pauli_add_two_electron_integral(cr, an, coeff, repr),
            }
        }
    }
}

fn pauli_add_one_electron_integral<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    if cr == an {
        pauli_add_one_electron_integral_equal(cr, an, coeff, pauli_repr);
    } else {
        pauli_add_one_electron_integral_nonequal(cr, an, coeff, pauli_repr);
    }
}

fn pauli_add_one_electron_integral_equal<T: Float>(
    cr: Orbital,
    _an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let term = coeff
        * T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();
    pauli_repr.add_term(code, term);

    code.set(cr.index(), Pauli::Z);
    pauli_repr.add_term(code, -term);
}

fn pauli_add_one_electron_integral_nonequal<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let term = coeff
        * T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();

    assert!(cr.index() < 64, "cr index out of bound");
    assert!(an.index() < 64, "cr index out of bound");

    // SAFETY:
    // We just checked if indices are within bound
    // we know that orbitals are ordered: cr <= an
    for i in cr.index() + 1..an.index() {
        unsafe {
            code.set_unchecked(i, Pauli::Z);
        }
    }
    unsafe {
        code.set_unchecked(cr.index(), Pauli::X);
        code.set_unchecked(an.index(), Pauli::X);
    }
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(cr.index(), Pauli::Y);
        code.set_unchecked(an.index(), Pauli::Y);
    }
    pauli_repr.add_term(code, term);
}

fn pauli_add_two_electron_integral<T: Float>(
    cr: (Orbital, Orbital),
    an: (Orbital, Orbital),
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let (p, q, r, s) = (cr.0.index(), cr.1.index(), an.0.index(), an.1.index());

    if p == s && q == r {
        pauli_add_two_electron_integral_pq(p, q, coeff, pauli_repr);
    } else if q == r {
        pauli_add_two_electron_integral_pqs(p, q, s, coeff, pauli_repr);
    } else {
        pauli_add_two_electron_integral_pqrs(p, q, r, s, coeff, pauli_repr);
    }
}

fn pauli_add_two_electron_integral_pq<T: Float>(
    p: usize,
    q: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    assert!(p < 64);
    assert!(q < 64);

    let term = coeff
        * T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    // I
    pauli_repr.add_term(code, term);

    // SAFETY: We just checked if indices are within bound
    unsafe {
        code.set_unchecked(p, Pauli::Z);
    }
    // Z_p
    pauli_repr.add_term(code, -term);
    unsafe {
        code.set_unchecked(p, Pauli::I);
        code.set_unchecked(q, Pauli::Z);
    }
    // Z_q
    pauli_repr.add_term(code, -term);
    unsafe {
        code.set_unchecked(p, Pauli::Z);
    }
    // Z_p Z_q
    pauli_repr.add_term(code, term);
}

fn pauli_add_two_electron_integral_pqs<T: Float>(
    p: usize,
    q: usize,
    s: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    assert!(p < 64);
    assert!(q < 64);
    assert!(s < 64);

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
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(q, Pauli::Z);
    }
    pauli_repr.add_term(code, -term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add_term(code, -term);

    unsafe {
        code.set_unchecked(q, Pauli::I);
    }
    pauli_repr.add_term(code, term);
}

fn pauli_add_two_electron_integral_pqrs<T: Float>(
    p: usize,
    q: usize,
    r: usize,
    s: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    assert!(p < 64);
    assert!(q < 64);
    assert!(r < 64);
    assert!(s < 64);

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
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add_term(code, -term);

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::X);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::X);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add_term(code, -term);

    unsafe {
        code.set_unchecked(p, Pauli::X);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::X);
    }
    pauli_repr.add_term(code, term);

    unsafe {
        code.set_unchecked(p, Pauli::Y);
        code.set_unchecked(q, Pauli::Y);
        code.set_unchecked(r, Pauli::Y);
        code.set_unchecked(s, Pauli::Y);
    }
    pauli_repr.add_term(code, term);
}
