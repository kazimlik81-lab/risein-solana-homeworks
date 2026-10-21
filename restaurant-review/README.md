# Restaurant Review dApp

This project contains a Solana review program and its Next.js wallet frontend. Users connect a wallet, create or update a review on devnet, and browse or filter reviews by location. Every review stores a title, rating, description, and location in a program-derived account.

## Program

The Rust program stores each review in a PDA derived from the reviewer's public key and title. Add and update instructions both serialize the location string. The account state retains it, and the frontend decoder reads the same Borsh field order. Ratings are validated from 1 to 10, malformed payloads are rejected, and combined UTF-8 text length is bounded to the allocated 1,000-byte account. In the UI, enter the same title used to create a review when updating it, then use the location search to filter reviews.

### Build and test

Install Rust/Cargo and the Solana CLI/toolchain, then run from the repository root:

```sh
cargo test --workspace
cargo build-sbf --manifest-path restaurant-review/program/Cargo.toml
```

The instruction tests verify that add/update payloads carry `location`, reject missing or trailing data, and round-trip the location in account state.

### Deploy to Solana devnet

1. Install the Solana CLI and configure a devnet wallet. Fund it with devnet SOL for rent and transaction fees.
2. Build the program using `cargo build-sbf --manifest-path restaurant-review/program/Cargo.toml`.
3. Deploy the generated shared object: `solana program deploy target/deploy/restaurant_review.so --url devnet`.
4. Copy the printed program ID into `frontend/.env.local` as `NEXT_PUBLIC_REVIEW_PROGRAM_ID=<program-id>` (see `.env.example`).
5. Keep the deployer's program keypair backed up securely; it controls future program upgrades.

## Frontend

1. Install Node.js, then run `npm install` from `frontend/`.
2. Copy `frontend/.env.example` to `frontend/.env.local` and set `NEXT_PUBLIC_REVIEW_PROGRAM_ID` to the deployed program's public key.
3. Run `npm run dev` from `frontend/` and open `http://localhost:3000`.
4. Connect a Solana wallet configured for devnet, enter a title, location, description, and rating from 1 to 10, then submit. The connected wallet must have devnet SOL for transaction fees.

The frontend defaults to the course example program ID. Replace it with your own deployed program ID after deploying the Rust program. The application targets devnet and does not send mainnet transactions.

To interact, run `npm install` and `npm run dev` from `restaurant-review/frontend/`, open `http://localhost:3000`, connect a wallet set to devnet, and submit a review with title, location, description, and a rating from 1 to 10. To update it, use the original title so the app derives the same PDA, then select **Update my review**. Use **Filter reviews by location** to search the saved location strings. A successful transaction includes a Solana Explorer devnet link.
