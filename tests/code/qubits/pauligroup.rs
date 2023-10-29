use f2q::{
    code::qubits::{
        Pauli,
        PauliGroup,
        PauliOp,
    },
    math::{
        Group,
        Root4,
    },
};

#[test]
fn identity() {
    let e = PauliGroup::identity();

    let g = PauliGroup::from(Pauli::new((0, 0)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(Pauli::new((1, 2)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);

    let g = PauliGroup::from(Pauli::new((12345, 67890)));
    assert_eq!(e * g, g);
    assert_eq!(g * e, g);
}

#[test]
fn paui_group_01() {
    let e = PauliGroup::identity();

    assert_eq!(e * PauliGroup::from(Root4::R0), PauliGroup::from(Root4::R0));
    assert_eq!(e * PauliGroup::from(Root4::R1), PauliGroup::from(Root4::R1));
    assert_eq!(e * PauliGroup::from(Root4::R2), PauliGroup::from(Root4::R2));
    assert_eq!(e * PauliGroup::from(Root4::R3), PauliGroup::from(Root4::R3));

    assert_eq!(PauliGroup::from(Root4::R0) * e, PauliGroup::from(Root4::R0));
    assert_eq!(PauliGroup::from(Root4::R1) * e, PauliGroup::from(Root4::R1));
    assert_eq!(PauliGroup::from(Root4::R2) * e, PauliGroup::from(Root4::R2));
    assert_eq!(PauliGroup::from(Root4::R3) * e, PauliGroup::from(Root4::R3));
}

#[test]
fn paui_group_02() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));
    let e = PauliGroup::identity();

    assert_eq!(g * g, e);
}

#[test]
fn paui_group_03() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R0, Pauli::with_ops([X]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R0, Pauli::with_ops([I, Y, Z]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R0, Pauli::with_ops([I, Y, Z]))
    );
}

#[test]
fn paui_group_04() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R0, Pauli::with_ops([Y]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R2, Pauli::with_ops([Z, Y, Z]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R3, Pauli::with_ops([Z, Y, Z]))
    );
}

#[test]
fn paui_group_05() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R3, Pauli::with_ops([I, Z]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R0, Pauli::with_ops([X, X, Z]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R1, Pauli::with_ops([X, X, Z]))
    );
}

#[test]
fn paui_group_06() {
    use PauliOp::*;
    let g = PauliGroup::new(Root4::R0, Pauli::with_ops([X, Y, Z]));

    let h = PauliGroup::new(Root4::R1, Pauli::with_ops([I, Z, X]));
    assert_eq!(
        g * h,
        PauliGroup::new(Root4::R0, Pauli::with_ops([X, X, Y]))
    );
    assert_eq!(
        h * g,
        PauliGroup::new(Root4::R0, Pauli::with_ops([X, X, Y]))
    );
}
