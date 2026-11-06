### LAW-COUPLING v1's served pulls stand on real frames, and the pure Z pole neither couples nor is coupled (proposal 78.175 §5, §8)
- **Network contract: the `FORMS-COUPLING` v1 clause's fixtures change.**
  The clause stays at kernel standing (signable and vetoable) on
  `ACT_LAWS` mode 7, with the same fixture and object writing (fixture tag
  12, object tag 9). It now serves 27 fixtures, nine pulls in place of
  seven, and the inputs and objects of pull cases 1–5 change. A kernel
  that signed the clause must reconstruct the restated fixtures and sign
  again, or veto it; no act or ledger layout changes. The law's version
  stays 1.
- **On frames `σ = 1 − 2o`**, so the stated `σ = ±1` with `o = 1/2` is no
  body. F3's cases are restated on hydrogen (`Z 1, U 1`: frames `(2, 1)`,
  `σ 1/3`, `o 1/3`) against the open-heavy kind (`Z 2, U 4`: frames
  `(2, 4)`, `σ −1/3`, `o 2/3`), `x = 1/5`, `x̄ = 1/2`, `w 1`, `n 2/3`:
  - lock, `Δ 0°`: `F −4/27`, `y 4/729`, `x′ 749/3649`;
  - split, the neighbour's frames swapped `(4, 2)`: `F 2/27`,
    `y −2/729`, `x′ 719/3643`;
  - `Δ 45°` (`h 1/2`): `F −2/27`, `y 2/729`, `x′ 739/3647`;
  - alone and the throat neighbour keep `x′ 1/5`; case 6 (`(3, 1)`
    against hydrogen, `F 2/27`, `x′ 319/1619`) and case 7 (near the bound)
    are kept.
- **The pure Z pole** (frames `(1, 0)`, `σ 1`, `o 0`) has no moment: as a
  neighbour its term is 0 (`F 0`), and as a body it is not moved
  (`y 0`, `x′ 1/5`). Two fixtures serve it, and
  `pure_z_pole_neither_couples_nor_is_coupled` claims it over fields
  across `[−1, 1]`.
- The former stated-dipole cases stay in `forms` only, as the test
  `algebra_check_of_pull`, outside the fixtures served.
- **78.175 §8:** each plane couples by its own dipole (netstratum #110);
  the held rate's conservation is spin per plane, `Σ m·ℓ_k`, the rate
  following the spin at fixed reach (a later fixture); the quantum level
  waits on a rate on `G`; the later alignment is read per invariant plane
  of the relative rotation, one angle per plane, never summed, a
  hyperbolic plane by its boost's exponent (netstratum's S³ `(α, β)` per
  dual pair; xylarium's Cl(12,1), pending xylarium #118); a law on a
  certified interval acts on the caller's bound nearer zero and not where
  the interval straddles zero.
- **Names only: cohesion is the viscosity** (operator ruling 2026-11-06).
  Per plane the viscosity is the cohesion, the held share
  `A/(A + Q) = 1 − ν(t) = ν(1/t)`; the open share `ν(t) = Q/(A + Q)`
  (`open_share`) is the fluidity, since viscosity resists flow. Both are
  `1/2` at the throat, and the inversion exchanges them. 78.175, the
  `forms::coupling` documentation, its tests (`viscosity_inverts` is now
  `fluidity_inverts`) and the MU-EQUATIONS row are renamed. No maths,
  fixture value or encoding changes, and the law version stays 1. No
  public function reads the held share yet; none is added.
