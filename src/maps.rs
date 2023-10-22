//! Mappings between various encodings.

use num::Float;

use crate::{
    codes::{
        fermions::FermiCode,
        qubits::PauliCode,
        Code,
    },
    terms::{
        SumRepr,
        Terms,
    },
    Error,
};

mod jordan_wigner;

pub trait Map<T, K>
where
    K: Code,
{
    type Error;
    type FromCode: Code;

    fn map<Ter>(
        &mut self,
        from_terms: &mut Ter,
    ) -> Result<impl Terms<T, K>, Self::Error>
    // this requires nightly Rust
    where
        Ter: Terms<T, Self::FromCode>;
}

struct JW<T> {
    // change this to JordanWigner, the struct below to JWTerms<'_,K>
    scratch_repr: SumRepr<T, FermiCode>,
}

impl<T> Map<T, PauliCode> for JW<T>
where
    T: Float,
{
    type Error = Error;
    type FromCode = FermiCode;

    fn map<Ter>(
        &mut self,
        from_terms: &mut Ter,
    ) -> Result<impl Terms<T, PauliCode>, Self::Error>
    where
        Ter: Terms<T, Self::FromCode>,
    {
        // SumRepr should have a method to reset itself.
        self.scratch_repr = SumRepr::new();
        from_terms.add_to(&mut self.scratch_repr)?;
        Ok(JordanWigner::new(&self.scratch_repr))
    }
}

/// Jordan-Wigner mapping.
///
/// This mapping is initialized with [`SumRepr<T,FermiCode>`],
/// but implements [`Terms<T, PauliCode>`].  The standard way
/// of using it is presented in the following example.
///
/// # Examples
///
/// ```rust
/// # use f2q::{
/// #     codes::{
/// #         fermions::{
/// #             An,
/// #             Cr,
/// #             FermiCode,
/// #             Orbital,
/// #         },
/// #         qubits::{
/// #             Pauli,
/// #             PauliCode,
/// #         },
/// #     },
/// #     maps::JordanWigner,
/// #     terms::{
/// #         SumRepr,
/// #         Terms,
/// #     },
/// # };
/// # fn main() -> Result<(), f2q::Error> {
///
/// let idx = 11;
/// let mut fermi_repr = SumRepr::new();
///
/// // Create orbital with qubit index 11
/// let p = Orbital::from_index(idx);
///
/// // Add it as one-electron interaction term to the sum with coefficient: 1.0
/// fermi_repr.add_term(FermiCode::one_electron(Cr(p), An(p)).unwrap(), 1.0);
///
/// // Map fermionic hamiltonian to a sum of Pauli strings
/// let mut pauli_repr = SumRepr::new();
/// JordanWigner::new(&fermi_repr).add_to(&mut pauli_repr)?;
///
/// // We should obtain the following two Pauli strings weights 0.5
/// let code_i0 = PauliCode::default();
/// let code_z0 = {
///     let mut code = PauliCode::default();
///     code.set(idx.try_into().unwrap(), Pauli::Z);
///     code
/// };
///
/// assert_eq!(pauli_repr.coeff(code_i0), 0.5);
/// assert_eq!(pauli_repr.coeff(code_z0), -0.5);
/// #   Ok(())
/// # }
/// ```
pub struct JordanWigner<'a, T> {
    repr: &'a SumRepr<T, FermiCode>,
}

impl<'a, T> JordanWigner<'a, T> {
    #[must_use]
    pub fn new(repr: &'a SumRepr<T, FermiCode>) -> Self {
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
        self.repr.iter().try_for_each({
            |(&coeff, &code)| {
                jordan_wigner::Map::try_from(code)
                    .map(|jw| jw.map(coeff).for_each(|x| repr.add_tuple(x)))
            }
        })
    }
}
