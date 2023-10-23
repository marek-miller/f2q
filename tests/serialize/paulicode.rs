use std::{
    fs::File,
    io::BufReader,
};

use f2q::code::qubits::{
    PauliOp,
    PauliCode,
};

#[test]
fn pauli_code_to_string() {
    assert_eq!(PauliCode::default().to_string(), "I");
    assert_eq!(PauliCode::new((1, 0)).to_string(), "X");
    assert_eq!(PauliCode::new((2, 0)).to_string(), "Y");
    assert_eq!(PauliCode::new((3, 0)).to_string(), "Z");

    assert_eq!(
        PauliCode::new((0, 1)).to_string(),
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIX"
    );
    assert_eq!(
        PauliCode::new((0, 2)).to_string(),
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIY"
    );
    assert_eq!(
        PauliCode::new((0, 3)).to_string(),
        "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIZ"
    );

    assert_eq!(
        PauliCode::new((u64::MAX, u64::MAX)).to_string(),
        "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ"
    );
}

#[test]
fn serialize_01() {
    let code = PauliCode::default();
    let json = serde_json::to_string(&code).unwrap();

    assert_eq!(json, "\"I\"");

    let code = PauliCode::from_paulis([PauliOp::I, PauliOp::X, PauliOp::Y, PauliOp::Z]);
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
        PauliCode::from_paulis([PauliOp::I, PauliOp::X, PauliOp::Y, PauliOp::Z])
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
    use PauliOp::{
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
    use PauliOp::*;
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
