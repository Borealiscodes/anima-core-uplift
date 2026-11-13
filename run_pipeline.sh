#!/usr/bin/env bash
# Master 4-Gate Verification Ring for `anima-core-uplift` (V2.0)
# Runs: [1] Lean 4 Proofs -> [2] Coq Symplectic Proofs -> [3] Rust #![no_std] Tests -> [4] Julia Q32.32 Telemetry

set -euo pipefail

echo "========================================================="
echo " 🛡️  ANIMA-CORE-UPLIFT (V2.0) — 4-GATE VERIFICATION RING"
echo "========================================================="

if command -v lake >/dev/null 2>&1; then
  echo "[1/4] Logic Gate: Verifying Lean 4 QuantizationBounds..."
  lake build
else
  echo "[1/4] Logic Gate: 'lake' not found locally (skipping Lean 4 check)."
fi

if command -v coqc >/dev/null 2>&1; then
  echo "[2/4] Calculus Gate: Verifying Coq SymplecticInvariants..."
  coqc verification/SymplecticInvariants.v
else
  echo "[2/4] Calculus Gate: 'coqc' not found locally (skipping Coq check)."
fi

if command -v cargo >/dev/null 2>&1; then
  echo "[3/4] Cage Assembly: Building & testing zero-float Rust kernel..."
  cargo test --release
else
  echo "[3/4] Cage Assembly: 'cargo' not found locally (skipping Rust build)."
fi

if command -v julia >/dev/null 2>&1; then
  echo "[4/4] Telemetry Lock: Running Julia Q32.32 FFI parity audit..."
  julia telemetry/ffi_bridge_q32.jl
else
  echo "[4/4] Telemetry Lock: 'julia' not found locally (skipping Julia audit)."
fi

echo "========================================================="
echo " ✅ VERIFICATION PIPELINE COMPLETE (Deterministic DynSys)"
echo "========================================================="
