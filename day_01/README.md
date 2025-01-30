# Day 1: Solana Hello World (Installation and Troubleshooting)

[Day 1](https://www.rareskills.io/post/hello-world-solana) of [RareSkills Solana Course](https://www.rareskills.io/solana-tutorial).

## Setup

Follow the [RareSkills tutorial](https://www.rareskills.io/post/hello-world-solana) for detailed steps.

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Solana CLI: `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`
3. Install Anchor:

```
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

avm install latest
avm use latest
```

4. Initialize and build an Anchor Program

```
anchor init day_1 # I'm using a mac
cd day_1
anchor build
```

5. Configure Solana to run on localhost: `solana config set --url localhost`
6. Run the test validator node on another terminal: `solana-test-validator`
7. Run Solana logs on another terminal: `solana logs`
8. Sync program_id with Anchor key: `anchor keys sync`
9. Airdrop SOL: `solana airdrop 100`
10. Run tests: `anchor test --skip-local-validator`

# Note:

- Command line version:
  ```
    rustc --version && \
    cargo  --version && \
    rustup --version && \
    solana --version
    rustc 1.79.0 (129f3b996 2024-06-10)
    cargo 1.79.0 (ffa9cf99a 2024-06-03)
    rustup 1.27.1 (54dd3d00f 2024-04-24)
    info: This is the version for the rustup toolchain manager, not the rustc compiler.
    info: The currently active `rustc` version is `rustc 1.79.0 (129f3b996 2024-06-10)`
    solana-cli 2.0.21 (src:843c6a1a; feat:607245837, client:Agave)
  ```

## Troubleshooting and References

- [error: package `solana-program v1.18.0` cannot be built because it requires rustc 1.72.0 or newer, while the currently active rustc version is 1.68.0-dev](https://github.com/solana-labs/solana/issues/34987)
- [error: use of unstable library feature `build_hasher_simple_hash_one`](https://solana.stackexchange.com/questions/8800/error-use-of-unstable-library-feature-build-hasher-simple-hash-one)
- [Solana devnet/testnet SOL airdrop](https://solana.com/developers/guides/getstarted/solana-token-airdrop-and-faucets)
- [Solana deploy "account data too small for instruction"](https://stackoverflow.com/questions/71267943/solana-deploy-account-data-too-small-for-instruction)
- [How to run different Anchor test files from yarn script command?](https://github.com/coral-xyz/anchor/issues/1317)
