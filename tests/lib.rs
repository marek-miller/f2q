use std::ops::RangeBounds;

use f2q::{
    qubit::{
        Pauli,
        PauliCode,
    },
    secnd::{
        Orbital,
        Spin,
    },
    terms::SumRepr,
    Error,
    Pairs,
};

#[test]
fn test_pauli_01() {
    assert_eq!(Pauli::try_from(0u32).unwrap(), Pauli::I);
    assert_eq!(Pauli::try_from(1u32).unwrap(), Pauli::X);
    assert_eq!(Pauli::try_from(2u32).unwrap(), Pauli::Y);
    assert_eq!(Pauli::try_from(3u32).unwrap(), Pauli::Z);
}

#[test]
fn test_pauli_02() {
    let err = Pauli::try_from(4u16).unwrap_err();
    matches!(err, Error::PauliIndex { .. });
}

#[test]
fn test_pauli_03() {
    assert_eq!(u8::from(Pauli::I), 0);
    assert_eq!(u8::from(Pauli::X), 1);
    assert_eq!(u8::from(Pauli::Y), 2);
    assert_eq!(u8::from(Pauli::Z), 3);
}

#[test]
fn test_paulicode_init() {
    let code = PauliCode::new((0b01, 0b00));
    assert_eq!(code.enumerate(), 0b01);
}

#[test]
fn test_paulicode_default() {
    let code = PauliCode::default();
    assert_eq!(code, PauliCode::new((0, 0)));
}

#[test]
fn test_paulicode_pauli_02() {
    let code = PauliCode::new((0b0101, 0b00));

    assert_eq!(code.pauli(0), Some(Pauli::X));
    assert_eq!(code.pauli(1), Some(Pauli::X));
    assert_eq!(code.pauli(2), Some(Pauli::I));
    assert_eq!(code.pauli(63), Some(Pauli::I));

    assert_eq!(code.pauli(64), None);
    assert_eq!(code.pauli(123), None);
}

#[test]
fn test_paulicode_pauli_mut_01() {
    let mut code = PauliCode::default();
    assert_eq!(code.pauli(7).unwrap(), Pauli::I);

    code.pauli_mut(7, |x| {
        if let Some(pauli) = x {
            *pauli = Pauli::Z;
        }
    });
    assert_eq!(code.pauli(7).unwrap(), Pauli::Z);
}

#[test]
fn test_paulicode_set_pauli_01() {
    let mut code = PauliCode::new((29_332_281_938, 0b00));
    assert_eq!(code.pauli(7).unwrap(), Pauli::I);

    code.set(7, Pauli::Y);
    assert_eq!(code.pauli(7).unwrap(), Pauli::Y);
}

#[test]
#[should_panic(expected = "index should be within 0..64")]
fn test_paulicode_set_pauli_02() {
    let mut code = PauliCode::default();
    assert_eq!(code.pauli(7).unwrap(), Pauli::I);

    code.set(65, Pauli::Y);
    assert_eq!(code.pauli(7).unwrap(), Pauli::Y);
}

#[test]
fn test_paulicode_set_pauli_03() {
    let mut code = PauliCode::default();

    for i in 0..13 {
        code.set(i, Pauli::X);
    }
    for i in 13..29 {
        code.set(i, Pauli::Y);
    }
    for i in 29..61 {
        code.set(i, Pauli::Z);
    }

    for i in 0..13 {
        assert_eq!(code.pauli(i).unwrap(), Pauli::X, "{i}");
    }
    for i in 13..29 {
        assert_eq!(code.pauli(i).unwrap(), Pauli::Y, "{i}");
    }
    for i in 29..61 {
        assert_eq!(code.pauli(i).unwrap(), Pauli::Z, "{i}");
    }
    for i in 61..64 {
        assert_eq!(code.pauli(i).unwrap(), Pauli::I, "{i}");
    }
}

#[test]
fn test_paulicode_codes_iter_01() {
    use Pauli::*;
    let result = PauliCode::new((0b01, 0b00))
        .iter()
        .take(3)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, I, I]);
}

#[test]
fn test_paulicode_codes_iter_02() {
    use Pauli::*;
    let result = PauliCode::new((0b11_1001, 0b00))
        .iter()
        .take(5)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, Y, Z, I, I]);
}

