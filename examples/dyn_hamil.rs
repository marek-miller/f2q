//! Dynamically generated Hamiltonian

use std::time::Instant;

use f2q::prelude::*;
use rand::Rng;

fn main() -> Result<(), f2q::Error> {
    let mut hamil = Hamil::Offset(0.) + Hamil::Offset(0.);

    for _ in 0..128 {
        let mut count = 0;
        let terms = Box::new(StackRepr::new(move || {
            let mut rng = rand::thread_rng();
            if count < 4096 {
                if let Some(fermion) = Fermions::two_electron(
                    (
                        Cr(Orbital::from_index(rng.gen_range(0..64))),
                        Cr(Orbital::from_index(rng.gen_range(0..64))),
                    ),
                    (
                        An(Orbital::from_index(rng.gen_range(0..64))),
                        An(Orbital::from_index(rng.gen_range(0..64))),
                    ),
                ) {
                    count += 1;
                    Some((rng.gen_range(-1.0..1.0), fermion))
                } else {
                    Some((0.0, Fermions::Offset))
                }
            } else {
                None
            }
        }));

        let mut rng = rand::thread_rng();
        if let Hamil::Sum(hl, hr) = hamil {
            if rng.gen_bool(0.5) {
                hamil = Hamil::Sum(Box::new(*hl + Hamil::Terms(terms)), hr);
            } else {
                hamil = Hamil::Sum(hl, Box::new(*hr + Hamil::Terms(terms)));
            }
        }
    }

    let now = Instant::now();
    let mut fermi_sum = SumRepr::new();
    hamil.add_to(&mut fermi_sum)?;

    println!(
        "Generated {} terms in {} ms.",
        fermi_sum.as_map().len(),
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let mut pauli_sum = SumRepr::new();

    JordanWigner::new(&fermi_sum).add_to(&mut pauli_sum)?;

    println!("Done.");
    println!(
        "Obtained {} terms in {} ms.",
        pauli_sum.as_map().len(),
        now.elapsed().as_millis()
    );

    Ok(())
}
