//! Convert full 32-fermion Hamiltonian using Jordan-Wigner mapping
//!
//! All integrals are present, the coefficients are arbitrary.

use std::{
    io::Write,
    time::Instant,
};

use f2q::{
    codes::{
        fermions::{
            An,
            Cr,
            FermiCode,
            FermiSum,
            Orbital,
        },
        qubits::PauliSum,
    },
    maps::JordanWigner,
    math::Pairs,
    terms::{
        SumRepr,
        Terms,
    },
};
use rand::Rng;

const ORBITAL_MAX_IDX: u32 = 64;

fn main() -> Result<(), f2q::Error> {
    let mut rng = rand::thread_rng();

    let orbitals = Orbital::gen_range(0..ORBITAL_MAX_IDX).collect::<Vec<_>>();
    let orbital_pairs = Pairs::new(&orbitals).collect::<Vec<_>>();

    let now = Instant::now();
    let mut fermi_sum = SumRepr::new();

    fermi_sum.add_term(FermiCode::Offset, 1.0_f64);
    for code in orbital_pairs
        .iter()
        .filter_map(|(&p, &q)| FermiCode::one_electron(Cr(p), An(q)))
    {
        // the coefficient is completely arbitrary
        fermi_sum.add_term(code, rng.gen_range(-1.0..1.0));
    }
    for code in Pairs::new(&orbital_pairs).filter_map(|((&p, &q), (&r, &s))| {
        FermiCode::two_electron((Cr(p), Cr(q)), (An(r), An(s)))
    }) {
        // the coefficient is completely arbitrary
        fermi_sum.add_term(code, rng.gen_range(-1.0..1.0));
    }

    println!(
        "Generated {} terms in {} ms.",
        fermi_sum.len(),
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let json = serde_json::to_string(&fermi_sum).unwrap();
    println!(
        "Serialized to JSON string of length {} in {} ms.",
        json.len(),
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let fermi_sum_de: FermiSum<f64> = serde_json::from_str(&json).unwrap();
    println!(
        "Deserialized to sumrepr with {} terms in {} ms.",
        fermi_sum_de.len(),
        now.elapsed().as_millis()
    );

    print!("Converting (Jordan-Wigner)... ");
    let _ = std::io::stdout().flush();

    let now = Instant::now();
    let mut pauli_sum = SumRepr::new();

    JordanWigner::new(&fermi_sum).add_to(&mut pauli_sum)?;

    println!("Done.");
    println!(
        "Obtained {} terms in {} ms.",
        pauli_sum.len(),
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let json = serde_json::to_string(&pauli_sum).unwrap();
    println!(
        "Serialized to JSON string of length {} in {} ms.",
        json.len(),
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let pauli_sum_de: PauliSum<f64> = serde_json::from_str(&json).unwrap();
    println!(
        "Deserialized to sumrepr with {} terms in {} ms.",
        pauli_sum_de.len(),
        now.elapsed().as_millis()
    );

    Ok(())
}
