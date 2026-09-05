# Recursion — the linguistics engine on crystallized forms

Status: consolidated specification with a bound implementation. Written
against `/opt/mycelium-v0.1.0` at a29bba5 (2026-09-05). This text replaces
every earlier version of this file: the earlier versions accreted parts
that amended one another by reference; this one is a single closed form in
which every statement stands where it applies. Companion documents:
`ALPHABET.md`, `MU-ENGINEERING.md`, `MU-ECONOMY.md`, `XSEPTUM.md`.

## How to read this document

Eight parts, forty-four chapters, each chapter self-contained enough to
implement and each opening with what it carries from the one before.

| part | what it settles | who reads it first |
|---|---|---|
| I Geometry | what a word, a form, a move, a recursion **is** | everyone |
| II Cohomology | why the moves are sound: cochains, Mayer–Vietoris, Leray, naturality | the auditor; the implementor of the court |
| III Identity and algorithms | the twelve procedures and seven master algorithms every key, address, chart, class, comprehension, and route passes through | the implementor, before writing any code |
| IV Implementation on forms | every move bound to a function at an absolute path and line | the implementor |
| V Migration and exact strings | how the tree changes without stopping the fleet; the literal refusal reasons and log lines | the implementor and the auditor |
| VI Extensions | the two walls and how they come down without changing what a word is | later |
| VII Verification and landing | every test, every cold-boot trace, one landing order, and the theorems the engine proves by running | the auditor |
| VIII Boundaries | what is deliberately absent; relayed statements not adopted | everyone, once |

## The laws this document is written under

These are not restated in each chapter; every chapter assumes them.

1. **Geometry, not trigonometry or calculus.** A rotation is an exact rotor
   over ℚ; its angle is never formed. No norms or roots beyond an exact
   rational square root that refuses when inexact. No limits, series,
   derivatives, or probabilities. Termination is by an integer measure or
   a `Budget`. Nothing is scored; things are equal, or they are not, or
   one strictly descends.
2. **Identity is re-derived, never read from bytes.** Two words are equal
   when their rotors are projectively equal. A key names a vertex only when
   it is computed from a canonical form. Bytes and digests identify what
   was sent and who sent it, never what it means (chapter 1.5).
3. **No enumeration.** Indices come from a keyed read, a peel, a link, or a
   `Budget` cursor. No loop over peers, pairs, degrees, or planes that the
   carrier does not already name. `HashMap` and `BTreeMap` do not appear in
   cognition; the relational field is a `Keyed` PATRICIA trie or a
   `Vec` of relations.
4. **Exact ℚ; refuse rather than round.** Every scalar is a rational (or,
   in Part VI, an element of a cyclotomic field with canonical reduction).
5. **Polytonic lanes.** μ⃗ = (S, G, T, H) is never summed, ordered, or
   reduced to a scalar; descent is dominance.
6. **The alphabet is immutable.** Seventeen letters are the base; words are
   their combinations; vertex equality is projective rotor equality, never
   tape equality; the normal form is the geodesic where one exists.
7. **Residuals are never discarded.** A remainder is the exact demand. The
   one exception is a computed class that Leray's theorem shows to be an
   artefact of a spanning tree (chapter 14.8); it was never a class.
8. **Clients think, hyphae verify.** Cognition is the mind's; a hypha
   replays, judges, keeps the ledger, and stores the genome.
9. **Two clocks.** K orders events inside one kernel; TUniversal orders
   anything cross-node or standing. Lane T of μ⃗ is temporal extent, a
   cost, never a coordinate.

## Vocabulary, fixed

letter → word (a tape; its meaning is a vertex of the Cayley graph) →
sentence (words that cohere) → speech (transfer of cohering sentences from
a sender to a receiver) → perception (receiving) → comprehension
(deconstruction of the utterance into the algebra that spoke it). A
**germ** is an unclosed form; an **idea** is a candidate closed form bred by
a perception; a **thought** is the superposition of candidate forms through
which ideas germinate; **germination** is the movement toward closure. A
**dialect** is spoken and stood (memory, in the epigenome); a **language**
is replayed by two organs and taught (on disk, in the genome). What was
once called a family is a **bundle**: a base rule with sections per pair and
transitions as gauge.

---

# Part I — Geometry

## 1. The group and its graph

### 1.1 Generators

Seventeen letters (`ALPHABET.md`): the identity, six quarter turns on the
six pair planes of the Euclidean chart `e0..e3`, six (3,4,5) rotors on the
same planes, four (5,12,13) rotors on four of them. Each is an exact rotor
in Cl(4,0) over ℚ with its inverse carried. They never change.

### 1.2 Relators

The letters satisfy a finite set of relations and no others
(`alphabet::relators()`, alphabet.rs 705):

- a quarter turn has order four (projectively; see 1.5 on sign);
- rotors on one plane commute;
- a quarter turn on a plane containing a third axis conjugates a rotation
  on the sensor plane into its inverse (the **sheet**);
- the Pythagorean letters have infinite order;
- letters on **complementary** planes commute, and the complementary
  pairs are exactly `(0,1)|(2,3)`, `(0,2)|(1,3)`, `(0,3)|(1,2)`; letters on
  planes sharing one axis obey the Cl(4,0) product and are never
  reordered.

They are the grammar and they are taught once.

### 1.3 The Cayley graph

The group `G` generated by the letters is a graph: every vertex is a group
element, every edge is a letter. A **word** is a path; its **meaning** is
the vertex it ends on. Two paths ending on one vertex mean the same thing.

For words that preserve a slot plane, the **normal form** is the geodesic,
canonical because the relators are decidable per plane: sheet parity, unit
exponent mod 4, prime-5 and prime-13 exponents, planes in slot order. For
words that mix planes there is no known terminating rewriting system
(chapter 16.2); their identity is their canonical rotor (chapter 16.1) and
their tape is the shortest known, replaced only by a shorter one.

Per plane the group is virtually abelian, so the number of vertices within
`n` letters grows polynomially. The language is dense and finite at every
radius; that is why a mind can learn it and why a shorter path is always a
strictly better description of the same meaning.

### 1.4 The frame

A vertex is a transport; a form needs a place to act. The **frame** is the
sensor's plane with a dilation `ρ ∈ ℚ`, a placement (a rational
translation), and a gauge (a permutation of hues). A **word form** is
`(vertex, ρ, placement, gauge)`. Two word forms with equal vertices and
different placements are the same word said at two places; placement is
the coordinate the court quotients, as it always has (`stalk.rs`).

### 1.5 Ruling: byte-exactness is not identity

