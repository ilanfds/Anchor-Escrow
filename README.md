# Anchor Escrow Program

A Solana escrow program built with the [Anchor](https://www.anchor-lang.com/) framework. It enables trustless peer-to-peer token swaps between two parties using an on-chain escrow account and a PDA-controlled vault.

## How It Works

1. **Make** – A *maker* creates an escrow by depositing Token A into a program-owned vault and specifying how much of Token B they want in return.
2. **Take** – A *taker* fulfills the escrow by sending the requested Token B to the maker and receiving the escrowed Token A from the vault.
3. **Refund** – The maker can cancel the escrow at any time, withdrawing their Token A back and closing the escrow account.

## Project Structure

```
programs/anchor_escrow/src/
├── lib.rs              # Program entry point & instruction routing
├── state.rs            # Escrow account definition
├── errors.rs           # Custom error codes
└── instructions/
    ├── make.rs         # Create escrow & deposit tokens
    ├── take.rs         # Fulfill escrow & swap tokens
    └── refund.rs       # Cancel escrow & withdraw tokens
```

## Tech Stack

- **Solana** – High-performance L1 blockchain
- **Anchor** – Rust framework for Solana programs
- **SPL Token Interface** – Supports both SPL Token and Token-2022

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (see `rust-toolchain.toml`)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Yarn](https://yarnpkg.com/)

### Build & Test

```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run tests (localnet)
anchor test
```

## License

MIT
