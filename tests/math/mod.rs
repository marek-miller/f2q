use f2q::math::{
    Group,
    Pairs,
    Root4,
};

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

#[test]
fn pairs_01() {
    let data = [0, 1, 2];
    let result = Pairs::new(&data).collect::<Vec<_>>();

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
fn pairs_02() {
    let data = vec![0; 17];
    let result = Pairs::new(&data).collect::<Vec<_>>();
    assert_eq!(result.len(), 17 * 17);
}

#[test]
fn pairs_empty() {
    let data: [usize; 0] = [];
    let result = Pairs::new(&data).collect::<Vec<_>>();

    assert_eq!(result, &[]);
}
