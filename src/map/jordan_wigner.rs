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
    math::ReIm,
    terms::{
        SumRepr,
        Terms,
    },
    Error,
};

enum Map {
    Cr(Orbital),
    An(Orbital),
}

impl TryFrom<Cr> for Map {
    type Error = Error;

    fn try_from(value: Cr) -> Result<Self, Self::Error> {
        (value.index() < 64)
            .then_some(Self::Cr(value.0))
            .ok_or_else(|| Error::QubitIndex {
                msg: "orbital index must be within 0..=63".to_string(),
            })
    }
}

impl TryFrom<An> for Map {
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

impl Map {
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
    ) -> impl Iterator<Item = (ReIm<T>, Pauli)> + 'a
    where
        T: Float + 'a,
        I: Iterator<Item = (ReIm<T>, Pauli)> + 'a,
    {
        let one_half =
            T::from(0.5_f64).expect("floating point conversion from 0.5");

        let (x, y) = pauli_codes_from_index(self.index());

        rhs.flat_map(move |(rhs_coeff, rhs_pauli)| {
            let (root_x, prod_x) = x * rhs_pauli;
            let (root_y, prod_y) = y * rhs_pauli;

            let term_x = rhs_coeff * ReIm::from(one_half) * ReIm::from(root_x);
            let term_y = rhs_coeff
                * match self {
                    Self::An(_) => ReIm::Im(one_half),
                    Self::Cr(_) => ReIm::Im(-one_half),
                }
                * ReIm::from(root_y);

            [(term_x, prod_x), (term_y, prod_y)].into_iter()
        })
    }
}

fn jw_map_two_hemitian<'a, T: Float + 'a>(
    op1: &'a Map,
    op2: &'a Map,
    coeff: T,
) -> impl Iterator<Item = (T, Pauli)> + 'a {
    let two = T::from(2.0_f64).expect("floating point conversion from 2.0");
    let start = [(ReIm::from(coeff), Pauli::identity())].into_iter();

    op1.mul_iter(op2.mul_iter(start)).filter_map(move |(x, p)| {
        if let ReIm::Re(xre) = x {
            Some((xre * two, p))
        } else {
            None
        }
    })
}

fn jw_map_four_hemitian<'a, T: Float + 'a>(
    op1: &'a Map,
    op2: &'a Map,
    op3: &'a Map,
    op4: &'a Map,
    coeff: T,
) -> impl Iterator<Item = (T, Pauli)> + 'a {
    let two = T::from(2.0_f64).expect("floating point conversion from 2.0");
    let start = [(ReIm::from(coeff), Pauli::identity())].into_iter();

    op1.mul_iter(op2.mul_iter(op3.mul_iter(op4.mul_iter(start))))
        .filter_map(move |(x, p)| {
            if let ReIm::Re(xre) = x {
                Some((xre * two, p))
            } else {
                None
            }
        })
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
            match code {
                Fermions::Offset => {
                    repr.extend(Some((coeff, Pauli::identity())))
                }
                Fermions::One {
                    cr,
                    an,
                } => {
                    let jw_cr = Map::try_from(cr)?;
                    let jw_an = Map::try_from(an)?;
                    repr.extend(jw_map_two_hemitian(&jw_cr, &jw_an, coeff));
                }
                Fermions::Two {
                    cr,
                    an,
                } => {
                    let jw_cr = (Map::try_from(cr.0)?, Map::try_from(cr.1)?);
                    let jw_an = (Map::try_from(an.0)?, Map::try_from(an.1)?);

                    repr.extend(jw_map_four_hemitian(
                        &jw_cr.0, &jw_cr.1, &jw_an.0, &jw_an.1, coeff,
                    ));
                }
            }
        }

        Ok(())
    }
}
