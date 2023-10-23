use f2q::code::qubits::Pauli;

#[test]
fn display() {
    assert_eq!(Pauli::I.to_string(), "I");
    assert_eq!(Pauli::X.to_string(), "X");
    assert_eq!(Pauli::Y.to_string(), "Y");
    assert_eq!(Pauli::Z.to_string(), "Z");
}

#[test]
fn pauli_serialize_01() {
    assert_eq!(
        serde_json::to_value(Pauli::I).unwrap().as_str().unwrap(),
        "I"
    );

    assert_eq!(
        serde_json::to_value(Pauli::X).unwrap().as_str().unwrap(),
        "X"
    );

    assert_eq!(
        serde_json::to_value(Pauli::Y).unwrap().as_str().unwrap(),
        "Y"
    );

    assert_eq!(
        serde_json::to_value(Pauli::Z).unwrap().as_str().unwrap(),
        "Z"
    );
}

#[test]
fn pauli_deserialize_01() {
    assert_eq!(serde_json::from_str::<Pauli>("\"I\"").unwrap(), Pauli::I);
    assert_eq!(serde_json::from_str::<Pauli>("\"X\"").unwrap(), Pauli::X);
    assert_eq!(serde_json::from_str::<Pauli>("\"Y\"").unwrap(), Pauli::Y);
    assert_eq!(serde_json::from_str::<Pauli>("\"Z\"").unwrap(), Pauli::Z);
}
