//! Project Anima Uplift (`anima-core-uplift` V2.0) — Unified `#![no_std]` Kernel
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Deterministic DynSys)
//! Canonical Reference: DOI 10.5281/zenodo.24149283

#![no_std]

pub mod holonomy_tracker;
pub mod lie_commutator_csr;
pub mod quest_algebra_q16;
pub mod replay_attestation;
pub mod symplectic_verlet_q32;

pub use holonomy_tracker::{ConnectionTransport4, HolonomyLoopTracker};
pub use lie_commutator_csr::{operators_commute, CsrError, SparseOperator80};
pub use quest_algebra_q16::{
    apply_quest_operator_80d_q16, apply_quest_operator_80d_safe, SymplecticStateQ16, DIM_80,
    MATRIX_LEN, Q16, Q16_HALF, Q16_ONE, Q16_ZERO,
};
pub use replay_attestation::{
    ReplayCheckpointFrame, FRAME_BYTES_LEN, MAGIC_ANIMA_V2, STATE_BYTES_LEN,
};
pub use symplectic_verlet_q32::{
    conservative_force_q32, q32_mul, step_symplectic_verlet_q32, SymplecticPhaseQ32, PHASE_DIM,
    Q32, Q32_HALF, Q32_ONE, Q32_ZERO,
};
