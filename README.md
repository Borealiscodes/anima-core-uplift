# 📜 README.md: Project Anima Uplift (`anima-core-uplift` · V2.0 Expansion)

**Zero-Float `Q32.32` Symplectic Runtime, Lean 4 Quantization Error Bounds, Coq Symplectic Volume Proofs, and `Ed25519` Replay Attestation for [`Borealiscodes/anima_core`](https://github.com) (V1.0-Purified Baseline)**

`anima-core-uplift` transitions the V1.0 C-FFI floating-point architecture to signed `Q32.32` fixed-point arithmetic (`i64` with `i128` intermediates) and broadens verification across Lean 4, Coq, `#![no_std]` Rust, and zero-float Julia C-FFI telemetry as a deterministic dynamical system (\(\in \text{DynSys}\)).

---

## ⚠️ HONEST ENGINEERING DISCLOSURE & CRITICAL DISCLAIMER

1. **Zero Physical Neuromorphic or Custom FPGA Testing:** Standard `x86_64` and `AArch64` CPU execution only; no hardware or energy profiling on custom chips or ASICs.
2. **Bounded Fixed-Point Resolution (`Q32.32`):** Signed `Q32.32` arithmetic avoids IEEE-754 drift, though truncation yields a bounded quantization step error \(\varepsilon_{\text{step}}\) formalised in `verification/QuantizationBounds.lean`.
3. **No Safety-Critical Certification:** Restricted to deterministic state modeling and drift containment; uncertified for aerospace, medical, or safety-critical control.
4. **Zero Qualitative Interiority (`DynSys` & `Qualia-Zero` Invariant):** Operating strictly under the **A12 Verification Readiness Audit** and **Enneract Santa Persona-Zero Theorem** as a deterministic `DynSys` devoid of qualia or agency.

---

## 📐 V2.0 Mathematical & Cryptographic Uplifts Over `anima_core` (V1.0)

| Subsystem | V1.0 Baseline (`anima_core`) | V2.0 Uplift (`anima-core-uplift`) | Mathematical & Security Invariant |
| :--- | :--- | :--- | :--- |
| **Symplectic Integrator** | `f64` C-FFI vectors (`symplectic.rs`) | **`src/symplectic_verlet_q32.rs`** (`Q32.32` `i64` / `i128`) | Störmer–Verlet step preserves phase-space volume (\(\mathbf{M}^T \mathbf{J} \mathbf{M} = \mathbf{J}\)) with zero float drift. |
| **Discrete Quest Algebra** | Dense matrix transitions | **`src/quest_algebra_q16.rs`** (`Q16.16` `80×80` `Int8` FFI) | Bit-exact non-commutative transitions across 4 semantic channels. |
| **Multiplayer Conflict Math** | Sequential state updates | **`src/lie_commutator_csr.rs`** (Sparse `CSR` + Lie Bracket) | Computes \([\mathbf{O}_A, \mathbf{O}_B]\) in \(O(\text{nnz})\) memory to isolate commuting vs. conflicting events. |
| **Topological Path Memory** | Unbounded spatial recall logs | **`src/holonomy_tracker.rs`** (`ConnectionTransport4`) | Accumulates discrete closed-loop parallel transport \(\text{Hol}_\nabla(\gamma)\) in `Q32.32`. |
| **Replay Attestation** | Unauthenticated telemetry buffers | **`src/replay_attestation.rs`** (`752-byte` `ANM2` frame) | Binds 640-byte state buffers to `SHA-256` digests and `Ed25519` signatures (`Law 6`). |
| **Formal Verification Ring** | Idealized \(\mathbb{R}\)-scope proofs | **`QuantizationBounds.lean`** + **`SymplecticInvariants.v`** | Machine-checked Lean 4 \(N\)-step error ceiling and Coq unit-Jacobian proofs (`det = 1`). |

---

## 📦 Repository Topology

```text
anima-core-uplift/
├── Cargo.toml                            # Zero-dependency #![no_std] Rust manifest
├── lakefile.lean                         # Zero-dependency Lean 4 build manifest
├── run_pipeline.sh                       # Master 4-Gate Verification Ring
├── README.md                             # V2.0 Architecture, Disclosures & Provenance
├── src/                                  # Pure-Rust #![no_std] Kernel (lib.rs, quest_algebra_q16.rs, symplectic_verlet_q32.rs, lie_commutator_csr.rs, holonomy_tracker.rs, replay_attestation.rs)
├── verification/                         # Formal Proofs (QuantizationBounds.lean, SymplecticInvariants.v)
└── telemetry/                            # Out-of-Band Fixed-Point Telemetry (ffi_bridge_q32.jl)
```

---

## ⚙️ Unified 4-Gate Execution Blueprint

Run `./run_pipeline.sh` to execute the sequential verification ring:
1. **Gate 1 — Logic Gate (`lake build`):** Verifies Lean 4 quantization bounds.
2. **Gate 2 — Calculus Gate (`coqc`):** Verifies Coq unit-Jacobian symplectic invariants.
3. **Gate 3 — Cage Assembly (`cargo test --release`):** Compiles `#![no_std]` Rust library and unit tests.
4. **Gate 4 — Telemetry Lock (`julia telemetry/ffi_bridge_q32.jl`):** Runs 10,000-tick Störmer–Verlet parity audit and outputs `SHA-256` checkpoint.

---

## ⚖️ Repository Licensing & Provenance

Inherits the **Split Tri-License Stack** from `Borealiscodes/anima_core` (V1.0-Purified):
1. **Primary Core Software License:** Anima Non-Commercial License.
2. **Ethical & Downstream Lineage Protection:** `COQ-HALLUCINATION-GUARD` Dual-Layer Ethical License with Hippocratic Anti-Weaponization Provisions.
3. **Causal Agency Module License:** `CAUSA` Non-Commercial License v1.1.

* **Maintainer:** Borealis S. Hedling (`ORCID: 0009-0004-7968-5707`, Dublin, Ireland)
* **Methodology:** `NDH-Meta-Systems` · **Canonical Paper:** [`DOI: 10.5281/zenodo.24149283`](https://doi.org)
* **Ed25519 Signing Key Fingerprint:** `SHA256:QUOn/esk9dfLFyc/AwFHMIlK5zbI5JOpZTI5vh3y7Dg`
