# Installation

Get Uncovr installed and ready to use.

## Prerequisites

- Rust 1.85 or later
- Cargo (included with Rust)

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Create a New Project

Start by creating a new Rust project:

```bash
cargo new my-api
cd my-api
```

## Add Uncovr

Add Uncovr to your `Cargo.toml`:

```bash
cargo add uncovr
```

This installs the latest version of Uncovr with default features (CORS and logging).

## Add Required Dependencies

Uncovr needs a few additional dependencies:

```bash
cargo add tokio --features full
cargo add serde --features derive
```

- **tokio**: Async runtime (required)
- **serde**: Serialization for JSON (required)

## Verify Installation

Check that everything is installed:

```bash
cargo build
```

If the build succeeds, Uncovr is ready to use.
