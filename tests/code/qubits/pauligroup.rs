use f2q::{
    code::qubits::{
        Pauli,
        PauliGroup,
    },
    math::Group,
};

#[test]
fn identity() {
    let e = PauliGroup::identity();

    let g = PauliGroup::from(Pauli::new((0, 0)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(Pauli::new((1, 2)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(Pauli::new((12345, 67890)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);
}
