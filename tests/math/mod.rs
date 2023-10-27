use f2q::math::{
    pairs,
    Group,
    Root4,
};

#[test]
fn pairs_01a() {
    let data = [0, 1, 2];
    let result = pairs(&data, &data).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&0, &0),
            (&0, &1),
            (&0, &2),
            (&1, &0),
            (&1, &1),
            (&1, &2),
            (&2, &0),
            (&2, &1),
            (&2, &2),
        ]
    );
}

#[test]
fn pairs_01b() {
    let data_x = [0, 1];
    let data_y = [0, 1, 2];
    let result = pairs(&data_x, &data_y).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[(&0, &0), (&0, &1), (&0, &2), (&1, &0), (&1, &1), (&1, &2),]
    );
}

#[test]
fn pairs_01c() {
    let data_x = [0, 1, 2];
    let data_y = [0, 1];
    let result = pairs(&data_x, &data_y).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[(&0, &0), (&0, &1), (&1, &0), (&1, &1), (&2, &0), (&2, &1),]
    );
}

#[test]
fn pairs_02() {
    let data = vec![0; 17];
    let result = pairs(&data, &data).collect::<Vec<_>>();
    assert_eq!(result.len(), 17 * 17);
}

#[test]
fn pairs_empty() {
    let data = [1];
    let data_empty: [usize; 0] = [];

    let result = pairs(&data, &data_empty).collect::<Vec<_>>();
    assert_eq!(result, &[]);

    let result = pairs(&data_empty, &data).collect::<Vec<_>>();
    assert_eq!(result, &[]);

    let result = pairs(&data_empty, &data_empty).collect::<Vec<_>>();
    assert_eq!(result, &[]);
}

#[test]
fn root4_identity() {
    assert_eq!(Root4::identity(), Root4::R0);
}

#[test]
fn root4_inverse() {
    assert_eq!(Root4::R0.inverse(), Root4::R0);
    assert_eq!(Root4::R1.inverse(), Root4::R1);
    assert_eq!(Root4::R2.inverse(), Root4::R3);
    assert_eq!(Root4::R3.inverse(), Root4::R2);
}

#[test]
fn root4_mul() {
    use Root4::*;

    assert_eq!(R0 * R0, R0);
    assert_eq!(R0 * R1, R1);
    assert_eq!(R0 * R2, R2);
    assert_eq!(R0 * R3, R3);

    assert_eq!(R1 * R0, R1);
    assert_eq!(R1 * R1, R0);
    assert_eq!(R1 * R2, R3);
    assert_eq!(R1 * R3, R2);

    assert_eq!(R2 * R0, R2);
    assert_eq!(R2 * R1, R3);
    assert_eq!(R2 * R2, R1);
    assert_eq!(R2 * R3, R0);

    assert_eq!(R3 * R0, R3);
    assert_eq!(R3 * R1, R2);
    assert_eq!(R3 * R2, R0);
    assert_eq!(R3 * R3, R1);
}

#[test]
fn root4_neg() {
    assert_eq!(-Root4::R0, Root4::R1);
    assert_eq!(-Root4::R1, Root4::R0);
    assert_eq!(-Root4::R2, Root4::R3);
    assert_eq!(-Root4::R3, Root4::R2);
}

#[test]
fn root4_conj() {
    assert_eq!(Root4::R0.conj(), Root4::R0);
    assert_eq!(Root4::R1.conj(), Root4::R1);
    assert_eq!(Root4::R2.conj(), Root4::R3);
    assert_eq!(Root4::R3.conj(), Root4::R2);
}
