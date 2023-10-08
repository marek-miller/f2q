# hamil-repr 🎇

High-octane representation of Pauli Hamiltonians with up to 64 qubits.

WIP 🚧

- Parse and convert Hamiltonian representations used in quantum chemical
  calculation into sums of Pauli strings most suitable for quantum algorithms
- Canonical representation of second-quantized Hamiltonian
- `SumRepr` can be serialized/deserialized quickly
- `Hamil` is dynamical and can store: `SumRepr`'s in various encodings,
  functions producing sum terms, iterators etc., all at the same time
- Interface easily extendible to other encodings by implementing `Terms` trait.
- Implement time evolution / dump QASM file

See documentation:

```sh
cargo doc --open
```
