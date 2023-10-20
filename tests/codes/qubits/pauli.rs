use f2q::{
    prelude::Pauli,
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
