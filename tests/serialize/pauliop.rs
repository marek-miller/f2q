use f2q::code::qubits::PauliOp;

#[test]
fn display() {
    assert_eq!(PauliOp::I.to_string(), "I");
    assert_eq!(PauliOp::X.to_string(), "X");
    assert_eq!(PauliOp::Y.to_string(), "Y");
    assert_eq!(PauliOp::Z.to_string(), "Z");
}

#[test]
fn pauli_serialize_01() {
    assert_eq!(
        serde_json::to_value(PauliOp::I).unwrap().as_str().unwrap(),
        "I"
    );

    assert_eq!(
        serde_json::to_value(PauliOp::X).unwrap().as_str().unwrap(),
        "X"
    );

    assert_eq!(
        serde_json::to_value(PauliOp::Y).unwrap().as_str().unwrap(),
        "Y"
    );

    assert_eq!(
        serde_json::to_value(PauliOp::Z).unwrap().as_str().unwrap(),
        "Z"
    );
}

#[test]
fn pauli_deserialize_01() {
    assert_eq!(
        serde_json::from_str::<PauliOp>("\"I\"").unwrap(),
        PauliOp::I
    );
    assert_eq!(
        serde_json::from_str::<PauliOp>("\"X\"").unwrap(),
        PauliOp::X
    );
    assert_eq!(
        serde_json::from_str::<PauliOp>("\"Y\"").unwrap(),
        PauliOp::Y
    );
    assert_eq!(
        serde_json::from_str::<PauliOp>("\"Z\"").unwrap(),
        PauliOp::Z
    );
}
