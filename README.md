# f2q 🎇

[![Test](https://github.com/Quantum-for-Life/f2q/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/Quantum-for-Life/f2q/actions/workflows/test.yml)

Fermion-to-qubit mappings. High-octane representation of Pauli Hamiltonians with
up to 64 qubits.

This is a software library and a command line tool to parse and convert quantum
chemistry Hamiltonians into a form suitable for quantum hardware based on qubit
gates.

# How to use it

Make sure you have the Rust compiler installed, in version at least 1.70: 🦀

```bash
rustc --version
```

If not, update Rust:

```bash
rustup update
```

or check out the
[official Rust website](https://www.rust-lang.org/learn/get-started) on how to
get started.

If you only want to use the accompanying command line tool: `f2q`, install the
executable like this:

```bash
cargo install f2q
f2q --help
```

To include the software library into you Rust project, use the package available
on [crates.io](https://crates.io/crates/f2q):

```bash
cargo add f2q
```

The full documentation is available
[online at docs.rs](https://docs.rs/f2q/latest/f2q/index.html). 📚

# Testing

To run the library's test suite, make sure you have the
[nightly Rust](https://rust-lang.github.io/rustup/concepts/channels.html)
toolchain installed:

```bash
rustup install nightly
```

Clone the `f2q` repository to your local system:

```bash
git clone https://github.com/Quantum-for-Life/f2q.git
cd f2q
```

and set nightly Rust as the default toolchain for this repo:

```bash
rustup override set nightly
```

Now, you can run the tests: ⚙️🪛

```
cargo test
```

# Contributing

All contributions are welcome. 💐

Before submitting patches, please reformat your code as specified in
`rustfmt.toml` by running:

```bash
cargo clippy --fix
cargo fmt
```

or if you have [`just`](https://just.systems/) installed:

```
just lint
```
