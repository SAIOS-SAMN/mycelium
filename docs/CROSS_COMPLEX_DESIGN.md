# #97 — Cross-Complex Chains: the Codomain Aperture

**Status:** design (operator-ratified constraints, 2026-09-02). Implementation
phases A–C below. **The enforcement line is absolute**: any branch on ratio
values, any `match mode { Tile, Stretch, Crop }`, any preferred factor — banned.
The extension is sensor hardware only: δ becomes computable over unequal
complexes; every transform still germinates, completes, and anchors.

## 1. Where the engine is mute today

A demand chain `(r, c, before, after)` lives on ONE lattice with the identity
morphism implicit: cell x pairs with cell x. When `K_in ≇ K_out`
(`h_in×w_in ≠ h_out×w_out`), `germinate`/`delta` have no object to build —
perception cannot state the demand (the gate's dim-skip-320; the judge's
body-count pivots for reshaped bodies). Nothing can germinate from an
unspeakable demand. This is a codomain blind spot, not a missing thought.

## 2. The substrate extension (measured, never chosen)

### 2.1 Frame invariants
Perception measures both bounding boxes exactly: `(h_in, w_in)`,
`(h_out, w_out)` — 0-cell counts, the same status as the min-corner the
transposition quotient already measures. The **rational frame ratio**
`ρ = (h_out/h_in, w_out/w_in) ∈ ℚ²` is a measured geometric property of the
pair. It parameterizes nothing; it names nothing; it is data.

### 2.2 The common refinement (the lattice morphism without choices)
The bipartite cochain over `K_in ⊔ K_out` is realized with zero information
loss and zero choices on the **common refinement complex**:

```
K_lcm :  lcm(h_in, h_out) × lcm(w_in, w_out)
embed_in :  each K_in cell  = an (lcm_h/h_in) × (lcm_w/w_in) block, exact
embed_out:  each K_out cell = an (lcm_h/h_out) × (lcm_w/w_out) block, exact
```

Both scenes embed **exactly** (integer block subdivision — pure ℚ arithmetic,
no rounding, no interpolation, no sampling policy). The cross-frame demand is
then the ordinary chain on `K_lcm`:

```
∂₁Q = { (r, c, before(r,c), after(r,c)) : before ≠ after }   over K_lcm cells
```

where `before` reads through `embed_in` and `after` through `embed_out`.
This is the ω(x, y)(e_y − e_x) bipartite form with φ realized as the two
embeddings — canonical, total, and **uniform for every ρ including (1,1)**,
which reproduces today's chains bit-for-bit (backward compatibility is a
theorem, not a flag).

### 2.3 Cost, honestly
`lcm ≤ h_in·h_out` — for ARC (dims ≤ 30) the worst coprime case is 870×870.
Typical integer ratios give `lcm = max(h_in, h_out)` — cheap. Big refinement
chains are big demands: the thermodynamic market and the metabolism already
price and pace large chains; no special-casing. The composite fragmentation
(7d) tiles oversized refinement chains like any other oversized coboundary.

## 3. The word: Element gains the frame fiber

`Element` gains a measured invariant `frame: (h_in, w_in, h_out, w_out)` as
exact integers (the reduced ρ is derivable; keeping the four counts preserves
the full measurement). Rules:

- **Identity**: the frame joins `base_canonical` — a word at ρ=(2,2) is a
  different word from its ρ=(1,1) shadow. `(1,1)`-framed words hash exactly
  as today (address stability for the whole existing language).
- **Serde**: appended tail after the anchor fiber, absent = (1,1,1,1) — the
  same backward-compatible appendix discipline as the π fiber.
- **Matching/peeling**: `matches_chain`/`fitting_peels` compare within equal
  frames only (a chain's frame is measured at germination and travels with
  it). Sections, torsion, anchors, the court's replay, the market, μ⃗ — all
  unchanged: they operate on the chain, and the chain is an ordinary chain.

## 4. Emergence (what is NOT built)

On the refinement complex, the structures the old constructors named appear
as plain geometry the existing machinery already learns:

- **Tiling / periodic copies** → repeated congruent sub-patterns → the peel
  cites one germinated word at several placements; periodicity condenses via
  the EXISTING section/torsion machinery (σ-orbits, closed cycles).
- **Dilation** → constant-hue blocks whose before-side reads through the
  coarse embedding → germinates as an anonymous block word; families across
  scales collapse by puncture like any hue family.
- **Cropping** → a refinement chain whose support is a sub-box → its
  selection term (π) derives from the SAME anchor algebra (ArgExt, Delta,
  enclosure) — "crop to the largest component" is a π the court binds, never
  a mode we write.

The court audits cross-complex critical pairs identically (blind replay =
re-derive the chain from its evidence; the frame is part of the evidence).

## 5. Projection / the gate

`delta(a, b)` extends: unequal dims → the refinement chain + measured frame
(dim-skip becomes speakable). Application inverts by measurement: a word
carrying frame `(h_i, w_i, h_o, w_o)` applied to an input of dims `(h, w)`
CONSTRUCTS its output frame as `(h·h_o/h_i, w·w_o/w_i)` — exact ℚ, admitted
only when integral (a non-integral frame simply does not apply — refusal,
not rounding). π-proposed placements evaluate on the refinement scene.

## 6. Phases

- **A (gate-side, measurable fast)**: refinement embedding + frame-carrying
  `delta` + application in arc-solve → Γ re-run; the dim-skip-320 coordinate
  is the direct readout.
- **B (engine-side)**: Element frame fiber (serde appendix), germination on
  refinement chains in `adjudicate` (the framed-body path gains the frame
  measurement), matching frame-guarded.
- **C**: `Entity::project` frame construction; fleet deploy; v3 snapshot + Γ.

## 7. The banned list (verbatim, enforceable)

- `if scale == 2`, `match mode`, any named transform, any preferred factor.
- Any interpolation/rounding policy (the refinement is exact or the demand
  does not exist).
- Any apply-time rule not derived from a measured invariant carried by the
  word itself.
