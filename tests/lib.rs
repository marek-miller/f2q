use std::ops::RangeBounds;

use f2q::{
    codes::{
        fermions::{
            An,
            Cr,
            FermiCode,
            FermiSum,
            Orbital,
            Spin,
        },
        qubits::{
            Pauli,
            PauliCode,
            PauliSum,
        },
    },
    maps::JordanWigner,
    math::{
        Group,
        Pairs,
        Root4,
    },
    terms::{
        SumRepr,
        Terms,
    },
};

mod codes;
mod serialize;

#[test]
fn test_sumrepr_init_01() {
    let code = PauliCode::new((1234, 0));
    let mut hamil = SumRepr::new();

    hamil.update(code, 4321.);
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
    let orb = Orbital::new(u32::MAX / 2, Spin::Up);
    assert_eq!(orb.index(), u32::MAX);
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
    let um = u32::MAX;
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

fn orbital_gen_range_idxs<R>(range: R) -> Vec<u32>
where
    R: RangeBounds<u32>,
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
    let mut fermi_sum = FermiSum::new();
    fermi_sum.add_term(FermiCode::Offset, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let coeff = pauli_sum.coeff(PauliCode::default());
    assert!(
        (coeff - MOCK_COEFF).abs() < f64::EPSILON,
        "{MOCK_COEFF} {coeff}"
    );
}

fn check_jordan_wigner_one_pp(index: u32) {
    let mut fermi_sum = SumRepr::new();

    let p = Orbital::from_index(index);
    let integral = FermiCode::one_electron(Cr(p), An(p)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let code = PauliCode::default();
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let code = {
        let mut code = PauliCode::default();
        code.set(u16::try_from(index).unwrap(), Pauli::Z);
        code
    };
    let coeff = pauli_sum.coeff(code);
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
    index1: u16,
    index2: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let integral = FermiCode::one_electron(Cr(p), An(q)).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let mut code = PauliCode::default();
    for i in index1 + 1..index2 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::X);
    code.set(index2, Pauli::X);
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.5;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    code.set(index1, Pauli::Y);
    code.set(index2, Pauli::Y);
    let coeff = pauli_sum.coeff(code);
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
    index1: u16,
    index2: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let integral =
        FermiCode::two_electron((Cr(p), Cr(q)), (An(q), An(p))).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let code = PauliCode::default();
    let coeff = pauli_sum.coeff(code);
    let expected = MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    code.set(index1, Pauli::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    code.set(index2, Pauli::Z);
    let coeff = pauli_sum.coeff(code);
    let expected = -MOCK_COEFF * 0.25;
    assert!(
        (coeff - expected).abs() < f64::EPSILON,
        "{coeff} {expected}"
    );

    let mut code = PauliCode::default();
    code.set(index1, Pauli::Z);
    code.set(index2, Pauli::Z);
    let coeff = pauli_sum.coeff(code);
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
    index1: u16,
    index2: u16,
    index3: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    assert!(index2 > index3);
    assert!(index1 <= index3);

    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let s = Orbital::from_index(u32::from(index3));
    let integral =
        FermiCode::two_electron((Cr(p), Cr(q)), (An(q), An(s))).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
    JordanWigner::new(&fermi_sum)
        .add_to(&mut pauli_sum)
        .unwrap();

    let mut code = PauliCode::default();
    for i in index1 + 1..index3 {
        code.set(i, Pauli::Z);
    }
    code.set(index1, Pauli::X);
    code.set(index3, Pauli::X);
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    index1: u16,
    index2: u16,
    index3: u16,
    index4: u16,
) {
    let mut fermi_sum = SumRepr::new();

    assert!(index1 < index2);
    assert!(index3 > index4);
    assert!(index1 <= index4);

    let p = Orbital::from_index(u32::from(index1));
    let q = Orbital::from_index(u32::from(index2));
    let r = Orbital::from_index(u32::from(index3));
    let s = Orbital::from_index(u32::from(index4));
    let integral =
        FermiCode::two_electron((Cr(p), Cr(q)), (An(r), An(s))).unwrap();
    fermi_sum.add_term(integral, MOCK_COEFF);

    let mut pauli_sum = PauliSum::new();
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
    let coeff = pauli_sum.coeff(code);
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
