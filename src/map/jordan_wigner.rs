use num::Float;

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
    let code = Pauli::parity_op(index);

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
        let term_x = ReIm::Re(one_half);
        let term_y = match self {
            Self::An(_) => ReIm::Im(one_half),
            Self::Cr(_) => ReIm::Im(-one_half),
        };

        rhs.flat_map(move |(rhs_coeff, rhs_pauli)| {
            [(term_x, x), (term_y, y)].into_iter().map(
                move |(lhs_coeff, lhs_pauli)| {
                    let (root, prod) = lhs_pauli * rhs_pauli;
                    (lhs_coeff * rhs_coeff * ReIm::from(root), prod)
                },
            )
        })
    }
}

fn jw_map_two_hemitian<'a, T: Float + 'a>(
    op1: &'a Map,
    op2: &'a Map,
    coeff: T,
) -> impl Iterator<Item = (T, Pauli)> + 'a {
    let two = T::from(2.0_f64).expect("floating point conversion from 2.0");
    let start = [(ReIm::Re(coeff), Pauli::identity())].into_iter();

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
    let start = [(ReIm::Re(coeff), Pauli::identity())].into_iter();

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

// #[test]
// fn jwmap_mul_iter_01() {
//     use PauliOp::*;
//     let jw_cr = Map::try_from(Cr(Orbital::with_index(0))).unwrap();
//     let jw_an = Map::try_from(An(Orbital::with_index(0))).unwrap();

//     let start = [(ReIm::from(2.), Pauli::default())].into_iter();

//     let result = jw_cr.mul_iter(start.clone()).collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Re(1.), Pauli::with_ops([X])),
//             (ReIm::Im(-1.), Pauli::with_ops([Y]))
//         ]
//     );

//     let result = jw_an.mul_iter(start.clone()).collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Re(1.), Pauli::with_ops([X])),
//             (ReIm::Im(1.), Pauli::with_ops([Y]))
//         ]
//     );

//     let result = jw_cr
//         .mul_iter(jw_an.mul_iter(start.clone()))
//         .collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Re(0.5), Pauli::with_ops([])),
//             (ReIm::Re(-0.5), Pauli::with_ops([Z])),
//             (ReIm::Re(-0.5), Pauli::with_ops([Z])),
//             (ReIm::Re(0.5), Pauli::with_ops([])),
//         ]
//     );

//     let result = jw_an
//         .mul_iter(jw_cr.mul_iter(start.clone()))
//         .collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Re(0.5), Pauli::with_ops([])),
//             (ReIm::Re(0.5), Pauli::with_ops([Z])),
//             (ReIm::Re(0.5), Pauli::with_ops([Z])),
//             (ReIm::Re(0.5), Pauli::with_ops([])),
//         ]
//     );
// }

// #[test]
// fn jwmap_mul_iter_02() {
//     use PauliOp::*;
//     let jw_cr = Map::try_from(Cr(Orbital::with_index(0))).unwrap();
//     let jw_an = Map::try_from(An(Orbital::with_index(1))).unwrap();

//     let start = [(ReIm::from(2.), Pauli::default())].into_iter();

//     let result = jw_cr.mul_iter(start.clone()).collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Re(1.), Pauli::with_ops([X])),
//             (ReIm::Im(-1.), Pauli::with_ops([Y]))
//         ]
//     );

//     let result = jw_an.mul_iter(start.clone()).collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Re(1.), Pauli::with_ops([Z, X])),
//             (ReIm::Im(1.), Pauli::with_ops([Z, Y]))
//         ]
//     );

//     let result = jw_cr
//         .mul_iter(jw_an.mul_iter(start.clone()))
//         .collect::<Vec<_>>();
//     assert_eq!(
//         result,
//         &[
//             (ReIm::Im(0.5), Pauli::with_ops([Y, X])),
//             (ReIm::Re(0.5), Pauli::with_ops([X, X])),
//             (ReIm::Re(-0.5), Pauli::with_ops([Y, Y])),
//             (ReIm::Im(0.5), Pauli::with_ops([X, Y])),
//         ]
//     );
// }
