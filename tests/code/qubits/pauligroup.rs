use f2q::{
    code::qubits::{
        PauliCode,
        PauliGroup,
    },
    math::Group,
};

#[test]
fn identity() {
    let e = PauliGroup::identity();

    let g = PauliGroup::from(PauliCode::new((0, 0)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(PauliCode::new((1, 2)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(PauliCode::new((12345, 67890)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);
}
