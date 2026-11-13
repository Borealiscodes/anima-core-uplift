//! Compressed Sparse Row (CSR) 80x80 Operators & Exact Lie Commutator Bracket (`src/lie_commutator_csr.rs`)
//! Computes `[A, B] = A * B - B * A` in `#![no_std]` fixed-capacity memory
//! to detect non-commutative state conflicts without heap allocation.
//! Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Deterministic DynSys)

#![no_std]

pub const DIM_80: usize = 80;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsrError {
    CapacityExceeded,
    OutOfBounds,
    UnsortedInput,
}

/// Fixed-capacity Compressed Sparse Row (`CSR`) 80x80 discrete `i16` operator matrix.
pub struct SparseOperator80<const MAX_NNZ: usize> {
    pub row_ptr: [u16; DIM_80 + 1],
    pub col_idx: [u8; MAX_NNZ],
    pub values: [i16; MAX_NNZ],
    pub nnz: usize,
}

impl<const MAX_NNZ: usize> SparseOperator80<MAX_NNZ> {
    pub const fn empty() -> Self {
        Self {
            row_ptr: [0u16; DIM_80 + 1],
            col_idx: [0u8; MAX_NNZ],
            values: [0i16; MAX_NNZ],
            nnz: 0,
        }
    }

    /// Constructs a `CSR` operator from row-major sorted `(row, col, val)` triplets.
    pub fn from_sorted_triplets(triplets: &[(u8, u8, i16)]) -> Result<Self, CsrError> {
        let mut out = Self::empty();
        let mut prev_key: Option<(u8, u8)> = None;

        for &(r, c, v) in triplets {
            if r as usize >= DIM_80 || c as usize >= DIM_80 {
                return Err(CsrError::OutOfBounds);
            }
            if let Some((pr, pc)) = prev_key {
                if (r, c) <= (pr, pc) {
                    return Err(CsrError::UnsortedInput);
                }
            }
            prev_key = Some((r, c));

            if v == 0 {
                continue;
            }
            if out.nnz >= MAX_NNZ {
                return Err(CsrError::CapacityExceeded);
            }
            out.col_idx[out.nnz] = c;
            out.values[out.nnz] = v;
            out.nnz += 1;
            out.row_ptr[r as usize + 1] += 1;
        }

        let mut r = 0;
        while r < DIM_80 {
            out.row_ptr[r + 1] = out.row_ptr[r + 1].saturating_add(out.row_ptr[r]);
            r += 1;
        }
        Ok(out)
    }

    /// Sparse matrix-vector product `y = A * x` in `O(nnz)` time.
    pub fn apply_q32(&self, x: &[i64; DIM_80], y: &mut [i64; DIM_80]) {
        let mut r = 0;
        while r < DIM_80 {
            let start = self.row_ptr[r] as usize;
            let end = self.row_ptr[r + 1] as usize;
            let mut acc: i64 = 0;
            let mut idx = start;
            while idx < end {
                let c = self.col_idx[idx] as usize;
                let w = self.values[idx] as i64;
                acc = acc.saturating_add(w.saturating_mul(x[c]));
                idx += 1;
            }
            y[r] = acc;
            r += 1;
        }
    }

    /// Exact dense entry lookup `A[row, col]` via binary search within the row slice.
    pub fn get(&self, row: usize, col: usize) -> i16 {
        if row >= DIM_80 || col >= DIM_80 {
            return 0;
        }
        let mut lo = self.row_ptr[row] as usize;
        let mut hi = self.row_ptr[row + 1] as usize;
        let target = col as u8;
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            let c = self.col_idx[mid];
            if c == target {
                return self.values[mid];
            } else if c < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        0
    }
}

/// Computes the exact Lie commutator bracket `[A, B] = A * B - B * A`.
pub fn operators_commute<const MAX_A: usize, const MAX_B: usize, const MAX_OUT: usize>(
    a: &SparseOperator80<MAX_A>,
    b: &SparseOperator80<MAX_B>,
    commutator_out: &mut SparseOperator80<MAX_OUT>,
) -> Result<bool, CsrError> {
    *commutator_out = SparseOperator80::empty();
    let mut is_zero = true;

    let mut r = 0;
    while r < DIM_80 {
        let mut c = 0;
        while c < DIM_80 {
            let mut diff: i32 = 0;

            let a_start = a.row_ptr[r] as usize;
            let a_end = a.row_ptr[r + 1] as usize;
            let mut idx = a_start;
            while idx < a_end {
                let k = a.col_idx[idx] as usize;
                let av = a.values[idx] as i32;
                let bv = b.get(k, c) as i32;
                if bv != 0 {
                    diff += av * bv;
                }
                idx += 1;
            }

            let b_start = b.row_ptr[r] as usize;
            let b_end = b.row_ptr[r + 1] as usize;
            idx = b_start;
            while idx < b_end {
                let k = b.col_idx[idx] as usize;
                let bv = b.values[idx] as i32;
                let av = a.get(k, c) as i32;
                if av != 0 {
                    diff -= bv * av;
                }
                idx += 1;
            }

            if diff != 0 {
                is_zero = false;
                if commutator_out.nnz >= MAX_OUT {
                    return Err(CsrError::CapacityExceeded);
                }
                commutator_out.col_idx[commutator_out.nnz] = c as u8;
                commutator_out.values[commutator_out.nnz] =
                    diff.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                commutator_out.nnz += 1;
                commutator_out.row_ptr[r + 1] += 1;
            }
            c += 1;
        }
        r += 1;
    }

    let mut row = 0;
    while row < DIM_80 {
        commutator_out.row_ptr[row + 1] =
            commutator_out.row_ptr[row + 1].saturating_add(commutator_out.row_ptr[row]);
        row += 1;
    }

    Ok(is_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lie_bracket_distinguishes_commuting_and_conflicting_events() {
        let op_a = SparseOperator80::<8>::from_sorted_triplets(&[(0, 0, -1)]).unwrap();
        let op_b = SparseOperator80::<8>::from_sorted_triplets(&[(1, 1, 2)]).unwrap();
        let mut comm = SparseOperator80::<16>::empty();

        assert_eq!(operators_commute(&op_a, &op_b, &mut comm), Ok(true));
        assert_eq!(comm.nnz, 0);

        let op_c = SparseOperator80::<8>::from_sorted_triplets(&[(0, 1, 1), (1, 0, 1)]).unwrap();
        let op_d = SparseOperator80::<8>::from_sorted_triplets(&[(0, 0, 2)]).unwrap();

        assert_eq!(operators_commute(&op_c, &op_d, &mut comm), Ok(false));
        assert!(comm.nnz > 0);
    }
}
