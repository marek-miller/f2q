use std::{
    fs::File,
    io::BufReader,
};

use f2q::codes::qubits::{
    Pauli,
    PauliCode,
};

#[test]
fn serialize_01() {
    let code = PauliCode::default();
    let json = serde_json::to_string(&code).unwrap();

    assert_eq!(json, "\"I\"");

    let code = PauliCode::from_paulis([Pauli::I, Pauli::X, Pauli::Y, Pauli::Z]);
    let json = serde_json::to_string(&code).unwrap();

    assert_eq!(json, "\"IXYZ\"");
}

#[test]
fn deserialize_01() {
    let data = r#"
              "I" 
     "#;
    let code: PauliCode = serde_json::from_str(data).unwrap();
    assert_eq!(code, PauliCode::default());

    let data = r#"
              "IXYZ" 
     "#;
    let code: PauliCode = serde_json::from_str(data).unwrap();
    assert_eq!(
        code,
        PauliCode::from_paulis([Pauli::I, Pauli::X, Pauli::Y, Pauli::Z])
    );
}

#[test]
fn deserialize_02() {
    let data = r#"
              "" 
     "#;
    let _ = serde_json::from_str::<PauliCode>(data).unwrap_err();

    let data = r#"
              "IP" 
     "#;
    let _ = serde_json::from_str::<PauliCode>(data).unwrap_err();

    // this is 65 chars
    let data = r#"
              "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" 
     "#;
    let _ = serde_json::from_str::<PauliCode>(data).unwrap_err();
}

fn check_serde(code: PauliCode) {
    let json = serde_json::to_string(&code).unwrap();
    let result: PauliCode = serde_json::from_str(&json).unwrap();
    assert_eq!(result, code);
}

#[test]
fn serde_01() {
    use Pauli::{
        I,
        X,
        Y,
        Z,
    };
    check_serde(PauliCode::default());
    check_serde(PauliCode::from_paulis([I, X, Y, Z]));
    check_serde(PauliCode::from_paulis([X, X, X]));
    check_serde(PauliCode::from_paulis([
        I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y,
        I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y,
        I, X, X, X, I, Y, Y, Y, I, X, X, X, I, Y, Y, Y,
    ]));
}

const PAULICODES: &str = "./tests/serialize/paulicodes.json";

fn paulicodes_compare() -> [PauliCode; 8] {
    use Pauli::*;
    [
        PauliCode::from_paulis([]),
        PauliCode::from_paulis([X, X]),
        PauliCode::from_paulis([I, Y]),
        PauliCode::from_paulis([I, X, Y, Z]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
        ]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X,
        ]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X,
        ]),
        PauliCode::from_paulis([
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
            X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X,
        ]),
    ]
}

#[test]
fn deserialize_paulicodes() {
    // Open the file in read-only mode with buffer.
    let file = File::open(PAULICODES).unwrap();
    let reader = BufReader::new(file);

    let codes: Vec<PauliCode> = serde_json::from_reader(reader).unwrap();
    assert_eq!(codes, paulicodes_compare());
}
