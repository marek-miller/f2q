//! Encoding of Hamiltonian terms.

use std::hash::Hash;

use fermions::FermiCode;
use qubits::Pauli;

/// Sum terms of a Hamiltonian
pub trait Code: Copy + Clone + Eq + Hash + Default {}

impl Code for FermiCode {}
impl Code for Pauli {}
impl Code for u64 {}

pub mod fermions;
pub mod qubits;
