use std::ops::Range;

use crate::code::fermions::{
    An,
    Cr,
    Fermions,
    Orbital,
    Spin,
};

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
    assert_eq!(Orbital::with_index(1).index(), 1);
    assert_eq!(Orbital::with_index(2).index(), 2);
    assert_eq!(Orbital::with_index(19).index(), 19);
}

#[test]
fn orbital_gen_range_01() {
    let orbitals: Vec<_> = Orbital::gen_range(0..0).collect();
    assert!(orbitals.is_empty());

    let um = u32::MAX;
    let orbitals: Vec<_> = Orbital::gen_range(um..um).collect();
    assert!(orbitals.is_empty());
}

#[allow(clippy::reversed_empty_ranges)]
#[test]
fn orbital_gen_range_03() {
    let orbitals: Vec<_> = Orbital::gen_range(2..0).collect();
    assert!(orbitals.is_empty());

    let orbitals: Vec<_> = Orbital::gen_range(3..1).collect();
    assert!(orbitals.is_empty());
}

fn orbital_gen_range_idxs(range: Range<u32>) -> Vec<u32> {
    Orbital::gen_range(range).map(|orb| orb.index()).collect()
}

#[test]
fn orbital_gen_range_04() {
    assert_eq!(orbital_gen_range_idxs(0..1), &[0]);
    assert_eq!(orbital_gen_range_idxs(0..2), &[0, 1]);
    assert_eq!(orbital_gen_range_idxs(0..3), &[0, 1, 2]);

    assert_eq!(orbital_gen_range_idxs(11..15), &[11, 12, 13, 14]);
}

#[test]
fn fermions_one_elec_init_01() {
    let _ = Fermions::one_electron(
        Cr(Orbital::with_index(0)),
        An(Orbital::with_index(0)),
    )
    .unwrap();

    let _ = Fermions::one_electron(
        Cr(Orbital::with_index(0)),
        An(Orbital::with_index(1)),
    )
    .unwrap();

    let _ = Fermions::one_electron(
        Cr(Orbital::with_index(1)),
        An(Orbital::with_index(1)),
    )
    .unwrap();

    let _ = Fermions::one_electron(
        Cr(Orbital::with_index(1)),
        An(Orbital::with_index(2)),
    )
    .unwrap();

    assert!(Fermions::one_electron(
        Cr(Orbital::with_index(1)),
        An(Orbital::with_index(0)),
    )
    .is_none());

    assert!(Fermions::one_electron(
        Cr(Orbital::with_index(32)),
        An(Orbital::with_index(2)),
    )
    .is_none());
}

#[test]
fn fermions_one_elec_from_01() {
    assert_eq!(Fermions::from(()), Fermions::Offset);

    Fermions::try_from((0, 0)).unwrap();
    Fermions::try_from((0, 1)).unwrap();
    Fermions::try_from((1, 1)).unwrap();
    Fermions::try_from((1, 2)).unwrap();

    Fermions::try_from((1, 0)).unwrap_err();
    Fermions::try_from((2, 0)).unwrap_err();
    Fermions::try_from((32, 2)).unwrap_err();
}

#[test]
fn fermions_two_elec_init_01() {
    let _ = Fermions::two_electron(
        (Cr(Orbital::with_index(0)), Cr(Orbital::with_index(1))),
        (An(Orbital::with_index(1)), An(Orbital::with_index(0))),
    )
    .unwrap();

    let _ = Fermions::two_electron(
        (Cr(Orbital::with_index(0)), Cr(Orbital::with_index(2))),
        (An(Orbital::with_index(1)), An(Orbital::with_index(0))),
    )
    .unwrap();

    let _ = Fermions::two_electron(
        (Cr(Orbital::with_index(0)), Cr(Orbital::with_index(1))),
        (An(Orbital::with_index(2)), An(Orbital::with_index(1))),
    )
    .unwrap();

    assert!(Fermions::two_electron(
        (Cr(Orbital::with_index(0)), Cr(Orbital::with_index(0))),
        (An(Orbital::with_index(0)), An(Orbital::with_index(0))),
    )
    .is_none());

    assert!(Fermions::two_electron(
        (Cr(Orbital::with_index(0)), Cr(Orbital::with_index(1))),
        (An(Orbital::with_index(0)), An(Orbital::with_index(0))),
    )
    .is_none());

    assert!(Fermions::two_electron(
        (Cr(Orbital::with_index(0)), Cr(Orbital::with_index(1))),
        (An(Orbital::with_index(1)), An(Orbital::with_index(1))),
    )
    .is_none());

    assert!(Fermions::two_electron(
        (Cr(Orbital::with_index(1)), Cr(Orbital::with_index(2))),
        (An(Orbital::with_index(1)), An(Orbital::with_index(0))),
    )
    .is_none());
}

#[test]
fn fermions_two_from_01() {
    Fermions::try_from((0, 1, 1, 0)).unwrap();
    Fermions::try_from((0, 2, 1, 0)).unwrap();
    Fermions::try_from((0, 1, 2, 1)).unwrap();

    Fermions::try_from((0, 0, 0, 0)).unwrap_err();
    Fermions::try_from((0, 1, 0, 0)).unwrap_err();
    Fermions::try_from((0, 1, 1, 1)).unwrap_err();
    Fermions::try_from((1, 2, 1, 0)).unwrap_err();
}
