use std::fmt::Debug;

use f2q::{
    code::{
        fermions::Fermions,
        qubits::{
            Pauli,
            PauliOp,
        },
    },
    map::JordanWigner,
    terms::{
        SumRepr,
        Terms,
    },
};
use num::Float;

fn jw_get_result<T: Float>(repr: &SumRepr<T, Fermions>) -> Vec<(T, Pauli)> {
    let mut jw_map = JordanWigner::new(repr);
    let mut result = vec![];
    jw_map.add_to(&mut result).unwrap();
    result.sort_by(|(_, pauli_a), (_, pauli_b)| pauli_a.cmp(pauli_b));
    result
}

fn jw_check_mapping<T: Float + Debug>(
    list: &[(T, Fermions)],
    expected: &[(T, Pauli)],
) {
    let repr = SumRepr::from_iter(list);
    assert_eq!(jw_get_result(&repr), expected);
}

#[test]
fn jw_offset() {
    jw_check_mapping(&[(1.0, Fermions::Offset)], &[(1.0, Pauli::identity())]);

    jw_check_mapping(
        &[(1.0, Fermions::Offset), (2.0, Fermions::Offset)],
        &[(3.0, Pauli::identity())],
    );
}

#[test]
fn jw_one_elec_01() {
    use PauliOp::*;

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 0)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::with_ops([Z]))],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((1, 1)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::with_ops([I, Z]))],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((2, 2)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::with_ops([I, I, Z]))],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((32, 32)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::new((0, 0b11)))],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((33, 33)).unwrap())],
        &[(1.0, Pauli::identity()), (-1.0, Pauli::new((0, 0b1100)))],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((63, 63)).unwrap())],
        &[
            (1.0, Pauli::identity()),
            (-1.0, Pauli::new((0, 0b11 << 62))),
        ],
    );
}

#[test]
fn jw_one_elec_02() {
    use PauliOp::*;

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 1)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, X])),
            (1.0, Pauli::with_ops([Y, Y])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 2)).unwrap())],
        &[
            (1.0, Pauli::with_ops([X, Z, X])),
            (1.0, Pauli::with_ops([Y, Z, Y])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((1, 4)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, X, Z, Z, X])),
            (1.0, Pauli::with_ops([I, Y, Z, Z, Y])),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((2, 5)).unwrap())],
        &[
            (1.0, Pauli::with_ops([I, I, X, Z, Z, X])),
            (1.0, Pauli::with_ops([I, I, Y, Z, Z, Y])),
        ],
    );
}

#[test]
fn jw_one_elec_03() {
    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 32)).unwrap())],
        &[
            (1.0, Pauli::new((0xffff_ffff_ffff_fffd, 0x1))),
            (1.0, Pauli::new((0xffff_ffff_ffff_fffe, 0x2))),
        ],
    );

    jw_check_mapping(
        &[(2.0, Fermions::try_from((0, 63)).unwrap())],
        &[
            (
                1.0,
                Pauli::new((0xffff_ffff_ffff_fffd, 0x7fff_ffff_ffff_ffff)),
            ),
            (
                1.0,
                Pauli::new((0xffff_ffff_ffff_fffe, 0xbfff_ffff_ffff_ffff)),
            ),
        ],
    );
}
