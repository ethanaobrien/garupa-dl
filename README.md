# Overview

This is a downloader tool written in rust to download assets from the CDN server for the game "BanG Dream! Girls Band Party"

This software was written for preservation purposes, to back up and save old content indefinitely

# Development Environment

This project is written in **Rust** (edition 2024) and is built and run with `cargo` (the Rust toolchain, rustc/cargo 1.98.0). It was written using RustRover.

The main crates used are:

- `tokio` — the async runtime that powers the `.await` network calls
- `reqwest` — the HTTP client, used to make network requests
- `http` — typed HTTP request headers
- `prost` / `prost-build` — decodes the protobuf responses. `build.rs` compiles the `.proto` files in `proto/` into Rust structs at build time
- `clap` — parses the command-line arguments
- `aes` + `cbc` — AES-128-CBC decryption with ISO 10126 padding, used to decrypt the encrypted API responses

# Useful Websites

- [Rust Standard Library](https://doc.rust-lang.org/std/) — async, `Result`/`?`, and slice handling
- [reqwest](https://docs.rs/reqwest/latest/reqwest/) — HTTP client
- [tokio](https://tokio.rs/tokio/tutorial) — async runtime
- [prost](https://docs.rs/prost/latest/prost/) — protobuf decoding
- [clap](https://docs.rs/clap/latest/clap/) — command-line argument parsing
- [cbc](https://docs.rs/cbc/latest/cbc/) / [aes](https://docs.rs/aes/latest/aes/) — AES-128-CBC decryption
- [ghidra](https://github.com/nationalsecurityagency/ghidra) / [il2cppdumper](https://github.com/perfare/il2cppdumper) - decompiling and understanding how the network asset layer works

# Future Work

- Add per-bundle integrity checks (a checksum). Resume currently trusts file size alone, so a truncated or corrupted bundle could be skipped as if it were complete.
- Multi-threaded downloads. They run one at a time at the moment, which is slow for a manifest with tens of thousands of bundles.
- Act on a detected update. `check_for_update` reports one but takes no further action.
