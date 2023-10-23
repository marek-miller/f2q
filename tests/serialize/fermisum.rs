use f2q::{
    code::fermions::{
        An,
        Cr,
        FermiCode,
        FermiSum,
        Orbital,
    },
    terms::SumRepr,
};
use serde_json::Value;

#[test]
#[allow(clippy::float_cmp)]
fn fermisum_serialize_01() {
    let mut repr = SumRepr::new();

    repr.add_term(FermiCode::Offset, 0.1);

    let json = serde_json::to_value(&repr).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "type": "sumrepr",
            "encoding": "fermions",
            "terms":  [
                {
                    "code": [],
                    "value": 0.1
                }
            ]
        }
        "#,
    )
    .unwrap();

    assert_eq!(json, expected);
}

#[test]
#[allow(clippy::float_cmp)]
fn fermisum_serialize_02() {
    let mut repr = SumRepr::new();

    repr.add_term(
        FermiCode::one_electron(
            Cr(Orbital::from_index(1)),
            An(Orbital::from_index(2)),
        )
        .unwrap(),
        0.2,
    );
    let json = serde_json::to_value(&repr).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "type": "sumrepr",
            "encoding": "fermions",
            "terms":  [
                {
                    "code": [1, 2],
                    "value": 0.2
                }
            ]
        }
        "#,
    )
    .unwrap();

    assert_eq!(json, expected);
}

#[test]
#[allow(clippy::float_cmp)]
fn fermisum_serialize_03() {
    let mut repr = SumRepr::new();

    repr.add_term(
        FermiCode::two_electron(
            (Cr(Orbital::from_index(0)), Cr(Orbital::from_index(1))),
            (An(Orbital::from_index(1)), An(Orbital::from_index(0))),
        )
        .unwrap(),
        0.3,
    );
    let json = serde_json::to_value(&repr).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "type": "sumrepr",
            "encoding": "fermions",
            "terms":  [
                {
                    "code": [0, 1, 1, 0],
                    "value": 0.3
                }
            ]
        }
        "#,
    )
    .unwrap();

    assert_eq!(json, expected);
}

#[test]
fn fermisum_serialize_04() {
    let mut repr = SumRepr::new();

    repr.add_term(FermiCode::Offset, 0.1);
    repr.add_term(
        FermiCode::one_electron(
            Cr(Orbital::from_index(1)),
            An(Orbital::from_index(2)),
        )
        .unwrap(),
        0.2,
    );
    repr.add_term(
        FermiCode::two_electron(
            (Cr(Orbital::from_index(0)), Cr(Orbital::from_index(1))),
            (An(Orbital::from_index(1)), An(Orbital::from_index(0))),
        )
        .unwrap(),
        0.3,
    );
    let json = serde_json::to_value(&repr).unwrap();

    let map = json.as_object().unwrap();

    assert_eq!(
        map.get("encoding").unwrap(),
        &Value::String("fermions".to_string())
    );

    let Value::Array(arr) = map.get("terms").unwrap() else {
        panic!()
    };

    assert_eq!(arr.len(), 3);
}

#[test]
#[allow(clippy::float_cmp)]
fn fermisum_deserialize_01() {
    let data = r#"
        {
            "type": "sumrepr",
            "encoding": "fermions",
            "terms": [
                {
                    "code": [],
                    "value": 0.1
                }
            ]
        }
    "#;

    let repr: FermiSum = serde_json::from_str(data).unwrap();

    assert_eq!(repr.len(), 1);
    assert_eq!(repr.coeff(FermiCode::Offset), 0.1);
}

#[test]
#[allow(clippy::float_cmp)]
fn fermisum_deserialize_02() {
    let data = r#"
        {
            "type": "sumrepr",
            "encoding": "fermions",
            "terms": [
                {
                    "code": [],
                    "value": 0.1
                },
                {
                    "code": [1, 2],
                    "value": 0.2
                }
            ]
        }
    "#;

    let repr: FermiSum = serde_json::from_str(data).unwrap();

    assert_eq!(repr.len(), 2);
    assert_eq!(repr.coeff(FermiCode::Offset), 0.1);
    assert_eq!(
        repr.coeff(
            FermiCode::one_electron(
                Cr(Orbital::from_index(1)),
                An(Orbital::from_index(2))
            )
            .unwrap()
        ),
        0.2
    );
}

#[test]
#[allow(clippy::float_cmp)]
fn fermisum_deserialize_03() {
    let data = r#"
        {
            "type": "sumrepr",
            "encoding": "fermions",
            "terms": [
                {
                    "code": [],
                    "value": 0.1
                },
                {
                    "value": 0.09,
                    "code": []
                },
                {
                    "code": [1, 2],
                    "value": 0.2
                }, 
                {
                    "code": [0,1,1,0],
                    "value": 0.3
                }
            ]
        }
    "#;

    let repr: FermiSum = serde_json::from_str(data).unwrap();

    assert_eq!(repr.len(), 3);
    assert_eq!(repr.coeff(FermiCode::Offset), 0.19);
    assert_eq!(
        repr.coeff(
            FermiCode::one_electron(
                Cr(Orbital::from_index(1)),
                An(Orbital::from_index(2))
            )
            .unwrap()
        ),
        0.2
    );
    assert_eq!(
        repr.coeff(
            FermiCode::two_electron(
                (Cr(Orbital::from_index(0)), Cr(Orbital::from_index(1))),
                (An(Orbital::from_index(1)), An(Orbital::from_index(0))),
            )
            .unwrap(),
        ),
        0.3
    );
}
