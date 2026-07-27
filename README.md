<h1 align="center">Paraloom Prover (WASM)</h1>

<p align="center">
  <strong>Client-side Groth16 proving for the Paraloom wallet — spend keys never leave the browser</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-stable-orange" alt="Rust"/>
  <img src="https://img.shields.io/badge/curve-BN254-informational" alt="BN254"/>
  <a href="https://github.com/paraloom-labs/paraloom-prover-wasm/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License"/></a>
</p>

<p align="center">
  <a href="https://github.com/paraloom-labs/paraloom-core">paraloom-core</a> •
  <a href="https://docs.paraloom.io">Documentation</a>
</p>

---

## What this is

The wallet builds its zero-knowledge proofs here, in the browser, from note
secrets that never cross the network. This crate compiles the Paraloom circuits
to WebAssembly and exposes them to JavaScript.

Which is the reason it is worth reading rather than trusting: it handles spend
keys. A prover you cannot inspect is a prover you are taking on faith, and the
whole point of a shielded pool is not having to.

## The circuits are not written here

`src/privacy/mod.rs` includes paraloom-core's own sources by path:

```rust
#[path = "../../../paraloom-core/src/privacy/circuits.rs"]
pub mod circuits;
```

So the browser proves against the same constraint system the validators verify
and the on-chain program checks — not a reimplementation that has to be kept in
step. The same applies to the Poseidon gadgets, the QAP reduction and the
proof-suite codec. A copy would be one edit away from disagreeing, and the
symptom would be proofs rejected with nothing pointing at why.

This crate holds the wasm boundary: hex and JSON in, hex out, plus the witness
assembly each circuit needs.

## Exports

| Function | What it does |
|---|---|
| `prove_transact` | Circuit v3 unified transact — the current settlement path. 2-in/2-out; deposit, transfer and withdrawal are the same proof with a different signed `public_amount`. |
| `prove_withdrawal_v2` | Spend-key withdrawal (pre-v3 flow). |
| `prove_transfer_v2` | Spend-key shielded transfer (pre-v3 flow). |
| `v3_note_pubkey` | `Poseidon(privkey)` — the spend public key a note commits to. |
| `v3_note_commitment` | The v3 note commitment. |
| `v3_merkle_path` | Builds a Merkle path locally from the leaf set, so the wallet never asks a server which leaf is its own. |
| `note_commitment` | The v2 note commitment. |

## Build

```bash
wasm-pack build --target web --release
```

Output lands in `pkg/`, which is gitignored — what ships is built at release
time, and a stale committed copy would be worse than none.

The build expects `paraloom-core` checked out as a sibling directory, since the
circuit sources are included by relative path.

```bash
cargo test                 # native tests, including a full transact round-trip
cargo test -- --ignored    # the slower proving smoke tests
```

## Trusted setup

Proofs are produced under `CircomReduction`, matching the phase-2 ceremony,
which runs in snarkjs on the public [Perpetual Powers of
Tau](https://github.com/privacy-ethereum/perpetualpowersoftau). A key and its
QAP reduction move together: arkworks' default reduction against a ceremony key
produces proofs that do not verify.

Ceremony transcripts and the verification command are published in
[paraloom-core](https://github.com/paraloom-labs/paraloom-core).

## Status

Pre-mainnet, devnet only. Circuit v3 is the live settlement path; the v2 entry
points remain for flows that predate it. See
[paraloom-core](https://github.com/paraloom-labs/paraloom-core) for the honest
list of what is and is not yet guaranteed, and
[`docs/bug-bounty.md`](https://github.com/paraloom-labs/paraloom-core/blob/main/docs/bug-bounty.md)
for the scope — this crate is in it.

## License

MIT.
