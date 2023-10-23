//! Encoding of Hamiltonian terms.

use std::hash::Hash;

use fermions::Fermions;
use qubits::Pauli;

pub mod fermions;
pub mod qubits;

/// Sum terms of a Hamiltonian
pub trait Code: Copy + Clone + Eq + Hash + Default {}

impl Code for Fermions {}
impl Code for Pauli {}
impl Code for u64 {}