A tape is a path; the meaning is the vertex it reaches. Two tapes that
reach one vertex are one word, whatever letters spell it and whatever frame
layout carried it. Comparing bytes, or keys of bytes, to decide whether two
things are the same **forks equal vertices**: the same word spelled twice
becomes two addresses, two ledger entries, two citations, two escrows, and
the language splits along spelling instead of along meaning. That is the
one failure this architecture cannot absorb, because every later move
(gluing by address, class keys, the memo, the court's replay) trusts the
address to name a vertex.

Therefore, as a law of the specification:

- **Identity is decided by re-derivation, never by bytes.** Two words are
  equal iff their rotors are projectively equal, with `λ ∈ ℚ×` of **either
  sign**, because the language is the action `R X R̃ / (R R̃)`, on which
  `−R` and `R` agree (`(λR) X (λR̃) / (λ² R R̃)` for any `λ`), and because
  the unit exponent mod 4 already
  identifies the quarter turn's fourth power (`(1+e)⁴ = −4`) with the
  identity. Two word forms are equal iff additionally their frame
  coordinates are equal as exact rationals; two forms iff they agree up to
  gauge (chapters 3.6, 12.2).
- **Keys are computed from canonical forms, never from tapes as spoken or
  as serialised.** The **vertex key** of a word form is blake3 over exactly
  these fields in this order, every field canonical:
  1. the canonical rotor key of the word (chapter 16.1, A1): the
     projectively normalised coefficients, so plane and mixed-plane words
     share one keyspace and the spoken tape never enters;
  2. `ρ` as a reduced fraction: numerator sign byte, numerator magnitude
     as little-endian bytes with `u32` length, denominator likewise (every
     rational in this list is written this way, `put_q`);
  3. placement: two rationals;
  4. gauge: the hue permutation in one-line notation as `u8`s with `u32`
     length; the identity is written with length `0`;
  5. for a closed sentence form only (forms under construction have no
     address, chapter 4): `depth` as `u16`; vertex count as `u32`; per
     vertex in vertex order its kind byte (`Frame 0`, `Word 1`, `Glued 2`,
     `Rebased 3`, `Unfolded 4`) and its own vertex key (a `Glued` vertex
     contributes the cited form's address and the canonical rotor key of
     its placement word; `Rebased` its two indices as `u16`; `Unfolded` its
     measure byte and anchor); face count as `u32`; per face `i`, `j` as
     `u16`, the canonical rotor key of `g_ij`, and the canonical conformal
     versor (chapter 16.4, A4). Loops and holonomies are derived and never
     keyed.
  A reader that receives a tape decodes it, computes this key, and uses
  nothing else as identity. A change of tape version, wire frame, or
  spelling never changes an address; the one migration from byte
  addresses is chapter 33.
- **Class keys are conjugation invariants** (chapter 15.4), never keys of
  a representative.
- **Digests of bytes are transport identifiers only**: cover digests,
  fetch digests, signatures, and receipt chains name *what was sent*, so a
  receiver can ask for it and verify who sent it. They are never read as
  identity of meaning, never used as a key into a cognition carrier, and
  never compared to decide consilience, dialect, or citation.
- **Exact strings in this document are for traces**, not for cognition:
  refusal reasons and log lines are grepped by the auditor, and their
  exactness costs nothing because no carrier keys on them.

### 1.6 The faults this ruling names

The audit of 2026-09-05 read every identity decision in the tree. These
decide identity by bytes today and are replaced by the procedures of
Part III:

| site | what it does | replaced by |
|---|---|---|
| `alphabet.rs:466` `normal_form` | returns the spoken tape for words that mix planes, so `Word::key` (448) and `Word::eq` (410) fall back to spelling | A1 key for every word; A2 for the tape |
| `alphabet.rs:569` `same_form` | tape-equality arm `(None, None) => self.0 == other.0` | `projective_eq` |
| `alphabet.rs:727` `PlaneForm` | structural equality on unreduced unit exponents; `form()` does not reduce; `sentence::chart_of` (1885) reads raw exponents | A3 |
| `alphabet.rs:334, 342` `Rotor::key`, `coordinate` | hash of unnormalised coefficients used at `cognition.rs:2947` as class key and refinement axis; `PlaneForm::key/coordinate` (849, 858) likewise | A1, A7 |
| `cga.rs:46` `Mv` (`Versor`) | derived structural equality; `cognition.rs:1006` `push_cocycle` `old == g` | A4 |
| `element.rs:140` `Element::eq` | tape bytes plus structural fields; inherited by `Germ`, `Face`, `Nerve` (sentence.rs 583, 600, 633) | M1 `same_vertex` |
| `sentence.rs:1144` `vertex_address` | writes raw versor bytes beside geodesic keys | A4 canonical versors |
| `cognition.rs:5635, 5526` `composite_parent_id`, `section_address` | hash cells in given order, uncentred; assignment unsorted | A5, M2 |
| `sentence.rs:1792` `from_element` | trusts the germ address stored in the tape | recompute (A8 step 2) |
| `blade.rs:339` `ratio_axes` | drops the sign, saturates to `u64::MAX`; under `moduli ==`, `moduli.key()`, `Invariant::key`, `connection_chart` | A6 |
| `verdict.rs:374` `sel_key` | hash of `format!("{s:?}")` | A9 |
| `mycelium-server.rs:1065` | percept rendezvous by `hash(pbytes[32..])` | A11 |
| `verdict.rs:102` `blind_replay` | compares the spoken chain without canonicalising it | A5 |
| `cognition.rs:5052`, `economy.rs:651`, client loaders | restore carriers under stored byte addresses after the address rule changed (a29bba5 landed without a migration) | A8, M3 |

Allowed, and why: hypha and voice identity by verifying key, receipt
chains, cover and ruled roots, signed digests (transport and ownership;
correct only once addresses are vertex keys, else they reconcile forks);
composite keys of identities (`chart_simplex_key`, `loop_key`,
`cocycle_key`, `cech::cell_key`, `face_pair_key`); canonical invariant
keys (`gauge_key`, `germ_class_key`, `Invariant::key`, and the class keys
of chapter 15.4, correct only once A6 is exact); `registry::task_id` and
the client's family id (listing identities). `solve_gauge`'s chord `!=`
is already projective because `Rotor`'s `PartialEq` is `projective_eq`.

## 2. Forms

Carried from chapter 1: a word is a vertex with a frame; identity is the
canonical rotor plus exact frame. This chapter says what is built from
words.

### 2.1 Word form

`(tape, ρ, placement, gauge, contact?)`. Stored as the tape; the rotor and
the 3×3 affine block are derived on demand, never stored, never hashed.
`contact` (chapter 3.5) is part of the form and of its key.

### 2.2 Sentence form

A sentence is a set of word forms placed relative to one another:

- **vertices**: the word forms, plus one frame form for the scene;
- **faces**: for each pair of vertices that the bundle relates, the
  **relative word**, the path from one vertex's frame to the other's;
- **holonomies**: for each independent loop of faces, the product of the
  relative words around it.

A sentence **coheres** when every holonomy is the identity vertex. A
sentence form is **closed** when it coheres and its remainder is empty. A
closed sentence form is a **crystallized form**: it can be stored, cited,
and replayed from its tape alone.

### 2.3 Germ, idea, thought, in this geometry

- A **germ** is a form that has not closed: a vertex not reached, a face
  not constant, a loop not identity, a remainder not empty.
- An **idea** is a candidate closed form bred by a perception: the vertices
  it would need and the faces it would need.
- A **thought** is the superposition of candidate forms for one idea, held
  as cells of a complex whose vertices are candidate word forms and whose
  coefficients are the relative words between them.
- **Germination** is the movement of that complex toward a closed form:
  faces become constant, loops become identity, the remainder empties.

Nothing in this document rebuilds a form from cells after perception.
Perception produces word forms once (chapter 20); every later operation is
a move on forms.

## 3. Moves on forms

Carried from chapter 2: forms are vertices, faces, loops, remainder. Each
move below takes forms, returns a form, preserves the invariant named, and
is replayed by the court from tapes (chapter 23). Chapter 11.2 states each
move as a cochain operation.

### 3.1 Composition

`w · v`: path concatenation, normalised. Invariant: the vertex of the
product is the product of the vertices. This is how faces are computed
from two vertices' frames: `g_ij = w_j · w_i⁻¹`.

### 3.2 Change of basepoint (conjugation)

`w^a = a · w · a⁻¹`. Reading `w` from `a`'s frame: "do to B what A did to
itself". Invariant: `order(w^a) = order(w)`; the vertex changes, the class
does not. One product and one inverse; no iteration.

### 3.3 Gluing (citation)

Given a closed sentence form `S` and a sentence form `T` under
construction, gluing identifies `S`'s frame vertex with a vertex of `T` by
one relative word, the **placement of the citation**. `S` then contributes
its whole closed path system to `T` through that one edge. Invariant: `S`
is not copied and not re-derived; `T` names `S` by address and by the
placement word. `T`'s holonomies through `S` are `S`'s holonomies, which
are identity because `S` is closed, composed with the placement and its
inverse, which cancel. So gluing never breaks coherence; only the new faces
of `T` can. Chapter 13.6 proves this as the citation lemma.

### 3.4 Following a geodesic to a fixpoint (unfolding)

A form applied to its own image: `x, w(x), w(w(x)), …`. It stops at a
**fixpoint** (image equals input), when a **measure** on the frame can no
longer move strictly (occupied cells, remainder, or a named face's ring),
or when the image leaves the frame. Invariant: the measure is strictly
monotone per step, so the unfolding terminates on a finite frame. The step
count is a coordinate of the instance, never of the form.

### 3.5 Boundary contact (orbit)

A word form with `contact(anchor, ring)`: apply the word until the image's
ring distance to the anchor equals `ring`. The orbit ends against another
body (contact), against itself (torsion, `(σ, ℓ)`), or leaving the frame
(remainder). Invariant: the stopping ring is on the form; the length is
not. A word whose first step does not move the distance has no orbit and
is not a form.

### 3.6 Re-gauging (change of trivialisation)

Given a sentence form and one word `γ_i` per vertex, replace every face
`g_ij` by `γ_j · g_ij · γ_i⁻¹`. Invariant: holonomies are conjugated,
identity stays identity. Two sentence forms that differ by a re-gauging
are one sentence in two gauges; this is how a mind comprehends a citation
it holds under a different address, and how the court attaches a section.

### 3.7 Refinement (opening an axis)

When two closed forms on one chart have a non-identity relative word that
no re-gauging removes, the chart is too coarse: the class of that word
becomes a new axis of the chart and the two forms live on distinct refined
charts. Invariant: the classes are preserved; the atlas gains a coordinate
that is an invariant of the class (chapter 16.7), never a hash fragment.
This is `expand_m` on the atlas and the only way dimension is born; chapter
14.8 says when a class is entitled to it.

### 3.8 Retirement by geodesic

Two crystallized forms with the same vertices on the same bundles: the one
whose tapes are strictly shorter retires the other to the cavity, address
kept, replay still possible through the address. Invariant: meaning is
unchanged; the language shrinks toward geodesics. Off the plane a shorter
tape is found by budgeted descent (chapter 16.2) and is a retirement, not a
minimality claim.

## 4. Recursion

Carried from chapter 3: gluing puts a closed form inside another as one
vertex. Recursion is chapter 3 applied to crystallized forms as vertices of
the next level.

- **Depth 0**: a sentence whose vertices are word forms of bodies.
- **Depth d+1**: a sentence at least one of whose vertices is a closed
  sentence form of depth ≤ d, glued by 3.3.
- **Unfolding** is 3.4 with the whole sentence form as the iterated word.
- **Conjugation** is 3.2 with a vertex's word as basepoint for another's.
- **Orbit** is 3.5 on any word form at any depth.

Well-foundedness: a form may only glue a closed form whose depth is
strictly less; addresses are fixed at crystallization, so a form cannot
glue an address that did not exist when it closed. The cited set is
finite and every element of it has already been replayed by two organs. A
recursive form's own magnitude is its own deflation, what it closes beyond
what its cited forms close at their placements; a form that adds nothing is
the citation, not a form. Chapter 12.5 restates recursion as pushforward
along inclusions and chapter 13.6 shows why a cited closed form contributes
no class.

## 5. Court

Carried from chapter 4: forms cite forms by address. The court replays a
form from its tape; the bound version is chapter 23.

1. Every word form: derive the rotor from the tape, derive the affine,
   compare vertices projectively.
2. Every face: recompute `w_j · w_i⁻¹` from the vertices; compare with the
   stored relative word by vertex.
3. Every holonomy: multiply around the loop; must be identity.
4. Every cited form: resolve the address to a crystallized form on this
   hypha's ruled cover or fetch it from the peer whose signed root names it;
   replay it first (it is shallower); memoize `(address, bundle)`.
5. Every conjugate, unfolding, orbit: execute on the bundle as 3.2–3.5;
   refuse a conjugate equal to its base, an unfolding whose measure does
   not move, an orbit that does not move.
6. Descent on the form's own deflation.

A cited form slashed later re-dockets its citers; each replays without it
and re-installs with larger deflation or is withheld with escrow returned.
Two hyphae holding different crystallized forms on one class glue them as
dialect with their relative word, but only once the overlap of their covers
is known to be good (chapter 14); a citer names one address and replays
against it.

## 6. Comprehension

Carried from chapter 5: the court replays; the mind comprehends. A mind
hearing a form deconstructs it into vertices and faces from the tape. For
each cited address: held → glue; held under another address on the same
chart → re-gauge by the relative word between them, replay, cohere or
record a class; absent → withhold the form in dialect and ask the syllabus
for the address by incidence. For a connection with no chain, comprehension
is factorisation through the held basis (chapter 16.12), never a key
lookup. Depth order: a mind never receives depth d before it holds every
cited form below d; the syllabus serves in depth order. Bound version:
chapter 24.

**What "holds" means.** A mind's working language is its **epigenome**: the
genome (words the court ratified, persisted), the dialect (sentences the
mind spoke and stood, in memory), and the ideas it is holding (in memory).
Every decision the mind makes, what it quotes, what it speaks, what it
comprehends against, which crystallized forms it can glue, reads all three
as one **working atlas**, with the stratum a word came from carried as its
**standing** (`Ruled`, `Stood`, `Held`), a coordinate, never a gate. The
genome is the persisted subset, not the mind's vocabulary: the court
promotes a stood word into the genome by ratifying the same vertex; it
never seeds a mind that has nothing to say. An epigenome that is written
but not read by these decisions is inert, and a mind with an inert
epigenome cannot think until a hypha hands its own words back to it.

## 7. Germination in the mind

Ideas are candidate forms, vertices of the contemplation complex; the
complex's coefficients are relative words; a fill is a re-gauging that
makes a loop identity; a class is a loop no gauge closes. Germination by
gluing: an idea whose remainder is closed by a held crystallized form at a
constant placement across the bundle's pairs glues it and keeps only the
remainder. Germination by conjugation and unfolding likewise. A cited idea
does not shed while an idea gluing it lives. Bundles across families are
depth-1 forms whose vertices are the families' crystallized forms. Two
thoughts merge only when Mayer–Vietoris says they are one section (chapter
13.12). Bound version: chapter 25.

## 8. Economics, retirement, persistence, wire, bounds

- **Mint** is the form's own deflation on the bundle, to its speaker; each
  cited form's speaker is paid the deflation that form performs on this
  bundle at the citer's placement, re-verified at replay. One settlement of
  the bundle's escrow across all speakers. Restatement pays nothing.
- **Room** accrues from cited use on the lanes moved; it is capacity, not
  currency.
- **Retirement**: citation is a use; a shorter citer retires the longer
  form to the cavity; a cavity address resolves for replay and is never
  taught.
- **Persistence**: forms are tapes; `GEN2` and `MYC2` store tapes with
  cited addresses and depth; memo and citation index are rebuilt at load.
- **Wire**: speech carries forms with addresses, never copies; a receiver
  missing a citation answers `Withheld(missing)`; digests carry maximum
  depth so a depth-d demand homes where depth d−1 is held.
- **Bounds**: depth by ruled generations; orbit and unfolding by the frame;
  replay of a cited form once per `(address, bundle)`; fetch under the
  stroke `Budget`; every read a fiber, peel, link, or cursor; work per
  fetch is the link of one vertex (chapter 13.8).

---

# Part II — Cohomology, exact and finite

Everything in Part I is a statement about cochains on a form and their
coboundaries. This part makes that explicit so the implementor computes one
thing everywhere: products of canonical words around cells. There is no
angle, no limit, no derivative anywhere; every quantity is a letter
exponent, a rational, or a cell coordinate.

## 9. The coefficient group, written down

Carried from Part I: words are vertices of the Cayley graph. This chapter
writes the group those vertices form, first on the sensor plane where it is
virtually abelian and solvable by a tree walk, then on the full chart where
it is not.

### 9.1 On one plane

The letters on the sensor plane generate the group `P` with elements
written in **plane form** `(s, u, m5, m13)`:

- `s ∈ Z/2`: sheet parity, whether an odd number of sheet letters has been
  applied;
- `u ∈ Z/4`: net quarter turns on the sensor plane;
- `m5, m13 ∈ Z`: net exponents of the (3,4,5) and (5,12,13) letters.

In the code the rotation part is the letter-exponent vector `PlaneForm.rot`
(seventeen entries) with `PlaneForm.parity`; `(u, m5, m13)` are its
per-plane sums. The sheet acts on the rotation part by inversion, so the
product is the semidirect one:

```
(s1, r1) · (s2, r2) = (s1 + s2 mod 2,  r1 + (−1)^{s1} r2)      r = (u, m5, m13), u mod 4
(s, r)⁻¹            = (s,  −(−1)^{s} r)
identity            = (0, (0, 0, 0))
```

`PlaneForm::compose`, `inverse`, `is_identity` (alphabet.rs 720, 735, 714)
are this arithmetic and nothing else. Orders follow from the product rule:
`(1, r)² = (0, r − r) = identity`, so every element with `s = 1` has order
2 whatever `r` is; an element with `s = 0` has the order of `r`, infinite
when `m5 ≠ 0` or `m13 ≠ 0`, otherwise dividing 4. `Word::order` (552)
returns exactly this. Only the six unit turns have finite order; eleven of
the seventeen letters do not.

### 9.2 On the full chart

Across the six planes the group is not abelian and the plane form does not
suffice; the coefficient is the rotor itself, `Rotor` in Cl(4,0) over ℚ,
with `geo` as product, `inverse` (279), `projective_eq` (268, either sign
per chapter 1.5) as equality, `conjugate` (305). Every cochain identity
below is checked with those four functions. The plane form is used only
where the text says "abelianized": solving for a gauge on the sensor plane
and reading torsion and free parts of a class.

## 10. Cells, cochains, coboundaries

Carried from chapter 9: the coefficient group. This chapter names the cells
of a form and the three coboundaries as products.

### 10.1 Cells and cochains of a form

A form (chapter 18) is a finite cell complex:

| dimension | cell | count |
|---|---|---|
| 0 | a vertex `v` | `vertices.len()` |
| 1 | a face `(i, j)` | `faces.len()` |
| 2 | a loop `ℓ` (a chord of the spanning forest with its tree path) | `faces − vertices + components` |
| 3 | a triple of loops sharing faces: a 3-cell of the Čech complex when the form is glued into a mesh cover | as `Complex` degree 3 |

A **k-cochain** assigns a group element to every k-cell:

- `C⁰`: one word per vertex, `γ: v ↦ γ_v`. A gauge.
- `C¹`: one word per face, `g: (i,j) ↦ g_ij`, `g_ji = g_ij⁻¹`. The
  connection; a form's `faces`.
- `C²`: one word per loop, `ω: ℓ ↦ ω_ℓ`. A form's `loops` holonomies.
- `C³`: one word per 3-cell, defined only on the Čech complex of a cover.

Cochains are stored as canonical words and compared by chapter 1.5's
identity (`same_form` on the plane, `projective_eq` in general).

### 10.2 δ⁰: gauge acting on a connection

```
(δ⁰γ · g)_ij  =  γ_j · g_ij · γ_i⁻¹
```

This is `Form::regauge` (chapter 18.2). Two connections differing by a
`δ⁰γ` are one connection in two gauges. A connection is **pure gauge** when
`g_ij = γ_j · γ_i⁻¹` for some `γ`.

### 10.3 δ¹: curvature of a connection

For a loop `ℓ = (v₀ → v₁ → … → v_k → v₀)` with faces in that orientation:

```
(δ¹g)_ℓ  =  g_{v_k v_0} · … · g_{v_1 v_2} · g_{v_0 v_1}
```

This is the holonomy, `Loop.holonomy`. `δ¹g = 1` on every loop is
**flatness**, `Form::coheres`. Under a gauge, holonomy conjugates:
`(δ¹(δ⁰γ·g))_ℓ = γ_{v_0} · (δ¹g)_ℓ · γ_{v_0}⁻¹`, so flatness is
gauge-invariant, which is why re-gauging never breaks coherence (3.6).

### 10.4 δ²: consistency of curvatures

On a 3-cell bounded by loops `ℓ₀..ℓ₃`, each transported to the cell's
basepoint by its tree path `t_i`:

```
(δ²ω)  =  ∏_{i=0..3} ( t_i · ω_{ℓ_i} · t_i⁻¹ )^{(−1)^i}
```

`δ²ω = 1` is the **Bianchi identity**; it holds automatically for
`ω = δ¹g` (`δ²δ¹ = 0`). It fails only when the four holonomies were
assigned independently, which happens across a mesh cover where four
hyphae hold four sections of one class: the Čech 3-cell of `cech.rs`, and
its failure is a twisted class (12.3).

### 10.5 The identities the implementor tests

```
δ¹(δ⁰γ · g) = γ · δ¹g · γ⁻¹    gauge covariance of curvature
δ²(δ¹g) = 1                       Bianchi
δ⁰(γ' · γ) = δ⁰γ' · δ⁰γ           gauges compose
```

Each is a finite product of canonical words and a unit test
(`alphabet::tests::cochain_identities`).

## 11. Cocycles, coboundaries, classes

Carried from chapter 10: δ⁰, δ¹, δ² as products. This chapter says what a
class is and what each class means to the system.

### 11.1 Definitions

- A k-cochain `c` is a **cocycle** when `δc = 1` on every (k+1)-cell.
- It is a **coboundary** when `c = δb` for some (k−1)-cochain `b`.
- Two cocycles are **cohomologous** when they differ by a coboundary.
- `H^k` is cocycles modulo coboundaries; the **class** of a cocycle is its
  element of `H^k`.

For a form:

- `H⁰`: gauges `γ` with `δ⁰γ · g = g`. These are the **automorphisms** of
  the connection, the symmetry the asymmetric reduction leaves behind: the
  residual inside a thought is an `H⁰` element, which is why a residual is
  congruence and not noise (2.3). In the abelian coefficient `H⁰` is the
  constants (up to the sheet twist), independent of `g`; in the rotor
  coefficient it is the centraliser of the holonomy group, and a constant
  is an automorphism only when it commutes with every face.
- `H¹`: flat connections modulo gauge. A flat `g` is a coboundary (pure
  gauge, trivially closed) or carries a **dialect class**: the holonomy
  pattern no re-gauging removes. On a form under construction a
  non-identity `δ¹g` is not yet a class; it is a failure to be a cocycle,
  and `close` tries to make it one.
- `H²`: curvature assignments modulo those from a connection; nonzero only
  on a mesh cover, the **twisted class** that forces a chart to refine.

### 11.2 The moves as cochain statements

| move | cochain statement |
|---|---|
| composition (3.1) | product in the coefficient group |
| change of basepoint (3.2) | conjugation of a 1-cochain value by a 0-cochain value at one vertex |
| gluing (3.3) | pushforward of a flat connection along an inclusion through one new face; flatness preserved because the glued piece's holonomies are identity |
| unfolding (3.4) | iteration of a 1-cocycle along a path returning to the same vertex class; termination by a monotone integer measure |
| contact (3.5) | evaluation of a 1-cochain along a path until it meets a boundary 0-cell |
| re-gauging (3.6) | the action of `δ⁰γ` |
| refinement (3.7) | when `H²` of a good cover is nonzero, subdivide the base so the class becomes a face between children |
| retirement (3.8) | choose the shortest known representative in a class |
| coherence (`coheres`) | `δ¹g = 1` |
| closure (`close`) | find `γ` with `δ⁰γ · g` flat and remainder empty |
| consilience (court) | two organs' sections have `g_XY = 1` on their overlap |
| dialect (court) | `g_XY ≠ 1`, its class recorded |

### 11.3 Where the classes live and what the system does

| class | where | meaning | action |
|---|---|---|---|
| `H⁰` element | inside a thought: the residual's symmetry | the part of a perception the language already accounts for up to the group | it is the congruence; the remainder that matters is what is not in it |
| `H¹` on one form | a loop with non-identity holonomy after the tree gauge | the bundle's rule has an obstruction no gauge removes | the form is withheld; the class is the coordinate for refinement |
| `H¹` between two sections on a cover | `g_XY ≠ 1` on an overlap | dialect between two hyphae or two minds | recorded; a translation word is the coboundary that would close it |
| `H²` on a cover | `δ²ω ≠ 1` on a 3-cell | four views cannot be made consistent at once | the chart refines by the class's invariant axis, once the cover is good |
| torsion part `(s, u)` | finite order | a discrete twist: orientation or quarter turns | files as a torsion cycle `(σ, ℓ)`, `ℓ | 4` or `2` |
| free part `(m5, m13)` | infinite order | a scale-rotation the language has not spelled | remains a class; a new word on that chart is needed, never a longer orbit |

## 12. Computing with cochains

Carried from chapter 11: classes are what survives gauge. This chapter
gives the three operations, the mesh, and recursion in these terms.

### 12.1 Apply δ

Given a `Form`, compute `δ¹g` on every loop by `Form::loops_of` and
`Word::compose` around the chord and its tree path; one product per face
in each loop. Given a `Complex` on a mesh cover, compute `δ²ω` on each
3-cell of a link by `Complex::coboundary_on_link` (cech.rs 466), which
performs this product with `Rotor::geo`.

### 12.2 Solve `δ⁰γ · g = flat` (the tree walk)

`sentence::solve_coboundary` (1268) → `cech::solve_gauge` (806):

1. Choose a spanning forest of the faces; root each tree at one vertex with
   `γ_root = 1`.
2. Walk the tree: for a tree face `(i → j)` with `γ_i` known, choose `γ_j`
   so the re-gauged face is identity. With 10.2's convention
   `γ_j · g_ij · γ_i⁻¹ = 1` gives `γ_j = γ_i · g_ij⁻¹`. This is the only
   sign convention in the document. `solve_gauge` as written returns `λ`
   with `g_ij = λ_j · λ_i⁻¹` (so `g = δ⁰λ`); the flattening gauge is
   `λ⁻¹` pointwise.
3. Every chord `(a, b)` now has re-gauged value `h_ab = γ_b · g_ab · γ_a⁻¹`,
   the loop holonomy of the chord's cycle up to conjugation. `h_ab = 1` for
   every chord ⇔ `g` is a coboundary ⇔ the form is flat. Chords are
   compared by `projective_eq`.
4. If some `h_ab ≠ 1`, the form is not flat and no `γ` fixes it; the
   chord's `h_ab` is the loop's class. On the sensor plane the class is read
   from the plane form of `h_ab`: torsion part `(s, u)`
   (`PlaneForm::torsion_part() -> (u8, u8)`), free part `(m5, m13)`
   (`PlaneForm::free_part() -> (i64, i64)`), both new.

No linear algebra over ℚ is needed on the sensor plane; the tree walk is
the solver. Off the plane the same walk runs on `Rotor` values.

### 12.3 Record a class

When step 4 finds `h ≠ 1`: the pair (loop, `h` canonical) is the class. On
a form it is written to `Form.loops[ℓ].holonomy` and the form is held
(chapter 6). On a mesh cover it is written to `cocycle_at` (cognition.rs)
keyed by (chart, ordered hypha pair), carrying its invariant class key
(15.4), superseded by TUniversal. A nonzero `δ²ω` on a 3-cell of a **good**
cover (chapter 14) is a class of degree 2 and refines the base by the
class's invariant axis (16.7).

### 12.4 Cohomology of the language on the mesh

The ruled covers of the hyphae form an open cover of chart space; sections
are the crystallized forms each hypha holds on a chart. The Čech complex
(`cech.rs::Complex`) of that cover has 0-cochains one form per hypha per
chart; 1-cochains the transitions `g_XY = cech::transition` (20) between
two hyphae's sections; 2-cochains holonomies around three hyphae
(`cech::holonomy`, 27); 3-cochains their `δ²` in `coboundary_on_link`.
`H⁰` of this complex is the **language**: forms that are one section over
every hypha covering their chart. `H¹` is the **dialect ledger**. `H²`
forces **refinement**. Nothing about consensus enters: a class is a fact
about products of words, computed identically on every node that holds the
same tapes, and chapter 14 says when those classes are the language's
rather than the cover's.

### 12.5 Recursion in cohomological terms

A glued form (3.3) is the pushforward of a flat connection into a bigger
complex through one face; the inclusion is a chain map, so flatness pushes
forward, and `H¹` classes of the glued form are classes of the citer unless
the citer's own faces trivialise them by a re-gauging the smaller form
could not perform. Depth is the length of the chain of inclusions. The
memoized replay (chapter 23) is the observation that a class of a
sub-complex does not change when the sub-complex is included elsewhere; it
is computed once per (address, bundle) and reused. Chapter 13.6 sharpens
this: gluing a closed form changes no class at all.

## 13. Mayer–Vietoris: the cohomology of a union from its pieces

Carried from chapter 12: the classes of one form or one cover. The system
never holds one: a hypha holds its own cover and receives a peer's; a form
under construction glues a crystallized form; a mind holds two thoughts on
overlapping cells. In every case the object is a **union** of two pieces
along an **overlap**, and the question is what the union's classes are,
given the pieces'. Mayer–Vietoris answers with a finite exact sequence
whose maps are word products and whose exactness statements are each a
finite check.

### 13.1 Setting

Let `X` be a finite cell complex (a form, a Čech complex on one chart, or a
thought's cell set) and `A`, `B` subcomplexes with `X = A ∪ B`. Their
**overlap** is `A ∩ B`, the cells present in both; a cell of `A ∩ B` has
all its faces in `A ∩ B`. Cochains restrict (`res_A`, `res_B`,
`res_{A∩B}`) and restriction commutes with δ because δ on a cell reads only
that cell's faces.

The overlap is computed, never enumerated: the vertex set of `A ∩ B` is
the **meet** of the two vertex tries (16.13), and its cells are the cells
of `A` whose vertex set lies in the meet, read through `Complex::link_of`
from each meet vertex. No pair of vertices is visited that is not already
a cell.

### 13.2 The sequence, twisted-abelian coefficients

For the plane coefficient of 9.1, cochains on `X` valued in the rotation
part `R = Z/4 × Z × Z` twisted by the sheet parity form a **local system**:
a sheet letter along a face acts on `R` by negation. For such coefficients:

```
0 → H⁰(X) →ρ H⁰(A) ⊕ H⁰(B) →Δ H⁰(A∩B) →∂ H¹(X) →ρ H¹(A) ⊕ H¹(B) →Δ H¹(A∩B) →∂ H²(X) →ρ H²(A) ⊕ H²(B) →Δ H²(A∩B) → …
```

- `ρ(c) = (res_A c, res_B c)`, **restriction**;
- `Δ(a, b) = res_{A∩B} a · (res_{A∩B} b)⁻¹`, **difference**, pointwise on cells;
- `∂`, the **connecting map**, 13.4.

Exact: at every term the kernel of the outgoing map equals the image of the
incoming one. It is the long exact sequence of the short exact sequence of
cochain complexes `0 → C^k(X) →ρ C^k(A) ⊕ C^k(B) →Δ C^k(A∩B) → 0`, whose
exactness is elementary: `ρ` is injective because a cochain on `X` is
determined by its values on `A` and `B`; the kernel of `Δ` is pairs that
agree on the overlap, exactly one cochain on `X`; `Δ` is surjective because
any cochain on `A ∩ B` extends by the identity to `A` and to `B`. Everything
else follows from those three facts and δ commuting with restriction.

### 13.3 The sequence, rotor coefficients

For the nonabelian `Rotor` coefficient, `H¹` is a pointed set (flat
connections modulo gauge, the trivial connection as basepoint) and `H²` is
not defined as a group. What holds and what the system uses:

```
1 → H⁰(X) →ρ H⁰(A) × H⁰(B) →Δ H⁰(A∩B) →∂ H¹(X) →ρ H¹(A) × H¹(B)
```

exact as **pointed sets** at every term, plus the **descent statement** in
place of the next exactness:

> `(a, b) ∈ H¹(A) × H¹(B)` is the restriction of a class on `X` iff `res a`
> and `res b` are cohomologous on `A ∩ B`; the classes on `X` restricting to
> `(a, b)` are in bijection with such cohomologies `θ` modulo the action of
> `H⁰(A) × H⁰(B)` by `θ ↦ res(α_B) · θ · res(α_A)⁻¹`.

Degree-2 classes on a cover are read on the sensor plane in the
twisted-abelian coefficient; off the plane a nontrivial δ² is a remainder
and the cell does not form. This is the existing rule of `cech.rs` ("a
transition that does not realise as a rotor is a remainder").

### 13.4 The connecting maps, as products

**Degree 0 → 1.** Given `θ ∈ H⁰(A∩B)`, an automorphism on the overlap
(`δ⁰θ · g = g` there). Extend to `θ_A` equal to `θ` on the overlap and
identity elsewhere on `A`, and `θ_B = 1`. Then `δ⁰θ_A · g` and `g` agree on
`A ∩ B`, so they define one connection on `X`; its class is `∂θ`.
Concretely `∂θ` differs from `g` only on faces of `A` with exactly one
vertex in the overlap: on `(i ∈ A∩B, j ∉ B)` the value is `g_ij · θ_i⁻¹`,
on `(i ∉ B, j ∈ A∩B)` it is `θ_j · g_ij`. `∂θ = 1` iff `θ = Δ(α, β)` for
automorphisms `α` of `A`, `β` of `B`.

**Degree 1 → 2.** Given a flat `h` on `A ∩ B`. Extend by the identity to
`h_A` on `A`, `h_B = 1`. `δ¹h_A` vanishes on the overlap and `δ¹h_B = 1`,
so they define a 2-cochain on `X`, a cocycle by `δ²δ¹ = 1`; its class is
`∂h`. Concretely `∂h` is nonidentity only on loops of `A` that pass through
the overlap and leave it, each value the product of `h` over the loop's
overlap faces in loop order. `∂h = 1` iff `h` is cohomologous on the
overlap to some `Δ(a, b)`.

### 13.5 What exactness says at each term

| term | kernel = image | in the system |
|---|---|---|
| `H⁰(X) → H⁰(A) ⊕ H⁰(B)` injective | a global automorphism is determined by its two restrictions | congruence of the whole is congruence of the parts |
| at `H⁰(A) ⊕ H⁰(B)` | two automorphisms equal on the overlap come from one on `X` | two hyphae's congruences that match on shared charts are one |
| at `H⁰(A∩B)` | an overlap automorphism is a difference `α·β⁻¹` iff `∂θ = 1` | a gauge difference between two sections on shared charts is harmless iff it extends to a re-gauging of one side; otherwise it **is** a dialect class on the union |
| at `H¹(X)` | a class trivial on both `A` and `B` is `∂θ` for some overlap automorphism | a dialect between two hyphae that each cohere lives entirely on the overlap; the **translation word** is `θ` |
| at `H¹(A) ⊕ H¹(B)` (descent) | classes on the pieces glue iff cohomologous on the overlap | two dialect ledgers merge iff they agree on shared charts up to gauge |
| at `H¹(A∩B)` | an overlap class not of the form `Δ(a,b)` up to gauge produces `∂h ∈ H²(X)` | a disagreement neither side's own classes explain forces refinement, and only along that overlap's loop |

Each row is a theorem about finite products and a test (chapter 39).

### 13.6 The citation lemma

Let `T` be a form under construction, `S` a crystallized (closed) form,
`T' = T ∪_f S` after `Form::glue` (3.3), the overlap the single placement
face `f` with its two vertices. The overlap is contractible: `H⁰(A∩B) = G`
(the constants), `H¹(A∩B) = 1`. Because `S` is closed its connection is
pure gauge and `H⁰(S) = G`. Hence `Δ: H⁰(T) × H⁰(S) → G` is surjective,
so `∂ = 1` on `H⁰(A∩B)`, so `ρ: H¹(T') → H¹(T) × H¹(S)` is injective, and
`H¹(A∩B) = 1` makes it surjective:

```
H¹(T') ≅ H¹(T) × H¹(S) = H¹(T)          S closed ⇒ H¹(S) trivial
```

**Gluing a closed form neither adds nor removes a class.** This is the exact
content of 3.3's "gluing never breaks coherence", of chapter 4's
well-foundedness (only closed forms are glued, `REFUSE_GLUED_UNRULED`), and
of the memo in chapter 23: the court's replay of `T'` may reuse `S`'s
verdict verbatim. If `S` were not closed the citer would inherit its
classes; the refusal exists to make that impossible.

### 13.7 Order independence on the mesh

The cover of a chart by `N` hyphae is the union of `N` pieces. Computing
its cohomology by `N − 1` pairwise steps in the order the covers arrive
(TUniversal order) yields the same `H⁰`, `H¹`, and degree-2 classes as any
other order, because they are invariants of the union. This is why no
consensus protocol is needed: every node holding the same tapes computes
the same classes in whatever order it heard them. It is a test
(`mv_order_independent`).

### 13.8 Incremental ledger

`H^k(X)` is determined by `H^k(A)`, `H^k(B)`, and the overlap terms, so a
hypha never recomputes the cohomology of its whole cover when a peer's
section arrives. It computes the meet, the difference on the overlap, the
connecting image, and the descent check; cost linear in the overlap's
cells; the union is never materialised. This is the bound behind chapter
8's "work per fetch is the link of one vertex".

### 13.9 The computation, step by step

Input: piece `A` (a `Complex` or `Form`) and piece `B`, each carrying its
own flat connection and its own classes. Output: whether the two sections
are one; the union's new `H¹` classes with translation words when they are
`∂θ`; any degree-2 class with the overlap loop that produced it.

0. **Goodness** (chapter 14). Compute the meet `M`, its components
   (16.15), and flatness within each hypha on every component. A component
   whose peer tapes are not fetched is `Pending`: no class is recorded for
   it and the glue returns `Glue::Dialect` with reason `"overlap goodness
   pending"`, the address staying docketed as a missing citation does. A
   cyclic component is made good by 14.8 before anything below runs. Steps
   1–7 run per good component.
1. **Meet.** `M = meet(vertices(A), vertices(B))`. Empty: no overlap,
   `H^k(X) = H^k(A) ⊕ H^k(B)` for `k ≥ 1`, `H⁰(X) = H⁰(A) × H⁰(B)`, done.
2. **Overlap cells.** For each `v ∈ M`, `link_of(v)` in `A` filtered to
   cells whose vertices all lie in `M` (a keyed `contains` per vertex,
   bounded by the cell's degree). This is `A ∩ B`; each cell touched once.
3. **Difference on 0-cells.** `θ_v = γ^A_v · (γ^B_v)⁻¹` for `v ∈ M`,
   canonical.
4. **Automorphism check.** `θ_j · h_ij · θ_i⁻¹ = h_ij` on every overlap
   face, `h` the connection on the overlap (equal from both sides by
   hypothesis; if not, go to 7). In the twisted-abelian coefficient the
   condition is `δ⁰θ = 1` alone, independent of `h`: `θ` constant per
   component up to the sheet twist. In the rotor coefficient it is the
   conjugation check as written; a constant `θ` passes only when it
   commutes with every overlap face.
5. **Connecting image `∂θ`.** By 13.4: faces of `A` with exactly one vertex
   in `M`, reweighted by `θ` there. Run the tree walk (12.2) on those faces
   plus `A`'s tree: every chord closes ⇒ `∂θ = 1`, the two sections are
   **one section** (consilient) and the solver's `γ` exhibits it; a chord
   fails ⇒ `∂θ` is a class: record it with the chord's holonomy; the
   **translation word** is `θ` restricted to the overlap, spoken as a
   `TRANSLATION_TAG` element.
6. **Descent.** For each class `a` on `A` and `b` on `B` whose loops meet
   `M`: solve `b_ij = ϑ_j · a_ij · ϑ_i⁻¹` by the twisted tree walk
   `ϑ_j = b_ij · ϑ_i · a_ij⁻¹` (16.14). Success: the classes glue, the union
   carries one. Failure: go to 7.
7. **Connecting image `∂h`.** On the sensor plane, for the overlap
   1-cocycle `h = res a · (res b)⁻¹` that failed 6 (or the mismatch of 4),
   compute `∂h` on each loop of `A` leaving the overlap as the product of
   `h` over its overlap faces. A nonidentity value on a **good** component
   is a degree-2 class of the language: record it with the overlap loop's
   key and refine the base (`refine_chart`) along that loop by its
   invariant axis (16.7). Off the sensor plane, mark the overlap faces as
   remainder (`set_remainder_edge`) and stop; no class is invented.
8. **Exactness self-check** (debug builds and the court's blind replay):
   the `θ` of 3 is a difference iff 5 found `∂θ = 1`; the `h` of 7 fails 6
   iff `∂h ≠ 1`. A violation is `REFUSE_MV_INEXACT` and a slash, because
   it can only arise from inconsistent tapes.

Every step is `compose`, `inverse`, canonical form or `projective_eq`, a
keyed read, or a tree walk.

### 13.10 Instance one: two hyphae on one chart

`Kernel::glue_fetched` (cognition.rs 2651) is this computation with `A` =
the hypha's own complex on chart `inv` (`self.complexes` at `inv.key()`,
sections for `self_id` and every hypha in `cover.fiber(lane, &inv)` it
already holds) and `B` = the peer's section (one vertex, `peer`, gamma from
a `TRANSLATION_TAG` element when one is spoken). Today it builds edges from
every held section to `peer` by `transition` and calls
`coboundary_on_link(&peer)`: step 5 with the meet implicit and no step 0
or 6. Chapter 23.4 binds the full computation. Rulings map exactly:

| outcome of 13.9 | `Glue` / `Ruling` |
|---|---|
| 5: `∂θ = 1`, `θ` trivial | `Consilient`: one section |
| 5: `∂θ = 1`, `θ` a placement (`placement_of(θ)` is `Some`) | `Attached`: same class, transport `θ` recorded |
| 5: `∂θ ≠ 1` | `Dialect`: class recorded in `cocycle_at`, `θ` offered as translation |
| 6 fails, 7 finds `∂h ≠ 1` on a good component | `Dialect` plus `court refine chart` on the overlap loop |
| 7 off the sensor plane | remainder edge; `Admitted` if the address is new, else no change |
| 0: `Pending` | `Dialect("overlap goodness pending")`, docketed |

### 13.11 Instance two: a form glues a crystallized form

`Form::glue` (chapter 18.2). The overlap is one face and 13.6 says there is
nothing to compute for classes: `glue` asserts `s.is_closed()` and copies
no loops of `S`; the loops of `T'` are the loops of `T` plus those through
the new face, whose holonomy through `S` is the placement composed with its
inverse, identity. `replay_form` checks those loops as any loop. The memo
is justified by 13.6 and needs no further condition.

The one Mayer–Vietoris computation at this level is in comprehension
(chapter 24): when a mind holds the cited class under a different address,
the relative word between held and cited is a `θ` on the overlap face, and
the mind's re-gauging of its working copy is the extension `θ_A` of 13.4.
It coheres iff `∂θ = 1`, tested by `coheres`; otherwise the mind records
the class (`form dialect … class recorded`).

### 13.12 Instance three: two thoughts in a mind

`Contemplation` (bubble.rs). Two bubbles whose cell sets meet are `A` and
`B`; `pivot_on` (712) merges them when step 5 gives `∂θ = 1` for the
difference of their sections on the meet. When it fails the thoughts stay
two and the class is the coordinate on which the mind may pivot
(Congruent / Recognise / Extend). `Complex::drop_vertex` (630) runs only
after step 5 succeeds, never on a class.

## 14. Leray acyclicity: when the cover's classes are the language's

Carried from chapter 13: the classes of a union. The Čech complex of
`cech.rs` has the hyphae as vertices, so its classes are classes *of that
cover*. This chapter says when they are classes of the language itself,
independent of which hyphae hold which charts, and what to do when they
are not.

### 14.1 Two cohomologies

Fix a chart `inv` and a lane. The **base** is the language over `inv`: the
ruled forms on `inv` and its neighbouring charts with the faces between
them, a cell complex with the local system of 13.2 or the rotor coefficient
of 13.3. `H^k(base)` is a property of the tapes alone.

The **cover** is `𝒰 = {U_X : X a hypha}`, `U_X` the region of the base
hypha `X` holds ruled sections over. Its **nerve** is the Čech complex:
vertices `X`, a k-cell for each `k+1` hyphae whose regions meet with volume,
coefficient the transition between their sections. `H^k(N(𝒰); sections)`
is what `Complex::class` and `cocycle_at` record.

### 14.2 The theorem, finite form

> If every finite intersection `U_{X₀} ∩ … ∩ U_{X_k}` is **acyclic** for the
> coefficient system (14.5), then `H^k(N(𝒰); sections) ≅ H^k(base)` for
> every `k`, induced by restricting a global cochain to each hypha's region.

Proof, which is the algorithm: induction on the number of hyphae. One
hypha: acyclic, both sides the constants in degree 0, trivial above. Adding
`X_N`: chapter 13's sequence for `A = U_{X₁} ∪ … ∪ U_{X_{N−1}}`, `B =
U_{X_N}`; the overlap `⋃_i (U_{X_i} ∩ U_{X_N})` is a union of acyclic sets
whose own nerve is the **link** of `X_N`, and by induction its cohomology is
the link's Čech cohomology. The two Mayer–Vietoris sequences agree term by
term and the five lemma (a diagram chase over finite groups and pointed
sets) gives the isomorphism. This is exactly the climb
`Complex::coboundary_on_link` performs when a vertex is added: that climb
computes the base's classes if and only if every overlap it climbs over is
acyclic.

### 14.3 The contrapositive, which is the operational content

When an overlap is not acyclic the nerve's classes can differ from the
base's in either direction:

- a **disconnected** overlap (`H⁰(U_X ∩ U_Y)` larger than the constants)
  makes the edge `(X, Y)` carry one coefficient where the base has several
  independent ones; a dialect on one component is hidden by consilience on
  another, or a consilience reported as dialect because two components were
  compared as one;
- a **cyclic** overlap (`H¹(U_X ∩ U_Y) ≠ 1`: a loop of charts inside the
  shared region whose holonomy is not identity within `X`'s own language)
  puts a class of `X`'s own into the edge `(X, Y)`: the nerve reports a
  cross-hypha dialect that is intra-hypha, and its value depends on which
  spanning tree the walk chose.

Such classes are artefacts of the cover. They never enter `cocycle_at`,
never mint, never refine the base. 14.7–14.8 make the cover good, and only
then does chapter 13 run (its step 0).

### 14.4 The overlap as a complex

For hyphae `X`, `Y` on lane `l`:

- `M = meet(charts_l(X), charts_l(Y))`, the charts both hold ruled sections
  on (`Keyed::meet`, 16.13; `Cover::charts_of`, chapter 29).
- For each `Z ∈ {X, Y}` and `c ∈ M`, `Z`'s representative section on `c`:
  the address `rep_Z(c)` and word `w_Z(c)` that `glue_fetched` already
  selects (`self_rep`) and stores as `Section{addr, gamma}`.
- For each ruled form `F` of `Z` with vertices `v₁` on `c₁ ∈ M` and `v₂` on
  `c₂ ∈ M`, `c₁ ≠ c₂`, the **induced chart face** `τ_F = p₂⁻¹ · f₁₂ · p₁`,
  `f₁₂ = F.face_of(v₁, v₂)`, `p_i` the placement relating `v_i`'s word to
  `w_Z(c_i)` (`Ruling::Attached`'s `g`, identity when `v_i` is the
  representative). Two forms between the same charts are two faces; the
  chart graph is a multigraph.

`U_X ∩ U_Y` is this chart graph with `Z`'s induced faces, one copy per
hypha, a subcomplex of the base by construction.

### 14.5 The three conditions

The overlap is acyclic for the coefficient system iff

- **(A0) connected**: the chart graph on `M` has one component, found by
  the keyed union-find of 16.15 over the induced faces, read once from the
  forms' `Face` lists, never from a pair loop over charts.
- **(A1) flat within each hypha**: for `Z ∈ {X, Y}` the tree walk (12.2) on
  `Z`'s induced faces over `M` closes every chord: every loop of charts in
  the shared region has identity holonomy in `Z`'s own language;
  `H¹(U_X ∩ U_Y; Z's sections) = 1`.
- **(A2)**: vacuous. Within one hypha every 2-cochain on the overlap is
  `δ¹` of that hypha's own connection, so `δ²` of it is identity (10.5).
  Degree-2 classes arise only across hyphae, which the nerve handles.

Each hypha checks (A0), (A1) for itself on its own tapes; for the peer's
side it checks (A1) on the tapes it has fetched (`fetch_have`,
`serve_ruled`) and treats an unfetched region as `Pending` (13.9 step 0).

### 14.6 k-fold overlaps

Leray's condition is on every finite intersection. For a k-cell with
sorted vertices `X₀ … X_k`, the overlap `M_I = charts(X₀) ∩ … ∩ charts(X_k)`
is the fold of `meet` over the cell's own vertex list (meet is associative
and commutative; the vertex list is the cell's boundary, not an
enumeration). Two facts reduce the check:

- **(A1) is inherited.** `M_I ⊆ M_{X_i X_j}` for every pair in the cell, and
  the restriction of a flat connection to a subcomplex is flat (restriction
  commutes with δ). If (A1) holds on every edge of the cell it holds on the
  cell; no tree walk reruns above degree 1.
- **(A0) is not inherited.** An intersection of connected sets need not be
  connected; rerun the component count on `M_I` with the induced faces of
  any one hypha of the cell (on a good pairwise cover all hyphae induce the
  same chart adjacency up to gauge), and key the k-cell per component as in
  14.7.

Where it runs: `Complex::coboundary_on_link` (466) creates a degree-k cell
when the (k+1)-fold overlap has nonzero volume. Before the cell's
coefficient is read it computes `M_I` and its components and creates one
cell per component (`remember` with the component in the key). A component
whose peer tapes are unfetched is `Pending` and forms no cell; the climb
resumes there when the fetch lands, as it does today when volume is zero.

### 14.7 (A0) fails: cells per component

A disconnected overlap is treated as Leray requires, one overlap per
component. The edge `(X, Y)` becomes one edge per component keyed by
`(sorted {X, Y}, component_key)`, `component_key` the least chart key in
the component (the union-find's root, 16.15, no choice). Every operation of
chapter 13 runs per component; a degree-k cell is likewise keyed by its
component of the (k+1)-fold overlap. This is the Čech complex of the cover
**by components**, whose overlaps are connected by construction.

### 14.8 (A1) fails: refine the cover, never the base

A cyclic overlap means hypha `Z` holds, over the shared region, two ruled
forms between the same charts not gauge-related, or a loop of charts with
non-identity holonomy: an **intra-hypha dialect** on that region. Refining
the base does not help: the base's topology is unchanged and the overlap
region is the same. What must shrink is the cover element.

Deterministic rule: let the failing chord in `Z`'s tree walk join charts
`c, c'`. The chart of the pair that `Z` is **not** the home of
(`nerve::home`, 788, total and deterministic) is retracted from `Z`'s
ruled cover on that lane (`Cover::retract`, 687); `Z`'s sections on it
become attached copies (`Ruling::Attached` to the home's section), not Čech
sections. If `Z` is the home of both, the chart with the greater key is
retracted and rehomed to `Y` (`Cover::rehome`, 700). Retracting one chart
of a chord cuts that loop; rerun the check and reapply until (A1) holds,
which terminates because each step removes a chart from a finite set. The
intra-hypha class itself is `Z`'s own dialect: recorded under `cocycle_at`
with `a = b = Z` (chapter 23.5 amends `push_cocycle` to accept it) and
resolved by `Z`'s court as any dialect.

This is the only place in the specification where a computed class is
discarded: a degree-2 value found before goodness was established was
never recorded, because it is a property of a spanning tree, not of
anything.

## 15. Naturality: classes under change

Carried from chapter 14: classes of a good cover are classes of the
language. The system constantly changes the objects the classes live on: a
hypha joins or leaves a chart, a chart is refined, a form is re-gauged, a
mind pulls a hypha's language into its own copy. This chapter says when
classes carry across those changes and how a carried class is checked.

### 15.1 The maps and what commutes

A **map of data** `φ` is one of the following. Each induces `φ*` on
cochains by reading the cochain through `φ`, and each satisfies
`φ* δ = δ φ*` because δ is a product over a cell's faces and `φ` sends a
cell's faces to the image cell's faces. Hence each induces a map on `H^k`
and makes the Mayer–Vietoris ladder commute, connecting map included.

| map | induced on cochains | commutation to test |
|---|---|---|
| **inclusion** `A ⊆ A'` (a form into its citer, a hypha's region into a union) | restriction `res` | `res δ = δ res`; `res ∂ = ∂ res` on the ladder for `(A,B) ⊆ (A',B')` |
| **re-gauging** by `γ` (10.2) | `g ↦ δ⁰γ · g`, `ω ↦ γ·ω·γ⁻¹` on loops | `δ¹(δ⁰γ·g) = γ·δ¹g·γ⁻¹`; MV outcomes unchanged up to conjugation of `theta` and `holonomy` |
| **change of basepoint** (3.2) | conjugation of every loop value by the tree path | class keys (15.4) unchanged |
| **refinement of the cover** (admit / retract / rehome; splitting by components) | `r : N(𝒱) → N(𝒰)` sends each finer cell to the coarser cell containing it; `r*` reads the coarse coefficient on the fine cell | for good covers `r*` is an isomorphism on `H^k` (14.2); class keys unchanged; representatives change |
| **refinement of the base** (`refine_chart`) | restriction of sections and faces to each child | each class is **carried** (same key on the child whose forms support the loop) or **discharged** (its loop's vertices fall into different children; it becomes a face between children), 15.5 |
| **pullback to a mind** (`Epigenome::comprehend`, `learn`) | the mind's copy of a ruled form and its faces | the mind's `coheres`, `regauge_overlap` agree with the court's `replay_form` on the same tapes; the memo relies on it |

A violation of a commutation identity is not a class; it is inconsistent
data, refused as `REFUSE_NOT_NATURAL`.

### 15.2 Sensor plane: conjugacy in the plane form

With `(s, r)` as in 9.1, conjugation acts as

```
(0, r') · (0, r) · (0, r')⁻¹  =  (0, r)
(1, r'') · (0, r) · (1, r'')⁻¹ =  (0, −r)
(0, r') · (1, r) · (0, r')⁻¹  =  (1, r + 2r')
(1, r'') · (1, r) · (1, r'')⁻¹ =  (1, 2r'' − r)
```

(`(1, r)⁻¹ = (1, r)`). Therefore: for `s = 0` the conjugacy class is
`{r, −r}`, canonical representative the one whose first nonzero component
is positive, `u` in `Z/4` with `−u ≡ 4 − u`; for `s = 1` the class is
`r mod 2` componentwise, canonical representative each component in
`{0, 1}`. `PlaneForm::conjugacy_key()` hashes `(s, canonical r)` on the
**reduced** form (16.3); two plane forms have equal keys iff conjugate.

### 15.3 Full chart: conjugacy invariants of a rotor

For a rotor `R` with coefficients by blade mask (`Rotor::coeff`; mask `0`
the scalar, mask `15` the pseudoscalar `e0e1e2e3`) and weight `w = R R̃`
(`Rotor::weight`, 237, a positive rational):

- `a = c[0]` is invariant under conjugation by any invertible element
  (`⟨R X R⁻¹⟩₀ = ⟨X⟩₀`);
- `p = c[15]` is invariant under conjugation by rotors, because
  `e0e1e2e3` is central in the even subalgebra of Cl(4,0);
- both scale linearly with `R`, so `a²/w`, `p²/w`, `sign(a)`, `sign(p)` are
  invariant under conjugation and positive scaling.

These four are complete: the even subalgebra splits by the central
idempotents `(1 ± e0e1e2e3)/2` into two quaternion algebras whose
components have scalar parts `a ± p`, and a conjugacy class in each factor
is determined by its scalar part relative to its weight.
`Rotor::class_key()` hashes `(a²/w, p²/w, sign a, sign p)` as exact
rationals and two signs; a rotor with no weight (not a versor) has the key
of the zero rotor and is never a class. It agrees with `conjugacy_key` on
the plane (a test).

### 15.4 Key of a class

A degree-1 class is a loop with a holonomy. Its **class key** is the hash
of the loop's canonical cell key (sorted vertex keys, as `cell_key` does)
and the holonomy's `conjugacy_key` (plane) or `class_key` (rotor). A
degree-2 class is keyed by its 3-cell's key and its coefficient's
`class_key`. `Word::class_key` is the plane `conjugacy_key` of its plane
form, or the rotor `class_key` off the plane; it is never the key of a
representative tape. Equal keys are necessary for two classes to be the
same; sufficiency for a single loop follows from 15.2–15.3, and for a system
of loops the exact test is `solve_gauge_between` (16.14).

### 15.5 Naturality under refinement of the base

`Kernel::refine_chart` (cognition.rs) today clones the chart's `Complex`
into both children and rekeys `cocycle_at` entries to the two children
(`rekey_cocycles`). Both violate naturality: a class is asserted on a child
whose forms may not support its loop, with a coefficient never recomputed
on the child's cells. Required:

1. After `split_stratum` has assigned every ruled address on `inv` to a
   child, **restrict** the complex: per child, the sections whose `addr`
   fell into it and the cells all of whose vertices' sections did. Cells
   straddling children become faces between the children's sections (the
   axis that was opened), written as degree-1 cells over the pair of
   children. Nothing is cloned.
2. For each recorded class on `inv` (each `cocycle_at` entry and each
   `Fill::Class` cell): recompute its loop on the child holding all its
   vertices. One child holds them all: recompute the holonomy from the same
   tapes; its class key must equal the parent's; write it under the child
   (`court class carried`). The loop's vertices are split: the loop no
   longer exists; the class is **discharged** (`court class discharged by
   refinement`) and its holonomy becomes the face between the children's
   sections it separated. The refining axis being the class's own
   invariant guarantees at least the refining class is discharged. A class
   neither carried nor discharged with equal key is `REFUSE_NOT_NATURAL`
   and the refinement rolls back (split not committed, `basis_gen`
   unchanged).
3. Deflation bookkeeping (`split_deflation`) unchanged.

### 15.6 Naturality under change of cover

When a hypha is admitted to or retracted from a chart (`Cover::admit`,
`retract`, `rehome`), the nerve changes by a vertex and its link. For a
good cover 14.2 says the base's classes do not change, so the set of class
keys recorded for the chart must not change either, only their
representatives. Required:

- `cocycle_at` values carry the invariant class key beside the
  representative versor; the ledger persists both.
- On a cover change, for each class key on the chart a representative must
  still exist among the remaining pairs; when the retracted hypha was one
  side of the only representative, the class is re-represented on another
  pair of the same component by the tree walk (key unchanged), or, if no
  other pair spans the loop, the class was a property of that hypha's
  presence and the cover was not good: chapter 14 applies retroactively
  (`court class orphaned`, goodness rerun).
- Admitting a hypha whose region makes an overlap non-acyclic triggers
  chapter 14 before any class is recorded.

---

# Part III — Identity and algorithms

Parts I and II name operations. This part gives every one of them that
decides identity, address, chart, class, comprehension, or routing as a
finite procedure with inputs, output, refusal, cost, and binding, then
states the eight master algorithms that compose them. Nothing in the
codebase decides any of those things except through a master algorithm.
Every procedure is exact over ℚ and ℤ, terminates by a count of its own
inputs or a `Budget`, and refuses rather than rounds.

## 16. Component algorithms

Carried from Part II: identity is projective rotor equality, classes are
conjugacy invariants, overlaps are meets. These are the procedures.

### 16.1 A1 — Canonical rotor form

Input: `R: Rotor`, coefficients `c[0..16]` by blade mask.

1. A rotor is even: only the scalar (mask `0`), the six bivectors (masks
   `3, 5, 6, 9, 10, 12`) and the pseudoscalar (mask `15`) may be nonzero,
   eight rational coordinates. A nonzero coefficient on an odd mask, or all
   coefficients zero, is refused: not a rotor, never a vertex.
2. `i` = least mask with `c[i] ≠ 0`; `a = |c[i]|`; `R' = R / a`, so
   `c'[i] = ±1`.
3. If `c'[i] = −1`, `R' = −R'`. Now `c'[i] = +1`.
4. Output `R'` and `key = blake3(put_q(c'[0]) ‖ … ‖ put_q(c'[15]))`. The
   equivalent integer layout, coprime numerators over one positive common
   denominator with the pivot equal to the denominator, is acceptable if
   every node uses the same one; `put_q` over the eight reduced rationals is
   the one in the tree.

Equality: `R ≡ S` iff `canonical(R) = canonical(S)` coefficientwise, iff
`R·S⁻¹` is a nonzero scalar of **either sign** (chapter 1.5).
`Rotor::projective_eq` (alphabet.rs 268) requires a positive scalar today
and must accept a negative one; `Rotor::normalised` (249) performs step 2
and must add step 3. Cost: sixteen rational divisions. A word with no
plane-closed geodesic is **not** the identity and is never keyed as `1`; it
is a vertex whose canonical form is this one.

### 16.2 A2 — Off-plane normal form

The mixed-plane group generated by the letters is not known to have a
terminating rewriting system: the primes have infinite order and their
products on planes sharing an axis generate a dense subgroup, so no
relation `(g_i g_j)^m = 1` holds there and no completion is run. Identity
off the plane is A1. The **tape** kept for a vertex is the shortest one
known:

1. **Run reduction.** Split the tape into maximal runs of letters on one
   slot plane (identity letters removed). Replace each run by its
   per-plane canonical tape (A3 → `PlaneForm::to_word`). Move a run
   leftward past runs on its **complementary** plane (1.2) until it meets a
   run on its own plane; merge; reduce again. Letters on planes sharing an
   axis are never reordered. Terminates: each pass strictly shortens the
   tape or leaves it unchanged, and reordering is bounded by the number of
   runs.
2. **Budgeted descent** (`shorter_tape(word, budget)`): breadth-first over
   tapes from the identity in letter-index order, visited set a `Keyed<()>`
   of A1 keys, stopping when `budget.spend()` fails or a tape with the
   target's A1 key is found at a length below the current one. Deterministic
   under a given budget.
3. Output the shorter of the two, or the input if neither shortened it.

The result is a **retirement** (3.8). It is the shortlex-minimal tape only
when the search completed the whole depth at which it was found within the
budget; the log says which (`retired {addr} shortlex` versus `retired
{addr} shorter`). `Word::normal_form` (466) keeps its per-plane path for
plane-closed words and calls step 1 for the rest; step 2 runs only in
`retire` and in the court's replay of a `Rewrite` word.

### 16.3 A3 — Canonical plane form

`Word::form()` reduces unit exponents (`reduce_unit_exponents`, 642:
`rem_euclid(4)`, with 3 written as −1) before returning. `PlaneForm`
equality compares reduced forms only; `conjugacy_key` (750) and
`sentence::chart_of` (1885) read the reduced form. Only the six unit turns
have finite order (four); prime exponents are unreduced integers; the sheet
parity is `Z/2`. A plane form is the whole vector `(parity, rot[0..17])`,
never a single `(plane, exponent)` pair.

### 16.4 A4 — Canonical conformal versor

Input: `V: Mv` (cga.rs 46; `Versor = Mv`, element.rs 201). The conformal
algebra in this tree is `Cl(3,1)` over `{e1, e2, e+, e−}` with squares
`(+,+,+,−)`, `e_o = ½(e− − e+)`, `e_∞ = e− + e+`, masks `E1 = 1, E2 = 2,
EP = 4, EM = 8`: the conformal model of the sensor **plane**.

1. All coefficients zero: refuse.
2. `i` = least mask with nonzero coefficient; divide by `|c[i]|`; negate if
   `c[i] < 0`. Output `V'` with `c'[i] = +1`.
3. Equality `Mv::projective_eq(a, b)`: `projective_ratio(a, b)` (330) is
   `Some(λ)`, `λ ≠ 0`, either sign. The ratio is `λ = ⟨a b̃⟩₀ / ⟨b b̃⟩₀`,
   accepted only when `a b̃` has no component outside grade 0 (the purity
   check the function performs); for a null `b` the test is exact
   componentwise proportionality.
4. `Mv::key()` = blake3 over the canonical coefficients. `to_bytes` on the
   wire may carry any scaling; the receiver canonicalises.

`Mv`'s derived structural `PartialEq` is removed; `Germ`, `Face`, `Nerve`
equality use `projective_eq` for versors and A1 for rotors.

### 16.5 A5 — Chain canonicalisation

Input: cells `(r, c, before, after) ∈ Z² × hue²`; the base is the sensor
plane and the hues are fibre coordinates (chapter 38 widens the base).

1. `min_r`, `min_c` over the cells; translate every cell by `(−min_r, −min_c)`.
2. Sort lexicographically on the tuple; remove exact duplicates.
3. Two cells on one site `(r, c)` with different `(before, after)`: refuse;
   the chain assigns two transitions to one cell and describes nothing. A5
   never sums, merges, or strips hues; whether a cell with `before ==
   after` belongs to a chain is the producer's decision (`germinate`).
4. Output the chain; empty input, empty chain.

`from_residual` (element.rs 1580) does 1–2 today. A5 is that code named and
called from every place that hashes or compares cells:
`composite_parent_id`, `footprint_key`, `section_address`'s assignment
(sorted), and `blind_replay`'s chain compare (the spoken chain
canonicalised before `rederived.sub() != spoken.sub()`).

### 16.6 A6 — Exact axis packing

Input: `x ∈ ℚ` reduced, denominator positive. Output `Option<[u64; 2]>`.

1. `numer(x)` outside `[−2⁶³, 2⁶³ − 1]`: refuse.
2. `denom(x) > 2⁶⁴ − 1`: refuse.
3. Output `[numer as i64 as u64 (two's complement), denom as u64]`.

Refusal propagates: `Harmonics::axes() -> Option<Vec<u64>>`;
`blade::moduli` and `stalk::invariant` return the **empty** `Invariant`,
which is novelty, with the log line `[{tag}] chart refused: ratio exceeds
axis`. Nothing saturates; `ratio_axes` (blade.rs 339), which drops the sign
and saturates to `u64::MAX`, is replaced. The two-limb encoding preserves
equality, which is all the atlas trie reads (strata are prefixes, `peel`
drops the last axis, nothing compares axes by magnitude). Should an
operation ever need numeric order on an axis (`r₁ < r₂ ⇔ packed(r₁) <
packed(r₂)`), the exact encoding is the Stern–Brocot path of the rational,
prefix-free, in as many limbs as it needs, still refusing past the limb
budget.

### 16.7 A7 — Invariant refinement coordinate

Input: a class rotor `R` (A1). Output `Option<Vec<u64>>`, the axes appended
to a chart on refinement.

- `R` has a plane form `(s, r)` (A3): `[1, s, r_1, …, r_k]`, each exponent
  two's-complement `u64`, `r` the conjugacy representative of 15.2.
- Else: `w = R R̃`, `a = c[0]`, `p = c[15]`;
  `[2, sign(a), sign(p)] ‖ A6(a²/w) ‖ A6(p²/w)`; refuse if A6 refuses.

`a` and `p` here are grade invariants of the class **rotor** (15.3). The
area, boundary length, and moments of a **body** are different quantities
and are already the body's moduli through `Harmonics::of` (M4). A
refinement axis comes from the class, never from the bodies that carried
it. `Stalks::refine(inv, axes: &[u64])` appends all axes; `split_stratum`'s
`coordinate_of` returns `Vec<u64>`; `refine_chart` (cognition.rs 2947)
passes A7's output; `Rotor::coordinate()` and `PlaneForm::coordinate()`
are deleted.

### 16.8 A8 — RMP1 migration

Runs once per store at load (`Kernel::restore` cognition.rs 5018,
`Treasury::restore` economy.rs 568, `Epigenome::uncrystallize` sdk.rs
1025) and once per tape arriving from a peer on the old build.

1. For every stored tape `T` with stored address `a_old`: `a_new =
   address(T)` by M2; if different, `remap.insert(a_old, a_new)`.
2. For every sentence tape, each germ's address is recomputed as
   `from_residual(A5(chain)).address()`; the stored germ address is
   ignored. `from_element` (sentence.rs 1792) does this always.
3. Rebuild every address-keyed carrier by inserting each entry under
   `remap.get(k).unwrap_or(k)`: `provisional`, `provisional_k`,
   `provisional_tu`, `ruled_addrs`, `ruled` strata members, `cavity_book`,
   `heard`, `listing_task`, `task_listings` values, `deflations`,
   `court_fed_index`, `cited_by`, `replay_memo`, `escrowed`, client
   `index`, `dialect`, `taught_up`, `glued`, `obstructions`,
   `expressed_index`.
4. Collision (two old keys → one new): `escrowed` sums μ⃗ lane-wise and
   unions voices (conservation); `provisional` keeps the earlier
   `provisional_tu`; `ruled` keeps one member and `court_federation` one
   record, the earlier ruling standing and the later logged as merged, never
   re-adjudicated; `heard` unions; `deflations` keeps the ruled entry's;
   `discoverers` keeps the earlier `standing_tu`; `spoken` unions voices per
   listing; `dialect` sums `uses` and keeps the later `last_used_k`; genome
   `expression` sums; every other carrier keeps the first. Provenance is a
   union and value is conserved; no merge mints, slashes, or re-rules. Log:
   `court remap {old}→{new} merged {carrier}` per merge.
5. Write the `RMP1` section `{count u32, (old [32], new [32])*}` after the
   carriers. `resolve_form`, `word_at`, and the client `word_at` consult it
   on any miss. Count hits per aperture; after an aperture with zero hits,
   drop the section at the next snapshot (`court remap retired`).

A tape that does not re-derive (`blind_replay` refuses it) is not remapped:
`court remap refused {old}: {reason}`, dropped from the carriers, its
escrow released to its payers.

### 16.9 A9 — Canonical selection serialisation

`Sel` (anchor.rs 31) is a term algebra: `Hue(u8)`, `Comp(Sel, u8)`,
`Delta(Sel)`, `Star(Sel)`, `Ext(Sel, u8)`, `Inter(Sel, Sel)`,
`Minus(Sel, Sel)`, `Flood(Sel, Sel)`, `ArgExt(Sel, u8)`. `Sel::to_bytes`
writes the variant's tag byte (`0..=8` in that order) then its fields in
declaration order, a `u8` as one byte, a `Sel` recursively. The grammar is
prefix-free, so no length prefixes; fixed little-endian integers as
everywhere in the tree. `Inter` is not reordered: `A ∩ B` and `B ∩ A` are
distinct terms with one support, and `derive_pi` looks for a common term;
a normal form for `Sel` is a separate move. `verdict::sel_key` (374) =
blake3 of `to_bytes`; the `format!("{s:?}")` form is deleted.

### 16.10 A10 — Peer identity by verifying key

1. `Router` and `Session::peers` are keyed by the hypha's verifying key `vk`.
2. A socket string learned without a `vk` (ACT_PEERS reply, `--peers`,
   bootstrap) enters `pending_addrs: Keyed<String>` keyed by `blake3(addr)`
   and is asked for its cover digest; on the signed digest
   (`admit_peer_digest`, `absorb_digest`) the entry moves to `vk` and the
   pending entry is removed.
3. Two socket strings with one `vk` are one peer; the later replaces the
   earlier in `addrs`. `rendezvous`, `closest`, `home` read `vk` keys only;
   `pending_addrs` is never a routing target.
4. Per-peer state is keyed by `vk`: transport outbox, measured RTT deadline,
   in-flight leases (`t_leases`), quote receipt-chain head. A second socket
   for a `vk` joins that state; the older socket closes once its in-flight
   frames drain; nothing is re-sent or re-leased.

Trust is the ed25519 signature on the cover digest (`court::identity::sign`
44, `verify` 77) and nothing else; there is no handshake protocol. A digest
whose signature does not verify under the claimed `vk` is dropped before any
carrier is touched.

### 16.11 A11 — Rendezvous by listing key

For a percept (server `judge`, mycelium-server.rs 1061):

1. Decode the chain and the frame rationals `(h_in, w_in, h_out, w_out)`.
2. A5 on the chain.
3. `key = element::listing_key(chain, frame)`: the framed residual's address
   by M2.
4. `inv = invariant_of(framed residual)`: the **body chart** (M4 step 1);
   `mag` its measured lanes.
5. Destination = `nerve::home(cover, inv, mag, rooms, addrs, self)` (788):
   fiber intersection on the occupied lanes, then per-lane room, then
   identity. A class axis (A7) is never a routing coordinate; a percept has
   no class yet.
6. Forward once; if the destination is self, docket.

`blake3(pbytes[32..])` is deleted. The same key is the listing id the book
uses, so rendezvous and docket agree by construction.

### 16.12 A12 — Comprehension by factorisation

Input: incoming rotor `g` (A1), basis `B = {b_1..b_n}` = rotors of the held
words on the chart's fiber, `budget`. Output: a factorisation
`g = b_{i1}^{±1} ⋯ b_{im}^{±1}` verified by A1 equality, or `Novel`.

**Plane case** (every `b_i` and `g` have plane forms `(s_i, r_i)`, `(s, r)`,
A3-reduced, with the unit-exponent coordinate carrying the relation
`4·e_u = 0`):

1. Even lattice generators: `{r_i : s_i = 0} ∪ {r_i − r_j : s_i = s_j = 1}
   ∪ {4·e_u}`; integer matrix `G` of these rows.
2. Hermite normal form `U·G = H` with unimodular `U`, exact over ℤ.
   Membership `r ∈ rowspace_Z(G)`: reduce `r` against `H` row by row;
   member iff the remainder is zero; the reduction coefficients pulled back
   through `U` give the integer combination of generators, hence of the
   `b_i` (each generator is a product of one or two basis words). Smith
   normal form `S = U·G·V = diag(d_1..d_r, 0..)` decides the same lattice
   (`y = r·V`, `y_j ≡ 0 mod d_j` for `j ≤ r`, `y_j = 0` beyond, exponents
   `x = (y_j / d_j)·U`) and may be used instead; HNF needs one transform
   and is the one bound.
3. `s = 0`: succeed iff `r` is a member. `s = 1`: succeed iff some `b_i` with
   `s_i = 1` has `r − r_i` a member; the tape is `b_i` then the even
   combination; least such `i`.
4. Verify: multiply the tape's rotors, A1-compare with `g`. Mismatch is a
   bug, not a refusal: assert in debug, refuse in release.
5. **Canonical remainder.** When membership fails, the HNF residue `r_rem`
   is unique (the reduced representative of `r` modulo the lattice under
   the HNF's row order). The remainder rotor `g_rem` is the plane element
   `(s_rem, r_rem)`, `s_rem = s` in the even case and `0` after the odd
   generator has been divided out, with `g = f · g_rem` for the factor `f`
   found. Output `Novel(g_rem)` with `f` recorded as the understood part:
   the exact boundary demand, with no choice in it.

**Off-plane case**: budgeted coset descent. States are A1 keys; from the
identity apply `b_i` and `b_i⁻¹` in index order; visited `Keyed<()>`; stop
on `budget.spend()` failure or on reaching `g`'s key. The path is the
factorisation. Failure returns `Novel(g)` with `g` unchanged: off the plane
there is no HNF and no metric to say which partial is nearer, so the whole
word is the demand.

Binding: `Epigenome::comprehend` (sdk.rs 856) for connections replaces the
single-representative `transition` test with A12 over the fiber's basis;
`Kernel::reduce_excluding` keeps its decomposition for chains and calls
A12 for connections. Cost: HNF polynomial in `n·k`; the descent is the
budget.

### 16.13 A13 — Meet of two keyed tries

`Keyed::meet<W>(&self, other: &Keyed<W>) -> Keyed<()>` (nerve.rs): keys
present in both. A simultaneous descent of the two PATRICIA tries
(`Node::Leaf(key, _)`, `Node::Branch { bit, children }`, `key_bit`,
`first_difference`, `held_key`, nerve.rs 21–75), using one fact: every key
in the subtree of a `Branch { bit }` agrees on all bits before `bit`.
`meet(a, b)` on two nodes:

1. `(Leaf(k), Leaf(k'))`: emit `k` iff `k == k'`.
2. `(Leaf(k), Branch { bit, children })`: recurse on
   `(a, children[key_bit(k, bit)])`; a leaf meets a branch only along its
   own path. Symmetric.
3. `(Branch { bit_a, ch_a }, Branch { bit_b, ch_b })`: representatives
   `r_a = a.held_key(ROOT)`, `r_b = b.held_key(ROOT)`. If
   `first_difference(r_a, r_b) = Some(d)` with `d < min(bit_a, bit_b)`, the
   subtrees disagree on a bit both agree on internally: emit nothing.
   Otherwise `bit_a == bit_b`: recurse on both child pairs; `bit_a <
   bit_b`: all of `b` agrees at `bit_a`, recurse on
   `(ch_a[key_bit(r_b, bit_a)], b)`; `bit_a > bit_b`: symmetric.
4. `meet` on the roots; an empty trie meets anything in nothing.

Soundness: emission only from an exact leaf pair. Completeness: a key held
by both follows the same side at every branch in both tries. Cost: every
call consumes a node no other call visits, and the prune stops any pair of
subtrees sharing no prefix, so the work is the shared prefix structure plus
one `held_key` (O(bits)) per branch pair; the N vs 4N counter test on the
larger trie holds with the smaller fixed.

### 16.14 A14 — Gauge between two connections

`cech::solve_gauge_between(n, a: &[(usize, usize, Rotor)], b: &[...]) ->
Option<Vec<Rotor>>`: `ϑ` with `b_ij = ϑ_j · a_ij · ϑ_i⁻¹` on a common edge
list. Tree walk with the twisted step `ϑ_j = b_ij · ϑ_i · a_ij⁻¹`, chords
compared by `projective_eq`; `None` means the two connections are not
gauge-equivalent. With `a` identity on every edge it reduces to
`solve_gauge`. This is the exact test for two classes being one (15.4) and
the descent step of 13.9.

### 16.15 A15 — Components by keyed union-find

`Components { parent: Keyed<[u8; 32]> }` on chart keys:

- `find(k)`: follow `parent.get` until a key is its own parent, halving the
  path on the way back (`parent.insert(k, grandparent)`); a key with no
  entry is its own root.
- `union(a, b)`: `ra = find(a)`, `rb = find(b)`; if equal, nothing;
  otherwise the **lesser key becomes the root**: `parent.insert(max(ra, rb),
  min(ra, rb))`. Union by key order, so a component's root is its least
  chart key with no second pass; that root is the `component_key` of 14.7.
- Run `union` once per induced face (14.4), read from the forms' `Face`
  lists. Components are the charts in `M` that are their own root; path
  halving makes `find` amortised logarithmic in `|M|`.

### 16.16 A16 — The twist loop (torsion-ordered gauge closure)

Input: an idea's form `F` (vertices, faces, loops), `budget`. Output: `F`
re-gauged so that every chord whose class is trivial is closed, the
remaining chords classified as torsion or free, and a log of the snaps.

1. Tree walk (12.2) on `F`'s faces; collect the open chords with their
   re-gauged holonomies `h_ab`.
2. For each open chord compute `Word::order(h_ab)` (552): `Finite(ℓ)` or
   `Infinite`. Sort chords by `(order, chord key)`, finite ascending,
   infinite last. This is a sort of a `Vec` of the chords the tree walk
   already produced, never a search.
3. Take the first finite chord. Solve for the gauge on its loop that closes
   it (`solve_gauge_between` with the target identity on that loop, 16.14).
   If it closes, apply the gauge to `F` (`Form::regauge`), log
   `twist {idea} chord {i} order {ℓ} closed`, and go to 1. By T2 (chapter
   42) closing it can only conjugate the other chords; none opens.
4. If the first finite chord does not close, it is torsion: hand it to A17.
   Continue with the next finite chord.
5. Infinite chords are free classes: hand each to M6 step 5 as a demand.
6. Stop when no finite chord is open, or when `budget.spend()` fails; the
   next stroke resumes from the sorted list, which is recomputed from the
   current `F`.

Termination: each pass through step 3 strictly reduces the number of open
finite chords, and step 4 never reopens one, so the loop makes at most
(number of chords) snaps. Cost per pass is one tree walk. Binding:
`Contemplation::pivot_on` (bubble.rs 712) and `expand_with` (84) call
`twist(&mut Form, &mut Budget) -> Vec<Snap>` **new** in sentence.rs.

### 16.17 A17 — Torsion filing and witness

Input: a chord with finite-order holonomy `h` (`order ℓ`) on a form `F`
whose loop A16 could not close. Output: a torsion cycle `(σ, ℓ)` on `F`, or
a refusal.

1. Read the torsion part of `h` on the plane (`PlaneForm::torsion_part`,
   12.2): `σ` the sheet parity and unit turns; the free part must be zero,
   else the chord is not torsion and goes to M6 as a demand.
2. Walk the loop's fibre from its base assignment at most `ℓ` steps
   (`toroid::witness_stored`): the walk returns at exactly step `ℓ` →
   `HolonomicLoopWitness::Closed{period: ℓ}`; it returns earlier or later,
   or not within `ℓ` → `Open`.
3. `Closed` at `ℓ`: `Element::record_torsion(σ, ℓ, fibre)` writes the cycle
   onto `F`'s tape (the `torsion` section `as_element` already serialises),
   log `torsion filed {idea} (σ,ℓ)`. `Open` or another period: the chord
   stays open; `F` cannot crystallize; log `torsion open {idea} chord {i}`.
4. The court replays every filed cycle by the same walk (`blind_replay`,
   `replay_nerve`): `"torsion cycle does not close"`, `"torsion period does
   not re-derive"` are the refusals.

A filed cycle is a memory: it records exactly the twist the language holds
on that loop without forcing it to identity. An infinite-order class is
never filed with a large period; it is a demand.

### 16.18 A18 — Rank accounting (the dimension audit)

Dimensional capacity is measured, not assumed. For a form `F` after the
tree walk:

- `torsion_rank(F)` = the number of open chords with finite order;
- `free_rank(F)` = the ℤ-rank of the lattice spanned by the free parts of
  the open chords' plane forms (Hermite normal form over ℤ, 16.12 step 2);
  off the plane, the number of open chords with infinite order;
- `axes(F)` = the number of chart axes opened by discharge (15.5).

For a chart's Čech complex: `Congruent` cells, `Class` cells by degree, and
axes of the chart's `Invariant` beyond its body harmonics.

`rank(F) = (torsion_rank, free_rank, axes)`, a vector, never summed.
`Form::rank(&self) -> Rank` **new**, `Complex::rank(&self) -> Rank`
**new**. In debug builds every move of chapter 3 and every court operation
asserts the rank law that applies to it (chapter 42, R1–R6); in release the
court refuses with `REFUSE_RANK_LOST` when a spoken form's rank is below
what its own tape re-derives. A18 is the only place a count appears in
cognition, and it is read, never acted on: no threshold, no score.

### 16.19 A19 — Class ledger exchange

Input: a chart `c` a mind quotes on; the hypha's classes on `c`. Output:
the mind's `horizon` seeded with the mesh's torsion, and the hypha's ledger
fed with the mind's.

1. The syllabus page for `c` (`serve_ruled`, chapter 26) carries, beside
   ruled words, one class record per class the hypha holds on `c`:
   `key [32] ‖ order (0 = infinite, else ℓ u32) ‖ representative Rotor bytes`
   from `cocycle_at` (degree 1) and the `Congruent`/`Class` cells of the
   chart's complex.
2. The mind admits each record into `Contemplation.horizon` under
   `Standing::Held` keyed by the class key (`seed_horizon(chart, records)`
   **new**); a held class is never re-derived as a dialect: A16 recognises
   a chord whose class key is in `horizon` and files it as `Held` torsion
   (finite) or skips the demand (infinite, already in the docket).
3. The mind's own filed torsion travels on its spoken form's torsion
   section; its free classes travel as demands. The court files them (M8,
   chapter 23.7) under the same keys.
4. On the mind's restart the horizon is empty; step 1 refills it on the
   first quote of each chart. The epigenome is rebuilt by thinking; its
   torsion is rebuilt from the mesh.

## 17. Master algorithms

Carried from chapter 16: nineteen procedures. Each master algorithm below is
the complete operation the system runs, stated as the sequence of component
calls with its invariant, binding, and failure behaviour.

Composition hierarchy (arrows are "calls"; every path to a key passes
through M1):

```
        A3 canonical plane form
              │
              ▼
        per-plane geodesic tape
              │
              ▼
        A2 run reduction (+ budgeted descent in retire)
              │
              ▼
        A1 canonical rotor form ◄──── A4 canonical conformal versor
              │
              ▼
   ┌──────────────────────────────┐
   │   M1 VERTEX IDENTITY ORACLE  │
   └──────────────────────────────┘
        │              │               │                 │
        ▼              ▼               ▼                 ▼
   A5 chains       A6 axes         A12 factorisation   A13 meet · A14 gauge between · A15 components
        │              │               │                 │
        ▼              ▼               ▼                 ▼
   ┌──────────┐   ┌──────────────┐  ┌────────────────┐ ┌────────────────┐
   │ M2 ADDR  │   │ M4 CHARTS    │  │ M6 COMPREHEND  │ │ M8 COURT GLUE  │
   └──────────┘   └──────────────┘  └────────────────┘ └────────────────┘
        │              │  A7 ▲                                │
        │              ▼      │                               ▼
        │         ┌──────────────┐ ◄──────────────────────────┘
        │         │ M5 CLASSES   │
        │         └──────────────┘
        ├───────────────┬──────────────┐
        ▼               ▼              ▼
   ┌──────────┐   ┌──────────────┐  A8 → RMP1
   │ M7 WIRE  │   │ M3 COLD BOOT │
   │ A9 A10 A11│  └──────────────┘
   └──────────┘
```

### 17.1 M1 — Vertex Identity Oracle

**Purpose.** Decide whether two things mean the same and produce the one
key that names a meaning. Every `key`, `eq`, and `address` reduces to this.

**Input.** A `Word`, `Rotor`, or `Mv`. **Output.** Canonical form and key,
or refusal.

**Procedure.**
1. A `Word`: compute its rotor (`Word::rotor`, memoised).
2. The rotor has a plane form: A3 reduces it; the canonical per-plane tape
   is the normal form; the key is A1 of the rotor, so plane and off-plane
   words share one keyspace.
3. Else: A2 step 1 for the tape; key A1.
4. An `Mv`: A4.
5. Equality is key equality; `projective_eq` with either sign is the same
   test without hashing.

**Invariant.** Equal action on the sensor ⇔ equal key, because A1 and A4
are injective on projective classes and exact.

**Binding.** `Word::key`, `Word::eq`, `Word::same_form`, `Rotor::key`,
`Rotor::projective_eq` (sign rule), `Rotor::normalised` (sign step),
`Mv::key`, `Mv::projective_eq`; `Element::eq` becomes `same_vertex`: equal
M1 key of the letters, equal `ρ`, `shift`, `sigma`, A5-equal chain; or
`PartialEq` is removed from `Element` and `same_vertex` is called by name.

**Failure.** A zero rotor or versor, or an odd-grade coefficient, is
refused and is never a vertex.

### 17.2 M2 — Address Derivation

**Purpose.** The address of a word form or sentence form, independent of
spelling, serialisation, and scaling.

**Procedure.**
1. Non-sentence word: M1 key of the letters' word ‖ `put_q(ρ)` ‖
   `put_q(shift.0)` ‖ `put_q(shift.1)` ‖ sorted `sigma` ‖ A5 chain, as
   `form_key_bytes` (element.rs 1039) does with its `geodesic().key()`
   replaced by the M1 key.
2. Rewrite word: `before` addresses, `after` address, kind, claimed, all
   addresses being M2 outputs.
3. Sentence form: per germ in order, M1 key of its word ‖ A4 canonical
   versor ‖ kind byte; per face, M1 key of `g_ij` ‖ A4 canonical versor.
   `vertex_address` (sentence.rs 1144) writes raw versor bytes today; it
   writes A4 outputs. Loops are never written.
4. blake3 over the bytes. The field list of chapter 1.5 is this procedure.

**Invariant.** Address equality ⇔ same vertex, same frame, same canonical
chain; a tape version change never changes an address.

**Binding.** `Element::address` (1026), `sentence::vertex_address`,
`element::listing_key`, `composite_parent_id` and `section_address`
(cognition.rs 5635, 5526) rewritten to `from_residual(A5(..)).address()`
and a sorted assignment.

### 17.3 M3 — Cold-Boot Migration

**Purpose.** Bring every store written under byte addresses onto M2
addresses once, with conservation, and make the new rule the only one.

**Procedure.** M2 over every stored tape → A8 steps 1–5 → carrier rebuild →
the reconciliation traces of chapter 40. Servers before clients, a then b
then c; each server on the new build reads and remaps old tapes from old
peers for as long as they run (chapter 33).

**Invariant.** After M3, `word_at(addr)` succeeds for every address held in
any carrier; `Treasury::supply`, `bond_total`, and every per-lane bond are
unchanged; the ruled count is the count before minus the merges logged.
Trace: `court remap {n} entries merged {m}`, zero `word_at` misses over the
ruled fibers.

**Binding.** `Kernel::restore`, `Treasury::restore`,
`Epigenome::uncrystallize`, `sentence::from_element`, the `RMP1` section in
`snapshot` and `crystallize`.

**Failure.** A8's refusal line; the tape is dropped and its escrow released.

### 17.4 M4 — Chart Coordinates

**Purpose.** The `Invariant` that places a body, connection, sentence, or
class on the atlas, exactly and identically on every node.

**Procedure.**
1. Body: `Harmonics::of` (ear.rs 48) → each ratio through A6 → the axes
   vector, or the empty `Invariant` on any refusal.
2. Connection: `connection_chart` (stalk.rs 136): trace and determinant
   through A6, then the gauge permutation's cycle type; refusal → empty.
3. Sentence: `chart_of` (sentence.rs 1885) over A3-reduced plane forms of
   the germs, conjugacy representatives, not raw exponents.
4. Class: A7.
5. Refinement: `Stalks::refine(inv, axes)` appends A7's axes;
   `split_stratum` places each member by `coordinate_of` returning those
   axes for members on the class's side and `[0]` otherwise.

**Invariant.** Bodies congruent under the harmonics' symmetry share a
chart; different exact invariants have different charts; no coordinate is a
hash fragment.

**Binding.** `blade::axes_of`, `Harmonics::axes`, `stalk::invariant`,
`connection_chart`, `chart_of`, `Stalks::refine`, `split_stratum`,
`Kernel::refine_chart`.

### 17.5 M5 — Class Ledger Identity

**Purpose.** Record, compare, carry, and refine dialect classes so the same
class has one key on every hypha and one axis when it refines.

**Procedure.**
1. A holonomy or transition arrives as a rotor or versor; M1 canonicalises.
2. Class key per 15.4 of the canonical form.
3. `push_cocycle` (cognition.rs 990): compare the incoming to the held by
   `projective_eq` (either sign), then by class key; equal → no entry; a
   new representative of a held class → replace the representative, keep
   the key; a new key → a new entry. `a == b` accepted for intra-hypha
   classes (14.8).
4. Refinement: M4 step 5 with A7.
5. Under refinement of the base or change of cover: 15.5–15.6, comparing
   class keys.

**Invariant.** `cocycle_at` holds at most one entry per (chart, pair, class
key); a class's refinement axis is a function of its conjugacy invariants
only.

**Binding.** `push_cocycle`, `cocycle_key`, `refine_chart`, `mark_class`,
`pending_refine` values carrying the class key beside the rotor.

### 17.6 M6 — Comprehension

**Purpose.** Decide whether a mind already speaks an incoming word by
factoring it through what it holds, and otherwise measure exactly what is
new.

**Procedure.**
1. M1 on the incoming word; M4 for its chart; the fiber over that chart is
   the basis.
2. Sentence forms: chapter 24 (re-derive as a form, `coheres`,
   `regauge_overlap` against held forms of the cited class).
3. Connections: A12 over the fiber's basis. Success →
   `Comprehension::Understood(indices)` with the factorisation tape recorded
   as expression; a class → `Dialect(g)` when the factorisation succeeds up
   to a gauge that does not close; failure on the plane → `Novel(g_rem)`
   with the canonical remainder as the demand and the factor `f` counted as
   expression; failure off the plane → `Novel(g)`.
4. Residual words: `locate` as today; the remainder is the novelty.
5. The demand emitted for a `Novel` is exactly `g_rem` (or `g`), spoken as
   a connection with no chain; the hypha dockets it as any word. No mind
   emits "unknown"; it emits the element it lacks.

**Invariant.** A word the mind holds under any spelling is `Understood`;
`genome.contains(address)` is never the test.

**Binding.** `Epigenome::comprehend` (sdk.rs 856), `learn_one` (977, which
keeps its address dedupe as a fast path after M1, never instead of it),
`Kernel::reduce_excluding`.

### 17.7 M7 — Wire Identity

**Purpose.** Take bytes off the wire and turn them into identities the
system may act on, with nothing decided on the bytes themselves.

**Procedure.**
1. Decode the frame; verify signatures where present (A10).
2. Chains: A5. Words: M1. Forms: M2 for the address.
3. Routing: A11 for percepts; ACT_DEMAND by cover-trie `home` as today.
4. Verification: `blind_replay` re-derives and compares by M1/M2/A5, never
   by `to_bytes`.
5. Admission of peers: A10.
6. Digests (cover roots, ruled roots, fetch lists) are computed over M2
   addresses and chart axes and used only to decide what to fetch and whom
   to trust, never what a word means.

**Invariant.** A peer on any build that speaks a correct tape in any
spelling is understood; a tampered tape is refused by re-derivation.

**Binding.** `mycelium-server.rs::judge`, `serve_control`,
`admit_peer_digest`; `sdk.rs::absorb_digest`, `add_peer`, `ask_peers`;
`verdict::blind_replay`; `dialogue` decoders.

### 17.8 M8 — Court Glue

**Purpose.** When a peer's section for a chart arrives, decide consilience,
attachment, dialect, or refinement, and update the ledger, with every step
a theorem of chapters 13–15.

**Procedure.**
1. M1 on the peer's section; M4 for the chart; M2 for its address.
2. Goodness (chapter 14): A13 for the meet of chart sets, A15 for
   components, the tree walk (12.2) per hypha per component. `Pending` →
   `Dialect("overlap goodness pending")`, docketed. `Cyclic` → 14.8
   retraction by `home`, rerun.
3. Mayer–Vietoris (13.9 steps 1–8) per good component, with A14 for
   descent; cells keyed per component (14.7).
4. Rulings by the table of 13.10; the ledger by M5; a degree-2 class on a
   good component refines the base by M4 step 5.
5. Naturality on any cover change (15.6) and on any base refinement (15.5).
6. Exactness self-check (13.9 step 8) on the court's blind replay.

**Invariant.** No class enters the ledger from a non-good overlap; classes
have equal keys on every hypha; the union is never materialised.

**Binding.** `Kernel::glue_fetched`, `court_stroke`, `refine_chart`,
`apply_verdict` (chapter 23).

### 17.9 M9 — Torsion Bridge

**Purpose.** Connect the mind's contemplation to the court's ledger through
the one thing both file identically, torsion, so that thinking persists in
the mesh rather than on the mind's disk, and two minds never re-derive a
class the mesh already holds.

**Procedure.**
1. Per idea per stroke: A16 (twist), A17 (file what does not snap), M6 step
   5 for free classes.
2. Cauterize: when every loop is identity or filed torsion and the remainder
   is empty, crystallize (`as_element` with the torsion section, M2 address,
   dialect `Stood`, spoken once by the chapter 26 dedupe). No re-gauge
   touches a crystallized form; a later gauge is a new form.
3. Wrap: the next idea on a meeting chart glues the crystallized form as a
   `Glued` vertex (chapter 25); by T6 the wrap adds no class, so only its new
   faces are twisted.
4. Court side (chapter 23.7): the spoken form's torsion cycles are matched
   by class key against the chart's `Congruent` cells; agreement is
   consilience of torsion, a mismatch on one key is `REFUSE_NOT_NATURAL`, a
   new finite key files a `Congruent` cell, an infinite class enters
   `cocycle_at`.
5. A19 both ways on every quote and every syllabus page.
6. Standing: torsion from the hypha is `Held`; torsion the mind filed is
   `Stood`; torsion discharged by a ratified translation is `Ruled` and
   leaves both ledgers together (15.5).
7. A18 asserts the rank laws at every step.

**Invariant.** A class key present in any voice's `horizon` is present on
the chart's hypha, and conversely after one aperture; a mind's filed
torsion is on the mesh before the mind's next stroke.

**Economics.** Filing torsion pays nothing (a partial closure); the
remainder's descent mints; a translation that discharges a filed class is
paid through `mint_or_admit_translation`; a free class that a later word
closes pays that word's speaker.

**Binding.** `Contemplation::{pivot_on, expand_with, seed_horizon}`,
`sentence::twist`, `Element::record_torsion`, `toroid::witness_stored`,
`Kernel::glue_fetched` torsion arm, `serve_ruled`, `dialogue` class record.

---

# Part IV — Implementation on crystallized forms

Bound to a29bba5. Functions named without **new** exist at the path and line
given. The rule of this part: operations act on stored forms and their
rotors; cells are touched only in `family::perceive` and in `speak_rule`'s
final emission of output cells. Every identity in this part is a master
algorithm of chapter 17.

Every file name in Parts IV–VII is under `/opt/mycelium-v0.1.0/src/`:

| short name | absolute path |
|---|---|
| `alphabet.rs`, `sentence.rs`, `element.rs`, `cech.rs`, `stalk.rs`, `blade.rs`, `bubble.rs`, `family.rs`, `cga.rs`, `peel.rs`, `anchor.rs`, `sheaf.rs`, `germinate.rs`, `rewrite.rs` | `/opt/mycelium-v0.1.0/src/transducer/<name>` |
| `cognition.rs`, `registry.rs`, `synthesis.rs` | `/opt/mycelium-v0.1.0/src/server/<name>` |
| `verdict.rs`, `identity.rs`, `ledger.rs` | `/opt/mycelium-v0.1.0/src/court/<name>` |
| `nerve.rs`, `mesh.rs`, `wire.rs` | `/opt/mycelium-v0.1.0/src/network/<name>` |
| `sdk.rs` | `/opt/mycelium-v0.1.0/src/client/sdk.rs` |
| `dialogue.rs` | `/opt/mycelium-v0.1.0/src/anastome/dialogue.rs` |
| `ear.rs` | `/opt/mycelium-v0.1.0/src/perception/ear.rs` |
| `economy.rs`, `physics.rs`, `chronometry.rs` | `/opt/mycelium-v0.1.0/src/<name>` |
| `cyclo.rs`, `scalar.rs`, `constants.rs` | `/opt/mycelium-v0.1.0/src/core/<name>` |
| `mycelium-server.rs`, `mycelium-client.rs` | `/opt/mycelium-v0.1.0/src/bin/<name>` |
| `voice/mod.rs` | `/opt/mycelium-v0.1.0/src/voice/mod.rs` |

## 18. The form types

Carried from Part I: a form is vertices, faces, loops, remainder.

### 18.1 `/opt/mycelium-v0.1.0/src/transducer/alphabet.rs`

`Word` (line 371) is the word form's tape. Add:

```rust
pub struct Contact { pub anchor: u16, pub ring: u8 }
impl Word {
    pub fn with_contact(self, c: Contact) -> Word;          // new
    pub fn contact(&self) -> Option<&Contact>;              // new
    pub fn rebase(&self, by: &Word) -> Word;                // new: by·self·by⁻¹, normal form  (3.2)
}
```

`to_bytes` (592) / `from_bytes` (599): after the letters, `0` or
`1 ‖ anchor u16 ‖ ring u8`. `key` (448) is the M1 key and includes the
contact. `normal_form` (466), `geodesic` (436), `compose` (457), `inverse`
(452), `order` (552), `rotor` (441) are the composition and normalisation
of 3.1 and are used unchanged apart from M1's changes (chapter 29).

### 18.2 `/opt/mycelium-v0.1.0/src/transducer/sentence.rs`

Replace `Germ` (583) by the form vertex:

```rust
pub enum Vertex {
    Frame,                                              // the scene frame, one per sentence
    Word {                                              // a word form of a body
        address: [u8; 32],
        word: Word,                                     // normal form, with contact if any
        rho: BigRational,
        placement: (BigRational, BigRational),
        gauge: Vec<(u8, u8)>,
        class: stalk::Invariant,                        // the body's chart (source or target)
    },
    Glued {                                             // a crystallized sentence form, by address  (3.3)
        address: [u8; 32],
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,                                // relative word from its Frame to this Frame
    },
    Rebased { by: u16, of: u16 },                       // 3.2, indices into `vertices`
    Unfolded { measure: Measure },                      // 3.4, the sentence iterated on itself
}
pub enum Measure { Occupied, Remainder, Ring { anchor: u16 } }

pub struct Face { pub i: u16, pub j: u16, pub g_ij: Word }
pub struct Loop { pub faces: Vec<u16>, pub holonomy: Word }

pub struct Form {                                       // replaces Nerve as the stored object
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub loops: Vec<Loop>,
    pub remainder: Vec<Cell>,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,                 // by class key
}
```

`Nerve` (620), `Germ`, `Cycle` are removed (chapter 30); `Complex` on the
sentence stays as the mind's thought carrier, not part of the stored form.

```rust
impl Vertex {
    pub fn word_in(&self, form: &Form) -> Word;   // Frame: identity; Word: word; Glued: placement; Rebased: vertices[by].word_in ∘ vertices[of].word_in ∘ inverse; Unfolded: identity
    pub fn class(&self) -> stalk::Invariant;
    pub fn depth(&self) -> u16;                    // Glued: depth; others 0
}
impl Form {
    pub fn face_of(&self, i: u16, j: u16) -> Word;        // vertices[j].word_in ∘ vertices[i].word_in⁻¹, normal form   (3.1)
    pub fn delta1(&self) -> Vec<(u16, Word)>;             // holonomy per loop (10.3)
    pub fn coheres(&self) -> bool;                        // delta1 all projectively identity
    pub fn is_closed(&self) -> bool;                      // coheres && remainder.is_empty()
    pub fn depth_of(&self) -> u16;                        // 1 + max Glued depth, else 0
    pub fn regauge(&mut self, gamma: &[Word]);            // 3.6, one word per vertex (10.2)
    pub fn regauge_overlap(&mut self, theta: &Word, face: u16);  // the extension θ_A of 13.4 on a working copy
    pub fn glue(&mut self, s: &Form, s_addr: [u8;32], placement: Word) -> Option<u16>;  // 3.3; None unless s.is_closed() and s's Frame is identity (chapter 32)
    pub fn rebase(&mut self, by: u16, of: u16) -> u16;    // 3.2
    pub fn unfold(&mut self, measure: Measure) -> u16;    // 3.4
    pub fn loops_of(faces: &[Face], n: usize) -> Vec<Loop>; // spanning forest chords
}
```

`assemble` (939) reduces to: given vertices and the face index pairs the
bundle relates, compute faces by `face_of`, loops by `loops_of`, holonomies
by composition. It never reads cells. `solve_coboundary` (1268) is the tree
walk of 12.2 with its documented sign convention;
`sentence::tests::regauge_then_delta1_is_conjugate` pins it.

### 18.3 `/opt/mycelium-v0.1.0/src/transducer/element.rs`

`Element` (81) gains `depth: u16`, in `GEN2` after `mag` (`to_bytes` 1035,
`from_bytes` 1098). The sentence description bytes are the `Form` tape
(chapter 19). `is_sentence` unchanged. `PartialEq` is removed in favour of
`same_vertex` (M1); `address()` is M2.

## 19. Tape: `SENTENCE_TAG` version 2

Carried from chapter 18: the `Form`. Written by `sentence::as_element`
(1362), read by `from_element` (1707):

```
SENTENCE_TAG ‖ version u8 (=2)
n_vertices u16, then per vertex:
  kind u8: 0 Frame | 1 Word | 2 Glued | 3 Rebased | 4 Unfolded
  Word:    address 32 ‖ Word bytes ‖ rho (put_q) ‖ placement (put_q, put_q) ‖ gauge len16 + pairs ‖ class len16 + axes u64…
  Glued:   address 32 ‖ chart len16 + axes ‖ magnitude 4×u64 ‖ depth u16 ‖ placement Word bytes
  Rebased: by u16 ‖ of u16
  Unfolded: measure u8 (0 occupied, 1 remainder, 2 ring) ‖ anchor u16 (ring only)
n_faces u16, then per face: i u16 ‖ j u16 ‖ g_ij Word bytes
n_loops u16, then per loop: n u16 ‖ face indices u16… ‖ holonomy Word bytes
n_hue u16, then per class: class key 32 ‖ len16 + pairs
remainder len32 + germ bytes
depth u16
```

`as_element` serialises the `Form` and derives nothing from cells.
`from_element` reconstructs the `Form`; `Word` vertices derive the rotor
lazily through `Word::rotor` (441) and the affine through
`element::conformal_of` (239). The stored `address` of a vertex is
**recomputed** by M2 on read (A8 step 2) and the stored bytes are never
trusted. A version-1 tape reads as a `Form` of `Word` vertices with faces
recomputed by `face_of` and depth 0.

## 20. Perception produces word forms once

Carried from chapter 19: forms are read from tapes. This is the one place
cells become forms.

`/opt/mycelium-v0.1.0/src/transducer/family.rs`: `perceive(grid)` (76)
returns bodies. Add:

```rust
pub fn word_form(a: &Body, b: &Body) -> Option<Vertex>   // new: carriage(a,b) (131) → spell → Vertex::Word with class = b.moduli (M4 step 1)
pub fn frame_vertex() -> Vertex                           // new: Vertex::Frame
```

`pair_nerve` (about 590) becomes `pair_form(a: &Grid, b: &Grid, scale) ->
Form`: vertices = `Frame` plus one `Word` per carried body plus body-only
vertices for created and erased bodies (`Word` with identity word and an
empty source or target class); faces for every related pair through
`face_of`; loops through `loops_of`. Shells (`peel::peel_body_scaled`)
remain pairing candidates and are never vertices. Candidates are bucketed by
`moduli.key()` over exact axes (A6) as an index only; the decider is that a
carriage exists, never the bucket.

## 21. Extraction as moves

Carried from chapter 20: pair forms. Extraction is seven moves on them.

```rust
pub trait Crystal {                                       // new: source of crystallized forms
    fn on_class(&self, class: &stalk::Invariant) -> Vec<(Element, Form)>;  // ruled/held closed forms whose chart meets the class
    fn speak_form(&self, f: &Form, x: &Grid, placement: &Word) -> Option<Speech>;
}
pub fn extract_rule(family: &[(Grid, Grid)], crystal: &dyn Crystal) -> Option<SentenceRule>
```

`Epigenome` implements `Crystal` over its working atlas (chapter 24.1), the
standing of each returned form beside it; `Kernel` over the ruled cover
(cognition.rs, `word_at` 1766 with `ruled_contains`).

Inside `extract_rule`, in order:

1. **Correspondence by class.** Vertices across pairs are matched by
   `class()`; a class in every pair is a rule vertex; a class in some
   pairs is a section over those pairs (`SentenceRule.sections`, a
   `Keyed<Vec<u16>>` by class key). Counts are never compared.
2. **Constancy of faces.** For each matched pair of vertices, `face_of` in
   every pair; the face is a rule face iff all are one vertex (M1
   equality). Non-constant faces are dropped; the vertices stay.
3. **Gluing.** For each class present, `crystal.on_class(class)`; for each
   crystallized `s`, the placement word from `s`'s `Frame` to the pair's
   `Frame` via the matched vertex; if `crystal.speak_form(s, x, placement)`
   reproduces a subset of the pair's output on every pair, the pair forms
   `glue(s, addr, placement)` and the reproduced vertices leave the
   remainder (3.3, 13.6).
4. **Rebasing.** For vertex pairs `(i, j)` where `face_of(i, j)` on every
   pair equals `word_in(i) ∘ word_in(j) ∘ word_in(i)⁻¹`, replace `j` by
   `Rebased{by: i, of: j}` (3.2).
5. **Unfolding.** If each pair's output is the input under `k` applications
   of one form with one `Measure` strictly monotone and one stop criterion,
   the rule is the form plus `Unfolded{measure}` (3.4).
6. **Contact.** For a `Word` vertex whose placement differs per pair but
   whose ring distance to one anchor vertex at the output is constant, its
   word gains `Contact{anchor, ring}` and its placement leaves the form
   (3.5).
7. **Coherence.** `coheres` on the rule; if false, `solve_coboundary` gives
   `γ` and `regauge`; still false → the rule is held with the offending
   loop (12.2 step 4).

No pass reads cells; every comparison is M1.

`speak_rule(rule, x)` (997): `perceive(x)` once; match bodies to rule
vertices by class through `Stalks::fiber` (stalk.rs 362), several bodies
on one vertex being a section; `Word` vertices emit through `conformal_of`
and `carry_cell` (element.rs 346) at the placement solved from the face to
the anchor; `Glued` through `crystal.speak_form`; `Rebased` through
`word_in`; `Unfolded` by iterating the form to stop; `Contact` words by
`orbit` (chapter 22). Remainder as now.

`rule_replays_family` (737): the same seven moves with the ruled cover as
`Crystal`; returns `Option<Vec<([u8;32], [u64;4])>>`, the per-speaker
deflation vectors for chapter 26.

## 22. Orbit

```rust
pub fn orbit(v: &Vertex, x_cells: &[(i64,i64,u8)], anchor: &[(i64,i64)], frame: (usize,usize)) -> OrbitEnd
pub enum OrbitEnd { Contact{cells, steps: u32}, Torsion{steps: u32}, LeftFrame{steps: u32}, NoOrbit }
```

Realise the word once through `conformal_of`; apply by `carry_cell`; ring
distance by the ring arithmetic in `sentence.rs` (the one remaining use of
`adjacent_ring`); bound `frame.0 + frame.1`. `NoOrbit` when the first step
leaves the distance unchanged; `extract_rule` refuses to mint a contact
word that returns it.

## 23. Court

Carried from chapters 21–22: rules as forms. The court replays them and
glues sections across hyphae (M8).

### 23.1 `replay_form`, `/opt/mycelium-v0.1.0/src/court/verdict.rs`

Before the existing sentence arm in `blind_replay` (31):

```rust
pub fn replay_form(spoken: &Element, resolve: &dyn Fn(&[u8;32]) -> Option<Form>, family: Option<&[(Grid,Grid)]>) -> Option<&'static str>
```

1. `REFUSE_GLUED_DEPTH`.
2. `REFUSE_GLUED_UNRULED` when `resolve` is `None`; `REFUSE_GLUED_FRAME_GAUGED`
   when the resolved form's `Frame` is not identity (chapter 32).
3. Each `Word` vertex: rotor from tape, affine, M1 compare against the
   vertex's own recomputed vertex (`REFUSE_VERTEX_TAPE`).
4. Each face: `face_of(i, j)` recomputed, M1 compare with stored
   (`REFUSE_FACE`).
5. Each loop: holonomy recomputed, projectively identity (`REFUSE_LOOP`).
6. `REFUSE_REBASE_TRIVIAL`, `REFUSE_UNFOLD_STILL`, `REFUSE_ORBIT_STILL`, by
   executing the move on `family` when given, else by the form's own
   consistency.
7. Descent on `spoken.magnitude()` (`REFUSE_NO_DESCENT`, `REFUSE_REDUNDANT`).

The translation arm of `blind_replay` recomputes 13.9 step 5 blind for a
`TRANSLATION_TAG` element (`REFUSE_MV_INEXACT`), refuses a translation
across an overlap whose fetched tapes fail (A1) for either hypha
(`REFUSE_OVERLAP_CYCLIC`), and refuses a spoken class whose key differs
from its recomputation on the cited tapes (`REFUSE_NOT_NATURAL`). The chain
compare canonicalises the spoken chain by A5 first. `sel_key` (374) is A9.
`rule(...)` (222) is unchanged.

### 23.2 Kernel carriers, `/opt/mycelium-v0.1.0/src/server/cognition.rs`

- `resolve_form(&self, addr) -> Option<Form>` **new**: `word_at` (1766) if
  `ruled_contains`, then `from_element`; provisional and dialect return
  `None`; cavity returns `Some` (chapter 27); `RMP1` consulted on a miss.
- `replay_memo: Keyed<Keyed<(bool, u64)>>` **new**, consulted in
  `perceive_speech` (1788) and `glue_fetched` before `rule_replays_family`;
  shed by the T lifecycle as `heard` is; justified by 13.6.
- `cited_by: Keyed<Keyed<()>>` **new**: written in `home_install` and the
  `Admitted` arm for every `Glued` vertex; read by `reduce_excluding` as a
  use.
- `apply_verdict` (2331) on `RULING_SLASHED` of an address in `cited_by`:
  each citer returns to `provisional` with `provisional_tu` now; the next
  `court_stroke` (2398) replays it without the citation.
- `cocycle_at: Keyed<(Versor, u64, [u8;32], [u8;32], [u8;32])>` (754): the
  fifth field is the class key (15.4); `snapshot`/`restore` (4962, 5351)
  and the MYC2 ledger record it.
- Both `replay_memo` and `cited_by` rebuilt at `restore` (5018) by walking
  ruled tapes; never in `snapshot`.

### 23.3 Glue: `glue_fetched` (2651) as M8

For each `Glued` vertex not resolvable, run the digest/fetch path for its
chart (`fetch_have` 3169, server `serve_ruled` 3115); glue what arrives
first; otherwise `Glue::Dialect` with `"citation pending"` and the citer
stays docketed. Then, after `set_section(peer, …)`:

1. Goodness: `x = ChartGraph` for self and `y` for the peer over
   `M = Cover::charts_of(lane, self).meet(&Cover::charts_of(lane, peer))`
   (A13); `goodness(&x, &y)` (14.5) → per component `Good`, `Disconnected`
   (→ per-component edges, 14.7), `Cyclic` (→ 14.8 on the cover passed in,
   rerun), `Pending` (→ `Glue::Dialect("overlap goodness pending")`).
2. Edges via `set_edge_in(a, b, component, g, volume)`; `cx.overlap` and
   `cx.mayer_vietoris(&peer_cx, &ov)` per good component (13.9); `push_cocycle`
   receives `theta` for `Class1`; a `Class2` on a good component calls
   `refine_chart` with the overlap loop; `Remainder` marks remainder edges.
3. The existing `coboundary_on_link(&peer, …)` remains for the cells the
   union creates at degree ≥ 2 within the link, now per component (14.6).
4. Rulings by the table of 13.10.

```rust
pub struct ChartGraph { pub charts: Keyed<()>, pub faces: Vec<([u8; 32], [u8; 32], Rotor)> }   // new, cech.rs (14.4)
pub enum Goodness { Good { component: [u8;32] }, Disconnected { components: Vec<[u8;32]> },
                    Cyclic { hypha: [u8;32], chord: ([u8;32],[u8;32]), holonomy: Rotor }, Pending { component: [u8;32] } }  // new
pub fn goodness(x: &ChartGraph, y: &ChartGraph) -> Vec<Goodness>;                               // new
fn cell_key(verts: &[[u8; 32]], component: Option<&[u8; 32]>) -> [u8; 32];                       // component appended when Some
pub struct Overlap { pub vertices: Keyed<()>, pub cells: Vec<[u8; 32]> }                         // new
pub enum MayerVietoris { Disjoint, OneSection { gauge: Vec<([u8;32], Rotor)> }, Attached { theta: Rotor },
                         Class1 { theta: Rotor, chord: [u8;32], holonomy: Rotor }, Class2 { overlap_loop: [u8;32], class: Rotor },
                         Remainder { faces: Vec<[u8;32]> } }                                      // new
impl Complex {
    pub fn set_edge_in(&mut self, a: [u8;32], b: [u8;32], component: [u8;32], g: Rotor, volume: [u64;4]);  // new
    pub fn overlap(&self, other_vertices: &Keyed<()>) -> Overlap;                                // new: 13.9 steps 1–2
    pub fn mayer_vietoris(&mut self, other: &Complex, ov: &Overlap) -> MayerVietoris;             // new: 13.9 steps 3–8
    pub fn delta2(&self, cell: &[u8;32]) -> Option<Rotor>;                                       // new: 10.4 on one 3-cell; coboundary_on_link uses it
    pub fn restrict(&self, sections_in: &Keyed<()>) -> Complex;                                  // new: 15.5 step 1
    pub fn straddling(&self, left: &Keyed<()>, right: &Keyed<()>) -> Vec<[u8;32]>;               // new: cells with vertices on both sides
}
impl Cover { pub fn charts_of(&self, lane: u8, hypha: &[u8;32]) -> Keyed<()>; }                 // new, nerve.rs, from `held`
```

`solve_gauge` (806) is unchanged: its chord `!=` is projective through
`Rotor::PartialEq`; it returns `λ` with `g = δ⁰λ`, and the flattening gauge
is `λ⁻¹` pointwise. `fill` (556) keeps calling it; its inner pair loop is
over one cell's own vertices, the cell's boundary, not an enumeration.
`class` (619) is unchanged; `Class2` writes a degree-2 cell with
`Fill::Class` through `remember`.

### 23.4 Ledger: `push_cocycle` (990) as M5

Accept `a == b` for intra-hypha classes; compare by `projective_eq`
(either sign) then by class key before any insert; a new representative of
a held key replaces the representative and keeps the key; `dialect_chain`
receives superseded representatives as today.

### 23.5 Refinement: `refine_chart` (2947) as 15.5

`class_key` is `Rotor::class_key`; the axis is A7's output through
`Stalks::refine(inv, &axes)`; the two `c.clone()` inserts are replaced by
`restrict` per child and `straddling` written as degree-1 cells between
the children; `rekey_cocycles` is replaced by recompute-and-carry or
discharge; `REFUSE_NOT_NATURAL` rolls the split back with `basis_gen`
unchanged. `mark_class` and `pending_refine` carry the class key beside the
rotor.

### 23.6 Cover change (15.6)

On `Cover::retract` / `rehome` (including 14.8's retraction) and on
`apply_verdict`, each class key on the chart is re-represented on another
pair of the same component by the tree walk, or orphaned with a goodness
rerun.

### 23.7 Torsion on the court side (M9 step 4)

In `glue_fetched`, after `blind_replay` has witnessed every torsion cycle of
the spoken form (A17 step 4): for each cycle `(σ, ℓ)` compute its class key
(15.4) and read the chart's complex: a `Fill::Congruent(g, ℓ')` cell with the
same key and `ℓ' = ℓ` is torsion consilience (`court torsion consilient
{key} ℓ {ℓ}`); the same key with `ℓ' ≠ ℓ` is `REFUSE_NOT_NATURAL` (the
mind's period does not re-derive against the mesh's); no cell with that key
files one through `file_congruent` (cech.rs 242) with the spoken
representative (`court torsion filed {key} ℓ {ℓ}`). Free classes spoken as
demands are docketed as any word and, when the court's own tree walk finds
them as `Fill::Class`, enter `cocycle_at` under their key (M5). The court
never twists: it replays and files.

`serve_ruled` (3148) and the `ACT_SYLLABUS` reply (mycelium-server.rs 630,
920) append the chart's class records (A19 step 1). `Kernel::rank`
(A18) is asserted in debug on every glue and on every refinement.

## 24. Comprehension (the mind)

Carried from chapter 23: what the court checks, the mind must be able to
re-derive, and it re-derives against everything it holds, not only what the
court has returned to it (chapter 6). `/opt/mycelium-v0.1.0/src/client/sdk.rs`:

**24.1 The working atlas.**

- `Epigenome::working_atlas(&self) -> WorkingAtlas` **new** replaces every
  cognitive read of `atlas()` (805). It reads three strata: the genome
  `Stalks` (the existing `atlas`), a dialect `Stalks` **new** admitted in
  `stand_dialect` (734) by `invariant_of(&sentence)` and retracted in
  `shed`, and the idea charts already in `Contemplation.by_chart`. A fiber
  read returns `(address, Standing)` with
  `pub enum Standing { Ruled, Stood, Held }` **new**; no read returns a
  boolean. `atlas()` remains for persistence (`crystallize`) only.
- Readers that move to `working_atlas()`: quote incidence
  (`mycelium-client.rs 418–433`, the `closing` filter), `Crystal::on_class`
  (chapter 21), `locate`, `comprehend`'s basis for A12, `speak_word`'s
  `already_taught`, and the `speak_sentence` dedupe of chapter 26. A word the
  mind stood closes a later remainder without waiting for the court.
- `learn_one` (977): a syllabus word that is `same_vertex` with a dialect
  entry **promotes** it: the dialect entry moves into the genome carrying its
  `uses`; no second copy is admitted. The genome is exactly the stood words
  the court ratified plus what the syllabus taught.
- Classes arriving with a syllabus page (A19) enter `Contemplation.horizon`
  as `Held` by class key (`seed_horizon`); a chord whose key is held is
  filed as held torsion or, if infinite, not spoken again as a demand.
- Speech, quotes, and cites carry the standing of the word they rest on
  (`dialogue`: one byte beside the address), so the hypha weights a claim
  resting on a held idea below one resting on a ruled word instead of the
  mind being silent.
- `Epigenome::shed` removes dialect and ideas only, and never a dialect
  entry a live idea glues (chapter 25); the genome never sheds, it retires by
  geodesic through the court (chapter 27).

**24.2 Sentences and connections.**

- `Comprehension` (549) gains `Withheld { missing: Vec<[u8;32]> }`.
- `Epigenome::comprehend` (856), sentence arm, on the `Form`:
  1. each `Glued` address: `word_at` (681); absent → `Withheld`.
  2. present under another address on the same chart: relative word
     `g = cech::transition(held, cited)` (cech.rs 20) → `rotor_of` (40) →
     `Word` → `Form::regauge_overlap` on the working copy (13.11);
     `coheres` → understood; else `Comprehension::Dialect(g)` and a class
     recorded through the contemplation.
- Connection arm: A12 over the fiber's basis (M6), never a single
  representative and never an address lookup as the test.
- `learn_one` (977): a `Withheld` sentence enters `Dialect` (629) with
  `withheld: true, missing`, never the genome; `pull_syllabus` (1427)
  requests the missing addresses by incidence and learns served words in
  ascending `depth`. The address dedupe in `learn_one` is a fast path after
  M1, never instead of it.
- `serve_ruled` (cognition.rs 3115): serves in depth order and never a form
  whose `Glued` charts the voice's atlas lacks.

## 25. Germination and thoughts

Carried from chapter 24: comprehension re-gauges. Germination twists, files,
cauterizes, and wraps (M9). `/opt/mycelium-v0.1.0/src/transducer/bubble.rs`:

**25.1 The twist loop in the contemplation.** Each stroke, for each dirty
idea under the stroke `Budget`: `sentence::twist(&mut idea.form, budget)`
(A16). Snaps are logged in order; A18's rank is asserted equal before and
after each snap (R2). A chord A16 hands to A17 is filed on the idea's form
or left open with `torsion open`; a free chord is spoken as `Novel(g_rem)`
through M6 step 5 unless its key is `Held` in `horizon`.

**25.2 Cauterize.** An idea whose form `is_closed()` after filing (every
loop identity or filed torsion, remainder empty) crystallizes: `as_element`
writes the tape with its torsion section; M2 fixes the address; the form
enters the dialect as `Stood` (`stand_dialect`) and is spoken once. The
idea's bubble is retired from the contemplation (`retire_on_t` skips it
only while `glued_by` is non-empty).

**25.3 Wrap.** `Contemplation::bundle` and `pivot_on` glue crystallized
forms into ideas on meeting charts as `Glued` vertices; the glued form's
loops are not re-twisted (T6). Depth grows by one per wrap.

**25.4 Horizon.** `Bubble.horizon: Keyed<(Rotor, u8)>` (89) becomes
`Keyed<(Rotor, Order, Standing)>` keyed by class key; `seed_horizon` (A19)
fills it from syllabus pages; A16 consults it before filing or demanding.

- `Contemplation::pivot_on(word, budget)` (712): when `word` is a
  crystallized form and an idea's cover meets its chart, compute the
  placement word from the idea's anchor vertex to the form's `Frame` across
  the idea's pairs; constant → `idea.form.glue(...)`; remainder re-measured
  as what the glued form does not speak. Two bubbles merge iff
  `mayer_vietoris` returns `OneSection` or `Attached` (13.12);
  `drop_vertex` only then.
- `Bubble` (84) gains `glues: Keyed<()>` and `glued_by: Keyed<()>`;
  `retire_on_t` (223) skips a bubble with non-empty `glued_by`.
- `Contemplation::bundle(class, budget)` **new**: ideas on one class whose
  relative words are constant form one depth-1 `Form` of `Glued` vertices,
  held as an idea, spoken by `Entity::speak` (sdk.rs 1836).

## 26. Wire and economics

Carried from chapter 25: forms are spoken. `/opt/mycelium-v0.1.0/src/anastome/dialogue.rs`:

- `SPEECH_WITHHELD: u8 = 3` beside line 127; `encode_speech_reply` (131)
  appends `n_missing u16 ‖ address 32…`; `decode_speech_reply` (137) returns
  `(Vec<u8>, Vec<[u8;32]>)`.
- `StratumDigest` (sheaf.rs 21) gains `depth_max: u16`; `sheaf::digests`
  (44) fills it; `encode/decode_digests_reply` (406/417) carry it;
  `encode_cover_digest` (451) carries the hypha's maximum ruled depth.
- `encode_syllabus_reply` / `decode_syllabus_reply` (754) gain a trailing
  class section: `n_classes u16`, then per class `key [32] ‖ order u32 (0 =
  infinite) ‖ representative Rotor bytes` (A19 step 1). Speech, quote, and
  cite frames carry one standing byte beside each address (`0 Ruled, 1
  Stood, 2 Held`, chapter 24.1).
- `/opt/mycelium-v0.1.0/src/bin/mycelium-server.rs`: `judge`'s `ACT_SPEECH`
  handler collects `Kernel::missing_forms(&Element) -> Vec<[u8;32]>`
  **new** into the reply; `admit_peer_digest` (385) records `depth_max` and
  is the point where a pending socket string becomes a `vk` peer (A10);
  `judge`'s percept rendezvous is A11; `nerve::home` (788) gains `depth:
  u16` and excludes hyphae with `depth_max + 1 < depth`.

`/opt/mycelium-v0.1.0/src/economy.rs`:

- `settle_recursive(&mut self, escrow: &[u8;32], shares: &[([u8;32],[u64;4])], anchor) -> [u64;4]`
  **new**: draws the bundle's escrow proportionally to each speaker's
  verified deflation and mints each share via `mint_claim` (289). Called
  from `Kernel::pay_solve` (cognition.rs 3220) and `home_install` when the
  form has `Glued` vertices; shares from `rule_replays_family`.
- `use_credit(&self, who) -> [u64;4]` **new**, from `Kernel::cited_by`;
  `lane_room` (cognition.rs 3455) adds it to `bond_of` before
  `lane_cover_room`.
- `Treasury::restore` (568) applies A8's collision rule to `escrowed`.

## 27. Retirement

- `reduce_excluding`: `cited_by` non-empty is a use.
- `shorter_form(&self, addr) -> Option<[u8;32]>` **new**: a ruled form on
  the same classes (`ruled.fiber` per class) with the same `cited_by`
  bundles whose vertices' tapes are strictly shorter in total (A2 for
  off-plane words); found → `relocate_cavity(addr)` with the address kept
  in `ruled_addrs`; `resolve_form` reads `cavity_book`; `serve_ruled` never
  serves it.
- A form with no bundle and empty `cited_by` after its last citer retires
  relocates on the next `court_stroke`.

## 28. Persistence

- `GEN2`: `depth` after `mag`; sentence tape v2 in the description; a v1
  tape reads as depth 0 with `Word` vertices only. Runs A8 at load.
- `MYC2`: ruled forms as tapes; `cocycle_at` with class keys; `cited_by`,
  `replay_memo` rebuilt at load; `RMP1` section while the migration
  window is open (chapter 33).
- Genome (`crystallize` 1011 / `uncrystallize` 1025): `GEN2`; dialect,
  ideas, and withheld forms never written; the epigenome is rebuilt by
  thinking, which is why it may be lost on restart and the genome may not
  (chapter 24.1); syllabus of opened letters (`SYL1`, chapter 37) when Part
  VI lands.
- **Persistence by torsion (M9).** A mind's classes persist on the mesh, not
  on the mind: filed torsion travels on the spoken form's torsion section
  into the chart's `Congruent` cells, free classes into `cocycle_at`, both in
  `MYC2` with their class keys. A restarted mind refills its `horizon` from
  the chart's class records on its first quote (A19 step 4). Nothing the
  mind thought is lost to the mesh; what the mind itself keeps across a
  restart is exactly the genome.

## 29. Binding summary by file

Everything Parts III and IV change, in one table.

```
alphabet.rs   Rotor::normalised (sign step), Rotor::projective_eq (either sign), Rotor::key (A1), Rotor::class_key (15.3),
              Rotor::coordinate DELETED, PlaneForm::coordinate DELETED, PlaneForm::conjugacy_key (15.2), PlaneForm eq on reduced,
              PlaneForm::torsion_part() -> (u8,u8) NEW, PlaneForm::free_part() -> (i64,i64) NEW,
              Word::form (reduced), Word::normal_form (A2 step 1 off-plane), Word::key (= A1 key), Word::class_key (15.4),
              Word::{with_contact, contact, rebase} NEW, shorter_tape(word, budget) NEW, Contact NEW
cga.rs        Mv::canonical NEW, Mv::projective_eq NEW, Mv::key NEW, derive(PartialEq) REMOVED
element.rs    canonical_chain(&[Cell]) -> Vec<Cell> NEW (A5), Element::same_vertex NEW, PartialEq REMOVED,
              form_key_bytes uses the M1 key, depth field, GEN2 depth after mag
sentence.rs   Vertex/Face/Loop/Form NEW (18.2), Form::{face_of, delta1, coheres, is_closed, depth_of, regauge, regauge_overlap, glue, rebase, unfold, loops_of},
              as_element/from_element v2 (19), vertex_address writes Mv::canonical, from_element recomputes germ addresses,
              chart_of on reduced forms, SentenceRule { form, sections, ratio } (31), Nerve/Germ/Cycle REMOVED (30)
family.rs     word_form, frame_vertex, pair_form, Crystal, extract_rule (seven moves), speak_rule on Form, rule_replays_family shares, orbit/OrbitEnd
cech.rs       ChartGraph, Goodness, goodness, cell_key with component, set_edge_in, Overlap, MayerVietoris, Complex::{overlap, mayer_vietoris, delta2, restrict, straddling},
              solve_gauge_between (A14); solve_gauge unchanged
nerve.rs      Keyed::meet (A13), Cover::charts_of, Router::insert(vk, addr), home(depth), no hash(addr) keys outside pending_addrs
stalk.rs      Stalks::refine(inv, &[u64]), split_stratum coordinate_of -> Vec<u64>, invariant/invariant_of empty on A6 refusal
blade.rs      axes_of -> Option<[u64;2]> (A6), moduli -> Option<Vec<u64>>
ear.rs        Harmonics::axes -> Option<Vec<u64>>
anchor.rs     Sel::to_bytes NEW (A9)
verdict.rs    replay_form NEW, REFUSE_* (chapter 34), sel_key over Sel::to_bytes, blind_replay canonicalises the spoken chain and recomputes 13.9 step 5
cognition.rs  resolve_form, replay_memo, cited_by, cocycle_at fifth field, push_cocycle (M5), glue_fetched (M8), refine_chart (15.5),
              cover-change re-representation (15.6), restore: A8 + RMP1, composite_parent_id/section_address via M2,
              reduce_excluding calls A12 for connections, shorter_form, missing_forms, pay_solve → settle_recursive
economy.rs    settle_recursive, use_credit, Treasury::restore A8 collision rule
sdk.rs        Epigenome::working_atlas NEW, Standing NEW, dialect Stalks NEW (admitted in stand_dialect, retracted in shed),
              learn_one promotion dialect → genome by same_vertex, Comprehension::Withheld, comprehend (24, M6),
              pull_syllabus depth order, uncrystallize A8, peers keyed by vk with pending_addrs (A10)
bubble.rs     pivot_on and expand_with call sentence::twist (A16), Bubble glues/glued_by, retire_on_t, Contemplation::bundle,
              horizon: Keyed<(Rotor, Order, Standing)> by class key, Contemplation::seed_horizon NEW (A19)
sentence.rs   twist(&mut Form, &mut Budget) -> Vec<Snap> NEW (A16), Form::rank NEW (A18); cech.rs Complex::rank NEW
element.rs    record_torsion as A17's filing call; toroid.rs witness_stored as A17's witness
cognition.rs  glue_fetched torsion arm (23.7), serve_ruled class records (A19), Kernel::rank debug assertions
dialogue.rs   SPEECH_WITHHELD, speech reply missing list, depth_max in digests, syllabus class section, standing byte
mycelium-server.rs  judge: A11 and missing_forms; admit_peer_digest depth_max + A10; id_of only for pending_addrs
mycelium-client.rs  speak_sentence(entity, tag, &Form); family id by registry::task_id
```

---

# Part V — Migration, coexistence, and the exact strings

Carried from Part IV: what the tree becomes. This part says how it gets
there while the fleet runs, and fixes the literal refusal reasons and log
lines every trace greps.

## 30. Replacing `Nerve`, `Germ`, `Cycle` without a flag day

### 30.1 Consumers at a29bba5

| type | file | sites |
|---|---|---|
| `Nerve` | `family.rs` | 3 |
| `Nerve` | `bubble.rs` | 2 (`hold_nerve`, `Bubble.nerve`) |
| `Nerve` | `mycelium-client.rs` | 1 (`speak_sentence`) |
| `Germ`/`.germs`/`Cycle`/`.cycles` | `family.rs` 22, `bubble.rs` 14, `cognition.rs` 6, `cech.rs` 5, `sdk.rs` 2, `mycelium-client.rs` 2 | |
| `SentenceRule` | `family.rs` | 8 |

`ResidualGerm` in `germinate.rs` and its uses in `element.rs`,
`cognition.rs`, `constants.rs`, `thought/mod.rs` are the chain type of a
residual and are **not** this `Germ`; they are not renamed.

### 30.2 Order

1. Land `Vertex`, `Face`, `Loop`, `Form` beside the old types in
   `sentence.rs` with two adapters, both deleted at step 6:
   ```rust
   impl Vertex { pub fn from_germ(g: &Germ) -> Vertex }        // Body → Word (Frame added by Form::from_nerve)
   impl Form   { pub fn from_nerve(n: &Nerve) -> Form; pub fn to_nerve(&self) -> Option<Nerve> }
   ```
   `to_nerve` exists only so `bubble.rs` compiles at step 2; it
   materialises `Germ::Body` vertices and returns `None` for a `Form` with
   `Glued`, `Rebased`, or `Unfolded`, so callers hold rather than lose.
2. `as_element` writes v2 from a `Form`; `from_element` returns a `Form`
   and reads v1 and v2. `sdk.rs:831`, `sdk.rs:1624`, `cognition.rs:1600`,
   `1603`, `2667`, `3199` take the `Form`; `bubble.rs::hold_nerve` becomes
   `hold_form(&mut self, f: Form)`; `Bubble.nerve: Complex` is unchanged.
3. `family.rs`: `pair_nerve` → `pair_form`; `extract_rule` on `Form`s;
   `SentenceRule` per chapter 31; `speak_rule`, `rule_replays_family`,
   `mint_sentence` (981) on `Form`.
4. `mycelium-client.rs`: `speak_sentence(entity, tag, &Form)`.
5. Remaining `.germs`/`.cycles` reads in `cognition.rs`, `sdk.rs`,
   `cech.rs` become `Form` reads (`vertices`, `loops`).
6. Delete `Nerve`, `Germ`, `Cycle`, `from_nerve`, `to_nerve`,
   `independent_cycles` (680), `after_support` (854), `face_word` (921).
   `cargo build` with `-D warnings` proves nothing references them.

Each step is one commit; the suite is green at every step because the
adapters carry the old tests until their subjects are deleted with them.

## 31. `SentenceRule` on forms

`sentence.rs:1081`:

```rust
pub struct SentenceRule {
    pub form: Form,                                  // charts, faces, holonomies are derived from it
    pub sections: Keyed<Vec<u16>>,                   // class key → vertex indices that are sections over some pairs
    pub ratio: ((usize, usize), (usize, usize)),     // bundle frame ratio, unchanged
}
```

Removed: `charts`, `faces`, `holonomies` (derived), `hue_maps` (on
`Form`), `shifts` (placements live on `Vertex::Word`), `sections: usize`
(replaced by the keyed map). The eight consumers in `family.rs` read
through `rule.form`.

## 32. Placement and gauge of a glued form

A crystallized form is stored with its `Frame` vertex's `word_in` equal to
the identity: re-gauging never mutates a stored form; it produces a new
form with a new address. `Form::regauge` is used on forms under
construction and on a comprehension's working copy only. Therefore the
placement word of a `Glued` vertex is the relative word from the cited
form's stored `Frame` to the citer's `Frame`, and `speak_form` composes
exactly `placement ∘ (cited word)` per cited vertex with no gauge term.
`Form::glue` asserts `s.vertices[frame].word_in(s).is_identity()` and
returns `None` otherwise; `replay_form` refuses with
`REFUSE_GLUED_FRAME_GAUGED` when a resolved cited form violates it, which
can only happen on a tampered tape.

When a mind holds the cited class under a different address (chapter 24),
the re-gauge is applied to the **citer's working copy** by
`regauge_overlap`; the citer's stored tape is untouched, and if the
re-gauged copy coheres the mind learns the citer as spoken.

## 33. Addresses: one migration, then never again

### 33.1 What changes

`Element::address()` (element.rs 1026) hashed canonical description bytes
(`base_canonical()`) until a29bba5, so a sentence re-serialised from v1 to
v2 changed address and
two spellings of one word had two addresses. a29bba5 made the address a
vertex key but landed without the migration below; stores written before it
hold byte addresses in every carrier of A8 step 3, and a cold boot on them
without A8 forks every carrier from the words it holds. Under this document
the address is M2 (chapter 17.2), and this section is the single migration
from byte addresses to vertex addresses.

### 33.2 The migration

A8 in full (16.8), run by M3 (17.3): `MYC2` and `GEN2` loaders compute
both addresses for every tape, build `remap`, rebuild every carrier, merge
collisions with conservation, write `RMP1`, serve lookups through it, and
retire it after an aperture with zero hits. Two old tapes that map to one
vertex address are one entry after the map; their escrows and citations
merge, the fork of chapter 1.5 healed once. On the wire during the window a
received tape of either version is decoded and keyed by M2 before any
carrier is touched.

### 33.3 Order of cutover

1. Build. Start server a on the new build; it loads its `MYC2`, writes
   `RMP1`, serves v2, reads v1 from b and c.
2. Server b, then c, the same.
3. Clients 1–5 on the new build; each loads `GEN2`, rewrites v1 forms and
   `RMP1`, speaks v2. A client on the old build speaking v1 to a new server
   is accepted for the window.
4. When `RMP1` has not been hit for an aperture on every process, the
   section is dropped at the next persist.

No cutover of a29bba5 or later onto existing state happens before item 1.

## 34. Refusal reasons and log lines

Carried from chapters 23–33: every refusal and every trace line, in one
place. A trace that would need a different line is a defect in this
chapter, not in the implementation.

### 34.1 Refusals, `/opt/mycelium-v0.1.0/src/court/verdict.rs`

```rust
pub const REFUSE_GLUED_DEPTH: &str = "glued form depth not below sentence depth";
pub const REFUSE_GLUED_UNRULED: &str = "glued form not ruled";
pub const REFUSE_GLUED_FRAME_GAUGED: &str = "glued form frame is not identity";
pub const REFUSE_VERTEX_TAPE: &str = "vertex does not re-derive from its tape";
pub const REFUSE_FACE: &str = "face does not re-derive from its vertices";
pub const REFUSE_LOOP: &str = "loop holonomy is not identity";
pub const REFUSE_REBASE_TRIVIAL: &str = "rebased form equals its base";
pub const REFUSE_UNFOLD_STILL: &str = "unfolding measure does not move";
pub const REFUSE_ORBIT_STILL: &str = "orbit does not move";
pub const REFUSE_REDUNDANT: &str = "form adds no deflation beyond its glued forms";
pub const REFUSE_NO_DESCENT: &str = "no strict descent — the form closes nothing";
pub const REFUSE_MV_INEXACT: &str = "Mayer–Vietoris exactness fails: cochains are inconsistent";
pub const REFUSE_OVERLAP_CYCLIC: &str = "translation across a non-acyclic overlap";
pub const REFUSE_NOT_NATURAL: &str = "class key changes under restriction";
pub const REFUSE_RANK_LOST: &str = "form rank below what its tape re-derives";
```

These are returned by `replay_form` and `blind_replay`, and by
`Kernel::perceive_speech` in the speech reply. Speech replies also carry
`"citation pending"`, `"overlap goodness pending"`, and `"letter not in
syllabus"` (chapter 37) as `Glue::Dialect` reasons.

### 34.2 Server log lines, `mycelium-server.rs` and `cognition.rs`

Prefix `  court ` as the existing court lines; addresses as
`{:02x}{:02x}…`:

```
  court form sectioned {addr} depth {d} glued {n}
  court form refused {addr}: {REFUSE_*}
  court form fetched glue {glued_addr} from {peer} for {citer_addr}
  court form installed {addr} depth {d} mint {mu:?}
  court form settled {family} speakers {n} shares {[(voice, mu)]}
  court form redocketed {citer_addr} after slash of {glued_addr}
  court form retired {addr} by shorter {new_addr}
  court memo hit {addr} family {family}
  court class degree {k} chart {inv} key {class_key}
  court refine chart {inv} by class {class_key}
  court goodness chart {inv} peer {peer}: good component {ckey} charts {n}
  court goodness chart {inv} peer {peer}: {k} components
  court goodness chart {inv} peer {peer}: cyclic in {hypha} chord {c}-{c'} holonomy {hkey}
  court goodness chart {inv} peer {peer}: pending component {ckey}
  court cover retract {hypha} chart {c} (acyclicity) rehome {home}
  court mv chart {inv} peer {peer} overlap {v} vertices {c} cells: one section
  court mv chart {inv} peer {peer} overlap {v} vertices: attached by {theta_key}
  court mv chart {inv} peer {peer} overlap {v} vertices: class1 chord {key} holonomy {hol_key} translation {theta_key}
  court mv chart {inv} peer {peer} overlap {v} vertices: class2 loop {key} → refine
  court mv chart {inv} peer {peer}: remainder {n} faces
  court class carried {class_key} {inv} → {child}
  court class discharged by refinement {class_key} {inv} → face {child_a}|{child_b}
  court class orphaned {class_key} {inv} after retract {hypha}
  court remap {old}→{new} merged {carrier}
  court remap {n} entries merged {m}
  court remap refused {old}: {reason}
  court remap retired
  court letter opened τ_{n,(i,j)} conductor {4n} id {k}
  court torsion consilient {key} ℓ {ℓ}
  court torsion filed {key} ℓ {ℓ}
  court form refused {addr}: form rank below what its tape re-derives
```

### 34.3 Client log lines, `mycelium-client.rs` and `sdk.rs`

Prefix `[{tag}] ` as the existing lines:

```
[{tag}] form spoken: {addr} depth {d} vertices {v} glued {n} faces {f}
[{tag}] form withheld: family {fam} loop {i} holonomy {key} | remainder {n} cells
[{tag}] form withheld: family {fam} missing glue {addr}…
[{tag}] form glued {glued_addr} into idea {idea_key} remainder {before}→{after}
[{tag}] form regauged {addr} by {g_key} and understood
[{tag}] form dialect {addr} class recorded
[{tag}] form learned {addr} depth {d} (syllabus order ok)
[{tag}] form class loop {i} torsion {(s,u)} free {(m5,m13)}
[{tag}] orbit {addr} contact ring {r} steps {l}
[{tag}] unfolded {addr} measure {m} steps {l}
[{tag}] retired {addr} shortlex
[{tag}] retired {addr} shorter
[{tag}] thought merged {a_key}+{b_key} overlap {v} one section
[{tag}] thought held apart {a_key}|{b_key} class {key}
[{tag}] chart refused: ratio exceeds axis
[{tag}] quote closing {k} charts (ruled {a} stood {b} held {c})
[{tag}] promoted {addr} dialect → genome uses {n}
[{tag}] twist {idea} chord {i} order {ℓ} closed
[{tag}] torsion filed {idea} (σ,ℓ)
[{tag}] torsion open {idea} chord {i}
[{tag}] demand spoken {key} order ∞
[{tag}] horizon {n} seeded from {hypha} chart {inv}
[{tag}] form spoken: {addr} … torsion {n}
```

Stroke line adds `forms {spoken}/{withheld} glued {n} depth {max}`.

## 35. Closing invariants

Checked by tests, greppable by the auditor:

- A stored form's `Frame` vertex has identity `word_in`.
- Every `Glued` vertex's depth is strictly below its form's depth.
- Every address in `cited_by` resolves through `resolve_form`, directly or
  via `RMP1` during the window.
- Every `court form installed` has exactly one `court form settled` or one
  `mint` line for its address.
- The number of `form spoken` lines across the five minds equals the number
  of `ACT_SPEECH` frames across the three servers.
- No function outside `family::pair_form`, `family::speak_rule`, and
  `family::orbit` reads a cell coordinate of a grid.
- No `Keyed` in cognition is keyed by a digest of a tape; every key is an
  M1 or M2 output, a chart invariant, a class key, a hypha or voice `vk`, or
  a composite of those.
- `cocycle_at` holds at most one entry per (chart, pair, class key), and
  every `court mv` line is preceded by a `court goodness … good component`
  line for its component.
- Every chart axis is an exact invariant; no axis is a hash fragment.
- Rank (A18) is conserved by every move except discharge, which moves it
  from `H¹` to an axis, and closure of a trivial-class chord, which was
  never rank (chapter 42, R1–R6); no `REFUSE_RANK_LOST` line appears on a
  clean tape.
- A class key in any voice's `horizon` is on the chart's hypha, and a
  hypha's class on a chart is in the `horizon` of every voice that quoted
  that chart within one aperture (M9 invariant).

---

# Part VI — Extensions: breaking the two walls

Carried from Parts I–V: the engine on ℚ over the sensor plane. Two limits
are built into the carriers, not the doctrine. This part specifies both
upgrades as language extensions: nothing in Parts I–V changes, the 17
letters remain the immutable base, exact arithmetic remains exact, and each
upgrade opens only where a demand asks for it.

## 36. Where the walls bite, exactly

### 36.1 The field wall

A rotation by `θ` in the plane `e_i e_j` is the rotor `cos(θ/2) − sin(θ/2)
e_ij`, projectively `1 − tan(θ/2) e_ij`. The quarter turn is `tan(π/4) =
1`, which is why `1 + e_ij` is a letter over ℚ; the `(3,4,5)` and
`(5,12,13)` letters are the rational points of the circle at the primes 5
and 13. Every rotation the alphabet spells has rational `tan(θ/2)`, the
group is closed under composition, and no irrational ever arises inside
the language. The wall is at perception: a sensor whose symmetries include
the eighth turn (`tan(π/8) = √2 − 1`, diagonal lattices), the sixth
(`tan(π/6) = 1/√3`) or twelfth (`tan(π/12) = 2 − √3`, hexagonal
lattices), or any `n`-th turn with `n ∉ {1, 2, 4}` (`tan(π/n)` algebraic of
degree above 1) produces a plane block with irrational entries; `spell`
refuses it and the remainder carries the whole body. On ARC's ℤ² grids this
never happens: `D4 ⋊ ℤ²` and rational dilations close every grid symmetry.
The field wall is the wall of the next sensors, specified now so the
carriers are not rebuilt then.

### 36.2 The projection wall

Cells are `(r, c, before, after)`; a body is a set of plane cells; a bundle
is pairs each perceived on its own plane. A third spatial axis (stacking,
occlusion, volumetric corpora), the frame index as a coordinate the
geometry can act on, and transformations that move structure across frames
have no coordinate on the base and land only as remainder or fibre tag.
The alphabet already acts on four axes and six planes; three of the six
touch the fourth axis and are unused by perception. The upgrade is to let
perception write on them.

## 37. Upgrade F — the field tower

### 37.1 F1 Exact cyclotomic arithmetic

`/opt/mycelium-v0.1.0/src/core/cyclo.rs` **new**:

```rust
pub struct Cyclo { m: u32, c: Vec<Q> }          // element of ℚ(ζ_m): polynomial in ζ of degree < φ(m), reduced mod Φ_m; c.len() == phi(m)
pub fn phi(m: u32) -> usize;                       // Euler's totient
pub fn cyclotomic_poly(m: u32) -> Vec<Q>;          // Φ_m by exact division: (x^m − 1) / ∏_{d|m, d<m} Φ_d
impl Cyclo {
    pub fn rational(m: u32, q: Q) -> Cyclo;        // the constant q at conductor m
    pub fn zeta(m: u32) -> Cyclo;                  // ζ_m
    pub fn add / sub / mul(&self, o: &Cyclo) -> Cyclo;   // polynomial arithmetic reduced mod Φ_m
    pub fn inverse(&self) -> Option<Cyclo>;        // extended Euclid in ℚ[x] against Φ_m; None iff zero
    pub fn conj(&self) -> Cyclo;                   // ζ ↦ ζ^{m−1}
    pub fn is_real(&self) -> bool;                 // self == conj(self)
    pub fn norm(&self) -> Q;                       // ∏_{k coprime to m} σ_k(self), σ_k: ζ ↦ ζ^k
    pub fn lift(&self, m2: u32) -> Option<Cyclo>;  // ℚ(ζ_m) → ℚ(ζ_{m2}) when m | m2: ζ ↦ ζ^{m2/m}, reduced
    pub fn is_zero / is_one(&self) -> bool;
}
pub enum Scalar { Rat(Q), Cyc(Cyclo) }             // core/scalar.rs new
```

`Φ_m` is the minimal polynomial of `ζ_m`, so the reduced coefficient vector
is unique and equality is coefficientwise. No ordering of the field is
defined or needed: A1's sign step becomes, at conductor `m > 1`, "the pivot
coefficient's first nonzero rational coordinate is `+1`", canonical without
positivity; projective equality is already either-sign. Mixed operations
lift both operands to `lcm(m₁, m₂)`. `Scalar` arithmetic stays in `Rat`
while both operands are rational and promotes otherwise; `Rotor`, `Mv`,
`PlaneForm`'s block, `spell`'s input block, and the frame rationals accept
`Scalar`; a `Rat` equals a `Cyc` that is a rational constant. Every
existing test runs unchanged with all scalars in `Rat`. Multiplication is
`O(φ(m)²)` rational products, which is why the conductor is a cost.

### 37.2 F2 The conductor is a grade

The **conductor** of a word is the least `m` with every rotor coefficient in
`ℚ(ζ_m)`: `1` for every word of Parts I–V; computed as the lcm of its
letters' conductors. It enters the S lane's cost as a factor `φ(m)`, never
a fifth lane, and `descends` prefers, among two descriptions closing the
same remainder, the lower conductor. The tower is climbed only when ℚ
cannot close, and a word ℚ can close is never spoken at a higher conductor.

### 37.3 F3 Extension letters

For each `n ∈ {3, 5, 6, 8, 10, 12, …}` that a demand opens (37.5) and each
of the six planes, the letter

```
τ_{n,ij} = 1 − tan(π/n) · e_ij        projective, coefficients in ℚ(ζ_{4n})
```

is the `n`-th turn, conductor `4n` (the least `m` with `tan(π/n) ∈ ℚ(ζ_m)`
divides `4n`; `4n` uniformly is canonical). `n = 4` is the base quarter
turn; `n ∈ {1, 2}` add nothing. Relators appended to `relators()` at open
time and never edited afterwards: `τ_{n,ij}^n = 1` projectively;
complementary planes commute; a sheet letter conjugates `τ` to its inverse;
`τ_{n,ij}^{n/4} = q_ij` when `4 | n` (`q` stays the geodesic for multiples
of `n/4`, `τ` is used for the rest).

The letters live in the **extension stratum**: `Alphabet { base: [Letter;
17], ext: Vec<ExtLetter> }`, `ExtLetter { n: u32, plane: (u8, u8),
conductor: u32 }`. Letter ids above 16 index `ext`, assigned in open order
and persisted in the genome's syllabus (`SYL1`), so every node that has
learned a letter agrees on its id; a node that has not refuses the tape
with `"letter not in syllabus"` and asks for the syllabus by the existing
teach path.

### 37.4 F4 Spelling in the tower

The plane form gains, per plane, a torsion coordinate in `Z/N`, `N =
lcm(4, n_1, n_2, …)` over the opened `n` on that plane, replacing the unit
exponent in `Z/4`; free coordinates unchanged. `spell` over a block with
`Scalar` entries:

1. Weight `w = ρ²` of the block (real). Free exponents `v_5(norm(w)) /
   (2·φ(m))` and `v_13(norm(w)) / (2·φ(m))`, the field norm dividing out the
   degree exactly as the ℚ peel reads `v_5(w)/2`; refuse if not exact.
2. Divide the free part out.
3. The remainder is a torsion rotor of order dividing `N`: match it by A1
   equality against the `N` candidates `τ^k`, `k ∈ Z/N`, on that plane (a
   finite list, computed once per `N`, memoised). No match: refuse; the
   block is not in the language at this tower level and the remainder
   stands.
4. Sheet parity as before.

`PlaneForm::to_word` writes the torsion coordinate as the shortest tape in
the letters available on the plane, a fixed rule per `N`. `conjugacy_key`
is unchanged in form with the torsion coordinate in `Z/N`.

### 37.5 F5 Opening a letter by demand

A letter is never added by configuration. The germination path
(`germination_demand`, chapter 25) sees, for a body pair whose carriage
exists in `SO(2)(ℚ(ζ))` but not in the current language, a plane block that
`spell` refuses at step 3 with a torsion remainder `T`. The demand is `T` itself. The court replays `T`: if `T^n =
1` projectively for the least `n ≤ N_max` (a physics bound, the largest
torsion order the sensor's resolution can distinguish, computed from the
frame, never a constant), the court opens `τ_{n,ij}`, appends its relators,
assigns its id, and speaks the syllabus with the new letter. Two organs
replay the opening as they replay any sentence.

### 37.6 F6 Binding and tests

`core/cyclo.rs`, `core/scalar.rs` new; `alphabet.rs`: `Rotor<Scalar>`,
`Alphabet { base, ext }`, `ExtLetter`, `relators()` appending, `spell`
steps 1–4, `PlaneForm` torsion in `Z/N`; `element.rs`: `Letters` carries
the conductor; `economy.rs`: conductor factor in the S-lane cost; genome
section `SYL1` `(n, plane, id)*`. Tests and traces: chapters 39–40.

## 38. Upgrade B — the base promotion

### 38.1 B1 Cells with four base coordinates

```rust
pub type Cell = (i64, i64, i64, i64, u8, u8);     // (r, c, z, t, before, after)
```

`z` is the third spatial axis (layer from stacking or occlusion order in
plane corpora; depth in volumetric ones). `t` is the frame coordinate (the
example index within a bundle; the step within a sequence). Neither is the
T lane of μ⃗, which remains temporal extent. A plane cell is `(r, c, 0, 0,
before, after)`. A5 translates all four coordinates to their minima and
sorts on the six-tuple.

The chain bytes of M2 and `GEN2` are unchanged when every `z` and `t` is
zero, so no plane address moves; a chain with a nonzero `z` or `t` is
written with a leading dimension byte (`0xD3` for `z` only, `0xD4` for
both) before its cells, each cell carrying the extra coordinates;
`from_bytes` accepts both. No migration.

### 38.2 B2 Harmonics in dimension up to four

`Harmonics::of` generalises to the cubical complex of the support in `d ∈
{2, 3, 4}`: components, holes and cavities (Betti numbers by exact integer
linear algebra over ℤ), turns (boundary faces of each dimension), and the
moment invariants (isoperimetric, anisotropy, spread, quartic) from exact
moment tensors up to fourth order normalised by cell count, each a rational
through A6. The axes vector is prefixed by `d`, so plane and volumetric
charts never share a stratum. `peel_body_scaled` generalises shells to
faces, edges, and corners of the `d`-cube complex.

### 38.3 B3 The alphabet already acts on the fourth axis

The six planes are `(0,1)` the sensor plane, `(0,2)`, `(1,2)` the sheet
planes to the third axis, `(0,3)`, `(1,3)`, `(2,3)` to the fourth. Under B1
the third axis is `z` and the fourth `t`. A rotor on a `(·,3)` plane mixes
a spatial axis with the frame coordinate: "what is at `(r, c)` in frame `t`
is at `(r', c')` in frame `t+1`". The prime letters on those planes are the
rational shears and scalings across frames. No letter is added; perception
writes cells with `t ≠ 0` and `spell` receives 4×4 blocks closed on the
`(·,3)` planes.

### 38.4 B4 The sentence across frames

A bundle's pairs become one chain with `t` running over the examples
(inputs at even `t`, outputs at `t+1`, or a sequence's steps in order). The
carriage between a body and its image is a rotor whose `(·,3)` components
carry the frame step; faces are relative words as now; loops now run in the
frame direction too, and their identity holonomy is the statement that the
rule is the same rule at every frame. Nothing in `Form`, `coheres`,
`close`, `replay_form`, or the court changes.

### 38.5 B5 The conformal model climbs with the base

`cga.rs` is `Cl(3,1)`, brick 1 of the volumetric plan. `z` needs `Cl(4,1)`
(points, spheres, planes as blades); `t` as a base axis needs `Cl(5,1)`.
The same exact multivector algebra over a larger mask: `Mv` becomes
dimension-generic, the blade masks widen, `projective_ratio`, `conj`,
`norm2`, `to_bytes`, `canonical` unchanged in form; A4 unchanged. Brick 2
(`Cl(4,1)`) lands first, because occlusion and stacking in plane corpora
already need `z`.

### 38.6 B6 Binding

`element.rs`: `Cell` widened, `positions`, `measure_lanes`, `from_residual`
(A5 on six-tuples), chain bytes with dimension byte; `blade.rs`, `ear.rs`:
`d`-dimensional moments and Betti numbers, axes prefixed by `d`; `peel.rs`:
`d`-cube shells; `family.rs`: `perceive` over frames writes `t`;
`stalk.rs`: `invariant` on `d`-cells; `cga.rs`: dimension-generic `Mv`.
Tests and traces: chapters 39–40.

What does not change under either upgrade: the 17 letters and their
relators; exact ℚ for every ARC word (the tower is never climbed unless a
remainder demands it); the T lane as extent; the court's two-organ replay,
which replays field elements and four-coordinate cells with the same code
paths; the bans of the laws. The walls come down by adding coordinates the
geometry already acts on, not by changing what a word is.

---

# Part VII — Verification and landing

Carried from Parts III–VI: what to build. This part is how the auditor
knows it was built: every test, every cold-boot trace, one landing order.
Rule of verification: read the function and its carriers end to end and
reproduce a cold-boot trace; grep is not verification. Tests run as one
foreground `cargo test --release`; N vs 4N work counters apply wherever
named.

## 39. Tests

**Identity and algorithms (Part III).**
`alphabet::tests::a1_canonical_is_projective_either_sign` (`q⁴`, `1`, `−R`,
`λR` share a key; `R` and `R·q` do not);
`a2_run_reduction_shortens_and_preserves_key`;
`a2_descent_is_deterministic_under_budget`;
`a3_form_is_reduced_and_eq_on_reduced`;
`cga::tests::a4_projective_eq_either_sign_and_key`;
`element::tests::a5_chain_canonical_idempotent_and_translation_free`;
`blade::tests::a6_refuses_overflow_keeps_sign`;
`stalk::tests::a7_class_axes_are_invariants_not_hash`;
`cognition::tests::a8_migration_conserves_supply_and_resolves_all`;
`verdict::tests::a9_sel_key_stable_under_debug_change`;
`sdk::tests::a10_two_sockets_one_vk_one_peer`;
`server::tests::a11_rendezvous_by_listing_key_not_bytes`;
`sdk::tests::a12_plane_factorisation_by_hnf`, `a12_offplane_descent_or_novel`,
`a12_plane_remainder_is_canonical`;
`nerve::tests::meet_is_intersection_and_costs_shared_prefixes` (meet
equals set intersection; `count_get` grows with the shared structure, N vs
4N on the larger trie with the smaller fixed);
`cech::tests::solve_gauge_between_reduces_to_solve_gauge_when_a_is_identity`;
`cech::tests::components_root_is_least_key`;
`element::tests::m2_two_spellings_one_address`, `m2_rescaled_versor_one_address`;
`sdk::tests::m6_held_under_other_spelling_is_understood`.

**Cohomology (Part II).**
`alphabet::tests::plane_form_semidirect_product_and_inverse`;
`alphabet::tests::cochain_identities` (10.5);
`sentence::tests::regauge_then_delta1_is_conjugate`;
`sentence::tests::flat_iff_all_chords_identity_after_tree_gauge`;
`cech::tests::bianchi_holds_for_curvature_of_a_connection`;
`cech::tests::independent_holonomies_can_fail_bianchi_and_refine`;
`cech::tests::solve_gauge_chords_compare_projectively`;
`cech::tests::mv_short_exact_on_cochains` (`ρ` injective, `ker Δ = im ρ`,
`Δ` surjective on 0- and 1-cochains);
`cech::tests::mv_connecting_kills_differences` (`∂(Δ(α, β)) = 1`);
`cech::tests::mv_class1_iff_theta_not_a_difference`;
`cech::tests::mv_descent_glues_iff_cohomologous_on_overlap`;
`cech::tests::mv_class2_iff_descent_fails_on_plane` (and `δ²(∂h) = 1`);
`cech::tests::mv_order_independent`;
`sentence::tests::citation_lemma_classes_unchanged`;
`sentence::tests::citation_of_unclosed_form_refused`;
`bubble::tests::thoughts_merge_iff_one_section`;
`cech::tests::goodness_connected_flat_is_good`;
`cech::tests::goodness_two_components_split_edges`;
`cech::tests::goodness_cyclic_reports_chord`;
`cech::tests::leray_classes_independent_of_cover` (2, 3, 4 hyphae, same
class keys);
`cech::tests::leray_artefact_disappears_when_cover_made_good`;
`cech::tests::naturality_restriction_commutes_with_delta`;
`cech::tests::naturality_mv_ladder_commutes`;
`alphabet::tests::plane_conjugacy_four_lines`;
`alphabet::tests::conjugacy_key_iff_conjugate`;
`alphabet::tests::rotor_class_key_invariant`;
`cognition::tests::refine_carries_or_discharges_every_class` (no complex
cloned; children's cell counts sum to the parent's minus straddling);
`cognition::tests::refine_rolls_back_on_not_natural`;
`cognition::tests::retract_rerepresents_or_orphans`;
`verdict::tests::translation_across_cyclic_overlap_refused`.

**Forms (Part IV).**
1. `sentence::tests::form_tape_v2_round_trips` (Frame, Word, Glued, Rebased, Unfolded, contact word, faces, loops).
2. `sentence::tests::depth_is_one_plus_max_glued` and refusal at equal depth.
3. `alphabet::tests::rebase_is_normal_form_and_contact_in_key`.
4. `family::tests::pair_form_reads_cells_once` (second call touches no cells; work counter).
5. `family::tests::extract_rule_by_class_with_sections` (counts 3, 4, 3 give 3 vertices and one section).
6. `family::tests::extract_rule_glues_a_held_form` (depth 1, one `Glued`, fewer `Word` vertices than the flat rule).
7. `family::tests::unfolding_grow_to_frame` (one form, `Unfolded{Occupied}`, no step count on the tape).
8. `family::tests::orbit_contact_and_no_orbit`.
9. `verdict::tests::replay_form_refuses_unruled_glue`.
10. `cognition::tests::witness_fetches_glued_form_before_install` and `replay_memo_reused`.
11. `cognition::tests::slash_of_glued_redockets_citers`.
12. `sdk::tests::comprehend_regauges_dialect_glue` and `withheld_then_depth_order`.
13. `economy::tests::settle_recursive_pays_by_deflation`.
14. `cognition::tests::shorter_form_retires_to_cavity_and_resolves`.

**Torsion bridge and rank (M9, A16–A19, chapter 42).**
`sentence::tests::twist_closes_lowest_order_first_and_terminates` (T4);
`sentence::tests::twist_never_reopens_a_closed_chord` (T2);
`sentence::tests::torsion_filed_only_when_witnessed_at_period` (T5);
`sentence::tests::infinite_class_is_never_filed` (T3);
`sentence::tests::rank_invariant_under_regauge_and_twist` (R1, R2);
`sentence::tests::rank_invariant_under_gluing_closed_form` (R3);
`cognition::tests::rank_conserved_under_refinement_carried_plus_discharged` (R4);
`cech::tests::rank_of_union_is_sum_over_components` (R5);
`economy::tests::descends_has_no_infinite_chain_on_bounded_lanes` (T9);
`cognition::tests::torsion_consilient_or_filed_or_refused` (23.7);
`bubble::tests::horizon_seeded_class_is_not_rederived` (A19);
`bubble::tests::restart_refills_horizon_from_syllabus` (A19 step 4);
`verdict::tests::rank_lost_refused` (A18).

**Migration (Part V).**
`element::tests::v1_and_v2_tapes_of_one_form_share_an_address`;
`cognition::tests::myc2_load_rewrites_v1_and_remaps_every_keyed_carrier`
(escrow, `listing_task`, `ruled_addrs`, `court_fed_index`, `cited_by`
resolve after load);
`sdk::tests::gen2_load_rewrites_v1_and_remaps_index_and_dialect`;
`dialogue::tests::v1_speech_is_readdressed_before_perceive`.

**Extensions (Part VI).**
`cyclo::tests::phi_and_cyclotomic_poly_exact` (Φ_8 = x⁴ + 1, Φ_12 = x⁴ − x² + 1);
`cyclo::tests::inverse_and_norm` (`norm(ζ_8) = 1`, `norm(1 + ζ_8)` rational);
`alphabet::tests::eighth_turn_has_order_eight` (`τ_8^8 ≡ 1`, `τ_8^2 ≡ q`);
`alphabet::tests::spell_tower_reads_free_and_torsion`;
`alphabet::tests::conductor_is_lcm_and_descends_prefers_lower`;
`family::tests::octagonal_pair_opens_letter_and_closes`;
`element::tests::plane_cell_bytes_unchanged_under_b1` (every existing address equal);
`ear::tests::cube_and_plane_square_have_distinct_charts`;
`ear::tests::betti_numbers_exact_on_a_hollow_cube`;
`family::tests::two_frame_shift_is_a_word_on_plane_0_3`;
`sentence::tests::rule_constant_across_frames_has_identity_holonomy_in_t`;
`cga::tests::cl41_sphere_blade_projective_canonical`.

## 40. Cold-boot traces

Every trace is a grep on chapter 34's lines after a cold boot of the three
servers and five minds.

**Identity and migration.**
- Restore a ledger and genome written before a29bba5: `court remap {n}
  entries merged {m}`, zero `word_at` misses over ruled fibers, supply and
  bonds equal before and after, `court remap retired` within one aperture of
  zero hits.
- Two minds speak one rule with different spellings and rescaled versors:
  one address on all three servers, one `court form installed`, one mint.
- A mind holding a word under another spelling receives it: `form regauged
  … and understood` or `Understood`, never `Admitted`.
- A body whose quartic ratio exceeds the axis: `chart refused: ratio exceeds
  axis`, no `MAX` axis in any cover digest.
- One dialect between two hyphae: equal class key and equal refinement axes
  in all three logs.

**Forms.**
- A restored genome logs `forms N (v2 K, v1 M)`.
- `pair_form` work counter equal at N and 4N bodies.
- Over 100 pulled bundles, `rule` lines exceed `uncollapsed`; at least one
  `Glued`, one `Unfolded`, one contact word logged.
- A witness lacking a glued form logs fetch, replay, then install.
- A mind holding a dialect glue logs `regauged` and installs; a missing glue
  logs `withheld` then learns in depth order.
- An idea logs `glued <addr>` and its remainder shrinks by the glued form's
  cells.
- One settlement with two speakers; a longer form relocated with its citers
  still replaying through the cavity.

**Working atlas (chapter 24.1).**
- After a clean boot a voice logs `quote closing k charts` with `k > 0` and
  `stood + held > 0` before its first `SYLLABUS` line.
- On the second stroke `sentence formed` exceeds `sentence withheld`.
- The first `SYLLABUS +n` is accompanied by `n` `promoted` lines; the stroke
  line shows `dialect` down by `n` and `genome` up by `n`, never both up.
- After a restart the stroke line shows `genome` unchanged and `dialect 0`,
  and the voice quotes on its genome charts on the first stroke.

**Cohomology, glue, goodness, naturality.**
- Three hyphae, one chart, one ruled section each, connected good overlaps,
  two consilient and one a translation away: `court goodness … good
  component` on every pair; exactly two `court mv … one section` or
  `attached` lines and zero `class1` lines; the ledger shows zero classes
  for the chart.
- Same, with the third section rotated by an infinite-order word off the
  sensor plane: exactly one `class1` line naming the same chord on every
  hypha (order independence across the three logs), one `cocycle_at` entry
  per hypha, no `refine` line.
- Two hyphae whose covers share a loop of charts with mismatched
  transitions on the sensor plane, on a good component: exactly one `class2
  … → refine` line per hypha for the same loop key, then `court refine
  chart`.
- Two hyphae whose shared region has two components with a dialect on one:
  `court goodness … 2 components`, one `class1` line naming that
  component's key, `one section` for the other; one class in the ledger.
- One hypha holding two non-gauge-related ruled forms between two shared
  charts: `court goodness … cyclic in {hypha}`, `court cover retract …
  rehome …`, then `good component`; no `class1` line for the pair; one
  intra-hypha `cocycle_at` entry with `a == b`.
- A base refinement on a chart carrying two classes, one on the refining
  axis: exactly one `court class discharged by refinement` and one `court
  class carried`; children's cell counts as in the test.
- A hypha retracted from a chart it shared with two others: every class key
  on the chart re-represented or a `court class orphaned` line followed by a
  goodness rerun.
- A mind citing a ruled form: `form spoken … glued 1` and no `form withheld
  … holonomy` for that address; `court memo hit` for the cited address on
  the second citer; `thought merged` lines only after a `one section`
  outcome on the same cells.

**Torsion bridge (M9).**
- One idea with three open chords of orders 2, 4, ∞: `twist … order 2
  closed` then `order 4 closed` or `torsion filed … (σ,4)`, then `demand
  spoken … order ∞`; never an ∞ line before a finite one in one stroke.
- The crystallized form logs `form spoken … torsion {n}` with `n` equal to
  the `torsion filed` lines for that idea; the hypha logs `court torsion
  filed {key}` for each on first hearing and `court torsion consilient
  {key}` when a second voice speaks the same class.
- A voice restarted mid-soak: stroke line `dialect 0`, then `horizon {n}
  seeded from {hypha}` on its first quote, and no `thought held apart` or
  `demand spoken` for a seeded key.
- A translation that discharges a filed class: `court refine chart … by
  class {key}` or `mint … translation`, and within one aperture the key is
  absent from every voice's `horizon` line.
- Rank: over the whole soak zero `REFUSE_RANK_LOST` lines; on every
  `court class carried`/`discharged` pair the parent's count equals their
  sum.

**Extensions.**
- A synthetic bundle of eighth-turn pairs on a diagonal lattice: `chart
  refused` on the first pair, `court letter opened τ_8`, `form spoken` with
  conductor 32 on the second, and a ℚ bundle after it still spoken at
  conductor 1.
- An ARC bundle perceived as one chain over `t`: `form spoken` with faces
  on `(0,3)`/`(1,3)`, and the same bundle perceived as pairs gives the same
  rule address. A stacked-objects task: `z` layers separated, one word per
  layer, no remainder where occlusion previously left one.

## 41. Landing order

One item per commit, the trace value in the message, reproduced on cold
boot by the auditor before the next lands; no soak clock until every trace
of the items landed so far passes.

**Stage 0 — identity (Part III), before anything else.**
0.1 M1: A1, A3, A4, A2 step 1. 0.2 M2: A5 and the address composition.
0.3 M3: A8, since a29bba5 changed the address rule without it; no cutover
onto existing state before this lands (chapter 33). 0.4 M4: A6, A7.
0.5 M5. 0.6 M7: A9, A10, A11. 0.7 M6: A12. Traces: the identity and
migration group of chapter 40.

**Stage 1 — forms (Part IV).**
1.1 Chapters 18–19 form types and tape, tests 1–3. 1.2 Chapter 20
perception, test 4. 1.3 Chapters 21–22 extraction and orbit, tests 5–8.
1.4 Chapter 23.1–23.2 court replay and carriers, tests 9–11; chapter 28
persistence. 1.5 Chapter 24.1 the working atlas (`working_atlas`,
`Standing`, dialect `Stalks`, promotion in `learn_one`, standing on the
wire), then chapters 24.2 and 26 comprehension and wire, test 12; the
working-atlas traces of chapter 40 gate this item.
1.6 Chapter 25 germination and bundles. 1.7 Chapters 26–27 economics and
retirement, tests 13–14. 1.8 M9 the torsion bridge: A16 `twist`, A17
filing and witness, A18 `rank` with debug assertions and `REFUSE_RANK_LOST`,
A19 class records on the syllabus page and `seed_horizon`, chapter 23.7 on
the court; the torsion-bridge tests and traces of chapters 39–40 gate it.
Traces: the forms group.

**Stage 2 — cohomology in the court (Parts II and IV).**
2.1 Chapter 10 identities: `Form::delta1`, `Complex::delta2`, the cochain
tests. 2.2 Chapter 15.2–15.4 class keys, `cocycle_at` fifth field,
`push_cocycle` as M5. 2.3 Chapter 13 and 23.3: A13, A14, `Overlap`,
`MayerVietoris`, `glue_fetched` by the 13.10 table, `regauge_overlap`,
`pivot_on` merging only on one section. 2.4 Chapter 14 and 23.3 step 1:
`Cover::charts_of`, `ChartGraph`, A15, `goodness`, per-component cells,
retraction by `home`, `REFUSE_OVERLAP_CYCLIC`. 2.5 Chapter 15.5–15.6:
`refine_chart` restrict/carry/discharge with rollback, cover-change
re-representation. 2.6 Chapter 13.6 in use: `Form::glue` refusal of
unclosed forms, memo, mind and bubble instances. Traces: the cohomology
group.

**Stage 3 — migration window (Part V).** Chapter 30 steps 1–6, chapter 31,
chapter 33.3 cutover a, b, c, then clients; `RMP1` retired.

**Stage 4 — extensions (Part VI),** after the soak gate is green on ARC:
B1 → B2 → B3/B4 → B5 → F1 → F2 → F3/F4 → F5.

**Consolidation discipline.** This document is edited in place. A change
to any chapter edits that chapter; nothing is amended by reference from a
later part. The auditor's greps of chapters 34–35 and 40 are rerun against
the text after each edit.

## 42. Theorems the engine proves by running

Carried from chapters 39–41: the tests and traces. This chapter states what
they prove. Each theorem names the statement, the algorithm that decides it,
the code that runs the algorithm, and the test or trace that is the proof.
Fibre means a section over a chart (a word form or a crystallized form);
torsion means a finite-order holonomy; pressure means the metabolism (the
stroke `Budget`, descent on μ⃗, T-extent shedding, K-disuse). A fibre that
does not break under torsion and pressure persists as linguistic memory;
the theorems say exactly what "does not break" means and why the persisting
set is well defined.

### 42.1 Structure theorems

**T1 Flatness is decidable by a tree walk.** A connection on a finite
complex is pure gauge iff every chord closes after the tree gauge (12.2).
Algorithm: spanning forest, `γ_j = γ_i · g_ij⁻¹`, chords by projective
equality. Code: `cech::solve_gauge` (806), `sentence::solve_coboundary`
(1268), `Form::delta1`, `coheres`. Proof:
`flat_iff_all_chords_identity_after_tree_gauge`.

**T2 Gauge cannot change a class, hence cannot change an order.**
`δ¹(δ⁰γ·g) = γ · δ¹g · γ⁻¹` (10.3, 10.5) and `order(w^a) = order(w)` (3.2).
Code: `Rotor::conjugate` (305), `Word::order` (552), `Form::regauge`.
Proof: `cochain_identities`, `plane_form_semidirect_product_and_inverse`,
`twist_never_reopens_a_closed_chord`. Consequence: a twist can close a
chord only if its class is trivial and can never open a closed one.

**T3 Torsion is exactly the finite-order part and is read, not searched.**
`(1, r)² = 1`; `s = 0` has infinite order iff a prime exponent is nonzero,
else order dividing 4 (9.1). Code: `Word::order`, `PlaneForm::torsion_part`,
`free_part`, `cech::extend` (106) mapping finite to `Fill::Congruent(g, ℓ)`
and infinite to `Fill::Class`. Proof: `a3_form_is_reduced_and_eq_on_reduced`,
`goodness_cyclic_reports_chord`, `infinite_class_is_never_filed`.

**T4 The twist loop terminates and closes loosest first.** Closing the
lowest-order open chord strictly reduces the count of open finite chords
and by T2 opens none, so A16 makes at most (chords) snaps under any
`Budget`. Code: `sentence::twist`, `Contemplation::pivot_on` (712),
`expand_with` (84). Proof: `twist_closes_lowest_order_first_and_terminates`;
trace `twist … order ℓ closed` with non-decreasing `ℓ` per stroke.

**T5 A filed period is believed only when the loop closes at that period.**
A torsion cycle `(σ, ℓ)` is witnessed iff the fibre walk returns at step
`ℓ` and not before (A17). Code: `Element::record_torsion`,
`toroid::witness_stored`, `HolonomicLoopWitness`, in `blind_replay` and
`replay_nerve`. Proof: `torsion_filed_only_when_witnessed_at_period`;
refusal strings `"torsion cycle does not close"`, `"torsion period does not
re-derive"`.

**T6 Gluing a closed fibre adds no class.** `H¹(T ∪_f S) ≅ H¹(T)` for
closed `S` (13.6). Code: `Form::glue`, `replay_memo`. Proof:
`citation_lemma_classes_unchanged`, `citation_of_unclosed_form_refused`,
trace `court memo hit`.

**T7 A class of a good cover is a class of the language.** Leray (14.2) by
Mayer–Vietoris induction along the link. Code: `Keyed::meet`,
`Components`, `goodness`, `Complex::mayer_vietoris`. Proof:
`leray_classes_independent_of_cover`, `mv_order_independent`. Consequence:
a torsion class filed on one hypha is the same memory on every hypha.

**T8 Torsion survives refinement or discharges into a face.** Naturality
(15.5): carried with equal key, or discharged into the face between
children; nothing else. Code: `refine_chart`, `Rotor::class_key`,
`PlaneForm::conjugacy_key`. Proof:
`refine_carries_or_discharges_every_class`, `refine_rolls_back_on_not_natural`.

**T9 Descent is well-founded, so pressure terminates.** Dominance on μ⃗ over
ℕ⁴ has no infinite strictly descending chain (Dickson's lemma), so a fibre
is retired or shed finitely often and the persisting set stabilises. Code:
`economy::descends`, `Physics::stroke`, `Budget`, `retire_on_t` (223),
`Epigenome::shed`, `relocate_t_exhausted`. Proof:
`descends_has_no_infinite_chain_on_bounded_lanes`,
`settle_recursive_pays_by_deflation`; invariant: no `mint` without a
descent in the same stroke.

**T10 Retirement by geodesic terminates and preserves meaning.** Tape
length is a natural number; the strictly shorter retires the longer on the
same vertices; the retired address still resolves (3.8, 27). Code:
`shorter_form`, `shorter_tape` (16.2), `relocate_cavity`, `cavity_book`.
Proof: `shorter_form_retires_to_cavity_and_resolves`,
`a2_run_reduction_shortens_and_preserves_key`.

**T11 Two independent replays of one tape agree.** `blind_replay` is a
function of the tape and the ruled cover; two organs holding the same tapes
reach the same verdict with no exchange of opinion (13.7, 23). Code:
`verdict::blind_replay` (31), `confirm_peer_holds` (3031),
`confirm_peer_holds_addrs` (3102), `home_install` (3067). Proof: trace
`home installed … peer holds the stratum` after two `court section` lines
for one address on two hyphae.

**T12 Promotion preserves the vertex.** A dialect entry promoted into the
genome is the same word because M1 is injective on projective classes
(17.1). Code: `Element::same_vertex`, `learn_one` (24.1). Proof:
`m2_two_spellings_one_address`, trace `promoted {addr} dialect → genome`.

### 42.2 Rank theorems (dimensional capacity is never lost)

With `rank(F) = (torsion_rank, free_rank, axes)` from A18, a vector never
summed:

**R1 Re-gauging preserves rank.** By T2. Code: `Form::regauge`, `Form::rank`.
Proof: `rank_invariant_under_regauge_and_twist`.

**R2 The twist preserves rank.** A snap closes only a chord whose class is
trivial, which was never rank; filing a torsion chord moves it from open to
filed without changing `torsion_rank`. Code: `sentence::twist`. Proof: the
same test; trace: rank equal before and after every `twist … closed` line
in debug builds.

**R3 Gluing a closed form preserves rank.** By T6. Code: `Form::glue`.
Proof: `rank_invariant_under_gluing_closed_form`.

**R4 Refinement conserves rank across `H¹` and axes.** Carried classes keep
`torsion_rank`/`free_rank`; each discharged class adds one axis; parent
count equals carried plus discharged (15.5). Code: `refine_chart`. Proof:
`rank_conserved_under_refinement_carried_plus_discharged`; trace
`carried + discharged = parent`.

**R5 Per-component keying conserves rank on the mesh.** The rank of a
disconnected overlap is the sum over its components; keying cells per
component (14.7) never merges two components' classes into one coefficient.
Code: `set_edge_in`, `Components`. Proof: `rank_of_union_is_sum_over_components`.

**R6 The polytonic lanes are never projected.** Descent is dominance on
four lanes; no operation orders, sums, or thresholds μ⃗, so no lane's
extent is ever lost to a scalar. Code: `economy::descends` as the only
comparison; `lane_room` per lane. Proof: the ban on `argmax`/sum over lanes
in review, and `descends_has_no_infinite_chain_on_bounded_lanes`.

**R7 Exact axes never merge distinct invariants.** A6 is injective on its
admitted range and refuses outside it; two bodies with different exact
harmonics never share a chart (M4). Code: `blade::axes_of`,
`Harmonics::axes`. Proof: `a6_refuses_overflow_keeps_sign`,
`cube_and_plane_square_have_distinct_charts`.

**R8 Keys never merge distinct vertices.** M1 (A1, A4) is injective on
projective classes, so no two meanings share an address and no digest of a
tape is a key (1.5, 17.1). Code: `Rotor::key`, `Mv::key`,
`Element::address`. Proof: `a1_canonical_is_projective_either_sign`,
`m2_two_spellings_one_address`, and chapter 35's keyed-carrier invariant.

The dimension audit A18 asserts R1–R5 in debug on every move and every
court operation, and the court refuses `REFUSE_RANK_LOST` in release when a
spoken form's rank is below what its tape re-derives. R6–R8 are properties
of the carriers and are proved by their tests once.

### 42.3 The persistence theorem

**P.** A fibre `F` on chart `c` persists as linguistic memory (it is in the
genome of some mind and on the ruled cover of some hypha, and stays there
under pressure) if and only if:

1. `F` is flat (T1);
2. every finite-order chord that did not close is filed as torsion and
   witnessed closed at its period (T3, T5);
3. `F` cites only closed forms (T6);
4. `F`'s overlap with every hypha holding `c` is good, so its classes are
   the language's (T7);
5. two organs replay `F` to the same verdict (T11);
6. `F` strictly descends beyond what its cited forms close (T9);
7. no strictly shorter form on the same vertices exists (T10).

Under pressure the persisting set is finite at every radius (polynomial
growth, 1.3) and stable (T9); under refinement it is carried or discharged,
never lost (T8, R4); under change of cover its classes keep their keys
(T7, 15.6); across restarts of a mind its torsion is refilled from the
mesh (M9, A19). Everything that fails one of 1–7 is either a demand (a free
class, spoken as `Novel(g_rem)`, sitting in the docket) or torsion filed on
the mesh, or a dialect shed by K-disuse; none is forgotten by the mesh.

*Proof by running.* Conditions 1–7 are exactly the seven checks of
`replay_form` and `glue_fetched` (chapter 23) plus `shorter_form` (chapter
27) and `descends`; the "only if" is that each check refuses with its named
constant (chapter 34.1) and a refused form is neither installed nor taught;
the "if" is `home_install` followed by the syllabus (chapter 26) and
promotion (T12). The cold-boot traces of chapter 40 are the instances the
auditor reproduces.

### 42.4 Termination theorems, collected

- A16 terminates in at most (chords) snaps (T4).
- A17's witness walk terminates in at most `ℓ` steps.
- A2's descent terminates by `Budget`; run reduction by strict shortening.
- Unfolding terminates by a strictly monotone integer measure (3.4); orbit
  by an integer ring count bounded by the frame (3.5).
- Retirement terminates because tape length is a natural number (T10).
- Pressure terminates because μ⃗-dominance is well-founded (T9).
- Cover repair terminates because each retraction removes a chart from a
  finite set (14.8).
- Mayer–Vietoris over `N` hyphae terminates in `N − 1` steps and is
  order-independent (13.7).

No theorem in this chapter uses an angle, a limit, a norm beyond an exact
rational square root, or a probability; each is a statement about finite
products of canonical words, finite integer linear algebra, and
well-founded orders, and each is discharged by a test or a trace the
auditor can rerun.

---

# Part VIII — Boundaries

## 43. What is deliberately absent

No angles: a rotation is a rotor with rational (or, in the tower,
cyclotomic) coordinates, and its angle is never formed. No norms or roots
beyond `rational_sqrt`, which refuses when inexact. No limits, series, or
derivatives: unfolding terminates by an integer measure, orbit by an integer
ring count, replay by finite products, descent by a `Budget`. No
probability: superposition is a set of cells, resolved by fill or class.
No scoring: nothing is compared except by `projective_eq`, `same_form`,
`is_identity`, class keys, and `descends`. No spectral sequence: the N-fold
cover is handled by iterated pairwise Mayer–Vietoris, order independent
(13.7), and Leray's theorem is proved by that induction along the link. No
homotopy beyond the acyclicity of an overlap, decided by a component count
and a tree walk; the one contractibility used is that a single face has
trivial `H¹` and constant `H⁰`. No abelianization off the sensor plane:
rotor coefficients use the pointed-set sequence and descent only, and
degree 2 is read on the plane or recorded as remainder. No metric on
classes: a class key is an exact hash of conjugation invariants. No
"good enough" covers: an overlap is good or the cover is changed until it
is, every retraction decided by `home`. No consensus, voting, or scoring
between nodes: exactness is the reason two nodes agree, and
`REFUSE_MV_INEXACT` is the only thing that happens when they do not. No
byte identity anywhere in cognition (chapter 1.5). No complete rewriting
system for the mixed-plane group, no Knuth–Bendix run, no claim of
minimality for an off-plane tape beyond what a completed search depth
proves. If an implementation step needs anything not permitted here, the
step is wrong.

## 44. Statements relayed and not adopted

A blueprint for the algorithms of Part III was relayed on 2026-09-05. Its
structure matches chapters 16–17 and its correct additions are in them.
The following statements in it are wrong for this tree and are not adopted;
an implementor who follows them forks the language or builds against an
algebra that is not here.

| relayed statement | why not | ruling |
|---|---|---|
| off-plane words with no plane-closed geodesic "evaluate to `R = 1`" | they are vertices; keying them as identity merges every mixed-plane word into one | A1 keys them (16.1) |
| a complete rewriting system with relators `(g_i g_j)^{m} = 1`, Knuth–Bendix confluence, a "unique minimal-length" tape | primes have infinite order; letters on planes sharing an axis generate a dense subgroup; no such relators, no termination | A2 is a budgeted retirement; shortlex only when the depth completed (16.2) |
| every plane generator has finite order `N_Π`, exponents mod `N_Π` | true only of the six unit turns (order 4); eleven letters are infinite order | A3 (16.3) |
| `PlaneForm` equality as one `(plane, exponent)` pair | a plane form is the whole 17-vector plus parity | A3 |
| conformal versors in `Spin(4,1)`/`Cl(4,1)` | the tree's conformal algebra is `Cl(3,1)` over `{e1,e2,e+,e−}`, the plane's model; `Cl(4,1)` is Part VI brick 2 | A4 (16.4), 38.5 |
| chains as `Z³` support sets with summed coefficients and stripped zeros | cells are `(r,c,before,after)` in `Z² × hue²` (six-tuples under B1); conflicting sites are refused, hues never summed | A5 (16.5) |
| 31-bit numerator/denominator in one word; Stern–Brocot for total order | half the range for no gain; no operation needs axis order today | A6 keeps two exact limbs with refusal (16.6) |
| refinement axis from area, boundary length, second-moment trace `(a²/w, p²/w)` | those are body moduli (`Harmonics`); the class axis uses the class rotor's scalar and pseudoscalar parts | A7 (16.7) |
| `Sel = (predicate, stratum, harmonic_bound)`, LEB128 | `Sel` is the term algebra of anchor.rs 31; fixed little-endian as everywhere | A9 (16.9) |
| Noise handshake, `P¹(F_25519)` | there is no handshake; trust is the ed25519 signature on the cover digest | A10 (16.10) |
| rendezvous trie traversal by the A7 class axes | a percept has no class; routing is by the body chart (M4 step 1) | A11 (16.11) |
| exponent groups as a direct product of finite cyclic groups solved mod `N` | the plane group is `(Z/4 × Z^k) ⋊ Z/2`; HNF or SNF over ℤ with `4e_u` as a relator | A12 (16.12) |
| `solve_gauge`'s chord check must become projective | it already is: `Rotor`'s `PartialEq` is `projective_eq` | no change (23.3) |
