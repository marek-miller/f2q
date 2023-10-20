use std::hash::Hash;

use fermions::Fermions;
use qubits::PauliCode;

/// Sum terms of a Hamiltonian
pub trait Code: Copy + Clone + Eq + Hash + Default {}

impl Code for Fermions {}
impl Code for PauliCode {}
impl Code for u64 {}

pub mod fermions;
pub mod qubits;
