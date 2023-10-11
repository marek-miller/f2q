//! Convert full 32-fermion Hamiltonian using Jordan-Wigner mapping
//!
//! All integrals are present, the coefficients are arbitrary.

use std::{
    io::Write,
    time::Instant,
};

use f2q::prelude::*;
use rand::Rng;

const ORBITAL_MAX_IDX: usize = 64;

fn main() {
    let mut rng = rand::thread_rng();

    let orbitals = Orbital::gen_range(0..ORBITAL_MAX_IDX).collect::<Vec<_>>();
    let orbital_pairs = Pairs::new(&orbitals).collect::<Vec<_>>();

    let now = Instant::now();
    let mut fermi_sum = SumRepr::new();

    fermi_sum.add_term(Fermions::Offset, 1.0);
    for code in orbital_pairs
        .iter()
        .filter_map(|(&p, &q)| Fermions::one_electron(p, q))
    {
        // the coefficient is completely arbitrary
        fermi_sum.add_term(code, rng.gen_range(-1.0..1.0));
    }
    for code in Pairs::new(&orbital_pairs).filter_map(|((&p, &q), (&r, &s))| {
        Fermions::two_electron((p, q), (r, s))
    }) {
        // the coefficient is completely arbitrary
        fermi_sum.add_term(code, rng.gen_range(-1.0..1.0));
    }

    println!(
        "Generated {} terms in {} ms.",
        fermi_sum.as_map().len(),
        now.elapsed().as_millis()
    );

    print!("Converting (Jordan-Wigner)... ");
    let _ = std::io::stdout().flush();

    let now = Instant::now();
    let pauli_sum = &mut SumRepr::new();
    JordanWigner::new(&fermi_sum).add_to(pauli_sum);

    println!("Done.");
    println!(
        "Obtained {} terms in {} ms.",
        pauli_sum.as_map().len(),
        now.elapsed().as_millis()
    );
}
