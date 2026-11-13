(*
  SymplecticInvariants.v — Coq Formal Verification of Discrete Symplectic Volume Preservation
  Proves over exact integers (Z) that discrete Störmer-Verlet shear sub-steps preserve
  canonical phase-space area / volume (det(Jacobian) = 1, Liouville's Theorem) with zero `admit`.

  Methodology: NDH-Meta-Systems (Sealed-Layer Neutral · Deterministic DynSys)
  Canonical Reference: DOI 10.5281/zenodo.24149283
*)

From Coq Require Import ZArith Lia.
Open Scope Z_scope.

(** Exact 2x2 Jacobian determinant of a linear map `(dq', dp') = (a*dq + b*dp, c*dq + d*dp)`. *)
Definition det2 (a b c d : Z) : Z := a * d - b * c.

(** Theorem 1 (Momentum Kick Shear Preserves Phase-Space Volume):
    A discrete momentum kick `(q', p') = (q, p + k * q)` has Jacobian matrix
    `[[1, 0], [k, 1]]` with determinant identically equal to `1`. *)
Theorem momentum_kick_unit_jacobian :
  forall (k : Z), det2 1 0 k 1 = 1.
Proof.
  intros k.
  unfold det2.
  lia.
Qed.

(** Theorem 2 (Position Drift Shear Preserves Phase-Space Volume):
    A discrete position drift `(q', p') = (q + dt * p, p)` has Jacobian matrix
    `[[1, dt], [0, 1]]` with determinant identically equal to `1`. *)
Theorem position_drift_unit_jacobian :
  forall (dt : Z), det2 1 dt 0 1 = 1.
Proof.
  intros dt.
  unfold det2.
  lia.
Qed.

(** Theorem 3 (Symplectic Composition Closure):
    Composing any two unit-determinant maps preserves unit determinant (`det(A * B) = 1`),
    proving that the full Kick-Drift-Kick Störmer-Verlet step is strictly volume-preserving. *)
Theorem symplectic_composition_unit_det :
  forall (a1 b1 c1 d1 a2 b2 c2 d2 : Z),
    det2 a1 b1 c1 d1 = 1 ->
    det2 a2 b2 c2 d2 = 1 ->
    det2 (a1 * a2 + b1 * c2)
         (a1 * b2 + b1 * d2)
         (c1 * a2 + d1 * c2)
         (c1 * b2 + d1 * d2) = 1.
Proof.
  intros a1 b1 c1 d1 a2 b2 c2 d2 H1 H2.
  unfold det2 in *.
  nia.
Qed.
