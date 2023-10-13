use std::ops::RangeBounds;

use f2q::{
    math::Pairs,
    prelude::JordanWigner,
    qubit::{
        Pauli,
        PauliCode,
    },
    secnd::{
        Fermions,
        Orbital,
        Spin,
    },
    terms::SumRepr,
    Error,
    Terms,
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
        .into_iter()
        .take(3)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, I, I]);
}

#[test]
fn test_paulicode_codes_iter_02() {
    use Pauli::*;
    let result = PauliCode::new((0b11_1001, 0b00))
        .into_iter()
        .take(5)
        .collect::<Vec<_>>();

    assert_eq!(result, &[X, Y, Z, I, I]);
}

#[test]
fn test_paulicode_codes_iter_03() {
    use Pauli::*;
    let result = PauliCode::new((0b0101_0000, 0b1111_1010))
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
    let coeff = hamil.coeff(code);
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
fn orbital_from_index_01() {
    assert_eq!(Orbital::from_index(1).index(), 1);
    assert_eq!(Orbital::from_index(2).index(), 2);
    assert_eq!(Orbital::from_index(19).index(), 19);
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

const MOCK_COEFF: f64 = 0.12345;

#[test]
fn jordan_wigner_01() {
    let mut fermi_sum = SumRepr::new();
    fermi_sum.add_term(Fermions::Offset, MOCK_COEFF);

    let mut pauli_sum = SumRepr::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let coeff = pauli_sum.as_map().get(&PauliCode::default()).unwrap();
    assert!(
        (coeff - MOCK_COEFF).abs() < f64::EPSILON,
        "{MOCK_COEFF} {coeff}"
    );
}

fn check_jordan_wigner_one_pp(index: usize) {
    let mut fermi_sum = SumRepr::new();

    let p = Orbital::from_index(index);
    let integral = Fermions::one_electron(p, p).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = SumRepr::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let code = PauliCode::default();
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let code = {
        let mut code = PauliCode::default();
        code.set(index, Pauli::Z);
        code
    };
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_one_pp() {
    check_jordan_wigner_one_pp(0);
    check_jordan_wigner_one_pp(1);
    check_jordan_wigner_one_pp(2);
    check_jordan_wigner_one_pp(63);
}

fn check_jordan_wigner_one_pq(
    index1: usize,
    index2: usize,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    let p = Orbital::from_index(index1);
    let q = Orbital::from_index(index2);
    let integral = Fermions::one_electron(p, q).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = SumRepr::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let mut code = PauliCode::default();
    for i in index1 + 1..index2 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::X);
    code.set(index2, Pauli::X);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    code.set(index1, Pauli::Y);
    code.set(index2, Pauli::Y);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_one_pq() {
    check_jordan_wigner_one_pq(0, 1);
    check_jordan_wigner_one_pq(0, 3);
    check_jordan_wigner_one_pq(0, 17);

    check_jordan_wigner_one_pq(11, 17);
    check_jordan_wigner_one_pq(11, 47);
}

fn check_jordan_wigner_two_pq(
    index1: usize,
    index2: usize,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    let p = Orbital::from_index(index1);
    let q = Orbital::from_index(index2);
    let integral = Fermions::two_electron((p, q), (q, p)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = SumRepr::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let code = PauliCode::default();
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    code.set(index1, Pauli::Z);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    code.set(index2, Pauli::Z);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    code.set(index1, Pauli::Z);
    code.set(index2, Pauli::Z);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_two_pq() {
    check_jordan_wigner_two_pq(0, 1);
    check_jordan_wigner_two_pq(0, 2);
    check_jordan_wigner_two_pq(0, 3);

    check_jordan_wigner_two_pq(11, 13);
    check_jordan_wigner_two_pq(11, 33);
}

fn check_jordan_wigner_two_pqs(
    index1: usize,
    index2: usize,
    index3: usize,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    assert!(index2 > index3);
    assert!(index1 <= index3);

    let p = Orbital::from_index(index1);
    let q = Orbital::from_index(index2);
    let s = Orbital::from_index(index3);
    let integral = Fermions::two_electron((p, q), (q, s)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = SumRepr::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let mut code = PauliCode::default();
    for i in index1 + 1..index3 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::X);
    code.set(index3, Pauli::X);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    for i in index1 + 1..index3 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::Y);
    code.set(index3, Pauli::Y);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    for i in index1 + 1..index3 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::X);
    code.set(index3, Pauli::X);
    code.set(index2, Pauli::Z);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    for i in index1 + 1..index3 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::Y);
    code.set(index3, Pauli::Y);
    code.set(index2, Pauli::Z);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_two_pqs() {
    check_jordan_wigner_two_pqs(0, 2, 1);
    check_jordan_wigner_two_pqs(0, 7, 3);
    check_jordan_wigner_two_pqs(11, 13, 12);

    check_jordan_wigner_two_pqs(11, 37, 22);
}

#[allow(clippy::too_many_lines)]
fn check_jordan_wigner_two_pqrs(
    index1: usize,
    index2: usize,
    index3: usize,
    index4: usize,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    assert!(index3 > index4);
    assert!(index1 <= index4);

    let p = Orbital::from_index(index1);
    let q = Orbital::from_index(index2);
    let r = Orbital::from_index(index3);
    let s = Orbital::from_index(index4);
    let integral = Fermions::two_electron((p, q), (r, s)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = SumRepr::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let base_code = {
        let mut code = PauliCode::default();
        for i in index1 + 1..index2 {
            code.set(i, Pauli::Z);
        }
        for i in index4 + 1..index3 {
            code.set(i, Pauli::Z);
        }
        code
    };

    let mut code = base_code;
    code.set(index1, Pauli::X);
    code.set(index2, Pauli::X);
    code.set(index3, Pauli::X);
    code.set(index4, Pauli::X);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::X);
    code.set(index2, Pauli::X);
    code.set(index3, Pauli::Y);
    code.set(index4, Pauli::Y);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::X);
    code.set(index2, Pauli::Y);
    code.set(index3, Pauli::X);
    code.set(index4, Pauli::Y);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::Y);
    code.set(index2, Pauli::X);
    code.set(index3, Pauli::X);
    code.set(index4, Pauli::Y);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::Y);
    code.set(index2, Pauli::X);
    code.set(index3, Pauli::Y);
    code.set(index4, Pauli::X);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::Y);
    code.set(index2, Pauli::Y);
    code.set(index3, Pauli::X);
    code.set(index4, Pauli::X);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = -MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::X);
    code.set(index2, Pauli::Y);
    code.set(index3, Pauli::Y);
    code.set(index4, Pauli::X);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = base_code;
    code.set(index1, Pauli::Y);
    code.set(index2, Pauli::Y);
    code.set(index3, Pauli::Y);
    code.set(index4, Pauli::Y);
    let coeff = pauli_sum.as_map().get(&code).unwrap();
    let expected = MOCK_COEFF * 0.125;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );
}

#[test]
fn jordan_wigner_two_pqrs() {
    check_jordan_wigner_two_pqrs(0, 1, 2, 0);
    check_jordan_wigner_two_pqrs(0, 1, 2, 1);
    check_jordan_wigner_two_pqrs(0, 1, 3, 2);

    check_jordan_wigner_two_pqrs(11, 32, 31, 19);
    check_jordan_wigner_two_pqrs(11, 31, 61, 29);
}
