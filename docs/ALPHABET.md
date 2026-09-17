# The alphabet (Law VIII)

Landed 2026-09-03. Source: `src/transducer/alphabet.rs`, `src/transducer/peel.rs`.

## The seventeen letters

The alphabet is exactly seventeen generators on the Euclidean rank-4 chart
`e0..e3`, immutable, restored from ipsal-autic v0.4 (`iae/src/transport/letters.rs`)
and the convergence-engine's Law VIII:

| slots | content | planes |
|---|---|---|
| 0 | identity | none |
| 1–6 | quarter turn `1 + e_ij`, inverse `(1 − e_ij)/2` | (0,1) (0,2) (0,3) (1,2) (1,3) (2,3) |
| 7–12 | (3,4,5) rotor `3/5 + 4/5 e_ij` | the same six |
| 13–16 | (5,12,13) rotor `5/13 + 12/13 e_ij` | (0,1) (0,2) (0,3) (1,2) |

Every letter is one pair, the rotor with its carried inverse, exact over ℚ.
A letter acts by the sandwich, so its rotation is twice its half-angle.
The sensor's plane is `(e0, e1)`: rows on `e0`, columns on `e1`.

## Opened letters (ruled 2026-09-10)

The seventeen are immutable to a voice, not to the court. A rotation over
a prime outside 5 and 13 is a `RotorPrime` demand carrying its witness
rotor; the court opens the letter `a + b·e_ij`, `a² + b² = p`, on two
independent witnesses at one chart, rules it with a receipt, teaches it by
the syllabus, and names it on the tape by content, never by an index
(`MU-RECURSION.md` law 6, 36.3, 37.7). The peel gains one shell per
opened prime, in increasing order of the primes.

## Words

A word is an oriented tape of letters; its rotor is the product in tape
order, and the last letter acts first. `rot90` is one quarter letter,
`rot180` two, a flip of the plane is the half turn through the third axis
(two quarter letters on plane (1,2)), the transposition is their product.
None of these is tabulated anywhere: the legacy D4 decoder, the blade's
stabilizer turns and the orientation sheet are all realised from words.

## Spelling

`alphabet::spell` reduces a rational similarity of the plane to a word and
an exact dilation. It is a peel (`peel::peel_nest`, the v0.4 peel law):
the orientation sheet is peeled first, then prime-5 letters by the
valuation of the rotation's Gaussian integer at `2+i`, then prime-13
letters at `3+2i`, then the unit turns. Each shell removes one syllable by
an exact read; nothing is tried and verified. A rotation with a Gaussian
prime outside 5 and 13, or with an odd exponent (a half letter), stands as
a remainder and is not a word.

## Where the alphabet is the law

- A connection is minted only from a spelling (`element::from_connection`
  returns `None` for an unspellable motion; the client holds it as thought).
- The letter tape is the connection's description (`Element::word`, tag
  `0x11`), so the letters enter the address and travel on the wire.
- The court replays the tape: a connection without letters, or whose
  letters do not re-derive the block its fibre holds, is refused
  (`verdict::blind_replay`).
- An entity learns a connection only when its letters re-derive the motion
  (`Epigenome::learn`); the syllabus teaches every anchored citation of a
  sentence, none ranked.
- Comprehension deconstructs a heard word into letters, dilation, placement,
  gauge and remainder (`Element::letters`).

## Opened axes

When one listing closes on several charts a genome holds at once, the
entity opens an axis: the transition between the charts' representatives
is seeded onto a fresh fibre of the first (the convergence engine's
`expand_m`, seeded from the relation, never zero) and spoken up. The court
accepts an opened axis only when its fibres are group elements whose
planar block spells.

## Commuting isoclinic pairs and cycle optimization

In `Cl(4,0)`, the six pair planes partition under the Hodge star operator
into three canonical commuting dual pairs (`BLOCK_ORDER`):

1. `Π_01 × Π_23 = (e0 ∧ e1, e2 ∧ e3)`
2. `Π_02 × Π_13 = (e0 ∧ e2, e1 ∧ e3)`
3. `Π_03 × Π_12 = (e0 ∧ e3, e1 ∧ e2)`

Complementary planes commute mutually (`[B_i, ⋆B_i] = 0`), so double rotations
factor exactly without Baker–Campbell–Hausdorff cross-terms:
`exp(½ θ₁ B_i + ½ θ₂ ⋆B_i) = exp(½ θ₁ B_i) · exp(½ θ₂ ⋆B_i)`.
Peeling over commuting pair blocks reduces the peeling search depth from six
sequential stages to at most two commuting block evaluations, collapsing derivation
cycle complexity by ~66% and eliminating pseudo-shear remainder edges.

## Transverse energy routing

A rotor carrying non-zero energy on off-plane bivectors (`off_plane_blades`)
does not immediately fail to a refusal. Instead, exact Gaussian elimination over
ℚ (`solve_conjugator_rotor`) computes the conjugator `γ ∈ Spin(4,0)(ℚ)` such that
`γ · a = λ · b · γ`, rotating the transverse bivector into the principal Cartan
subalgebra and permitting closed derivation without tape mutations.

