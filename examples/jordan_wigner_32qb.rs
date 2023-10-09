use std::{
    io::Write,
    time::Instant,
};

use hamil_repr::{
    maps::JordanWigner,
    sec::{
        Integral,
        Orbital,
    },
    terms::SumRepr,
    Pairs,
    Terms,
};

const ORBITAL_MAX_IDX: usize = 64;
const DELTA: f64 = 0.012_345;

fn main() {
    let now = Instant::now();

    let orbitals = Orbital::gen_range(0..ORBITAL_MAX_IDX).collect::<Vec<_>>();

    let mut fermi_sum = SumRepr::new();
    fermi_sum.add(Integral::Constant, 1.0);

    let mut coeff = DELTA;

    for (&p, &q) in Pairs::new(&orbitals) {
        if let Some(integral) = Integral::one_electron(p, q) {
            fermi_sum.add(integral, coeff);
        }
        coeff += DELTA;
    }

    for (&p, &q) in Pairs::new(&orbitals) {
        for (&r, &s) in Pairs::new(&orbitals) {
            if let Some(integral) = Integral::two_electron((p, q), (r, s)) {
                fermi_sum.add(integral, coeff);
            }
            coeff += DELTA;
        }
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
