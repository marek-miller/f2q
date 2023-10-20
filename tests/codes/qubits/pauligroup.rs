use f2q::{
    codes::qubits::PauliGroup,
    math::Group,
    prelude::PauliCode,
};

#[test]
fn dentity() {
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
