use f2q::codes::fermions::{
    An,
    Cr,
    FermiCode,
    Orbital,
};

#[test]
fn display() {
    let code = FermiCode::Offset;
    assert_eq!(code.to_string(), format!("[]"));

    let code = FermiCode::one_electron(
        Cr(Orbital::from_index(1)),
        An(Orbital::from_index(2)),
    )
    .unwrap();
    assert_eq!(code.to_string(), format!("[1, 2]"));

    let code = FermiCode::two_electron(
        (Cr(Orbital::from_index(1)), Cr(Orbital::from_index(2))),
        (An(Orbital::from_index(5)), An(Orbital::from_index(4))),
    )
    .unwrap();
    assert_eq!(code.to_string(), format!("[1, 2, 5, 4]"));
}

#[test]
fn serialize_01() {
    let code = FermiCode::Offset;
    let json = serde_json::to_string(&code).unwrap();
    assert_eq!(json, "[]");

    let code = FermiCode::one_electron(
        Cr(Orbital::from_index(1)),
        An(Orbital::from_index(2)),
    )
    .unwrap();
    let json = serde_json::to_string(&code).unwrap();
    assert_eq!(json, "[1,2]");

    let code = FermiCode::two_electron(
        (Cr(Orbital::from_index(1)), Cr(Orbital::from_index(2))),
        (An(Orbital::from_index(5)), An(Orbital::from_index(4))),
    )
    .unwrap();
    let json = serde_json::to_string(&code).unwrap();
    assert_eq!(json, "[1,2,5,4]");
}

#[test]
fn deserialize_01() {
    let data = r"
                []
    ";
    let code: FermiCode = serde_json::from_str(data).unwrap();
    assert_eq!(code, FermiCode::Offset);

    let data = r"
                [1, 2]
    ";
    let code: FermiCode = serde_json::from_str(data).unwrap();
    let expected = FermiCode::one_electron(
        Cr(Orbital::from_index(1)),
        An(Orbital::from_index(2)),
    )
    .unwrap();
    assert_eq!(code, expected);

    let data = r"
                [1, 2, 5, 4]
    ";
    let code: FermiCode = serde_json::from_str(data).unwrap();
    let expected = FermiCode::two_electron(
        (Cr(Orbital::from_index(1)), Cr(Orbital::from_index(2))),
        (An(Orbital::from_index(5)), An(Orbital::from_index(4))),
    )
    .unwrap();
    assert_eq!(code, expected);
}
