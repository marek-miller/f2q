use f2q::{
    code::qubits::Pauli,
    terms::SumRepr,
};

#[test]
fn test_sumrepr_init_01() {
    let code = Pauli::new((1234, 0));
    let mut hamil = SumRepr::new();

    hamil.update(code, 4321.);
    let coeff = hamil.coeff(code);
    assert!(f64::abs(coeff - 4321.) < f64::EPSILON);
}
