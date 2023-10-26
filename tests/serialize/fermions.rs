use f2q::code::fermions::{
    An,
    Cr,
    Fermions,
    Orbital,
};

#[test]
fn display() {
    let code = Fermions::Offset;
    assert_eq!(code.to_string(), format!("[]"));

    let code = Fermions::one_electron(
        Cr(Orbital::with_index(1)),
        An(Orbital::with_index(2)),
    )
    .unwrap();
    assert_eq!(code.to_string(), format!("[1, 2]"));

    let code = Fermions::two_electron(
        (Cr(Orbital::with_index(1)), Cr(Orbital::with_index(2))),
        (An(Orbital::with_index(5)), An(Orbital::with_index(4))),
    )
    .unwrap();
    assert_eq!(code.to_string(), format!("[1, 2, 5, 4]"));
}

#[test]
fn serialize_01() {
    let code = Fermions::Offset;
    let json = serde_json::to_string(&code).unwrap();
    assert_eq!(json, "[]");

    let code = Fermions::one_electron(
        Cr(Orbital::with_index(1)),
        An(Orbital::with_index(2)),
    )
    .unwrap();
    let json = serde_json::to_string(&code).unwrap();
    assert_eq!(json, "[1,2]");

    let code = Fermions::two_electron(
        (Cr(Orbital::with_index(1)), Cr(Orbital::with_index(2))),
        (An(Orbital::with_index(5)), An(Orbital::with_index(4))),
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
    let code: Fermions = serde_json::from_str(data).unwrap();
    assert_eq!(code, Fermions::Offset);

    let data = r"
                [1, 2]
    ";
    let code: Fermions = serde_json::from_str(data).unwrap();
    let expected = Fermions::one_electron(
        Cr(Orbital::with_index(1)),
        An(Orbital::with_index(2)),
    )
    .unwrap();
    assert_eq!(code, expected);

    let data = r"
                [1, 2, 5, 4]
    ";
    let code: Fermions = serde_json::from_str(data).unwrap();
    let expected = Fermions::two_electron(
        (Cr(Orbital::with_index(1)), Cr(Orbital::with_index(2))),
        (An(Orbital::with_index(5)), An(Orbital::with_index(4))),
    )
    .unwrap();
    assert_eq!(code, expected);
}
