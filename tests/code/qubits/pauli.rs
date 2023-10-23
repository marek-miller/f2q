use f2q::{
    code::qubits::Pauli,
    Error,
};

#[test]
fn pauli_01() {
    assert_eq!(Pauli::try_from(0u32).unwrap(), Pauli::I);
    assert_eq!(Pauli::try_from(1u32).unwrap(), Pauli::X);
    assert_eq!(Pauli::try_from(2u32).unwrap(), Pauli::Y);
    assert_eq!(Pauli::try_from(3u32).unwrap(), Pauli::Z);
}

#[test]
fn pauli_02() {
    let err = Pauli::try_from(4u16).unwrap_err();
    matches!(err, Error::PauliIndex { .. });
}

#[test]
fn pauli_03() {
    assert_eq!(u8::from(Pauli::I), 0);
    assert_eq!(u8::from(Pauli::X), 1);
    assert_eq!(u8::from(Pauli::Y), 2);
    assert_eq!(u8::from(Pauli::Z), 3);
}
