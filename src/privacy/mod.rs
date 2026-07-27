// Reuse the real circuit and hash code from paraloom-core, included by path so
// there is a single source of truth for the audit-critical circuit. The module
// tree mirrors paraloom-core's so the files' `crate::privacy::poseidon` and
// `super::` references resolve unchanged.

#[path = "../../../paraloom-core/src/privacy/poseidon.rs"]
pub mod poseidon;

// The snarkjs-compatible QAP reduction `circuits.rs` proves under. The
// ceremony key comes out of snarkjs, which prepares the powers of tau in
// Lagrange basis, so the prover has to match it — the browser and the node
// must produce proofs the same way or only one of them verifies.
#[path = "../../../paraloom-core/src/privacy/circom_reduction.rs"]
pub mod circom_reduction;

#[path = "../../../paraloom-core/src/privacy/circuits.rs"]
pub mod circuits;

// Circuit v3 (#350): the circom-compatible fixed-width Poseidon the unified
// transact circuit hashes with — bit-identical to the on-chain sol_poseidon
// syscall, which is what lets the program maintain the tree itself.
#[path = "../../../paraloom-core/src/privacy/poseidon_circom.rs"]
pub mod poseidon_circom;

// `circuits.rs` only needs the asset-id primitives from paraloom-core's `types`
// module; the full module pulls in note encryption (crypto_box) the prover does
// not use, so we mirror just these two stable items here. Native-SOL only.
pub mod types {
    pub type AssetId = [u8; 32];
    pub const NATIVE_SOL_ASSET: AssetId = [0u8; 32];
}
