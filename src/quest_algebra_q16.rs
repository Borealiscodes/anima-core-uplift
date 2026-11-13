//! 80-Dimensional Symplectic Quest Algebra Kernel (`src/quest_algebra_q16.rs`)
//! Zero-float signed Q16.16 fixed-point arithmetic (1.0 == 65536) with C-ABI FFI parity.
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Deterministic DynSys)

#![no_std]

pub const DIM_80: usize = 80;
pub const MATRIX_LEN: usize = DIM_80 * DIM_80; // 6,400 Int8 entries

/// Signed Q16.16 fixed-point scalar (16 integer bits + 16 fractional bits; 1.0 == 65536).
pub type Q16 = i32;

pub const Q16_ZERO: Q16 = 0;
pub const Q16_HALF: Q16 = 1 << 15; // 32768 (0.5)
pub const Q16_ONE: Q16 = 1 << 16;  // 65536 (1.0)

/// Explicit 20-channel semantic slice offsets matching DEVNOTE-001 (0-based Rust indexing).
pub const WORLD_RULES_RANGE: core::ops::Range<usize> = 0..20;
pub const CHAR_AFFINITY_RANGE: core::ops::Range<usize> = 20..40;
pub const QUEST_COORDS_RANGE: core::ops::Range<usize> = 40..60;
pub const ENV_TENSION_RANGE: core::ops::Range<usize> = 60..80;

/// C-ABI compatible 80D state vector matching Julia's `SymplecticStateQ16` layout.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymplecticStateQ16 {
    pub q: [Q16; 40],
    pub p: [Q16; 40],
}

/// Safe pure-Rust non-commutative 80x80 discrete `Int8` operator projection (`next = Op * current`).
pub fn apply_quest_operator_80d_safe(
    op_matrix: &[i8; MATRIX_LEN],
    current: &[Q16; DIM_80],
    next: &mut [Q16; DIM_80],
) {
    let mut row = 0;
    while row < DIM_80 {
        let mut acc: i64 = 0;
        let row_offset = row * DIM_80;
        let mut col = 0;
        while col < DIM_80 {
            let weight = op_matrix[row_offset + col] as i64;
            if weight != 0 {
                acc = acc.saturating_add(weight.saturating_mul(current[col] as i64));
            }
            col += 1;
        }
        next[row] = acc.clamp(i32::MIN as i64, i32::MAX as i64) as Q16;
        row += 1;
    }
}

/// C-ABI FFI entry point compatible with Julia `ccall` (`Ptr{Int8}, Ptr{Int32}, Ptr{Int32}`).
///
/// # Safety
/// Caller must ensure `op_ptr` points to 6,400 valid `i8` elements, and `curr_ptr` / `next_ptr`
/// point to 80 valid, non-overlapping `i32` elements.
#[no_mangle]
pub unsafe extern "C" fn apply_quest_operator_80d_q16(
    op_ptr: *const i8,
    curr_ptr: *const Q16,
    next_ptr: *mut Q16,
) {
    if op_ptr.is_null() || curr_ptr.is_null() || next_ptr.is_null() {
        return;
    }
    let op = &*(op_ptr as *const [i8; MATRIX_LEN]);
    let curr = &*(curr_ptr as *const [Q16; DIM_80]);
    let next = &mut *(next_ptr as *mut [Q16; DIM_80]);
    apply_quest_operator_80d_safe(op, curr, next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faction_betrayal_transition_executes_with_zero_float_drift() {
        let current = [Q16_HALF; DIM_80];
        let mut next = [Q16_ZERO; DIM_80];
        let mut op = [0i8; MATRIX_LEN];

        for i in 0..DIM_80 {
            op[i * DIM_80 + i] = 1;
        }
        for r in 0..10 {
            op[r * DIM_80 + r] = -1;
        }
        for r in 10..20 {
            op[r * DIM_80 + r] = 0;
        }
        op[21 * DIM_80 + 21] = -1;

        for r in 40..50 {
            op[r * DIM_80 + r] = 0;
            op[r * DIM_80 + (r + 10)] = 1;
        }
        for r in 60..70 {
            op[r * DIM_80 + r] = 1;
            op[r * DIM_80 + (r - 20)] = 1;
        }

        apply_quest_operator_80d_safe(&op, &current, &mut next);

        assert_eq!(next[0], -Q16_HALF);
        assert_eq!(next[15], Q16_ZERO);
        assert_eq!(next[21], -Q16_HALF);
        assert_eq!(next[40], Q16_ZERO);
        assert_eq!(next[50], Q16_ONE);
        assert_eq!(next[60], Q16_ONE);
    }
}
