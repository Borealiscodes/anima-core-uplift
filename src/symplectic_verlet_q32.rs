//! Exact Q32.32 Symplectic Störmer–Verlet Integrator (`src/symplectic_verlet_q32.rs`)
//! Preserves canonical phase-space volume (Liouville's Theorem: M^T J M = J)
//! across coupled 40D position (`q`) and 40D momentum (`p`) coordinates.
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Deterministic DynSys)

#![no_std]

pub const PHASE_DIM: usize = 40; // 40 position + 40 momentum = 80D phase space

/// Signed Q32.32 fixed-point scalar (32 integer bits + 32 fractional bits; 1.0 == 2^32).
pub type Q32 = i64;

pub const Q32_ZERO: Q32 = 0;
pub const Q32_HALF: Q32 = 1i64 << 31;
pub const Q32_ONE: Q32 = 1i64 << 32;

/// Multiplies two Q32.32 scalars using 128-bit intermediate precision (`(a * b) >> 32`).
#[inline]
pub const fn q32_mul(a: Q32, b: Q32) -> Q32 {
    let prod = (a as i128).saturating_mul(b as i128);
    (prod >> 32) as Q32
}

/// 80-dimensional canonical symplectic phase-space state `(q, p)`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymplecticPhaseQ32 {
    pub q: [Q32; PHASE_DIM], // Generalized position / world coordinates
    pub p: [Q32; PHASE_DIM], // Conjugate momentum / environmental tension
}

/// Evaluates conservative restoring force `F(q) = -∇V(q)` with optional cubic cusp bifurcation
/// (`F_i(q) = -k * q_i - beta * q_i^3`) in exact Q32.32 fixed-point arithmetic.
pub fn conservative_force_q32(
    q: &[Q32; PHASE_DIM],
    stiffness_k: Q32,
    cubic_beta: Q32,
) -> [Q32; PHASE_DIM] {
    let mut force = [Q32_ZERO; PHASE_DIM];
    let mut i = 0;
    while i < PHASE_DIM {
        let qi = q[i];
        let linear = q32_mul(stiffness_k, qi);
        let qi2 = q32_mul(qi, qi);
        let qi3 = q32_mul(qi2, qi);
        let nonlinear = q32_mul(cubic_beta, qi3);
        force[i] = linear.saturating_add(nonlinear).saturating_neg();
        i += 1;
    }
    force
}

/// Advances `(q, p)` by fixed timestep `dt` using the 2nd-order symplectic Velocity-Verlet
/// (Kick-Drift-Kick) scheme (`det(Jacobian) == 1`).
pub fn step_symplectic_verlet_q32(
    state: &mut SymplecticPhaseQ32,
    dt: Q32,
    stiffness_k: Q32,
    cubic_beta: Q32,
) {
    let half_dt = dt >> 1;

    // 1. Half-step momentum kick
    let f0 = conservative_force_q32(&state.q, stiffness_k, cubic_beta);
    let mut i = 0;
    while i < PHASE_DIM {
        state.p[i] = state.p[i].saturating_add(q32_mul(half_dt, f0[i]));
        i += 1;
    }

    // 2. Full-step position drift
    i = 0;
    while i < PHASE_DIM {
        state.q[i] = state.q[i].saturating_add(q32_mul(dt, state.p[i]));
        i += 1;
    }

    // 3. Second half-step momentum kick using updated position forces
    let f1 = conservative_force_q32(&state.q, stiffness_k, cubic_beta);
    i = 0;
    while i < PHASE_DIM {
        state.p[i] = state.p[i].saturating_add(q32_mul(half_dt, f1[i]));
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symplectic_verlet_conserves_hamiltonian_over_ten_thousand_ticks() {
        let mut state = SymplecticPhaseQ32 {
            q: [Q32_ZERO; PHASE_DIM],
            p: [Q32_ZERO; PHASE_DIM],
        };
        state.q[0] = Q32_ONE;

        let k = Q32_ONE;
        let beta = Q32_ZERO;
        let dt = Q32_ONE / 64;

        for _ in 0..10_000 {
            step_symplectic_verlet_q32(&mut state, dt, k, beta);
        }

        let q2 = q32_mul(state.q[0], state.q[0]);
        let p2 = q32_mul(state.p[0], state.p[0]);
        let hamiltonian = (q2 + p2) >> 1;
        let drift = (hamiltonian - Q32_HALF).abs();

        assert!(drift < (Q32_ONE / 2000));
    }
}
