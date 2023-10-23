use f2q::code::qubits::{
    Pauli,
    PauliOp,
};

#[test]
fn init() {
    let code = Pauli::new((0b01, 0b00));
    assert_eq!(code.enumerate(), 0b01);
}

#[test]
fn default() {
    let code = Pauli::default();
    assert_eq!(code, Pauli::new((0, 0)));
}

#[test]
fn pauli_02() {
    let code = Pauli::new((0b0101, 0b00));

    assert_eq!(code.pauli(0), Some(PauliOp::X));
    assert_eq!(code.pauli(1), Some(PauliOp::X));
    assert_eq!(code.pauli(2), Some(PauliOp::I));
    assert_eq!(code.pauli(63), Some(PauliOp::I));

    assert_eq!(code.pauli(64), None);
    assert_eq!(code.pauli(123), None);
}

#[test]
fn pauli_mut_01() {
    let mut code = Pauli::default();
    assert_eq!(code.pauli(7).unwrap(), PauliOp::I);

    code.pauli_mut(7, |x| {
        if let Some(pauli) = x {
            *pauli = PauliOp::Z;
        }
    });
    assert_eq!(code.pauli(7).unwrap(), PauliOp::Z);
}

#[test]
fn set_pauli_01() {
    let mut code = Pauli::new((29_332_281_938, 0b00));
    assert_eq!(code.pauli(7).unwrap(), PauliOp::I);

    code.set(7, PauliOp::Y);
    assert_eq!(code.pauli(7).unwrap(), PauliOp::Y);
}

#[test]
#[should_panic(expected = "index should be within 0..64")]
fn set_pauli_02() {
    let mut code = Pauli::default();
    assert_eq!(code.pauli(7).unwrap(), PauliOp::I);

    code.set(65, PauliOp::Y);
    assert_eq!(code.pauli(7).unwrap(), PauliOp::Y);
}

#[test]
fn set_pauli_03() {
    let mut code = Pauli::default();

    for i in 0..13 {
        code.set(i, PauliOp::X);
    }
    for i in 13..29 {
        code.set(i, PauliOp::Y);
    }
    for i in 29..61 {
        code.set(i, PauliOp::Z);
    }

    for i in 0..13 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::X, "{i}");
    }
    for i in 13..29 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::Y, "{i}");
    }
    for i in 29..61 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::Z, "{i}");
    }
    for i in 61..64 {
        assert_eq!(code.pauli(i).unwrap(), PauliOp::I, "{i}");
    }
}

#[test]
fn codes_iter_01() {
    use PauliOp::*;
    let result = Pauli::new((0b01, 0b00))
        .into_iter()
        .take(3)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, I, I]);
}

#[test]
fn codes_iter_02() {
    use PauliOp::*;
    let result = Pauli::new((0b11_1001, 0b00))
        .into_iter()
        .take(5)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, Y, Z, I, I]);
}

#[test]
fn codes_iter_03() {
    use PauliOp::*;
    let result = Pauli::new((0b0101_0000, 0b1111_1010))
        .into_iter()
        .take(36)
        .collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
        ]
    );
}

#[test]
fn from_paulis_01() {
    use PauliOp::*;

    assert_eq!(
        Pauli::from_paulis([I, X, Y, Z]),
        Pauli::new((0b1110_0100, 0b00))
    );
}

#[test]
fn from_paulis_02() {
    use PauliOp::*;

    assert_eq!(
        Pauli::from_paulis([
            I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
        ]),
        Pauli::new((0b0101_0000, 0b1111_1010))
    );
}

#[test]
fn from_u128() {
    assert_eq!(Pauli::from(0u128).enumerate(), 0u128);
    assert_eq!(Pauli::from(1u128).enumerate(), 1u128);
    assert_eq!(
        Pauli::from(11_111_111_111_111_111_u128).enumerate(),
        11_111_111_111_111_111_u128
    );
    assert_eq!(
        Pauli::from(1_234_567_898_765_432_112_345_678_987_654_321_u128)
            .enumerate(),
        1_234_567_898_765_432_112_345_678_987_654_321_u128
    );
    assert_eq!(Pauli::from(u128::MAX).enumerate(), u128::MAX);
}
