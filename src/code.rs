//! Encoding of Hamiltonian terms.

use std::hash::Hash;

use fermions::FermiCode;
use qubits::PauliCode;

/// Sum terms of a Hamiltonian
pub trait Code: Copy + Clone + Eq + Hash + Default {}

impl Code for FermiCode {}
impl Code for PauliCode {}
impl Code for u64 {}

pub mod fermions;
pub mod qubits;
