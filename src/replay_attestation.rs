//! Replay Attestation & Canonical Wire Envelope (`src/replay_attestation.rs`)
//! Serializes 80D Q32.32 phase states (`640 bytes`) into a fixed `744-byte` deterministic
//! replay frame binding step counter, SHA-256 state digest, and Ed25519 signature bytes
//! without heap allocation (`#![no_std]`).
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Deterministic DynSys)

#![no_std]

use crate::symplectic_verlet_q32::{SymplecticPhaseQ32, PHASE_DIM};

pub const MAGIC_ANIMA_V2: [u8; 4] = *b"ANM2";
pub const STATE_BYTES_LEN: usize = PHASE_DIM * 2 * 8; // 80 * 8 = 640 bytes
pub const FRAME_BYTES_LEN: usize = 4 + 4 + 8 + 32 + 64 + STATE_BYTES_LEN; // 752 bytes

/// Tamper-evident replay checkpoint frame for deterministic forensic audit.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplayCheckpointFrame {
    pub magic: [u8; 4],            // "ANM2"
    pub schema_version: u32,       // 2
    pub tick: u64,                 // Monotonic simulation step index
    pub state_sha256: [u8; 32],    // SHA-256 over the 640-byte little-endian (q, p) buffer
    pub ed25519_sig: [u8; 64],     // Detached Ed25519 signature over header + state_sha256
    pub phase: SymplecticPhaseQ32, // Exact Q32.32 (q, p) coordinates
}

impl ReplayCheckpointFrame {
    /// Serializes `(q, p)` into a canonical 640-byte little-endian buffer matching
    /// `checkpoint_sha256_hex` in `telemetry/ffi_bridge_q32.jl`.
    pub fn serialize_state_le(phase: &SymplecticPhaseQ32, out: &mut [u8; STATE_BYTES_LEN]) {
        let mut offset = 0;
        let mut i = 0;
        while i < PHASE_DIM {
            let bytes = phase.q[i].to_le_bytes();
            out[offset..offset + 8].copy_from_slice(&bytes);
            offset += 8;
            i += 1;
        }
        i = 0;
        while i < PHASE_DIM {
            let bytes = phase.p[i].to_le_bytes();
            out[offset..offset + 8].copy_from_slice(&bytes);
            offset += 8;
            i += 1;
        }
    }

    /// Constructs a signed replay checkpoint envelope from precomputed digest and signature.
    pub const fn new(
        tick: u64,
        state_sha256: [u8; 32],
        ed25519_sig: [u8; 64],
        phase: SymplecticPhaseQ32,
    ) -> Self {
        Self {
            magic: MAGIC_ANIMA_V2,
            schema_version: 2,
            tick,
            state_sha256,
            ed25519_sig,
            phase,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symplectic_verlet_q32::{Q32_ONE, Q32_ZERO};

    #[test]
    fn state_serialization_matches_julia_little_endian_layout() {
        let mut phase = SymplecticPhaseQ32 {
            q: [Q32_ZERO; PHASE_DIM],
            p: [Q32_ZERO; PHASE_DIM],
        };
        phase.q = Q32_ONE; // 0x0000_0001_0000_0000

        let mut buf = [0u8; STATE_BYTES_LEN];
        ReplayCheckpointFrame::serialize_state_le(&phase, &mut buf);

        // Little-endian representation of 2^32 (`0x0000000100000000`) has byte 4 == 0x01
        assert_eq!(buf[0..8], [0, 0, 0, 0, 1, 0, 0, 0]);
    }
}
