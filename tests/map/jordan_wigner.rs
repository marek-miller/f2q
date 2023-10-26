use f2q::{
    code::{
        fermions::Fermions,
        qubits::Pauli,
    },
    map::JordanWigner,
    terms::{
        SumRepr,
        Terms,
    },
};

fn jw_get_result(repr: &SumRepr<f64, Fermions>) -> Vec<(f64, Pauli)> {
    let mut jw_map = JordanWigner::new(repr);
    let mut result = vec![];
    jw_map.add_to(&mut result).unwrap();
    result.sort_by(|(_, pauli_a), (_, pauli_b)| pauli_a.cmp(pauli_b));
    result
}

#[test]
fn jw_offset() {
    let repr = SumRepr::from([(1.0, Fermions::Offset)]);
    assert_eq!(jw_get_result(&repr), &[(1.0, Pauli::identity())]);

    let repr =
        SumRepr::from([(1.0, Fermions::Offset), (2.0, Fermions::Offset)]);
    assert_eq!(jw_get_result(&repr), &[(3.0, Pauli::identity())]);
}

#[test]
fn jw_one_elec() {}
