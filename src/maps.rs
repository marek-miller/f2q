//! Mappings between various encodings

use num::Float;

use crate::{
    qubit::{
        Pauli,
        PauliCode,
    },
    sec::{
        Integral,
        Orbital,
    },
    terms::SumRepr,
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
    ) {
        for (&code, &coeff) in self.repr.as_map() {
            match code {
                Integral::Constant => {
                    repr.add(PauliCode::default(), coeff);
                }
                Integral::OneElectron {
                    cr,
                    an,
                } => pauli_add_one_electron_integral(cr, an, coeff, repr),
                Integral::TwoElectron {
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
    let one_half =
        T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let code = PauliCode::default();
    pauli_repr.add(code, coeff * one_half);

    let mut code = PauliCode::default();
    code.set(cr.enumerate(), Pauli::Z);
    pauli_repr.add(code, -coeff * one_half);
}

fn pauli_add_one_electron_integral_nonequal<T: Float>(
    cr: Orbital,
    an: Orbital,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let one_half =
        T::from(0.5).expect("cannot obtain floating point fraction: 0.5");

    let mut code = PauliCode::default();
    // we know that orbitals are ordered: cr <= an
    for i in cr.enumerate() + 1..an.enumerate() {
        code.set(i, Pauli::Z);
    }
    code.set(cr.enumerate(), Pauli::X);
    code.set(an.enumerate(), Pauli::X);
    pauli_repr.add(code, coeff * one_half);

    code.set(cr.enumerate(), Pauli::Y);
    code.set(an.enumerate(), Pauli::Y);
    pauli_repr.add(code, -coeff * one_half);
}

fn pauli_add_two_electron_integral<T: Float>(
    cr: (Orbital, Orbital),
    an: (Orbital, Orbital),
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let (p, q, r, s) = (
        cr.0.enumerate(),
        cr.1.enumerate(),
        an.0.enumerate(),
        an.1.enumerate(),
    );

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
    let frac =
        T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    // I
    pauli_repr.add(code, coeff * frac);
    code.set(p, Pauli::Z);
    // Z_p
    pauli_repr.add(code, -coeff * frac);
    code.set(p, Pauli::I);
    code.set(q, Pauli::Z);
    // Z_q
    pauli_repr.add(code, -coeff * frac);
    code.set(p, Pauli::Z);
    // Z_p Z_q
    pauli_repr.add(code, coeff * frac);
}

fn pauli_add_two_electron_integral_pqs<T: Float>(
    p: usize,
    q: usize,
    s: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let frac =
        T::from(0.25).expect("cannot obtain floating point fraction: 0.25");

    let mut code = PauliCode::default();
    for i in p + 1..s {
        code.set(i, Pauli::Z);
    }
    code.set(p, Pauli::X);
    code.set(s, Pauli::X);
    pauli_repr.add(code, frac * coeff);

    code.set(q, Pauli::Z);
    pauli_repr.add(code, -frac * coeff);

    code.set(p, Pauli::Y);
    code.set(s, Pauli::Y);
    code.set(q, Pauli::I);
    pauli_repr.add(code, frac * coeff);

    code.set(q, Pauli::Z);
    pauli_repr.add(code, -frac * coeff);
}

fn pauli_add_two_electron_integral_pqrs<T: Float>(
    p: usize,
    q: usize,
    r: usize,
    s: usize,
    coeff: T,
    pauli_repr: &mut SumRepr<T, PauliCode>,
) {
    let frac =
        T::from(0.125).expect("cannot obtain floating point fraction: 0.125");

    let mut code = PauliCode::default();

    for i in p + 1..q {
        code.set(i, Pauli::Z);
    }
    for i in s + 1..r {
        code.set(i, Pauli::Z);
    }

    code.set(p, Pauli::X);
    code.set(s, Pauli::X);
    code.set(r, Pauli::X);
    code.set(s, Pauli::X);
    pauli_repr.add(code, frac * coeff);

    code.set(p, Pauli::X);
    code.set(s, Pauli::X);
    code.set(r, Pauli::Y);
    code.set(s, Pauli::Y);
    pauli_repr.add(code, -frac * coeff);

    code.set(p, Pauli::X);
    code.set(s, Pauli::Y);
    code.set(r, Pauli::X);
    code.set(s, Pauli::Y);
    pauli_repr.add(code, frac * coeff);

    code.set(p, Pauli::Y);
    code.set(s, Pauli::X);
    code.set(r, Pauli::X);
    code.set(s, Pauli::Y);
    pauli_repr.add(code, frac * coeff);

    code.set(p, Pauli::Y);
    code.set(s, Pauli::X);
    code.set(r, Pauli::Y);
    code.set(s, Pauli::X);
    pauli_repr.add(code, frac * coeff);

    code.set(p, Pauli::Y);
    code.set(s, Pauli::Y);
    code.set(r, Pauli::X);
    code.set(s, Pauli::X);
    pauli_repr.add(code, -frac * coeff);

    code.set(p, Pauli::X);
    code.set(s, Pauli::Y);
    code.set(r, Pauli::Y);
    code.set(s, Pauli::X);
    pauli_repr.add(code, frac * coeff);

    code.set(p, Pauli::Y);
    code.set(s, Pauli::Y);
    code.set(r, Pauli::Y);
    code.set(s, Pauli::Y);
    pauli_repr.add(code, frac * coeff);
}
