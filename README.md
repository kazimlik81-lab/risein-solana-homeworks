# Rise In Solana Homeworks

Solutions for the Counter and Token Transfer projects in Rise In's Solana course.

## Homework 1 — Counter updates

`homework-1-counter` accepts a `u32` value for increment and decrement. Increment reports overflow as invalid account data, decrement saturates at zero, and the existing update and reset instructions remain supported. The instruction decoder rejects malformed payloads.

## Homework 2 — Fixed token transfer

`homework-2-token-transfer` uses SPL Token `transfer_checked` to send a fixed 10,000 base units from the program-derived source account. The test funds the source with 15,000 and checks that the destination receives 10,000 while 5,000 remain at the source.

## Restaurant Review project

`restaurant-review/` contains the Rust review program and a Next.js wallet frontend. The program stores user reviews in PDAs and supports add/update instructions. See its README for build and devnet setup.

## Build and test

With Rust and Cargo installed, run from the repository root:

```sh
cargo test --workspace
```

The token-transfer tests use Solana Program Test and the local SPL Token program; no mainnet transaction or real token is needed.