#[test]
fn test_paulicode_codes_iter_03() {
    use Pauli::*;
    let result = PauliCode::new((0b0101_0000, 0b1111_1010))
        .iter()
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
fn test_paulicode_from_paulis_01() {
    use Pauli::*;

    assert_eq!(
        PauliCode::from_paulis([I, X, Y, Z]),
        PauliCode::new((0b1110_0100, 0b00))
    );
}

#[test]
fn test_paulicode_from_paulis_02() {
    use Pauli::*;

    assert_eq!(
        PauliCode::from_paulis([
            I, I, X, X, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
            I, I, I, I, I, I, I, I, I, I, Y, Y, Z, Z
        ]),
        PauliCode::new((0b0101_0000, 0b1111_1010))
    );
}

#[test]
fn test_sumrepr_init_01() {
    let code = PauliCode::new((1234, 0));
    let mut hamil = SumRepr::new();

    hamil.as_map_mut().insert(code, 4321.);
    let coeff = hamil.coeff(&code);
    assert!(f64::abs(coeff - 4321.) < f64::EPSILON);
}

#[test]
fn test_spin_init_01() {
    let spin = Spin::Down;
    assert_eq!(u8::from(spin), 0);
    let spin = Spin::Up;
    assert_eq!(u8::from(spin), 1);

    let spin = Spin::default();
    assert_eq!(u8::from(spin), 0);
}

#[test]
fn test_orbital_enumerate_01() {
    let orb = Orbital::default();
    assert_eq!(orb.index(), 0);

    let orb = Orbital::new(3, Spin::Down);
    assert_eq!(orb.index(), 6);

    let orb = Orbital::new(8, Spin::Up);
    assert_eq!(orb.index(), 17);
}

#[test]
#[should_panic(expected = "orbital index out of bound")]
fn test_orbital_enumerate_02() {
    let orb = Orbital::new(usize::MAX / 2, Spin::Up);
    assert_eq!(orb.index(), usize::MAX);
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

#[test]
fn orbital_gen_range_01() {
    let orbitals: Vec<_> = Orbital::gen_range(0..0).collect();
    assert!(orbitals.is_empty());

    let orbitals: Vec<_> = Orbital::gen_range(..0).collect();
    assert!(orbitals.is_empty());

    let orbitals: Vec<_> = Orbital::gen_range(0..=0).collect();
    assert_eq!(orbitals.len(), 1);

    let orbitals: Vec<_> = Orbital::gen_range(..=0).collect();
    assert_eq!(orbitals.len(), 1);
}

#[test]
fn orbital_gen_range_02() {
    let um = usize::MAX;
    let orbitals: Vec<_> = Orbital::gen_range(um..um).collect();
    assert!(orbitals.is_empty());

    let orbitals: Vec<_> = Orbital::gen_range(um..).collect();
    assert_eq!(orbitals.len(), 1);

    let orbitals: Vec<_> = Orbital::gen_range(um..=um).collect();
    assert_eq!(orbitals.len(), 1);
}

#[allow(clippy::reversed_empty_ranges)]
#[test]
fn orbital_gen_range_03() {
    let orbitals: Vec<_> = Orbital::gen_range(2..0).collect();
    assert!(orbitals.is_empty());

    let orbitals: Vec<_> = Orbital::gen_range(3..1).collect();
    assert!(orbitals.is_empty());
}

fn orbital_gen_range_idxs<R>(range: R) -> Vec<usize>
where
    R: RangeBounds<usize>,
{
    Orbital::gen_range(range).map(|orb| orb.index()).collect()
}

#[test]
fn orbital_gen_range_04() {
    assert_eq!(orbital_gen_range_idxs(0..1), &[0]);
    assert_eq!(orbital_gen_range_idxs(0..=1), &[0, 1]);
    assert_eq!(orbital_gen_range_idxs(0..2), &[0, 1]);
    assert_eq!(orbital_gen_range_idxs(0..=2), &[0, 1, 2]);
    assert_eq!(orbital_gen_range_idxs(0..3), &[0, 1, 2]);
    assert_eq!(orbital_gen_range_idxs(0..=3), &[0, 1, 2, 3]);

    assert_eq!(orbital_gen_range_idxs(11..15), &[11, 12, 13, 14]);
    assert_eq!(orbital_gen_range_idxs(11..=15), &[11, 12, 13, 14, 15]);
}
