use f2q::{
    code::qubits::PauliOp,
    Error,
};

#[test]
fn pauli_01() {
    assert_eq!(PauliOp::try_from(0u32).unwrap(), PauliOp::I);
    assert_eq!(PauliOp::try_from(1u32).unwrap(), PauliOp::X);
    assert_eq!(PauliOp::try_from(2u32).unwrap(), PauliOp::Y);
    assert_eq!(PauliOp::try_from(3u32).unwrap(), PauliOp::Z);
}

#[test]
fn pauli_02() {
    let err = PauliOp::try_from(4u16).unwrap_err();
    matches!(err, Error::PauliIndex { .. });
}

#[test]
fn pauli_03() {
    assert_eq!(u8::from(PauliOp::I), 0);
    assert_eq!(u8::from(PauliOp::X), 1);
    assert_eq!(u8::from(PauliOp::Y), 2);
    assert_eq!(u8::from(PauliOp::Z), 3);
}
