# Restaurant Review dApp

This project contains the Solana review program and the Next.js frontend. Users connect a wallet, create a review on devnet, and browse reviews stored in program-owned accounts.

## Program

The Rust program stores each review in a PDA derived from the reviewer's public key and title. It supports adding and updating a review, validates ratings from 1 to 10, and rejects malformed instruction payloads. Account text is bounded so it fits the allocated 1,000-byte account. In the UI, use **Submit** to add a review or **Update my review** to change your existing review; when updating, enter the same title used to create it.

To build and test the Rust workspace from the repository root, install Rust/Cargo and run:

```sh
cargo test --workspace
```

## Frontend

1. Install Node.js, then run `npm install` from `frontend/`.
2. Copy `frontend/.env.example` to `frontend/.env.local` and set `NEXT_PUBLIC_REVIEW_PROGRAM_ID` to the deployed program's public key.
3. Run `npm run dev` from `frontend/` and open `http://localhost:3000`.
4. Connect a Solana wallet configured for devnet, enter a title, description, and rating from 1 to 10, then submit. The connected wallet must have devnet SOL for transaction fees.

The frontend defaults to the course example program ID. Replace it with your own deployed program ID after deploying the Rust program. The application targets devnet and does not send mainnet transactions.
