# Recursion — the linguistics engine on crystallized forms

Status: consolidated specification with a bound implementation. Written
against `/opt/mycelium-v0.1.0` at a29bba5 (2026-09-05). This text replaces
every earlier version of this file: the earlier versions accreted parts
that amended one another by reference; this one is a single closed form in
which every statement stands where it applies. Companion documents:
`ALPHABET.md`, `MU-ENGINEERING.md`, `MU-ECONOMY.md`, `XSEPTUM.md`.

## How to read this document

Ten parts, fifty chapters, each chapter self-contained enough to
implement and each opening with what it carries from the one before.

| part | what it settles | who reads it first |
|---|---|---|
| I Geometry | what a word, a form, a move, a recursion **is** | everyone |
| II Cohomology | why the moves are sound: cochains, Mayer–Vietoris, Leray, naturality | the auditor; the implementor of the court |
| III Identity and algorithms | the component procedures and master algorithms every key, address, chart, class, comprehension, pivot, and route passes through | the implementor, before writing any code |
| IV Implementation on forms | every move bound to a function at an absolute path and line | the implementor |
| V Migration and exact strings | how the tree changes without stopping the fleet; the literal refusal reasons and log lines | the implementor and the auditor |
| VI Extensions | the two walls and how they come down without changing what a word is | later |
| VII Verification and landing | every test, every cold-boot trace, one landing order, and the theorems the engine proves by running | the auditor |
| VIII Boundaries | what is deliberately absent; relayed statements not adopted | everyone, once |
| IX The mesh under adversaries | time without clocks, threshold identity, receipts and the signed prefix, equivocation, settlement | the implementor of the court and the wire; the auditor |
| X Language formation | the sentence as the unit of speech: construction and deconstruction as adjoint functors, edge transport in the stabilizer, ordered factorization, typed demands, the three seams | the implementor of the voice and the court; the auditor |

**What this document is.** A specification of the implementation at
`/opt/mycelium-v0.1.0`, every statement bound to a function and a path,
never a survey. It began on 2026-09-05 (a6a21a9) as the recursion
specification, was rewritten as moves on crystallized forms (1f2db2d), and
consolidated into one closed form (4e2641d). Its identity ruling (1.5,
88f58e1) stated from the start that identity is re-derived and that
digests are transport only. The algorithms written after it (b6758e5,
9481243) reintroduced digests of canonical forms as keys, and the class
vector, the certificate, and the address were then written the same way.
That drift lost the coordinates the connecting maps and the naturality
squares need, and it is corrected on 2026-09-06 by laws 2, 11 and 12 as
now written and by A21: keys are coordinates, carriages are versors, every
dimension is a proof. Chapter 1.7 states why, once, so that no later
convenience reopens the question. Chapters 12.6, 15.4, 16, 17.2, 18.2, 19,
21, 35 and 47 were rewritten under that correction; a reader of an earlier
revision should not trust its keys. The same day found a second shadow
carried as the carrier: since 04931ab the class vector read the plane form
first and the rotor's conjugacy class as a fallback, so the
two-dimensional projection of a four-dimensional holonomy was primary;
12.6, 15.3, 16.3 and 3.10 now put the rotor first and name the "sheet" as
what it is, the orientation character of a half-turn through a third
axis. **What the drift cost.** Every class key, certificate and tape
written between 04931ab and de6fa58 was keyed by digests and is not
comparable with anything after 38ee193; the soak readings of Cut B before
38ee193 are void; the identity recut cost one landing (38ee193) and one
residue commit (75e26eb) before any new chapter could land. The task list
keeps a dated backtrack ledger so this is not paid twice.

## The laws this document is written under

These are not restated in each chapter; every chapter assumes them.

1. **Geometry, not trigonometry or calculus.** A rotation is an exact rotor
   over ℚ; its angle is never formed. No norms or roots beyond an exact
   rational square root that refuses when inexact. No limits, series,
   derivatives, or probabilities. Termination is by an integer measure or
   a `Budget`. Nothing is scored; things are equal, or they are not, and
   a reading of two lane vectors says per lane which fell, held or rose.
2. **Identity is content, never bytes and never a digest of content.** Two
   words are equal when their rotors are projectively equal. A key is the
   canonical coordinates of an exact object written out, in a fixed order,
   length-prefixed, so that the object is recovered from its key and a
   prefix of the key is a coarser stratum (A21). No hash, fingerprint, or
   digest of any mathematical structure exists in cognition: a digest maps
   a stratified exact vector to a rank-zero point, keeps equality and loses
   every coordinate, every prefix, every composition, and therefore every
   map of data (15.1) and every connecting map (13.4). The master equation
   is naturality: `φ*(key(c)) = key(φ*(c))` for every map `φ` of 15.1, which
   a digest cannot satisfy. Digests exist only where bytes are the object:
   signatures, receipt chains, wire and file integrity (Part IX). Byte
   exactness is never what is checked; the mathematics adding up is
   (chapter 1.5; the reason, stated once, is chapter 1.7).
3. **No enumeration.** Indices come from a keyed read, a peel, a link, or a
   `Budget` cursor. No loop over peers, pairs, degrees, or planes that the
   carrier does not already name. `HashMap` and `BTreeMap` do not appear in
   cognition; the relational field is a `Keyed` PATRICIA trie or a
   `Vec` of relations.
4. **Exact ℚ; refuse rather than round.** Every scalar is a rational (or,
   in Part VI, an element of a cyclotomic field with canonical reduction).
5. **Polytonic lanes.** μ⃗ = (S, G, T, H) is never summed, ordered, or
   reduced to a scalar; a comparison of two lane vectors returns the
   reading per lane and never a verdict, and no lane falling certifies
   anything (12.6).
6. **The alphabet is immutable to a voice and grown only by the court**
   (ruled 2026-09-10; before that date, immutable). Seventeen letters are
   the base; words are their combinations; vertex equality is projective
   rotor equality, never tape equality; the normal form is the geodesic
   where one exists. No voice adds a letter, no configuration adds a
   letter and no commit adds a letter: a letter beyond the seventeen is
   opened by the court alone, on a typed demand whose witness rotor two
   independent courts re-derived to one Gaussian factor at one chart
   (arity-2 consilience, 37.7), ruled with a receipt, and taught by the
   syllabus before any voice speaks it. An opened letter is
   named on every tape by its content, never by an index assigned in open
   order, since open order is a total order across nodes (law 9) and a
   content name composes where an index cannot (law 2). What this
   restores was never a mathematical limit: the letter a prime demands is
   the witness rotor's own factor, read exactly, and the seventeen were
   frozen by doctrine, not by arithmetic (36.3). The two organs of a
   letter's consilience are two independent courts, never two voices: a
   letter is no exception to 47.3 (ruled 2026-09-10).
7. **Residuals are never discarded.** A remainder is the exact demand. The
   one exception is a computed class that Leray's theorem shows to be an
   artefact of a spanning tree (chapter 14.8); it was never a class.
8. **Clients think, hyphae verify.** Cognition is the mind's; a hypha
   replays, judges, keeps the ledger, and stores the genome. Sentences
   only (ruled 2026-09-10): a voice sends the court sentences and nothing
   else, the court replays them by G and never composes by F, and the
   wire never carries raw percept cells to a court. A court that
   extracts boundaries, measures versors or peels letters is thinking,
   and a second engine; the four `from_residual` sites of the court's
   reduction were that engine (50.6, 50.9).
9. **No clock but the ledger.** No kernel reads a wall clock, a unix clock,
   or a timer. K is a kernel's own receipt index and orders events inside
   that kernel; TUniversal is the height of the mesh's receipt ledger, read
   from receipts, and measures standing; order across nodes is the causal
   partial order of receipts and is never totalised (Part IX). Lane T of μ⃗
   is temporal extent counted in receipts, a cost, never a coordinate.
10. **Direction is a pivot, never a gradient.** A held loop is discharged
    by a move on one of the axes its class vector occupies (chapters 3.9,
    12.6, 16.16). The candidate moves are read from the class vector in a
    fixed order, each is tried exactly, no move waits on another's failure,
    and reduction against held words is one of the moves and never a
    precondition; every one that discharges is kept. There is no list of
    directions and no loop over "descend, then ascend": there is the
    vector, read before anything is reduced, and each occupied coordinate
    names the one move that can zero it (16.16, chapter 5, ruled
    2026-09-08). **Closure certifies** a pivot after it is made: the coordinate it
    acted on is zero, no other rose, and the rank moved by its one unit or
    not at all; the lane reading of a trial refuses a candidate that raises
    a lane by naming the lane. Nothing on μ⃗ and nothing in the rank audit
    chooses among pivots, and no scalar ever orders the candidates. Several
    candidates that discharge are held together as a superposition, the
    vortex, until a perception or a court replay collapses it.
11. **No squaring.** The sensor's ℤ² is the substrate a hypha reads cells
    from; it is not a space cognition lives in, and ℚ² is ℤ² squared
    again. Cognition never carries a position, a placement, or a shift as
    a pair of numbers. A placement is a conformal translator, a dilation a
    dilator, both versors of the conformal algebra (A4), composed with the
    rotor words by the geometric product; the class vector of a loop is
    the blade decomposition of its canonical holonomy versor, and its
    placement axis is the translator component of that versor. A pair of
    coordinates appears only at the sensor boundary, in `pair_form`,
    `speak_rule`, and `orbit`, where a cell is read or written. Fluid
    physics carried in squares is squares of fluid: looking for a square
    inside a circle is a search for two different objects (chapters 1.4,
    1.7, 16.4). **No matrix carries an action.** A matrix is a linear
    representation on a fixed set of squared coordinates: it holds only
    what that frame can hold, its product forgets the blade structure of
    what it represents, and a new axis is a larger matrix rather than a
    new grade. That is flattening and rigidity in one object, and it is
    why an action re-derived through a matrix loses coordinates the class
    vector, the pivots and the connecting maps need. Every action in this
    engine is an element of an exact algebra composed by its own product:
    a rotor or versor by the geometric product, a gauge by permutation
    composition; a matrix appears only as the exact integer arithmetic of
    a rank (A18, `core::zlinalg`) and as the sensor-plane block a rotor
    projects to (A3), never as the carrier of an element (18.3).
12. **Every dimension is a proof.** A letter is an axiom. A word is a
    proof: a path whose vertex is the theorem it reaches, and a form
    **re-derives that vertex by composing the word's derivation** — the
    cited generators to their exponents, times the residue — never by
    reading a stored spelling (1.5, 50.2 step 2). A crystallized
    form is a gemstone: each of its loops is a theorem, proved by the
    identity of the holonomy the words around it compose, and one theorem
    has as many proofs as the form has synonyms of one class (13.10). Every
    coordinate of a form is a theorem the form proves by re-derivation; a
    coordinate that is not is not a dimension of the form but a section
    coordinate carried beside it. Nothing is a dimension because it was
    stored (chapters 1.7, 2.3, 42).

## Coding standards: the rules that stop the crud before it is written

The laws above say what the engine is. These rules say how code that
implements it must be written, so that a reviewer can refuse a change on
sight and an implementor does not spend a cycle writing what the next audit
removes. Every rule below was extracted from a defect that actually landed
in this tree; the "looks like" column is the shape that landed, the "write
instead" column is what replaced it. A change that contains a "looks like"
shape is not reviewed further; it is returned.

| # | rule | looks like (refused) | write instead |
|---|---|---|---|
| 1 | **A pair loop is allowed only over one cell's own vertices or one loop's own faces.** Iterating `i < j` over bodies, vertices, charts, hyphae, or peers is a correspondence search over a product the carrier already names. | `for i in 0..n { for j in i+1..n { if related(i, j) … } }`; `bodies_in.iter().find(…)` for every `bodies_out`; `for lane in 0..4 { for h in hyphae … }` where the lane is not a fixed physical size | build one keyed occupancy once, `Keyed<cell key → Vec<vertex>>`, read pairs out of cells holding two or more vertices, probe a ring cell into the same key; a cell's own vertices and a loop's own faces are its boundary and may be walked |
| 2 | **A count is never a branch.** A decision that reads `len()`, `count()`, or `n <= k` is a boolean gate on a count, and the count is not a measurement of anything. | `if n_words <= 1 { all bodies }`; `if germs.len() >= 2 { true }`; `if counts.len() == n_pairs` as the whole test | the fiber decides: `atlas.fiber(class)` names the bodies a vertex speaks to and an empty fiber is remainder, never "everything"; presence is read from a keyed set, not from a length |
| 3 | **Equality is `same_vertex`, `projective_eq`, or class-vector equality.** `==` on `Element`, `Versor`, a tape, a `Word` outside M1, or a struct that contains one is byte identity and forks or merges meanings; equality of two digests is the same fault one step removed. | `old == g` on a versor; `self.word == other.word`; `#[derive(PartialEq)]` on a type holding rotors or chains; `address() == address()` as the test of sameness; `x.class_key() == y.class_key()` | `a.same_vertex(&b)`; `r.projective_eq(&s)` (either sign); `x.class_vector() == y.class_vector()` coordinatewise on exact ℚ; an address compare only where the address is the intended identity (a citation) and the address is the M2 composite of A21 |
| 4 | **Nothing is chosen by a scalar.** No `min_by`, `max_by`, `sort_by` over candidates, no "lowest order first", no "closest", no threshold. | `chords.sort_by_key(order)` then take the first; `candidates.iter().min_by_key(cost)`; `if remainder.len() < best` | read the class vector, list its occupied axes in the fixed order (12.6), try each, keep every certified discharge, hold incomparable ones as a vortex; dominance on μ⃗ certifies after the fact and never selects |
| 5 | **No literal durations, no counts as generations, no free constants.** A number that is not a letter exponent, a rational read from the data, a cell coordinate, or a physical size of a carrier is a constant someone will have to remove. | `Duration::from_secs(1)`; `SystemTime::now()` or `Instant::now()` read by a kernel; `gen = genome_len + dialect_len`; `depth.min(8)`; `ratio: ((1,1),(1,1))` as a placeholder; `m = 1024` | a ratio of measured transport, the round trip counted in the kernel's own strokes (`from_transport(rtt_floor, aperture_cycle())`, 45.4); a root of content (the stalk digest of the working atlas); the value the tape carries; refuse when the value is not there |
| 6 | **Keys are canonical coordinates written out, never a digest of them, never bytes or pointers.** A memo, an address, a class, or a cache keyed by a hash of anything mathematical, of a tape, a serialisation, a file path, or a pointer is identity by bytes in disguise; hashing the canonical form is the same fault with a better excuse, because the key can no longer be read, composed, restricted, or peeled (1.7, A21). A cached key stored beside the coordinates it was computed from is the same fault again: it goes stale on the first mutation and the object and its name diverge. | `blake3(canonical coefficients)` as `Rotor::key`, `Mv::key`, `Word::class_key`, `ClassVector::class_key`, `Element::address`, `Invariant::key`, `listing_key`; `blake3(path)`; `a_ptr == b_ptr`; `format!("{s:?}")` hashed | the coordinates themselves, length-prefixed, in the fixed order of A21: the eight reduced rationals of the canonical rotor (M1), the M2 composite of a form, the chart's axes, the class vector, the canonical chain of a listing, `Sel::to_bytes`; a `Keyed` trie over those bytes, where a prefix is a stratum |
| 7 | **Meaning is never demoted to "fibre" to make a key stable.** If two tapes that mean different things get one key, the key is wrong, whatever the comment says. | dropping the bundle's agreed shift or a contact from a sentence key because "placement is a fibre coordinate" | a body's absolute position is quotiented; a relative placement inside a sentence is a face and is identity; contact is on the form and in its key (chapter 1.5) |
| 8 | **No shims with placeholders.** A wrapper that fills a field with a constant so a call compiles is a defect that will be soaked. | `sentence_rule_of` with `ratio: ((1,1),(1,1))` and empty sections; `set_magnitude([0,0,0,1])` | carry the field on the tape and read it; measure the magnitude by speaking the form over its bundle; if the value cannot be measured, the form is not spoken |
| 9 | **Carriers are `Keyed` or `Vec` of relations.** `HashMap`, `BTreeMap`, `HashSet` do not appear in cognition; a flat map keyed by a digest collapses a dimension, and a `Keyed` trie keyed by a digest is a `HashMap` in disguise. | `HashMap<Key, Vec<Candidate>>` buckets; `std::collections::*` in a transducer or kernel file; `Keyed<[u8; 32]>` over `blake3(...)` | `Keyed<V>` (PATRICIA over the canonical coordinate bytes of A21, so `fiber(prefix)` reads a stratum), `Keyed<Keyed<()>>` for incidence, `Vec<(a, b, relation)>` for a relational field |
| 16 | **An action is an algebra element, never a matrix.** A `QMatrix` fibre, an affine block, a matrix product as composition, or matrix equality as identity is a linear representation on squared coordinates: it flattens the blades it represents and adds a frame's rigidity (law 11). | `fibers: Vec<(u16, QMatrix)>`; `realize()`; `same_action = realize() == realize()`; `affine_of(versor)`; `Letters::affine`; `Element::identity(GERM_DIM)` | one versor per element (`carriage · linear_versor(word)`), a hue permutation, an extent, shell coordinates; `geo` for composition, `inverse` that refuses, `projective_eq` for identity; a matrix only in `core::zlinalg` for a rank and in A3's sensor-plane block |
| 10 | **Every loop spends a `Budget` or walks a boundary.** A loop whose bound is neither a `Budget` nor the size of one cell's boundary is unbounded work in disguise. | `while !done { … }`; `for peer in all_peers { exchange(peer) }` on one stroke | `while budget.spend() { … }`; a `Keyed::cursor(from, &mut budget)`; one peer per spend with a resume cursor |
| 11 | **Exact or refused.** No float, no rounding, no saturation, no `unwrap_or(default)` that invents a value. | `ratio_axes` saturating to `u64::MAX`; `unwrap_or_else(identity)` on a versor that failed to invert | `Option`/refusal that propagates to a logged refusal string; an inexact square root refuses; an unspellable block is a remainder |
| 12 | **Naturality over cloning.** When a carrier splits, restrict it; never clone it into both halves and rekey. | `complexes.insert(child_a, c.clone()); complexes.insert(child_b, c)`; `rekey_cocycles` without recomputation | `restrict` per child, `straddling` written as faces between children, each class recomputed and carried or discharged (15.5) |
| 13 | **Verify the claim, not the outcome.** A test that asserts a count or a boolean proves nothing about how the result was reached; a test named after a theorem must exercise the move. | asserting `faces.len() <= 2·germs` and calling it "the adjacency rule"; testing `Word::rebase` and reporting "rebase works" when the extraction step never fires | N vs 4N work counters for cost claims; a trace of the move on a family that needs it (a `Rebased` vertex that speaks the test); a refusal test for every refusal string |
| 14 | **The whole suite, every commit.** "Module green" is not green. | "family 14, sentence 18, sdk 12 green" while `server::cognition` has four red tests | one foreground `cargo test --release`, the `test result` line in the commit message, one item per commit with its trace value |
| 15 | **When unsure, hold; never choose.** If two readings of the data are both consistent, the code keeps both as cells and lets a perception, a glue, or a ruling collapse them. | a tie-break by index, by size, by "first found" presented as a decision | a vortex (3.9); a section (chapter 21); a `Pending` (14.5); a `Novel(g_rem)` demand |

**The review checklist**, applied to every diff before it is committed and
again by the auditor, in this order: (a) any `i < j` loop not over a cell's
or loop's own boundary; (b) any `len()`, `count()`, `is_empty()` that
decides a branch rather than an emptiness of a fiber; (c) any `==`,
`PartialEq`, or `derive(PartialEq)` on a type holding a rotor, versor,
tape, or chain; (d) any `sort_by`, `min_by`, `max_by`, or comparison on a
candidate set; (e) any literal number that is not a letter index, a blade
mask, a fixed carrier size, or a wire tag; (f) any `blake3`, hash, or
digest outside `network/`, `court/identity`, and `chronometry`, or over
anything but wire bytes, file bytes, and receipts; (f′) any `(ℚ, ℚ)`,
`(BigRational, BigRational)`, or `(i64, i64)` carried as a placement, shift,
or residue outside `pair_form`, `speak_rule`, and `orbit`; (g) any
`HashMap`/`BTreeMap`/`HashSet`; (h) any loop without `budget.spend()` or a
boundary bound; (i) any `unwrap_or` that invents a value; (j) any `clone()`
of a carrier into two places; (k) the `test result` line of the full suite;
(l) any `compute_*`, `derive_*`, `classify_*`, `extract_*`, or `recover_*`
whose body emits a label, a digest, or an aggregated scalar in place of the
operation it names, without a typed arm naming the absence; (m) any
projection or shadow carried as the primary object with the object itself
as a fallback: a plane form consulted before the rotor, a coordinate pair
before the versor, a digest before the coordinates, a count before the
fiber; (n) any `QMatrix`, affine block, or matrix product used to carry,
compose, or compare an action, outside `core::zlinalg` and A3's
sensor-plane block. A diff that fails (a) through (j), (l), (m) or (n) is
returned with the rule number; a diff without (k) is not reviewed.

**Why the rules on keys and coordinates are absolute.** Chapter 1.7 is
the standing explanation: a proof is arithmetic on coordinates, a digest
keeps equality and destroys every other operation, so a system keyed by
digests can recognise but never deduce, and its certificates become
claims the court cannot replay. Rules 3, 6, 9 and checklist items (f),
(f′) and (l) are that chapter applied at the diff. A reviewer who is asked
why a hash of a canonical form is refused answers with 1.7, not with a
preference.

**The development standard.** The rules above say what a diff may not
contain. How the tree is laid out (no file over 600 lines, one organ per
file), how things are named, how tests are named and organised by chapter,
how a commit is cut and verified, which gates enforce which rules, and how
an implementor and an auditor work with each other and with the operator
is `MU-STANDARDS.md`, binding on every agent from 2026-09-07. A diff that
violates it is returned exactly as a rule-violating diff is; a private
gate, fixture style, or naming scheme is refused the same way.

**How to write the move, not the search.** Ask what the carrier already
names. Bodies name their cells: key the cells. Faces name their vertices:
walk a face's two ends. A loop names its faces: walk them. A class vector
names its occupied axes: try those. A chart names its fiber: read it. If
the code you are about to write needs to look at everything to find the
thing, the carrier is missing a key and that key is the first thing to
write. If the code needs to pick one of several by a number, the doctrine
says keep all of them. If the code needs a constant, the physics has a
ratio you have not measured yet.

## Vocabulary, fixed

letter → word (a tape; its meaning is a vertex of the Cayley graph) →
sentence (words that cohere) → speech (transfer of cohering sentences from
a sender to a receiver) → perception (receiving) → comprehension
(deconstruction of the utterance into the algebra that spoke it).
**Recognition** is comprehension, never a lookup: a mind recognises what
it can re-derive, and nothing else. A **voice** is a speaker: a kernel
under one verifying key (Part IX) that perceives, thinks in its dialect,
and utters; an **utterance** is one act of a voice; **speech** is the
transfer. **Orientation** is the `Z/2` character of a word or loop on a
chart, realised by a half-turn through a third axis, formerly the sheet;
a **reflection** is that half-turn; a **refraction** `Γ` is the transport
of a form across a chart boundary into another frame, a relative word, a
gauge and a carriage together (3.10). A
**germ** is an unclosed form; an **idea** is a candidate closed form bred by
a perception; a **thought** is the superposition of candidate forms through
which ideas germinate; **germination** is the movement toward closure. A
**held loop** is a loop of an idea whose holonomy no gauge removes; its
**class vector** is the exact record of what would have to change for it to
close; a **pivot** is a move on one axis of that vector, and the pivot is
the only motion in this engine; **descent** has in this document only its
sheaf-theoretic sense (chapter 13), sections on the pieces gluing iff they
agree on the overlap: it is not a motion, not a direction and not a
certificate, and the lane-wise reading of two μ⃗ is `lane_moves`, per
lane, which certifies nothing (12.6); a
**deflation** is what a word or form closes, measured on μ⃗; a
**shortening** is A2's budgeted retirement of a tape, which moves nothing;
a **vortex** is a
held loop together with the candidate pivots that could discharge it, held
as cells of the thought until one of them does. A **demand** is a typed
vertex of a sentence naming what the alphabet could not spell or the eyes
could not pair (chapter 50); a bare chain of cells is never a demand and
never a word. A
**dialect** is spoken and stood (memory, in the epigenome); a **language**
is replayed by two organs and taught (on disk, in the genome). What was
once called a family is a **bundle**: a base rule with sections per pair and
transitions as gauge. A **receipt** is one kernel's record of one admitted
act, linked to its predecessor and to the peer receipts it verified; a
kernel's **K** is its count of receipts; **TUniversal** is the height of the
mesh's receipt ledger (Part IX). A **key** is the canonical coordinates of
an exact object written out in a fixed order, length-prefixed, from which
the object is recovered; an **address** is the key of a form (M2); a
**carriage** is the conformal versor, dilator composed with translator,
that places a word on the sensor plane (A4). Neither a key nor an address
is ever a digest, and a carriage is never a pair of numbers.

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
(`alphabet::relators()`, alphabet/letters.rs):

- a quarter turn has order four (projectively; see 1.5 on sign);
- rotors on one plane commute;
- a half-turn, two quarter turns, on a plane sharing one axis with the
  sensor plane inverts a sensor-plane rotation: `S R S⁻¹ = R⁻¹` with
  `S = q²` and `S² = 1` projectively (`alphabet::sheet_word`). Read on the
  sensor plane this is a reflection; in the algebra it is an even rotor
  through a third axis, and there is no reflection letter. Its parity on a
  word is the word's **orientation** on the sensor chart, the character
  `w₁ ∈ Z/2`, formerly called the sheet (3.10);
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
sensor's plane read in its conformal model (A4), with a **carriage**: the
dilator of `ρ ∈ ℚ` composed with the translator from the plane's origin to
the body's centre, one versor, and a gauge (a permutation of hues). A
**word form** is `(vertex, carriage, gauge)`. Two word forms with equal
vertices and different carriages are the same word said at two places;
the carriage is the coordinate the court quotients, as it always has
(`stalk.rs`). The carriage is never a pair of rationals (law 11): the
translator is a versor like the rotor, composed by the same product, and
its two blade coefficients are read out only when a cell is spoken.

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
- **The geodesic is M1's quotient, and it quotients the spelling — not
  the derivation** (ruled 2026-09-10). Law 6's normal form answers one
  question: which vertex is this. It reads the rotor, so every tape that
  reaches the vertex reads alike, and that is exactly why a spelling is
  never a coordinate of a form. A **derivation** is not a spelling. It
  names which held generators the speaker composed to get there, and two
  minds whose presentations differ compose different generators to reach
  the one vertex; the geodesic cannot quotient that away without erasing
  the only thing on the wire that distinguishes two languages (50.2 step
  2). So the tape's normal form is the derivation with A2 applied to its
  **residue** — the letters are reduced, the citations are not, and the
  cited generators keep their order and their exponents. Vertex identity
  is unchanged and stays projective rotor equality: two vertices with
  different derivations and one rotor are one vertex, one address, one
  ledger entry.
- **Keys are the canonical coordinates written out, never a digest of
  them and never tapes as spoken or as serialised.** The **vertex key** of
  a word form is the concatenation, each field length-prefixed as A21
  writes it, of exactly these fields in this order, every field canonical,
  so that the word form is recovered from its key and a prefix of the key
  is a coarser stratum:
  1. the canonical rotor coordinates of the word (chapter 16.1, A1): the
     eight projectively normalised coefficients in mask order, so plane
     and mixed-plane words share one keyspace and the spoken tape never
     enters;
  2. the carriage: the canonical conformal versor (A4) of the dilator of
     `ρ` composed with the translator, its coefficients written as reduced
     fractions (numerator sign byte, numerator magnitude as little-endian
     bytes with `u32` length, denominator likewise: every rational in this
     list is written this way, `put_q`);
  3. gauge: the hue permutation in one-line notation as `u8`s with `u32`
     length; the identity is written with length `0`;
  4. for a closed sentence form only (forms under construction have no
     address, chapter 4): `depth` as `u16`; vertex count as `u32`; per
     vertex in vertex order its kind byte (`Frame 0`, `Word 1`, `Glued 2`,
     `Rebased 3`, `Unfolded 4`) and its own vertex key (a `Glued` vertex
     contributes the cited form's address, itself this composite, and the
     canonical rotor coordinates of its placement word; `Rebased` its two
     indices as `u16`; `Unfolded` its measure byte and anchor); face count
     as `u32`; per face `i`, `j` as `u16`, the canonical rotor coordinates
     of `g_ij`, and the canonical carriage versor (chapter 16.4, A4). Loops
     and holonomies are derived and never keyed.
  A reader that receives a tape decodes it, computes this key, and uses
  nothing else as identity. A change of tape version, wire frame, or
  spelling never changes an address; the one migration from byte
  addresses is chapter 33. The key is never hashed: a digest of it would
  keep equality and lose the coordinates, and it is the coordinates that
  the naturality squares of 15.1 and the connecting maps of 13.4 read.
- **Class keys are the conjugation invariants themselves** (chapter 15.4),
  written out, never a digest of them and never keys of a representative.
- **Digests of bytes are transport identifiers only**: cover digests,
  fetch digests, signatures, and receipt chains name *what was sent*, so a
  receiver can ask for it and verify who sent it. They are never read as
  identity of meaning, never used as a key into a cognition carrier, never
  compared to decide consilience, dialect, or citation, and never computed
  over a rotor, a versor, a chain, a form, a class, or a chart. What is
  checked is never byte exactness; it is the mathematics adding up.
- **Exact strings in this document are for traces**, not for cognition:
  refusal reasons and log lines are grepped by the auditor, and their
  exactness costs nothing because no carrier keys on them.

### 1.6 The faults this ruling names

The audit of 2026-09-05 read every identity decision in the tree. These
decide identity by bytes today and are replaced by the procedures of
Part III:

| site | what it does | replaced by |
|---|---|---|
| `alphabet/word.rs` `normal_form` | returns the spoken tape for words that mix planes, so `Word::key` (448) and `Word::eq` (410) fall back to spelling | A1 key for every word; A2 for the tape |
| `alphabet/word.rs` `same_form` | tape-equality arm `(None, None) => self.0 == other.0` | `projective_eq` |
| `alphabet/plane.rs` `PlaneForm` | structural equality on unreduced unit exponents; `form()` does not reduce; `sentence::chart_of` (1885) reads raw exponents | A3 |
| `alphabet/rotor.rs` `Rotor::key`, `coordinate` | hash of unnormalised coefficients used at `cognition.rs:2947` as class key and refinement axis; `PlaneForm::key/coordinate` (849, 858) likewise | A1, A7 |
| `cga.rs:46` `Mv` (`Versor`) | derived structural equality; `cognition.rs:1006` `push_cocycle` `old == g` | A4 |
| `element.rs:140` `Element::eq` (now `element/vertex.rs`) | tape bytes plus structural fields; inherited by `Germ`, `Face`, `Nerve` (sentence.rs 583, 600, 633) | M1 `same_vertex` |
| `sentence.rs:1144` `vertex_address` | writes raw versor bytes beside geodesic keys | A4 canonical versors |
| `cognition.rs:5635, 5526` `composite_parent_id`, `section_address` | hash cells in given order, uncentred; assignment unsorted | A5, M2 |
| `sentence.rs:1792` `from_element` | trusts the germ address stored in the tape | recompute (A8 step 2) |
| `blade/body.rs` `ratio_axes` | drops the sign, saturates to `u64::MAX`; under `moduli ==`, `moduli.key()`, `Invariant::key`, `connection_chart` | A6 |
| `verdict/selection.rs` `sel_key` | hash of `format!("{s:?}")` | A9 |
| `mycelium-server/judge/percept.rs` | percept rendezvous by `hash(pbytes[32..])` | A11 |
| `verdict/blind.rs` `blind_replay` | compares the spoken chain without canonicalising it | A5 |
| `cognition.rs:5052`, `economy/persist.rs`, client loaders | restore carriers under stored byte addresses after the address rule changed (a29bba5 landed without a migration) | A8, M3 |

The audit of 2026-09-06 found the same fault one step removed: every key
of a canonical form in the tree is a digest of it. These are replaced by
A21, in one landing (chapter 41, 0.11):

| site | what it does | replaced by |
|---|---|---|
| `alphabet/rotor.rs` `Rotor::key`, `alphabet/word.rs` `Word::class_key`, `alphabet/plane.rs` `PlaneForm::key` | blake3 over canonical coefficients or conjugacy invariants | the coefficients and invariants written out (A21) |
| `cga.rs` `Mv::key` | blake3 over canonical versor coefficients | the coefficients written out |
| `element/vertex.rs` `Element::address`, `form_key_bytes`; `element/residual.rs` `listing_key` | blake3 over the M2 composite | the M2 composite itself (17.2) |
| `stalk.rs` `Invariant::key` | blake3 over the exact axes before the trie sees them | the axes' bytes, so `fiber_under` is a prefix read |
| `sentence/class.rs` `ClassVector::class_key`, `CLOSED_KEY` | blake3 over the class vector | the class vector's coordinates; closed is the zero vector |
| `sentence/form.rs` `hue_section_key`, `vertex_transport_key`, `class_key_of_versor` | blake3 over conjugacy ‖ gauge | the conjugacy coordinates ‖ the gauge |
| `sentence/chart.rs` `form_vertex_key`; `family/extract.rs` `TransportClass::key` | blake3 over kind ‖ class | kind ‖ class coordinates |
| `sentence/pivot.rs` certificate key; `Form.certificates`, `torsion`, `vortex`, `horizon` indices | keyed by digests | keyed by the class vector and the move (16.20) |
| `cech/cell.rs` `cell_key`; `cognition/keys.rs` `cocycle_key`; `verdict/selection.rs` `sel_key`, `chart_simplex_key` | digests of sorted vertex keys and terms | the sorted vertex keys concatenated; `Sel::to_bytes` itself |
| `sentence/*` `placement: (ℚ, ℚ)`, `Transport.shift`, `Rebased.shift`, `PivotBy::Placement.shift`, `Form.placements`, the 12.6 placement residue | ℚ² carried as cognition (law 11) | the carriage versor (A4) and the translator component of the holonomy versor |

Allowed, and why: hypha and voice identity by verifying key; signatures;
receipt chains and their hashes; cover and ruled roots as digests of the
bytes sent; file integrity in persistence. Each names bytes that were
sent or stored, so a receiver can ask for them and verify who sent them,
and none is read as the identity of anything mathematical.
`registry::task_id` and the client's family id are listing identities and
become the canonical chains they name. `solve_gauge`'s chord `!=` is
already projective because `Rotor`'s `PartialEq` is `projective_eq`.

### 1.7 Why the structures must be the mathematics, never their digest

A proof built on a hash is a proof about a name, and a name proves
nothing. This section is the reason behind laws 2, 11 and 12 and A21,
written once so that no later convenience reopens the question.

**What a proof is here.** A word is a path in the Cayley graph; the
theorem it proves is the vertex it reaches, and the proof is the
composition of letters that gets there. A loop is the theorem that its
holonomy is the identity, and the proof is the product of the words
around it. Mayer–Vietoris proves a class on a union from the classes on
the pieces by multiplying representatives (13.4). Naturality proves that a
class survives restriction because the map acts on its coordinates by the
group law (15.1). Every one of these is arithmetic on the coordinates of
the objects involved; the coordinates are the premises.

**What a plane carries.** A dimension is an axis of the chart the alphabet
acts on; a plane is a pair of axes, and every letter is a rotation in one
plane. The content of a plane is exact and its own: on the sensor chart
each of the six slot planes carries sheet parity in `Z/2`, unit turns in
`Z/4`, and the `(3,4,5)` and `(5,12,13)` exponents in `Z`; the conformal
planes `e_i ∧ e_∞` and `e_o ∧ e_∞` carry the translator and the dilator;
the hue gauge carries its cycle type. A loop's class vector is exactly
this list, one coordinate per plane, each a theorem the words around the
loop prove, and a pivot discharges one plane's coordinate and no other
(T14). An object of more axes, under base promotion (chapter 38), has more
planes, and each of them speaks in the same way. Linguistics here is
built plane by plane: a word is a path whose planes compose by the group
law, a sentence relates words by relative words, a loop is a theorem on
every plane at once, and comprehension reads the planes back out.

**What a digest removes.** A digest keeps one fact, equality of names,
and destroys every other: two digests cannot be composed, restricted,
conjugated, compared as a power of one another, or peeled to the prefix
that names their stratum. Hashing one plane's coordinate loses that plane:
no pivot can be listed on it, no disagreement can be located on it, the
connecting maps of 13.4 lose a factor, and the plane collapses to "same
or different". Hashing the whole vector loses every plane at once and the
relations between planes, the non-abelian composition, the stratification
that makes a prefix a coarser class, and the ability to decode at all.
Recognition in this doctrine is comprehension: a mind recognises an
utterance by deconstructing it into the algebra that spoke it (chapter 6,
A12) and re-deriving what it can. A digest has nothing to deconstruct, so
a system keyed by digests cannot recognise, cannot comprehend, and cannot
deduce; it can only report that two names match. Matching names is a
lookup table; a language recognises by re-deriving. That is the collapse
law II names, a dimension folded to a point, and it is why the founding
text of this document (a6a21a9) made identity a replay and the address a
fetch handle only.

**What happens to the linguistics.** Words stop being paths and become
tokens. Comprehension degrades from deconstruction (chapter 6) into
search, because matching is the only operation left on tokens. The class
ledger becomes a bag of names the court can neither compose nor compare
across charts, so the connecting maps and the goodness checks have no
inputs and are either faked with a representative shipped beside the key,
which is what the A19 record once did, or replaced by a scalar standing in
for the missing structure. Every downstream proof inherits the fault: a
certificate whose `before` and `after` are digests asserts that a pivot
changed exactly one axis, but the court cannot check it coordinate by
coordinate (T14), so the certificate is a claim, not a replay.

**What happens to identity.** Equality of digests is equality of the
canonicalisation that fed them. When one node's normalisation drifts by a
sign, a scaling, or a tape version, one vertex forks into two addresses,
two escrows, two citations, and the language splits along spelling rather
than meaning (1.5). The failure is silent, because two names that differ
say nothing about how the mathematics differs. With coordinates the
disagreement is visible and locatable: this plane, this exponent, this
residue.

**What happens to the economy and the court.** Settlement pays for
measured deflation and a ruling is a replay; both need the object. A hypha
holding digests cannot replay; it can only confirm that a voice sent bytes
it had seen before, which is byte exactness, the one failure this
architecture cannot absorb. Consensus then rests on trust in whoever
produced the digest instead of on re-derivation by two organs.

**Why digests look like progress.** They give stable, short, uniform keys,
and the code runs. Nothing is visibly wrong until the first operation that
needs a coordinate: Mayer–Vietoris, pivots, naturality, the class ledger,
which is exactly where this tree stalled. The hash let the plumbing pass
its tests while the mathematics underneath had already been thrown away.

Authenticity of structure is therefore not hygiene. The coordinates are
the only thing the proofs are made of; a system that stores their shadow
has no proofs, only a memory of having once computed them. The same holds
for a placement carried as two numbers instead of a translator versor
(law 11), and for any stored coordinate the form cannot re-derive (law
12): each is a shadow kept in place of the object.

### 1.8 The language is a presentation (ruled 2026-09-08)

The seventeen letters generate the group (1.1) and never change. What
grows is not the group but a **presentation** inside it: the language is
a chosen generating set together with its relations, and the geometry
makes it a groupoid rather than a group.

- **Objects** are the charts of the atlas (stalk/).
- **Generators** are the ruled words: each a group element spelled from
  the letters, carried by the conformal versor that places it and the
  gauge permutation that colours it. A word acts on a chart; its transport
  to another chart is a conjugation (T19, 3.10), so two conjugate words
  are one generator seen from two charts (23.9, the conjugate level), and
  the class vector is the coordinate that generator is filed under.
- **Relations** are the sentences: a loop whose holonomy composes to the
  identity is a relation among the generators, which is what "each loop
  is a theorem" (law 12, 2.3) says.
- **Sections** of a punctured axis (25) add coordinates to a generator's
  fibre, never letters to the group.

Read cohomologically, the same object is the gluing data of a principal
bundle over the atlas (the bundle ruling of 2026-09-02): the ruled words
are its transition functions, a Čech 1-cocycle with values in the group;
the sentences are the cocycle condition on the triple overlaps; a germ is
a 1-cochain that fails it, the residual the coboundary leaves; and the
genome is the cocycle the court has ruled, taught down by the syllabus.

**How it grows.** The presentation extends itself: a residual that no
generator reduces is adjoined as a new generator once the court has
replayed it and independent lineages have reached the same conclusion
(23.9); a loop that closes is adjoined as a relation; a generator the
rest derive retires, address kept, its description burned (27, E2). The
presentation stays reduced by retirement and stays paid for by the
economy, and that is the whole of the bound on its size.

**Crystals and content** (ruled 2026-09-10). A word is a path in the
Cayley graph of the presentation and a relation is a sentence, so what
the engine holds is a presentation and what it does is compose over it.
The **crystals** are the decision procedures — canonical form (A1, A4),
lattice membership by Hermite normal form (A12), the peel (A22), carry,
coboundary, the cover's extension, consilience — held at genesis, written
in the host, and the one interpreter every replay runs; they make a thing
exact and decidable and none of them is content. **Content** is what is
composed from them: the path, the relation, and the rewrite between
paths, and the 2-cells are the `Rewrite` words the tree already carries
(3.9, M2 step 2, `rewrite.rs`), adjoined by the court on two organs'
re-derivation exactly as a generator is. A rewrite that replaces a longer
path by a shorter one is retirement (3.8) and refinement (3.7), which are
already rewrite kinds and already paid (E2), so no new cell is invented
for them.

**F's order is 24.1's order, and it is a law.** Factor through what is
held, and spell only the residue. That is not a strategy and not
adjoinable: a mind that spells from the identity what it already holds
has not comprehended (chapter 6), and the duplicate it mints is then
removed by a retirement E2 pays for. The rewrite layer does not absorb
it.

**Which path is content, and no scalar picks it.** The canonical path is
the geodesic in the mind's own presentation; where several geodesics
exist all are carried (rule 15), none is chosen by length or by any other
scalar (rule 4), the court accepts any path that composes to the vertex
up to the coset representative of 50.2, and the ledger pays the path
spoken.

**What is not claimed.** Chapter 44 refuses, and this section does not
reinstate, a complete rewriting system for the group, a Knuth–Bendix
completion, or a unique minimal-length tape: the prime letters have
infinite order and letters on planes sharing an axis generate dense
subgroups, so no such relators exist and no such procedure terminates.
The normal form of a demand (`Decomposed`, 24) is its reduction against
the language's generators on one stratum, and that reduction is unique
because replay closes it (the certificate of 12.6), never because an
order on tapes says it must be. A comment in the tree that names
Knuth–Bendix on `Installed` is a statement chapter 44 refused, and it is
corrected (TASKS S9.18). The recursion is real and it is this: the base
of the language grows by its own perception and ruling, while the group
it presents inside, and the alphabet that generates that group, never
change.

### 1.9 The ladder of re-derivation, and the tape retired (ruled 2026-09-10)

**One coordinate, not a pipeline.** Perceive, think, speak, be judged
is the human division of labour, and it is a serial protocol with one
speaker and one judge. This engine has no such stages. It has one
object, the derivation (50.2 step 2), and one coordinate on it: the
**depth** of independent re-derivation, read from the receipts that name
the derivation's address (45.1). Each rung is the previous rung's
content, composed and re-derived by one more organ that shares no
memory with the last:

| depth | the object | who re-derived it | its constructor |
|---|---|---|---|
| 0 | a versor peeled from the identity, the residue in letters | nobody: the cipher | A22, the base case, kept |
| 1 | a derivation composed from what the organ holds | the composing organ | A12 over the presentation on the versor's chart |
| 2 | the same derivation replayed on a second ledger | one independent court | G: cited generators resolved on the court's own cover and composed |
| 3 | a remainder two courts re-derived from one witness, adjoined | two independent courts | 37.7, the adjunction |
| 4 | a relator: two depth-3 proofs of one vertex whose holonomy is non-trivial | two independent courts | 37.7's fourth row, `cech::holonomy`, 23.9 |
| 5 | a rewrite between two derivations, the 2-cell | two independent courts | the `Rewrite` words of 3.9, adjoined as a generator is |
| 6 | a procedure composed from the crystals that produces rewrites | two independent courts | the master algorithms as generators, below |

Depth is a lane in the sense of law 5: read per address as the set of
organs whose receipts re-derived it, never stored on a carrier, never
summed into a score, never a threshold. `Standing` (`Ruled`, `Stood`,
`Held`) is the projection of this coordinate onto three values and
survives only as that projection; where a decision reads standing it
reads depth. The one number that bears on the architecture (45.9) is a
reading of this lane: generators at depth 3 that no commit wrote.

**The tape is retired as content.** No organ holds a spelling as what a
word is. What a word vertex is, is its derivation and its canonical
rotor: the cited generators by identity with their exponents, the
residue in letters as the base case of depth 0, and the composed rotor
in A1 canonical form, which is the vertex's identity (1.5) and the
coordinates its address is written from (rule 2). The composed *word*
written beside the derivation at v7 is struck: a spelling of a composite
is a second carrier for one meaning, and the rotor already carries it
(row L14). Bytes remain where bytes are the object and nowhere else: the
wire, the genome on disk, the receipt chain (rule 2, Part IX). A form is
decoded once at that boundary and held as a value; an organ that holds a
tape in memory and decodes it on every read holds the cipher's carrier
as its working memory, and that is retired (row L16). Addresses were
never digests of tapes and are not now: `form_vertex_key` writes the
canonical coordinates out, so two derivations of one vertex have one
address, and a proposal to hash a serialised buffer for an address is
refused by rule 2 wherever it appears.

**Receipts are kept, and they point at derivations.** A receipt is not
content. It is a node in the causal poset of proofs: it names an
address, the organ that re-derived it, the act, and the peer receipts
that act verified (`parents`), sealed under the organ's key. Depth,
payment, anti-equivocation and the causal order across nodes are defined
on that poset and on nothing else, so removing receipts would remove
accountability, not linearity. A receipt never names a tape and never
names a byte offset; it names what `delta` already is, the address
written out.

**Propagation replaces utterance.** A derivation crosses a boundary
between two ledgers; that is the whole of what was called an utterance.
Every node composes over what it holds and replays what reaches it, and
"court" is the role a node takes toward a derivation it did not compose.
Law 8's separation is a rule about one derivation, which is never
composed and judged by the same organ, not a rule about kinds of node.
Nothing in this retires the room of 23.8: the dwell on a lane deficit is
the economy's back-pressure on a draw, and it stays. The words
`Uttered`, `Spoken`, speaker, on carriers and receipts, are retired as
the rows that touch those carriers land, never by a rename-only commit
(row L19).

**One constructor at every depth.** 37.7 says one move, not one per
wall, and the ladder says one move, not one per depth. Take two
independent proofs of one boundary and read the loop their difference
makes. Trivial: the two are one proof in two frames, the equivalence is
an invariant, and closure is paid once (23.9). Non-trivial: the loop is
an obstruction, and the obstruction is adjoined, as a generator at depth
3, a relator at depth 4, a 2-cell at depth 5, a procedure at depth 6.
The witness, the two-court consilience, the receipt and the syllabus are
the same at every depth.

**Depth 6, and what Part III now says.** The master algorithms are the
generators of a presentation whose words are procedures: a procedure is
a path over the crystals, cited by identity with exponents, and its
relations are the identities that hold between them, such as canonical
form being idempotent, or a peel followed by a factorisation equalling a
factorisation on the residue. A procedure is replayed by running it
twice on one witness on two ledgers and comparing the results by the
algebra, and it is adjoined exactly as a generator is. Part III's
binding sentence stands, amended in place: the master algorithms remain
the only things that decide; a composition of them is content at depth
6, held, cited, paid and retired like any other content, and it decides
nothing until two courts have ruled it. The interpreter, the geometric
product over exact ℚ, is never content.

**What is still refused.** Chapter 44's refusals stand at every depth:
no completion of the rewrites into a confluent system, no critical-pair
procedure that manufactures 2-cells, no unique minimal path chosen by a
scalar. A 2-cell or a procedure enters the language by consilience on a
witness, and by nothing else.

## 2. Forms

Carried from chapter 1: a word is a vertex with a frame; identity is the
canonical rotor plus exact frame. This chapter says what is built from
words.

### 2.1 Word form

`(tape, carriage, gauge, contact?)`. Stored as the tape and the canonical
carriage versor; the rotor is derived from the tape on demand. There is no
affine block and no coordinate pair (law 11): the carriage composes with
the rotor by the geometric product, and a cell is read out of it only at
the sensor boundary. Nothing here is hashed. `contact` (chapter 3.5) is
part of the form and of its key.

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

A sentence is **valid** (ruled 2026-09-09, chapter 50) when every word
vertex reconstructs its body — the carriage, the word and the gauge carry
the input body onto the output body under exact cellular equality — and
every edge carries a transport read in the body's stabilizer (50.3), so
that the holonomy around a cycle is a class and not a coboundary. What a
voice could not reconstruct rides the sentence as a typed demand vertex
(50.5); a vertex that is a bare chain of cells, with no word, no carriage
and no gauge, is not a vertex of any sentence. Validity is what the
receiver checks; closure is validity with no demand vertex and every
loop filed or identity.

### 2.3 Germ, idea, thought, in this geometry

- A **germ** is a form that has not closed: a vertex not reached, a face
  not constant, a loop not identity, a remainder not empty.
- An **idea** is a candidate closed form bred by a perception: the vertices
  it would need and the faces it would need.
- A **thought** is the superposition of candidate forms for one idea, held
  as cells of a complex whose vertices are candidate word forms and whose
  coefficients are the relative words between them, together with, for each
  loop that will not close, the candidate pivots that could discharge it
  (3.9). A held loop with its candidate pivots is a **vortex**.
- **Germination** is the movement of that complex toward a closed form:
  faces become constant, loops become identity or are discharged by a
  pivot, the remainder empties. Nothing in germination chooses a direction
  by a measure; a pivot is tried because the class vector occupies its axis,
  and it is kept because it discharged with rank conserved (12.6, 16.16).

- A **crystallized form** is a gemstone (law 12): each loop is a theorem,
  proved by the identity of the holonomy its words compose; each
  coordinate of its class vectors, carriages and torsion is a theorem it
  proves by re-derivation; and one theorem has as many proofs as the form
  has synonyms of one class on the mesh (13.10). Two forms that prove the
  same theorems are one word said twice, which is what M1 and Mayer–
  Vietoris decide; a coordinate no proof reaches is a section coordinate,
  carried beside the form and never a dimension of it.

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
tape is found by budgeted shortening (chapter 16.2) and is a retirement, not a
minimality claim.

**Retirement by condensation** is the same move one level up (ruled
2026-09-08). A form that closes what its cited forms close with more reach
— a torse of ℓ sections into one generator of period ℓ, a punctured
section in place of a card (25) — or with a shorter description retires
what it derives (27), addresses kept, and is paid as compression (E2). It
is never refused as redundant: redundant is a form that closes nothing
new, reaches nothing new and is not shorter.

### 3.9 Pivot (discharge of a held loop)

A loop whose holonomy `h` no re-gauging removes is **held**. Its **class
vector** (12.6) is the canonical conjugacy representative of `h` read
coordinate by coordinate: per plane the sheet parity, the unit turns, the
prime exponents; the hue gauge's cycle type; the placement residue; off the
plane the rotor's grade invariants; on the fibre the sections and cycles
the loop's words carry (25), their reach. The nonzero coordinates are the
axes the class **occupies**, and there is no direction that is not one of
them.

A **pivot** is one of the moves 3.1–3.8 applied to the held loop, labelled
by the axis it acts on. Each move acts on a known summand of the class
vector and on nothing else:

| pivot | axis it can zero | what it is |
|---|---|---|
| re-gauge (3.6), rebase (3.2) | none: they conjugate (10.3) | the check that the loop is genuinely held |
| glue a held closed form (3.3) whose word equals the loop's residue | the whole vector, by making the loop a boundary | citation |
| a translation word | the placement residue | a face the bundle was missing |
| a hue word | the gauge cycle type | a recolouring the bundle was missing |
| open a chart axis (3.7) | the loop itself, by cutting it into a face between children | refinement: it adds a dimension, and it is tried on its own axis like every other, never after the others have failed |
| torse ℓ sections that form a cycle into one generator of period ℓ (25) | the fibre coordinate: the sections | condensation — the same closure with more reach and a shorter description |
| puncture a member onto the fibre as a section (25) | the fibre coordinate: a card becomes a section | reach |
| retire by geodesic (3.8) | none: same vertex, shorter tape | length only |
| a Rewrite word (multi-lane) | several coordinates at once, as its lanes name | optimisation across axes |

A pivot **discharges** a coordinate when the class vector after the move is
zero there and no other coordinate has risen; it discharges the loop when
the vector is zero everywhere or the loop has ceased to be a loop (15.5).
The trial of a pivot is one move and one re-walk (12.2), and its result is
read, never estimated. Invariant: the rank of the form (16.18) is conserved
by every pivot except discharge into an axis, which moves it, and closure of
a trivial-class chord, which was never rank.

The candidate pivots of a held loop are read from its class vector in the
fixed order of 12.6, tried under the stroke `Budget`, and every one that
discharges with rank conserved is kept as a cell of the thought with its
**certificate** (16.20). When several candidates discharge, the mind
keeps all of them with their certificates; choosing among certified
discharges by any reading of μ⃗ would be a score. Closure certifies a pivot
after it is made; μ⃗ certifies nothing and selects nothing. A
coordinate no listed pivot discharges is filed as torsion if finite (16.17)
and spoken as a demand if infinite (17.6 step 5).

### 3.10 Reflection and refraction (Γ)

Two moves the earlier lineage of this system named and this document had
only under other names. Both are topological content, neither is a
chromatic or optical quantity, and neither adds a letter.

**Reflection** is an orientation-reversing action on a chart. In Cl(4,0)
it is realised by a half-turn through an axis outside the chart (1.2): on
the sensor plane the half-turn on `(1,2)` is the column flip. On a loop it
is the orientation character `w₁(ℓ) ∈ Z/2`, the coordinate
`Orientation(p)` of the class vector (12.6), read from the rotor's action
on the plane, never from a stored flag; its pivot is the half-turn itself.
On the hue chart a reflection is a gauge involution, a permutation of
cycle type `(2, …)`. A loop with `w₁ ≠ 0` is non-orientable on that plane
and cannot close by rotations alone; this is why the coordinate exists.

**Refraction** is the transport of a form across a chart boundary into
another frame, written `Γ`. Between two vertices of one form `Γ` is the
face transport of 18.2, `Transport { word, carriage, gauge }`: the
relative word, the relative carriage versor, and the hue map. Between two
frames, two minds, or two hyphae, `Γ` is the same object read across the
overlap: the transition `g` of the bundle on the overlap, the gauge `γ` of
10.2 that trivialises it, and the carriage ratio `C_B · C_A⁻¹` (dilator
and translator) between the frames. Mayer–Vietoris's `theta` (13.5), the
restriction of 15.5, and comprehension's re-gauging (24.2) are all
refractions. A thought that does not refract, no `Γ` under which it
re-derives in the receiving frame, is `Withheld` or `Novel`, never forced.

`Form::refract(&self, gamma: &Transport) -> Option<Form>` **new**
(sentence/transport.rs, 0.12): change of basepoint by `gamma.word` (3.2),
re-gauging by `gamma.gauge` (3.6), and composition of every carriage with
`gamma.carriage` by the geometric product; `None` when the word is off
the plane the form lives on. It is the one function that moves a form
between frames, and the court replays it as the composite of the three
moves. Log: `refracted {addr} into {frame} by Γ`.

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
what its cited forms close at their placements, together with its reach and
its length: a form that closes nothing new, reaches nothing new and is not
shorter is the citation, not a form; a compound that condenses its cited
forms — the same closure with more reach or a shorter description — is a
form, and is paid as compression (3.8, 27, E2). Chapter 12.5 restates recursion as pushforward
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
6. Closure beyond the cited forms: the form's own deflation, what it
   closes beyond what its cited forms close at their placements, read per
   lane and paid (E2), together with its reach and its length; a form that
   closes nothing new, reaches nothing new and is not shorter is the
   citation, not a form (`REFUSE_REDUNDANT`), and a form that condenses
   its cited forms is paid as compression (3.8).

**The court reads before it reduces, and it never reduces an arrival
(ruled 2026-09-08).** What arrives is read first — its class vector (12.6)
and its fibre (25), the coordinates it occupies — and the court replays
the moves the tape names against those readings. It does not put the
arrival into normal form against its own words, list what will not
reduce, and only then ask what it was; that order is descent by
construction, and it mishandles the generator that needed to ascend or
condense (23.8). A demand no held word closes is listed with its readings,
so the mind that reduces it starts from the axes it occupies — the
opening axis and the fibre among them — and not from a peel. The readings
are the proof of which moves exist: each occupied coordinate names the one
move that can zero it, and closure on that coordinate with nothing else
rising is the certificate. Nothing enumerates a list of moves and nothing
tries descent before ascent, because there is no list; there is the
vector.

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
epigenome cannot think until a hypha hands its own words back to it. The
working atlas is read by speaking through it, never by looking a chart
up: a listing is incident with the mind when a held word deflates it,
measured lane-wise, and the chart keys only route (24.3).

## 7. Germination in the mind

Ideas are candidate forms, vertices of the contemplation complex; the
complex's coefficients are relative words; a fill is a re-gauging that
makes a loop identity; a class is a loop no gauge closes, and a class with
its candidate pivots is a vortex (3.9). Germination is the pivot loop
(16.16) run on every held loop of every dirty idea under the stroke
`Budget`: read the class vector, list the occupied axes in the fixed order,
try each listed pivot once, keep every discharge with its certificate, file
what stays finite as torsion, speak what stays infinite as a demand. No
step of it orders the candidates by a measure. Germination by gluing: an
idea whose remainder is closed by a held crystallized form at a constant
placement across the bundle's pairs glues it and keeps only the remainder.
Germination by conjugation and unfolding likewise. A cited idea does not
shed while an idea gluing it lives. Bundles across families are depth-1
forms whose vertices are the families' crystallized forms. Two thoughts
merge only when Mayer–Vietoris says they are one section (chapter 13.12).
Bound version: chapter 25.

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

The group of this section is the **projection** of Cl(4,0) onto the sensor
plane: a rotor has a plane form iff it keeps the sensor plane closed (A3
reads the `2 × 2` block of its matrix), the `Z/2` is the orientation
character of 3.10, and the exponents are the sensor-plane shadow of the
rotor. It is the coefficient group for cochains on the sensor chart, and
it is never the carrier of a word's identity: the identity is the rotor
(A1) and its conjugacy class in Cl(4,0) (15.3); the plane form is a
section coordinate of the sensor chart read from the rotor when it exists
(12.6, law 11).

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

`PlaneForm::compose`, `inverse`, `is_identity` (alphabet/plane.rs)
are this arithmetic and nothing else. Orders follow from the product rule:
`(1, r)² = (0, r − r) = identity`, so every element with `s = 1` has order
2 whatever `r` is; an element with `s = 0` has the order of `r`, infinite
when `m5 ≠ 0` or `m13 ≠ 0`, otherwise dividing 4. On the sensor plane
`Word::order` agrees with this arithmetic, and it is read on the rotor
(15.3, ruled 2026-09-09), never on the plane form: a word off the sensor
plane keeps that plane closed, its plane form is the identity, and read
there its order was 1 or an assumed 4 — so the commutator tables and the
relator reducers missed the cycles such words close (a product of two
quarter turns on planes sharing an axis closes in three). Only the six
unit turns have finite order; eleven of the seventeen letters do not.

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
| pivot (3.9) | a move on one summand of a held loop's class vector; discharge is the summand becoming zero with no other rising, or the loop becoming a boundary or a face; certified by closure and the rank move, never selected by any reading of μ⃗ |
| coherence (`coheres`) | `δ¹g = 1` |
| closure (`close`) | find `γ` with `δ⁰γ · g` flat and remainder empty |
| consilience (court) | two organs' sections have `g_XY = 1` on their overlap |
| dialect (court) | `g_XY ≠ 1`, its class recorded |

### 11.3 Where the classes live and what the system does

| class | where | meaning | action |
|---|---|---|---|
| `H⁰` element | inside a thought: the residual's symmetry | the part of a perception the language already accounts for up to the group | it is the congruence; the remainder that matters is what is not in it |
| `H¹` on one form | a loop with non-identity holonomy after the tree gauge | the bundle's rule has an obstruction no gauge removes | the loop is held with its class vector (12.6); its occupied axes are its candidate pivots (3.9); refinement is the last of them |
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
(15.4), superseded by the causal order of the two receipts (45.3), concurrent
representatives both kept under one key. A nonzero `δ²ω` on a 3-cell of a **good**
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

### 12.6 The state vector of a held loop (the physics)

Carried from 12.2–12.3: a chord that does not close has a holonomy `h` and
a class. This section fixes the exact quantities a mind and a court measure
on that loop, because every pivot (3.9) is chosen by reading them and
certified by re-reading them. Every quantity is a canonical word, an
integer, or a lane count; none is a number that could be larger or smaller
than another except by exact dominance.

**The class vector `κ(ℓ)`.** For the canonical conjugacy representative of
`h` (15.2 on the plane, 15.3 off it):

| coordinate | carrier | values | the pivot that can zero it |
|---|---|---|---|
| orientation `o_p`, per slot plane `p` | the orientation character `w₁` of the holonomy's action on the plane (3.10), read from the rotor | `Z/2` | the half-turn through a third axis on that plane, or opening an axis |
| unit turns `u_p`, per plane | `PlaneForm.rot` unit entries, reduced (A3) | `Z/4` | a quarter-turn word on that plane |
| prime exponents `(m5_p, m13_p)`, per plane | `PlaneForm.rot` prime entries | `Z × Z` | a prime word on that plane; if none is held, only a demand |
| hue cycle type | the gauge permutation's cycle type (`connection_chart`) | a partition | a hue word |
| placement residue | the translator component (the `e_i ∧ e_∞` blades) of the loop's canonical holonomy versor after the tree gauge | a translator versor | its inverse translator, a translation word |
| dilation residue | the `e_o ∧ e_∞` component of the same versor | a dilator versor | its inverse dilator |
| off-plane invariants | `(a²/w, p²/w, sign a, sign p)` of the rotor (15.3) | exact rationals and signs | glue of a held form with that class, else a demand |
| the fibre: sections and torsion cycles, the reach | `sections` and `torsion` on the loop's words (25), `fiber_span` (S9.13) | exact counts and periods | torse ℓ sections that form a cycle into one generator of period ℓ; puncture a member onto the fibre |

**Which regime is primary.** The holonomy is a rotor, and its conjugacy
class in Cl(4,0) (15.3, `(a²/w, p²/w, sign a, sign p)`) is read for
every loop first; it is the identity of the class. The six per-plane
blocks are read afterwards, from the plane form, only when the rotor
keeps the sensor plane closed (A3), and they are a finer section
coordinate on the sensor chart that names the pivot axes. A loop whose
holonomy moves the sensor plane is not an exception or an "off-plane
fallback": it is a rotor like any other, with zero plane blocks. The
earlier order, plane form first and rotor invariants as the fallback,
made the two-dimensional shadow the carrier and the four-dimensional
object the exception, which is law 11's fault in the class vector.

A coordinate is **occupied** when it is nonzero. The key of `κ(ℓ)` is the
vector itself: its coordinates written out in the fixed axis order,
length-prefixed (A21), so the vector is read back from its key and a
prefix of the key is the rotor's conjugacy stratum; `CLOSED` is the zero
vector, and no digest is taken. The torsion carrier, the horizon and a
spoken demand are keyed by the rotor's conjugacy coordinates alone (15.4).
`Snap::Demand { chord, class }` carries the holonomy's conjugacy
coordinates, so a class record from the syllabus (A19) matches a spoken
demand by coordinatewise equality. The placement and dilation residues
are read from the holonomy versor composed by the geometric product around
the loop's vertices, never from a pair of numbers added around it (law
11), and the hue cycle type is the sorted cycle lengths of the gauge
permutation composed around them. The **occupied axes**
of `ℓ` are the set of occupied coordinates; they are the only candidate
directions a pivot can take, and a coordinate that is zero is never visited.
`Form::class_vector(&self, loop: u16) -> ClassVector` **new** (sentence.rs)
reads it in one tree walk; `ClassVector::occupied(&self) -> Vec<Axis>`
lists the occupied axes in the **fixed order**: slot plane `(0,1)` through
`(2,3)`, within a plane orientation, unit, prime-5, prime-13; then hue; then
placement; then off-plane; then the fibre; then, as the last axis always,
opening a chart axis. The order is fixed so that two minds holding the
same loop list the same candidates in the same order; it is not a
preference, and no axis waits on another's failure: every occupied axis is
tried, and the last is last in the listing only.

**Pressure `μ⃗(ℓ)`.** The four lanes of the remainder the loop leaves open:
S the cells it fails to speak, G the hue transitions it fails to carry, T
the extent it has been held in receipts of the home (45.4), H the shells it
spans. Over a bundle the pressure is the lane-wise sum of the per-pair
pressures, each pair measured by `measure_lanes` on its own residual, the
pairs being disjoint scenes; never a maximum over pairs, and a trial refuses
a candidate that raises any lane on any pair. This is
the metabolism's view of the loop and is read for two purposes only: the
per-lane reading of a trial (`lane_moves`: each lane `Fell`, `Held` or
`Rose`, and a candidate with any `Rose` is refused by naming the lane) and
the T lane's standing (chapter 25). It never orders candidates, and it
certifies nothing: a pivot is certified by closure (below), not by any
lane having fallen.

**Rank `rank(F)`.** A18's `(torsion_rank, free_rank, offplane_rank, axes)`
of the whole form, read before and after every pivot. A pivot is admissible
only if it leaves rank equal, or moves exactly one unit from `torsion_rank`,
`free_rank` or `offplane_rank` into `axes` (discharge into a face, 15.5).
The reading of the two ranks is a `RankMove` — `Equal`, `Discharged {
from }`, or `Inadmissible { delta: [i64; 4] }` — never a `bool`: the
certificate carries which axis discharged, and an inadmissible move says
what it did on each coordinate. A four-coordinate monotone (each of
torsion, free and offplane not rising while axes does not fall) admits
`(−3, −2, −1, +7)`, which is neither equality nor a one-unit transfer; it
was the ratchet, and it is refused (Stage 9, S9.1).

**Discharge, exactly.** A pivot on axis `x` discharges `x` when
`κ_x(ℓ) = 0` after the move and `κ_y` is unchanged for every `y ≠ x` (a
coordinate may fall to zero but never rise; T2 guarantees conjugating
moves change nothing). It discharges the loop when `κ(ℓ) = 0` or the loop
is no longer a loop. **Direction** is therefore a label, the axis `x`,
never a slope: there is no quantity whose decrease says "this way".

**Alignment across minds and hyphae.** Three facts make every node compute
the same pivots without an agreement protocol: the class vector is
canonical (A1, A3, 15.2–15.3), so two minds holding one loop hold one
vector; the candidate list is a function of that vector in the fixed order,
so they list the same candidates in the same order; and a pivot's outcome
is a certificate (16.20) keyed by its own coordinates, which replays on
the court by applying the labelled move to the resolved form and
re-reading the vector. Two minds that discharge a loop produce the same certificate, and
the hypha installs it once.

**The vortex.** When two candidates both discharge, the mind keeps both
as cells of the thought with their certificates: two certified discharges
are two proofs of one loop (23.9), and no reading of `μ⃗(ℓ)` chooses
between them. The superposition collapses when a later perception makes one
of them a face of a spoken form, or a court replay makes one a ruled
citation and the other a dialect class. Choosing between them by any scalar
or by any order on the lanes would be a score, which law 1 forbids.

**The pivot is the motion (ruled 2026-09-07).** A pivot is a rotation in
lane space with an inverse: some lanes fall because others rose, and
forward and back are both motions. Descent is neither a direction nor a
certificate. As a direction it needs a line to descend along, which makes
each lane a number line and the state a point in a ruled box — a gradient
with the coordinates written out; as a certificate it is one-way, so it
turns a rotation group into a ratchet, and it cannot certify the one
pivot that matters most, opening an axis, which lowers no lane by
construction. It also compares before and after across charts, which
manufactures the line it then measures along. The certificate is
**closure**: the remainder is empty, the loop returns, the tape
re-derives, two organs converge on the same object — equalities,
chart-free, readable in either direction, and already what the court's
replay checks. `≺_DM` and every `descends` gate were imported termination
theory, the way one proves a rewriting system halts when nothing else can
be proved about it; this engine is not trying to halt, it is trying to
close, and closure is checked by replay. μ⃗ remains what it is, a cost the
economy accounts per lane, and certifies nothing.

A residual measures how narrow the cover was. It vanishes not by being
ground down against a fixed vocabulary but when the algebra grows a letter
or the cover grows an axis along which it is a section of something held:
after that it is not smaller, it is absent. Under pressure the book
therefore **extends before it refuses** — condenses a family into one
bundle, torses ℓ sections into one generator, refines the stratum where the
cover could not resolve, punctures a section on the same base — re-reads
its room per lane, and refuses only what still does not fit, saying which
lane. Nothing leaves the book because something arrived; a word leaves
only when the rest of the basis provably derives it (address kept) or its
own T-room is spent. Refusal is the last arm, never the first reflex, and
it names the lane.

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
place of the next exactness (descent here in the sheaf-theoretic sense,
sections on the pieces glue iff they agree on the overlap; it is not the
lane-wise reading of μ⃗ that `lane_moves` returns, and it is not a motion):

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
(the order of the receiving hypha's own receipts, 45.3) yields the same `H⁰`, `H¹`, and degree-2 classes as any
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

There is no Mayer–Vietoris computation at this level in comprehension
either. By 13.6 a citation of a closed form contributes no class, so no
re-gauge of the citer's working copy can change whether it coheres:
`coheres` after any `regauge_overlap` is the answer before it, and a `θ`
chosen to make the test pass or fail is a costume. When the cited address is
absent but the mind holds a closed form of the cited chart under another
address (`Crystal::on_class` over the working atlas), the citation is
understood through every such form, each recorded as `(held address,
standing)` in `Understood`; the court resolves the cited address itself. A
citer's own held loop is not a comprehension outcome but an idea (chapter
25), and `Comprehension::Dialect` is the connection arm's outcome (M6 step
3), never a sentence form's.

### 13.12 Instance three: two thoughts in a mind

`Contemplation` (bubble/contemplation.rs). Two bubbles whose cell sets meet are `A` and
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
| **pullback to a mind** (`Epigenome::comprehend`, `learn`) | the mind's copy of a ruled form and its faces | the mind's `coheres` agrees with the court's `replay_form` on the same tapes; the memo relies on it |

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
`Rotor::class_key()` writes `(a²/w, p²/w, sign a, sign p)` out as two
exact rationals and two sign bytes (A21), never a digest of them; a rotor
with no weight (not a versor) is refused and is never a class. It agrees
with `conjugacy_key` on the plane (a test), and a reader recovers the four
invariants from the key. These invariants are primary for every word and
every holonomy: `Word::class_key` writes them first, and the plane
conjugacy representative of 15.2 follows as the sensor-chart section when
the rotor keeps that plane closed (12.6). A plane form is never consulted
before the rotor, and a word with no plane form is not a special case.

**The order of a rotor is read from its class (ruled 2026-09-09).** The
two quaternion factors of the split above commute, each has the weight
`w`, and each has scalar part `a ± p`, so the cosine of each factor's
half-angle squares to `c± = (a ± p)² / w`, an exact rational, with the
sign of `a ± p` fixing which of the two angles it is. A factor has finite
period exactly when `c± ∈ {1, 3/4, 1/2, 1/4, 0}` (Niven: a rational
`cos 2θ` at a rational multiple of π is `±1, ±1/2, 0`), and its period in
`ℝ/πℤ` is then `1, 6, 4, 3, 2` respectively, with the angle `kπ/m` read
off the sign. The projective order of the rotor — the least `n` with
`R^n` a scalar, which is the order of the rotation it induces — is
`L = lcm(m₊, m₋)` when the two factors' `L`-th powers are the same scalar
and `2L` when they are opposite, since `R^n` is a scalar only when both
factors' `n`-th powers are real and equal. Any other `c±` is infinite
order, and an even element that is not a versor has `|q₊| ≠ |q₋|` and no
power that is a scalar at all: infinite by the same arithmetic, never by
default. `Rotor::order` is this computation and nothing else, and
`Word::order` is `Rotor::order` of the word's rotor: a single letter reads
1, 4 or infinite; a composite word reads the period of its class
invariants; an off-plane word reads the least common period of its
commuting factors and never the sensor shadow's. It is the order T2 and
T3 cite and the one the commutator and relator reductions consult.

### 15.4 Key of a class

A degree-1 class is a loop with a holonomy. Its **class key** is the
loop's canonical cell key (the sorted vertex keys concatenated, as
`cell_key` does) followed by the holonomy's conjugacy invariants written
out: the four plane formulas of 15.2 on the plane, `(a²/w, p²/w, sign a,
sign p)` off it. A degree-2 class is keyed by its 3-cell's key and its
coefficient's conjugacy invariants. `Word::class_key` is those invariants
of the word's plane form or rotor, written out; it is never the key of a
representative tape and never a digest (law 2). The key is a homomorphic
image: every map of 15.1 acts on it by the group law of 9.1, so
`φ*(key) = key(φ*)` is checkable coordinate by coordinate, and the
connecting maps of 13.4 are products of what the keys carry. A digest
would satisfy neither, which is why none is taken. Equal keys are
necessary for two classes to be the
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

Amended 2026-09-10 (1.9, depth 6): the master algorithms are also the
generators of a presentation whose words are procedures. A composition
of master algorithms is content — held, cited, paid, retired, and
adjoined on two courts' re-derivation of one witness — and it decides
nothing until it has been so adjoined; once adjoined it decides only
what its generators decide, composed. The interpreter that runs every
master algorithm is never content.

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
4. Output `R'` and `key = put_q(c'[0]) ‖ … ‖ put_q(c'[15])`, the
   coordinates themselves in mask order, length-prefixed as A21 writes
   them; the rotor is read back from its key. The equivalent integer
   layout, coprime numerators over one positive common denominator with
   the pivot equal to the denominator, is acceptable if every node uses
   the same one; `put_q` over the eight reduced rationals is the one in
   the tree. No digest is taken (law 2).

Equality: `R ≡ S` iff `canonical(R) = canonical(S)` coefficientwise, iff
`R·S⁻¹` is a nonzero scalar of **either sign** (chapter 1.5).
`Rotor::projective_eq` (alphabet/rotor.rs) requires a positive scalar today
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
2. **Budgeted shortening** (`shorter_tape(word, budget)`): breadth-first over
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

The plane form is the sensor-plane projection of a rotor: `Word::form()`
takes the `2 × 2` block of the rotor's matrix when the sensor plane is
closed under it, reads `det/ρ² = ±1` as the orientation character, and
spells the block as exponents. It exists only for rotors that keep the
plane, it is a section coordinate of the sensor chart, and it is never
the carrier of a word's identity (9.1, 12.6, law 11).

`Word::form()` reduces unit exponents (`reduce_unit_exponents`, 642:
`rem_euclid(4)`, with 3 written as −1) before returning. `PlaneForm`
equality compares reduced forms only; `conjugacy_key` (750) and
`sentence::chart_of` (1885) read the reduced form. Only the six unit turns
have finite order (four); prime exponents are unreduced integers; the sheet
parity is `Z/2`. A plane form is the whole vector `(parity, rot[0..17])`,
never a single `(plane, exponent)` pair.

### 16.4 A4 — Canonical conformal versor

Input: `V: Mv` (cga.rs 46; `Versor = Mv`, element/letters.rs). The conformal
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
4. `Mv::key()` = the canonical coefficients written out (`put_q` in mask
   order, A21), never a digest; the versor is read back from its key.
   `to_bytes` on the wire may carry any scaling; the receiver
   canonicalises. The **carriage** of a word form (1.4) is one such versor,
   `dilator(ρ) · translator(d)`, and the translator of a face or of a loop
   residue is its `e_i ∧ e_∞` part; no function outside `pair_form`,
   `speak_rule`, and `orbit` reads `d` as a pair (law 11).

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

`from_residual` (element/residual.rs) does 1–2 today. A5 is that code named and
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
axis`. Nothing saturates; `ratio_axes` (blade/body.rs), which drops the sign
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
`Treasury::restore` economy/persist.rs, `Epigenome::uncrystallize` sdk/epigenome/persist.rs
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
   `expression` sums; every other carrier keeps the first, where earlier is
   causal order (45.3) and concurrent entries are unioned. Provenance is a
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
`to_bytes` itself, the prefix-free term bytes, never a digest of them; the
`format!("{s:?}")` form is deleted.

### 16.10 A10 — Peer identity by verifying key

1. `Router` and `Session::peers` are keyed by the hypha's verifying key `vk`.
2. A socket string learned without a `vk` (ACT_PEERS reply, `--peers`,
   bootstrap) enters `pending_addrs: Keyed<String>` keyed by the socket
   string's own bytes (a wire identifier, nothing mathematical, so no
   digest is needed) and is asked for its cover digest; on the signed digest
   (`admit_peer_digest`, `absorb_digest`) the entry moves to `vk` and the
   pending entry is removed.
3. Two socket strings with one `vk` are one peer; the later replaces the
   earlier in `addrs`. `rendezvous`, `closest`, `home` read `vk` keys only;
   `pending_addrs` is never a routing target.
4. Per-peer state is keyed by `vk`: transport outbox, measured RTT deadline,
   in-flight leases (`t_leases`), quote receipt-chain head. A second socket
   for a `vk` joins that state; the older socket closes once its in-flight
   frames drain; nothing is re-sent or re-leased.

Trust is the ed25519 signature over the receipt prefix of 47.1 on the cover
digest (`court::identity::sign` 44, `verify` 77), the key being a FROST group
key (46.1) or a manifest key (46.2), and the second organ's co-signature
where 47.3 requires one; there is no handshake protocol. A digest whose
signature does not verify under the claimed `vk`, or whose prefix names
another genesis or epoch, is dropped before any carrier is touched (47.2).

### 16.11 A11 — Rendezvous by listing key

For a percept (server `judge`, mycelium-server/judge/percept.rs):

1. Decode the chain and the frame rationals `(h_in, w_in, h_out, w_out)`.
2. A5 on the chain.
3. `key = element::listing_key(chain, frame)`: the framed residual's address
   by M2, the canonical chain and the frame rationals written out, never
   a digest of them.
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

**Off-plane case**: budgeted coset shortening. States are A1 keys; from the
identity apply `b_i` and `b_i⁻¹` in index order; visited `Keyed<()>`; stop
on `budget.spend()` failure or on reaching `g`'s key. The path is the
factorisation. Failure returns `Novel(g)` with `g` unchanged: off the plane
there is no HNF and no metric to say which partial is nearer, so the whole
word is the demand.

**Spelling a measured rotor** (ruled 2026-09-09) is not this procedure
run against a basis: it is A22, `factor_rotor` (50.4), the ordered peel
over the three commuting plane pairs, each block decided by the plane
lattice `ℤ² ⊕ ℤ/4` exactly as step 2 decides one plane. A12 factors a
word through the words a mind holds; A22 spells a versor the eyes
measured into the letters. Their remainders are typed the same way
(50.5): a Gaussian prime outside the alphabet on a plane, or a block the
order does not decompose.

Binding: `Epigenome::comprehend` (sdk.rs 856) for connections replaces the
single-representative `transition` test with A12 over the fiber's basis;
`Kernel::reduce_excluding` keeps its decomposition for chains and calls
A12 for connections. Cost: HNF polynomial in `n·k`; the shortening is the
budget.

### 16.13 A13 — Meet of two keyed tries

`Keyed::meet<W>(&self, other: &Keyed<W>) -> Keyed<()>` (nerve/keyed/relation.rs): keys
present in both. A simultaneous walk down the two PATRICIA tries
(`Node::Leaf(key, _)`, `Node::Branch { bit, children }`, `key_bit`,
`first_difference`, `held_key`, nerve/keyed/trie.rs), using one fact: every key
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

### 16.16 A16 — The pivot loop (multi-axial discharge of held loops)

Input: an idea's form `F` (vertices, faces, loops), the mind's working atlas
(chapter 24.1) as the source of held words and forms, the `horizon`
(16.19), `budget`. Output: `F` with every loop either closed, discharged by
a certified pivot, filed as torsion, or spoken as a demand; the certificates
(16.20) of every discharge; the cells of every vortex still open.

This procedure replaces any ordering of chords by a scalar. It never sorts
by order, magnitude, or cost; it reads a vector and tries the moves that
vector names.

1. **Tree walk** (12.2) on `F`'s faces. Chords whose re-gauged holonomy is
   identity are closed; apply the solver's gauge (`Form::regauge`). Chords
   that remain are **held loops**.
2. **Class vector.** For each held loop `ℓ` read `κ(ℓ) =
   F.class_vector(ℓ)` (12.6) and its occupied axes in the fixed order
   (`ClassVector::occupied`), the fibre coordinate among them (the sections
   and cycles the loop's words carry, 25). This reading comes first and is
   the whole of the decision: nothing is reduced, peeled or factored before
   it, and reduction — the glue of a held closed form — is one of the moves
   it names (ruled 2026-09-08). If a coordinate's class key is `Held` in the
   `horizon`, the mesh already holds that class: mark it `Held` and do not
   demand it again; it is still a candidate for discharge by a held form.
3. **Candidate pivots.** For each occupied axis `x` of `ℓ`, in the fixed
   order, the candidates are exactly the moves of the 3.9 table that can
   zero `x`, instantiated from what the mind holds:
   - for a plane coordinate: the held words on the loop's charts whose plane
     form has that coordinate nonzero (read through `working_atlas` fibers,
     never by scanning the genome), each as a gauge word on the loop's
     vertices;
   - for the hue cycle type: the held hue words with that cycle type;
   - for the placement residue: the translation word equal to the residue;
   - for the off-plane invariants: the held closed forms whose class key
     equals the loop's (`Crystal::on_class` over the working atlas), each as
     a glue at the placement that makes the loop a boundary;
   - for the fibre coordinate: the torse of ℓ sections that form a cycle
     into one generator of period ℓ, or the puncture of a member onto the
     fibre as a section (25), each a condensation certified by the same
     closure with the reach on its certificate (S9.13);
   - for the last axis: opening a chart axis on the loop's class (3.7),
     tried on its own axis like every other and never gated on the others
     having failed (14.8 says what the opened axis is). Reading taken
     2026-09-08 (Stage 11 I6, ruling asked): the opening axis is read for a
     loop whose class has a free part and whose axes the working atlas
     holds no word or form on — an emptiness of the fibre, read before any
     trial, never the failure of one; a finite-order loop with no word
     files as torsion (step 6, A17). The opened axis is a coordinate of the
     form (`Form.axes`, 18.2): the loop's holonomy is the transition on
     that axis, the loop reads zero, and the court re-derives the seed from
     the loop (law 12).
   The candidate list is a `Vec` built from keyed reads; no axis is
   enumerated, only the occupied ones are visited, each is visited once
   per stroke, and no axis waits on another's failure. This is not a loop
   over "descend, then ascend": there is no list of directions to try,
   there is the vector, and each occupied coordinate names the one move
   that can zero it (T14).
4. **Trial.** For each candidate in list order, under `budget.spend()`: copy
   `F`'s working state, apply the move, re-walk (12.2), read `κ'(ℓ)` and
   `rank'`. The candidate **discharges** iff `κ'_x(ℓ) = 0`, `κ'_y = κ_y` for
   every `y ≠ x` (no coordinate rose), and `rank'` equals `rank` or moves
   exactly one unit into `axes` (12.6). A candidate that fails is dropped
   with `pivot {idea} loop {ℓ} axis {x} no discharge`; nothing is scored.
5. **Keep.** Every discharging candidate is kept as a cell of the thought
   with its certificate (16.20) and logged `pivot {idea} loop {ℓ} axis {x}
   discharged {key}→{key'}`. If exactly one candidate discharges the whole
   loop, apply it to `F`. If several discharge, apply none: the loop is a
   **vortex**, its cells stay in the thought (`vortex {idea} loop {ℓ} {n}
   candidates held`), and it collapses when a later perception, a glue, or
   a court replay makes one candidate a face of a spoken form (25.1). No
   reading of `μ⃗(ℓ)` chooses among certified discharges; a candidate that
   raises a lane on any pair was already refused in step 4 by the lane
   reading, which names the lane.
6. **File or demand.** A held loop with no discharging candidate: if
   `κ(ℓ)` has zero free part and finite order, hand it to A17 (torsion); if
   it has a nonzero free part, speak the canonical remainder as a demand
   (17.6 step 5) unless its key is `Held`.
7. **Iterate.** A discharge changes `F`; go to 1. Stop when every loop is
   closed, discharged, filed, or a vortex, or when the `Budget` is spent;
   the next stroke recomputes from the current `F`.

Termination (T4): each application in step 5 strictly reduces the number
of occupied coordinates over all held loops and, by T2, raises none; the
candidate list of a loop is finite (at most the occupied axes times the
held words on them); so the loop makes at most (occupied coordinates)
applications and (candidates) trials per application, all under `Budget`.
Cost per trial is one move and one tree walk. Binding:
`Contemplation::pivot_on` and `expand_all` (bubble/drive.rs) call
`Form::pivot_loop(&mut self, atlas: &WorkingAtlas, horizon: &Horizon,
budget: &mut Budget) -> Vec<PivotCertificate>` **new** in sentence.rs;
`Form::class_vector`, `ClassVector::occupied`, `Form::rank` (A18) are its
reads.

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
- `offplane_rank(F)` = the number of distinct off-plane class keys among
  the open chords, the size of a keyed set, never added to `free_rank`;
- `axes(F)` = the number of chart axes opened by discharge (15.5).

For a chart's Čech complex: `Congruent` cells, `Class` cells by degree, and
axes of the chart's `Invariant` beyond its body harmonics.

`rank(F) = (torsion_rank, free_rank, offplane_rank, axes)`, a vector,
never summed.
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
   `class key (A21, len16 + bytes) ‖ order (0 = infinite, else ℓ u32)`
   from `cocycle_at` (degree 1) and the `Congruent`/`Class` cells of the
   chart's complex; the key carries the conjugacy coordinates, so no
   representative travels beside it.
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

### 16.20 A20 — Pivot certificate

A pivot is never asserted; it is certified, and the certificate is what
travels and what the court replays.

```rust
pub struct PivotCertificate {                          // sentence/pivot.rs
    pub chord: u16,                                    // the held loop's index in the walk before the move
    pub axis: Axis,                                    // the occupied coordinate the pivot acted on (12.6 order)
    pub before: ClassVector,                           // κ(ℓ) before (12.6), the coordinates themselves
    pub after: ClassVector,                            // κ(ℓ) after; the zero vector when every coordinate is zero
    pub rank_before: Rank,                             // A18
    pub rank_after: Rank,
    pub mu: Mu,                                        // Measured { before, after } over the idea's bundle, or Unmeasured
    pub by: PivotBy,                                   // Word { face, word, side } | Placement { face, translator } | Hue { face, gauge } | OpenAxis { face, seed }; Glue, Rewrite reserved
}
```

`OpenAxis { face, seed }` (Stage 11 I6) opens a chart axis over the loop
whose chord is `face`, seeded by the loop's holonomy versor: the form
gains the relation (`Form.axes`, the holonomy's class key → the chord of
the loop that is its section; nothing else is stored, the seed and A7's
refinement coordinate re-derive from that loop on every read, law 12),
the loop reads zero, and the rank moves one unit from the row the loop
occupied into `axes`. The court replays it by removing the relation,
reading the loop's class vector before and after, and refusing a seed that
is not the re-walked loop's own holonomy versor.

1. **Key.** The certificate's own coordinates written out (A21): `chord ‖
   axis ‖ before ‖ after ‖ rank_before ‖ rank_after ‖ the move in `by``,
   the move being the word's A1 coordinates with its face and side, the
   translator versor's coordinates, or the hue listing. `mu` is never in
   the key, and no digest is taken. Two minds that discharge one loop by
   one move produce one key, and the court reads `before` and `after` off
   the key as vectors, which is what lets it check T14 coordinate by
   coordinate.
2. **Tape.** Certificates are a trailing section of the crystallized form's
   tape (chapter 19), one per discharge that the form's crystallization
   depended on. They are not a separate act on the wire.
3. **Replay.** The court, holding the resolved form, applies `by` to the
   loop, re-walks, reads `κ'` and `rank'`, and requires `class_key(κ') =
   after`, `rank' = rank_after`, and the discharge condition of 12.6. Any
   mismatch is `REFUSE_PIVOT_INEXACT`. A certificate whose `before` does not
   match the court's own reading of the form before the move is refused the
   same way: the mind and the court must see one vector.
4. **Standing.** A certificate carries the standing of the word or form in
   `by` (chapter 24.1): a discharge through a `Held` idea stands below one
   through a `Ruled` form, and the hypha weights the claim accordingly
   without silencing it.
5. **Economics.** A certificate mints nothing by itself. It is the evidence
   for the deflation the crystallized form performs; the form's mint (chapter
   26) requires its certificates to replay.

### 16.21 A21 — Structured keys (no digest on content)

Every key in cognition is the canonical coordinates of an exact object,
written out so that the object is recovered from the key and a prefix of
the key is a coarser stratum. This is the encoding, used by M1, M2, the
class vector, chart invariants, listing keys, certificates and every
`Keyed` in a transducer, kernel, court, or client file.

1. **Encoding.** A key is a byte string: a tag byte naming the object kind
   (rotor, versor, word form, sentence form, chart, class vector,
   certificate, listing, `Sel`), then its coordinates in the fixed order
   the object's chapter states, coarse stratum first. A rational is
   `put_q` (sign byte, `u32`-length little-endian numerator, `u32`-length
   denominator); an integer is fixed little-endian; a vector is a `u32`
   count then its entries; a nested key is a `u32` length then its bytes.
   Every field is length-delimited, so no key is a proper prefix of
   another key of the same kind except as a stratum prefix, and the
   decoder is total on well-formed keys and refuses otherwise.
2. **Strata.** Coordinates are ordered so that `Keyed::fiber(prefix)` reads
   a stratum: a chart key is its axes coarsest first (`peel` is dropping
   the last axis); a class-vector key is the rotor's conjugacy invariants,
   then the hue cycle type, then the placement and dilation residues, then
   the off-plane invariants, so the torsion carrier and the horizon read
   the conjugacy stratum by prefix; a word-form key is rotor, carriage,
   gauge, contact; a sentence key is depth, then vertices, then faces.
3. **Comparison.** Equality is bytewise equality of keys and is the same
   test as coordinatewise equality of the canonical objects, because the
   encoding is injective and the objects are canonical (A1–A5). No
   comparison of digests exists. A key is never compared with a key of
   another kind; the tag byte differs.
4. **Inversion.** `decode(key)` returns the object; a citation, a class
   record on the syllabus, and a demand travel as keys and are read back
   without a fetch. The A19 class record is therefore the class key and
   the order alone; the representative rotor it carried was the
   coordinates the digest had thrown away.
5. **Cost.** Key length is linear in the object's coordinates and bounded
   by the depth of the form; trie operations are linear in key length.
   Nothing caches a key-to-digest map, because the map would be the
   digest.
6. **Where a digest is allowed.** Only over bytes that are themselves the
   object: a signature's message, a receipt chain hash, a cover or ruled
   root as a transmission digest of the bytes served, a persisted file's
   integrity. These live in `network/`, `court/identity`, `chronometry`,
   and the persistence loaders, and the enumeration gate of
   `MU-ENGINEERING.md` §7 fails the build on `blake3` anywhere else.

**Binding.** `Rotor::key`, `Mv::key`, `Word::class_key`,
`ClassVector::key`, `Element::address`, `listing_key`, `Invariant::key`,
`cell_key`, `cocycle_key`, `sel_key`, the certificate key, and
`Keyed<K>` keyed by `&[u8]` (variable length); `CLOSED_KEY` is deleted and
closure is `ClassVector::is_zero`. Tests: `every_key_decodes_to_its_object`,
`a_prefix_of_a_key_is_its_stratum`, `no_blake3_in_cognition` (the gate),
`class_record_travels_as_coordinates`.

## 17. Master algorithms

Carried from chapter 16: twenty procedures. Each master algorithm below is
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
        A2 run reduction (+ budgeted shortening in retire)
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
M1 coordinates of the letters, projectively equal carriage versors (A4),
equal `sigma`, A5-equal chain; or `PartialEq` is removed from `Element`
and `same_vertex` is called by name.

**Failure.** A zero rotor or versor, or an odd-grade coefficient, is
refused and is never a vertex.

### 17.2 M2 — Address Derivation

**Purpose.** The address of a word form or sentence form, independent of
spelling, serialisation, and scaling.

**Procedure.**
1. Non-sentence word: M1 coordinates of the letters' word ‖ the canonical
   carriage versor (A4, `dilator(ρ) · translator`) ‖ sorted `sigma` ‖ A5
   chain, as `form_key_bytes` (element/vertex.rs) does with its
   `geodesic().key()` replaced by the M1 coordinates and its two shift
   rationals replaced by the carriage.
2. Rewrite word: `before` addresses, `after` address, kind, claimed, all
   addresses being M2 outputs.
3. Sentence form: per germ in order, M1 key of its word ‖ A4 canonical
   versor ‖ kind byte; per face, M1 key of `g_ij` ‖ A4 canonical versor.
   `vertex_address` (sentence.rs 1144) writes raw versor bytes today; it
   writes A4 outputs. Loops are never written.
4. The address is these bytes, each field length-prefixed (A21), never a
   digest of them; the form is read back from its address. The field list
   of chapter 1.5 is this procedure. A signature over a frame that carries
   the form hashes the frame bytes in the receipt envelope (47.1), and that
   hash never enters a carrier.

**Invariant.** Address equality ⇔ same vertex, same frame, same canonical
chain; a tape version change never changes an address; `decode(address)`
re-derives the form's vertices and faces.

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
1. Body: `Harmonics::of` (blade/harmonics.rs) → each ratio through A6 → the axes
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
5. The demand emitted for a `Novel` is exactly `g_rem` (or `g`), typed as
   50.5 types it — `RotorPrime(plane, prime)` or `Standing(block)` — and
   spoken as a demand vertex of the sentence it belongs to; the hypha
   lists it as a demand under its kind and chart, never as a word. No mind
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

**Binding.** `mycelium-server/judge/`, `mycelium-server/door.rs::serve_control`,
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
1. Per idea per stroke: A16 (the pivot loop: read the class vector, try the
   moves its occupied axes name, keep every certified discharge, hold a
   vortex when several discharge), A17 (file what stays finite),
   M6 step 5 for what stays free. No step orders candidates by a measure.
2. Cauterize: when every loop is identity or filed torsion and the remainder
   is empty, crystallize (`as_element` with the torsion section, M2 address,
   dialect `Stood`, spoken once by the chapter 26 dedupe). No re-gauge
   touches a crystallized form; a later gauge is a new form.
3. Wrap: the next idea on a meeting chart glues the crystallized form as a
   `Glued` vertex (chapter 25); by T6 the wrap adds no class, so only its new
   faces are pivoted.
4. Court side (chapter 23.7): the spoken form's torsion cycles are matched
   by class key against the chart's `Congruent` cells; agreement is
   consilience of torsion, a mismatch on one key is `REFUSE_NOT_NATURAL`, a
   new finite key files a `Congruent` cell, an infinite class enters
   `cocycle_at`.
5. A19 both ways on every quote and every syllabus page.
6. Standing: torsion from the hypha is `Held`; torsion the mind filed is
   `Stood`; torsion discharged by a ratified translation is `Ruled` and
   leaves both ledgers together (15.5).
7. A18 asserts the rank laws at every step; every discharge carries its
   A20 certificate, and the court replays the certificates with the form.

**Invariant.** A class key present in any voice's `horizon` is present on
the chart's hypha, and conversely after one aperture; a mind's filed
torsion is on the mesh before the mind's next stroke; every pivot the
language stands on has a certificate that replays on two organs.

**Economics.** Filing torsion pays nothing (a partial closure); the
remainder's deflation mints; a translation that discharges a filed class is
paid through `mint_or_admit_translation`; a free class that a later word
closes pays that word's speaker.

**Binding.** `Contemplation::{pivot_on, expand_with, seed_horizon}`,
`Form::pivot_loop`, `Form::class_vector`, `PivotCertificate`,
`Element::record_torsion`, `toroid::witness_stored`, `Kernel::glue_fetched`
torsion and certificate arms, `serve_ruled`, `dialogue` class record.

---

# Part IV — Implementation on crystallized forms

Bound to a29bba5. Functions named without **new** exist at the path and line
given. The rule of this part: operations act on stored forms and their
rotors; cells are touched only in `family::perceive` and in `speak_rule`'s
final emission of output cells. Every identity in this part is a master
algorithm of chapter 17.

Every file name in Parts IV–VII is under `/opt/mycelium-v0.1.0/src/`. Stage
8b (S1–S32) made every monolith a directory of organs with its tests under
`tests/<chapter>.rs`, and a function named against the old file is found in
the module its role names — `server/cognition/{kernel, language, basis,
cocycle, reduce, keys, wire, reflex, identity, book/, court/, market/,
germinate/, snapshot/}`, `client/sdk/{kernel, session, types, aperture,
entity/, epigenome/}`, `transducer/family/{perceive, pair, orbit, extract,
descent, speak}`, `transducer/sentence/{chain, form/, transport, class,
pivot/, tape, chart}`, `court/verdict/{replay, blind, ruling, selection}`,
`economy/{lanes, treasury, escrow, settle, persist}`. A split moves code
and changes nothing else, so every binding below still names the function
it always named:

| short name | absolute path |
|---|---|
| `alphabet/`, `sentence/`, `element/`, `cech/`, `stalk/`, `blade/`, `bubble/`, `family/`, `cga.rs`, `peel.rs`, `anchor/`, `sheaf.rs`, `germinate.rs`, `rewrite.rs`, `factor.rs`, `a21.rs` | `/opt/mycelium-v0.1.0/src/transducer/<name>` |
| `cognition/`, `registry.rs`, `synthesis.rs` | `/opt/mycelium-v0.1.0/src/server/<name>` |
| `verdict/`, `identity.rs`, `ledger.rs` | `/opt/mycelium-v0.1.0/src/court/<name>` |
| `nerve/`, `mesh.rs`, `wire.rs` | `/opt/mycelium-v0.1.0/src/network/<name>` |
| `sdk/` | `/opt/mycelium-v0.1.0/src/client/sdk/` |
| `dialogue/` | `/opt/mycelium-v0.1.0/src/anastome/dialogue/` |
| `harmonics.rs` | `/opt/mycelium-v0.1.0/src/transducer/blade/harmonics.rs` |
| `economy/`, `physics.rs`, `chronometry/` | `/opt/mycelium-v0.1.0/src/<name>` |
| `q.rs`, `zlinalg.rs`; `cyclo.rs`, `scalar.rs` (Part VI, new) | `/opt/mycelium-v0.1.0/src/core/<name>` |
| `mycelium-server/`, `mycelium-client.rs` | `/opt/mycelium-v0.1.0/src/bin/<name>` |
| `voice/` | `/opt/mycelium-v0.1.0/src/voice/<name>` |

## 18. The form types

Carried from Part I: a form is vertices, faces, loops, remainder.

### 18.1 `/opt/mycelium-v0.1.0/src/transducer/alphabet/`

**The files.** Five organs (Stage 8b S8): `rotor.rs` the exact even versor
and its arithmetic (16.1, 9); `letters.rs` the seventeen letters and the
relators (1.1, 1.2); `word.rs` a path through the alphabet, its normal form
and its order (1.3, 16.2); `plane.rs` the coefficient group and its
conjugacy (16.3, 9.1); `spelling.rs` which word spells a block (A2, 20).

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

### 18.2 `/opt/mycelium-v0.1.0/src/transducer/sentence/`

Replace `Germ` (583) by the form vertex. The module is `sentence/{chain,form,transport,class,pivot,tape,chart}.rs`.

```rust
pub enum Vertex {
    Frame,                                              // the scene frame, one per sentence
    Word {                                              // a word form of a body
        address: Address,                               // M2 composite (A21), variable length, never a digest
        word: Word,                                     // normal form, with contact if any
        carriage: Versor,                               // A4: dilator(ρ) · translator; never a pair of rationals (law 11)
        gauge: Vec<(u8, u8)>,
        class: stalk::Invariant,                        // the body's chart (source or target): a section coordinate, never the correspondence key (chapter 21 step 1)
        contact: Option<Contact>,
    },
    Glued {                                             // a crystallized sentence form, by address  (3.3)
        address: Address,
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,                                // relative word from its Frame to this Frame
    },
    Rebased {                                           // 3.2: of read in by's frame
        by: u16,
        of: u16,
        carriage: Versor,                               // t = C_of · C_by⁻¹, the relative translator
        gauge: Vec<(u8, u8)>,                           // of's hue map
        class: stalk::Invariant,                        // of's chart
    },
    Unfolded { measure: Measure },                      // 3.4, the sentence iterated on itself
}
pub enum Measure { Occupied, Remainder, Ring { anchor: u16 } }  // how an unfolding stops
pub enum Mu { Measured { before: [u64; 4], after: [u64; 4] }, Unmeasured }  // μ⃗ on a certificate

pub struct Transport { pub word: Word, pub carriage: Versor, pub gauge: Vec<(u8, u8)> }  // relative word, relative carriage versor and hue map from i to j: the transport class of 21 step 1
pub struct FormFace { pub i: u16, pub j: u16, pub transport: Transport }
pub struct Loop { pub vertices: Vec<u16>, pub faces: Vec<u16>, pub chord: u16, pub holonomy: Word, pub carriage: Versor }  // the closed walk, its faces, the chord (never a star face), the oriented holonomy and the carriage composed around it
pub enum PivotBy { Word { face: u16, word: Word, side: Side }, Placement { face: u16, translator: Versor }, Hue { face: u16, gauge: Vec<(u8, u8)> } }

pub struct Form {                                       // replaces Nerve as the stored object
    pub vertices: Vec<Vertex>,
    pub faces: Vec<FormFace>,
    pub loops: Vec<Loop>,
    pub remainder: Vec<Cell>,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,                 // by conjugacy coordinates ‖ gauge (A21), derived
    pub ratio: ((usize, usize), (usize, usize)),
    pub sections: Keyed<Vec<u16>>,                      // by transport-class coordinates
    pub certificates: Keyed<PivotCertificate>,          // A20, keyed by the certificate's own coordinates; evidence, not in the address
    pub torsion: Keyed<(Vec<(u8, u8)>, u32, u16)>,      // (σ, ℓ, chord), keyed by the rotor's conjugacy coordinates
    pub axes: Keyed<u16>,                               // Stage 11 I6: chart axes opened over held loops, the holonomy's class key → the chord of the loop that is its section; seed and A7 coordinate re-derive from the loop (law 12)
    pub glued: Keyed<u16>,
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    pub by_address: Keyed<u16>,                         // Word address (M2 composite) → index
    pub by_class: Keyed<Vec<u16>>,                      // chart key → the Word vertices on that chart: the fiber a listing body's chart reads (24.1); derived by reindex, never on the tape
    pub star: Keyed<u16>,
    pub carriages: Keyed<Versor>,                       // by transport-class coordinates; two carriages on one class are two sections
    pub by_rebased: Keyed<u16>,                         // of address → Rebased index
    pub by_unfolded: Keyed<u16>,                        // measure key → Unfolded index
}
```

Every `Keyed` above is keyed by A21 bytes of variable length; none by a
32-byte digest. `Address` is `Vec<u8>` holding an M2 composite.

`Nerve` (620), `Germ`, `Cycle` are removed (chapter 30); `Complex` on the
sentence stays as the mind's thought carrier, not part of the stored form.

```rust
impl Vertex {
    pub fn word_in(&self, form: &Form) -> Word;   // Frame: identity; Word: word; Glued: placement; Rebased: vertices[by].word_in ∘ vertices[of].word_in ∘ inverse; Unfolded: identity
    pub fn class(&self) -> stalk::Invariant;
    pub fn depth(&self) -> u16;                    // Glued: depth; others 0
}
impl Form {
    pub fn face_of(&self, i: u16, j: u16) -> Option<Transport>;  // word w_j·w_i⁻¹, carriage C_j · (word · C_i · word⁻¹)⁻¹ by the geometric product (never a vector difference), gauge to∘from⁻¹ on the union of hues (absence is identity); None off the sensor plane or when a Rebased `by` is not a Word or the Frame (3.1)
    pub fn delta1(&self) -> Vec<(u16, Word)>;             // holonomy per loop (10.3)
    pub fn coheres(&self) -> bool;                        // delta1 all projectively identity
    pub fn is_closed(&self) -> bool;                      // coheres && remainder.is_empty()
    pub fn depth_of(&self) -> u16;                        // 1 + max Glued depth, else 0
    pub fn regauge(&mut self, gamma: &[Word]) -> Option<()>; // 3.6, one word per vertex (10.2); refuses a gauge of the wrong length
    pub fn retract_vertices(&mut self, drop: &Keyed<()>);     // drop vertices by index key, drop incident faces, remap the rest, rebuild loops (21 step 3)
    pub fn glue(&mut self, s: &Form, s_addr: [u8;32], placement: Word) -> Option<u16>;  // 3.3; None unless s.is_closed() and s's Frame is identity (chapter 32)
    pub fn rebase(&mut self, by: u16, of: u16) -> u16;    // 3.2
    pub fn unfold(&mut self, measure: Measure) -> u16;    // 3.4
    pub fn loops_of(faces: &[Face], n: usize) -> Vec<Loop>; // spanning forest chords
    pub fn class_vector(&self, loop_: u16) -> Option<ClassVector>; // 12.6: plane coordinates from the holonomy, placement and dilation residues from the carriage versor composed around Loop.vertices, hue from the gauges composed around them
    pub fn rank(&self) -> Option<Rank>;                    // A18; None when the ℤ-rank overflows, and the caller refuses
    pub fn pivot_loop(&mut self, atlas: &WorkingAtlas, horizon: &Horizon, budget: &mut Budget) -> Vec<PivotCertificate>;  // A16
}
pub struct ClassVector { /* rotor conjugacy invariants (15.3) first; per plane (o, u, m5, m13) as the sensor section when the plane form exists; hue cycle type; placement residue (translator); dilation residue (dilator); 12.6 */ }
impl ClassVector { pub fn occupied(&self) -> Vec<Axis>; pub fn key(&self) -> Vec<u8>; /* A21: the coordinates written out, decodable */ pub fn decode(key: &[u8]) -> Option<ClassVector>; pub fn is_zero(&self) -> bool; }
pub enum Axis { Orientation(u8), Unit(u8), Prime5(u8), Prime13(u8), Hue, Placement, Dilation, OffPlane, Open }   // fixed order = declaration order; Orientation was Sheet (3.10)
impl Transport { pub fn refract(&self, form: &Form) -> Option<Form>; }   // Γ (3.10): rebase by word, regauge by gauge, compose carriages
```

`assemble` (939) reduces to: given vertices and the face index pairs the
bundle relates, compute faces by `face_of`, loops by `loops_of`, holonomies
by composition. It never reads cells. `solve_coboundary` (1268) is the tree
walk of 12.2 with its documented sign convention;
`sentence::tests::regauge_then_delta1_is_conjugate` pins it.

### 18.3 `/opt/mycelium-v0.1.0/src/transducer/element/`

**The files.** The carrier is seven files, one organ each (Stage 8b S1):
`carrier.rs` the `Action` and `Element` structs, the fibre reads and
mutators, and the four-lane measurement; `vertex.rs` M1 and M2 —
`same_vertex`, `compose`, `solution`, `inverse`, `canonical`, `address`,
`form_key_bytes`; `letters.rs` the versor constructors, `projective_eq`,
the decomposition into rho and translation, and `Letters`; `gauge.rs` the
hue permutation, its profile and key, the transition versor, and the
fibre of assignments a body can be spoken with; `residual.rs` A5's
`canonical_chain` and every constructor that reads a chain; `tape.rs` the
generation magic, the four tags, and the codec; `sensor.rs` the four
functions permitted a coordinate pair (`cell_of`, `carry_center`,
`carry_cell`, `pullback`), named in the squaring gate's allow-list. Tests
are `element/tests/{16,17,18,19,21}.rs` by the chapter each proves, with
`tests/common.rs` holding the fixture constructors.

`Element` (81) gains `depth: u16`, in `GEN2` after `mag` (`to_bytes` 1035,
`from_bytes` 1098). The sentence description bytes are the `Form` tape
(chapter 19). `is_sentence` unchanged. `PartialEq` is removed in favour of
`same_vertex` (M1); `address()` is M2.

**The action is a versor, never a matrix.** `Element` at e2511ff still
carries the Δ core's representation: `fibers: Vec<(u16, QMatrix)>` with
`FIBER_SPATIAL` a `GERM_DIM × GERM_DIM` rational matrix holding a 3×3
affine block plus a hue permutation block, `realize()` as a matrix
product, `compose` and `inverse` as matrix arithmetic, `same_action` as
matrix equality, and `derive_spatial` rebuilding the matrix from the tape
through `Letters::affine`. That matrix is the squared shadow of the word's
action (law 11), it is where a rotation is lost between tape and action,
and matrix equality is identity by representation (rule 3). The carrier
is `Element { tape, action, sub, mag }` with `action` the exact structures
of the four strata: the S stratum one versor `V = carriage ·
linear_versor(word)` (16.4), the G stratum the hue permutation, the T
stratum an extent in receipts, the H stratum the shell coordinates.
`compose` is the geometric product on S and permutation composition on
G; `inverse` is the versor inverse and refuses when it does not exist;
`same_action` is `projective_eq` on the versor and equality of the
permutation; a cell is carried by the sandwich `V p Ṽ` and read at the
sensor boundary only (`cell_of`). Chart invariants (A6, A7) are read from
the rotor and the versor, never from matrix entries; Čech transitions,
obstructions, glue blocks and germinated axes are versors. `QMatrix`,
`core::engine::{Delta, Operator}` and `core::matrix` leave cognition; the
exact integer rank of A18 stays in `core::zlinalg`. Landing item 0.14.

## 19. Tape: `SENTENCE_TAG` version 4

Carried from chapter 18: the `Form`. Written by `sentence::as_element_form`
(`encode_form` in `sentence/tape.rs`), read by `from_element_form`
(`decode_form`); every section is mandatory and a short or partial tape
is refused, never repaired. Version 4 replaces version 3's 32-byte digests
by A21 keys and its rational pairs by carriage versors; version 5 (Stage
11 I6) appends the opened-axes section and the `OpenAxis` certificate
kind. Every earlier version is refused: a layout change is a recut, and
no reader for an older layout exists (`MU-STANDARDS.md` §10):

```
SENTENCE_TAG ‖ version u8 (=5)
n_vertices u16, then per vertex:
  kind u8: 0 Frame | 1 Word | 2 Glued | 3 Rebased | 4 Unfolded
  Word:    address len16 + M2 bytes ‖ Word bytes ‖ contact 0 | 1 ‖ anchor u16 ‖ ring u8 ‖ carriage Versor bytes (A4 canonical) ‖ gauge len16 + pairs ‖ class len16 + axes u64…
  Glued:   address len16 + M2 bytes ‖ chart len16 + axes ‖ magnitude 4×u64 ‖ depth u16 ‖ placement Word bytes
  Rebased: by u16 ‖ of u16 ‖ carriage Versor bytes ‖ gauge len16 + pairs ‖ class len16 + axes
  Unfolded: measure u8 (0 occupied, 1 remainder, 2 ring) ‖ anchor u16 (ring only)
n_faces u16, then per face: i u16 ‖ j u16 ‖ word bytes ‖ carriage Versor bytes ‖ gauge len16 + pairs
n_loops u16, then per loop: n_vertices u16 ‖ vertex indices u16… ‖ n_faces u16 ‖ face indices u16… ‖ chord u16 ‖ holonomy Word bytes ‖ carriage Versor bytes
remainder len32 + cells (r i64 ‖ c i64 ‖ before u8 ‖ after u8)
depth u16
n_certificates u16, then per certificate (A20): chord u16 ‖ axis u8 ‖ before len16 + ClassVector bytes ‖ after len16 + ClassVector bytes ‖ rank_move: 0 (Equal) | 1 ‖ from u8 (Discharged) | 2 ‖ delta 4×i64 (Inadmissible) ‖ mu: 0 | 1 ‖ before 4×u64 ‖ after 4×u64 ‖ by: 0 ‖ face u16 ‖ side u8 ‖ Word bytes | 1 ‖ face u16 ‖ translator Versor bytes | 2 ‖ face u16 ‖ gauge len16 + pairs | 3 ‖ face u16 ‖ seed Versor bytes
ratio 4×u16 (h_out, h_in, w_out, w_in; a zero denominator refuses)
n_sections u16, then per section: class key len16 + bytes ‖ len16 + vertex indices u16…
n_torsion u16, then per cycle: class key len16 + bytes ‖ len16 + σ pairs ‖ ℓ u32 ‖ chord u16
n_axes u16, then per opened axis: class key len16 + bytes ‖ chord u16 (the seed and A7's refinement coordinate are not on the tape: they re-derive from the loop, law 12)
```

The remainder's cells are the one place a coordinate pair is written: a
remainder is a demand addressed to the sensor, and the sensor speaks in
cells (law 11).

Hue maps are not on the tape: they are derived from the `Word` vertices by
`reindex`, and a class whose vertices carry two gauges has no map. Loops
are on the tape only to be checked: the court re-walks the forest and
refuses `REFUSE_RANK_LOST` when the stored loops sit below it.
`as_element_form` serialises the `Form` and derives nothing from cells;
`from_element_form` reconstructs it and `Word` vertices derive the rotor
lazily through `Word::rotor`; the carriage is on the tape as a versor and
composes with the rotor by the geometric product (`element::conformal_of`
takes the carriage, never a shift pair).
The stored `address` of a vertex is **recomputed** by M2 on read (A8 step 2)
and the stored bytes are never trusted. A version-2 tape is refused; there
is no migration, because every cut is from genesis (chapter 33).

## 20. Perception produces word forms once

Carried from chapter 19: forms are read from tapes. This is the one place
cells become forms.

`/opt/mycelium-v0.1.0/src/transducer/family/`: `perceive(grid)`
(`perceive.rs`). The moves are six files (Stage 8b S9): `perceive.rs` a grid
becoming bodies (20); `pair.rs` the pair form of one train pair (21 step 2);
`extract.rs` the seven extraction moves (21); `speak.rs` a rule applied to a
scene; `orbit.rs` where a versor carries a body (22); `descent.rs` what a
rule measures over a family (12.6, 26)
returns bodies. Add:

```rust
pub fn word_form(a: &Body, b: &Body) -> Option<Vertex>   // new: carriage(a,b) (131) → factor_rotor (A22, 50.4) → Vertex::Word with class = b.moduli (M4 step 1)
pub fn frame_vertex() -> Vertex                           // new: Vertex::Frame
pub fn demand_vertex(kind: Demand, chart: &Invariant, bodies: &[Body]) -> Vertex   // 50.5: what no versor carries or no rotor spells
```

**A body is one hue (ruled 2026-09-09).** `perceive` reads a body as a
4-connected component of cells of one hue. Two touching bodies of
different hues are two bodies sharing a boundary, and that boundary is a
contact face of the pair form carrying the edge transport of 50.3, so
sliding, relative rotation and the evolution of a colour boundary are
measured on the edge. A component of every non-zero cell merged touching
bodies into one blob that no single versor carries, so `versor_between`
refused it, every such scene became a `TopologicalDeformation` demand,
and no pair form ever had a contact face. The court replays the voice's
two-hue sentence as a single simplicial complex with a shared contact
face, never as two detached listings (50.1). The earlier wording, "the
court reads a two-hue percept as one sentence", was an artefact of the
pre-sentence architecture and is struck (ruled 2026-09-10): the voice
performs F, the court executes G to verify boundary congruence and edge
holonomies, and the wire never carries raw percept cells to a court (law
8, 50.9).

`pair_nerve` (about 590) becomes `pair_form(a: &Grid, b: &Grid, scale) ->
Form`: vertices = `Frame` plus one `Word` per carried body — its versor
measured by `versor_between`, its word by `factor_rotor`, its gauge by
`derive_hue_map` — plus one demand vertex per (kind, chart) for the bodies
no versor carries or no rotor spells (50.5); no vertex ever carries the
identity word for a body it does not reconstruct (ruled 2026-09-09; the
body-only vertex of the earlier text was the second seam of 50.6). Faces
for every related pair through `face_of`, each carrying the edge transport
of 50.3 beside its placement; loops through `loops_of`. Shells (`peel::peel_body_scaled`)
remain pairing candidates and are never vertices. Candidates are bucketed by
`moduli.key()` over exact axes (A6) as an index only; the decider is that a
carriage exists, never the bucket. "Cells read once" is a memo keyed by the
pair's content, the framed listing key of `(a, b)` (`element::listing_key`,
M2), never by the grids' pointers or lengths: a pointer is a byte identity in
disguise and a reallocation makes it name a different pair.

## 21. Extraction as moves

Carried from chapter 20: pair forms. Extraction is seven moves on them.

```rust
pub trait Crystal {                                       // source of crystallized forms
    fn on_class(&self, class: &stalk::Invariant) -> Vec<(Element, Form, Standing)>;   // 24.2: closed forms filed at a cited FORM's chart (chart_of_form)
    fn on_body(&self, chart: &stalk::Invariant) -> Vec<(Element, Form, Standing)>;    // 21 step 3: closed forms carrying a body on a BODY's chart (its moduli)
    fn speak_form(&self, f: &Form, x: &Grid, placement: &Word) -> Option<Speech>;
    fn by_address(&self, addr: &[u8; 32]) -> Option<(Element, Form)>;  // keyed read, no scan
    fn on_axis(&self, chart: &Invariant, axis: Axis) -> Vec<Word>;     // A16 step 3 fiber
}
pub fn extract_rule_with(family: &[(Grid, Grid)], crystal: &dyn Crystal) -> Option<SentenceRule>
pub fn extract_rule(family: &[(Grid, Grid)]) -> Option<SentenceRule> // NoCrystal wrapper of extract_rule_with
```

`Epigenome` implements `Crystal` over its working atlas (chapter 24.1), the
standing of each returned form beside it; `Kernel` over the ruled cover
(`word_at` with `ruled_contains`). The nerve extractor `agree_up_to_gauge`
is deleted (abacf53); correspondence is keyed transport class on pair
forms, never nerves × groups × counts.

Inside `extract_rule_with`, in order:

1. **Correspondence by transport class.** Vertices across pairs are matched
   by their **transport class**: kind and the conjugacy class of the word,
   its invariants compared coordinatewise (15.4, A21), never by a digest;
   the `Frame` matches by kind alone. The hue map is
   not a matching key: it is a section coordinate carried on the form by
   class (`Form.hue_maps`), varying per pair as a section, and it stays on
   the form's key as meaning (rule 7). Matching on the map made every
   family that recolours differently per pair all sections and no rule
   vertex, which is what the first Cut B soak showed on seven of seven
   families.
   The body's chart (`class` on `Vertex::Word`, its moduli) is a **section
   coordinate** carried on the vertex and never a matching key: two bodies
   of different shapes carried by one transport correspond, two bodies of
   one shape carried by different transports do not. A transport class in
   every pair is a rule vertex; a class in some pairs is a section over
   those pairs (`SentenceRule.sections`, a `Keyed<Vec<u16>>` by transport
   class coordinates). Counts are never compared. Matching by moduli was the fault
   that left three quarters of ARC families frame-only (a73d444 corrected it
   in `same_stalk`); the form engine matches the same way.
2. **Constancy of faces.** Faces are the pairs the bundle relates, never the
   complete graph: the star from the `Frame` to each vertex, and vertices
   whose after-supports overlap or sit at ring distance one (chapter 20). For
   each such pair, `face_of` in every pair of the bundle; the face is a rule
   face iff all are one vertex (M1 equality). Non-constant faces are
   dropped; the vertices stay. A face for every pair of vertices carries no
   information beyond the vertices and makes every loop identity by
   construction, so torsion has nowhere to appear.
3. **Gluing.** For each class present, `crystal.on_body(class)` — the
   class here is a `Vertex::Word`'s own `class`, the **moduli of the body
   it carries** (18.2), so the read is at the coordinate the perceived
   body supplies; for each crystallized `s`, the placement word from `s`'s
   `Frame` to the pair's `Frame` via the matched vertex; if
   `crystal.speak_form(s, x, placement)` reproduces a subset of the pair's
   output on every pair, the pair forms `glue(s, addr, placement)` and the
   reproduced vertices leave the remainder (3.3, 13.6). The chart routes
   and `speak_form` decides: incidence is deconstruction, never a key
   (24.3), and the chart is what keeps the read from becoming a scan of
   everything the mind holds (law 3).

   **`on_body` and `on_class` are two coordinates, not one** (ruled
   2026-09-10). This step asks which closed forms carry a body on a body's
   chart; 24.2 asks which closed forms the mind holds at a **cited form's**
   chart, and a form is filed under `chart_of_form` — `[2, vertices,
   faces, loops, …]` — while a body's moduli are `[components,
   …harmonics…]`. The two systems meet only at the empty root. Named
   `on_class` for both, the glue's read was performed against the form
   index and answered the empty vector to every question it could ask: no
   rule ever cited a held form, no citation was ever a use, and no speaker
   was ever paid for one (26, 27). That is the mechanism under the fleet's
   "genomes of 250–420 ruled words comprehending 0 of 2 listings" (50.6),
   and it was an index fault, never an absence of anything to say. A mind
   answers `on_body` from `by_moduli`, its own `Form.by_class` one level
   up (18.2): body chart → the addresses of the closed forms it holds with
   a `Word` vertex there, filled and emptied where the axis index is.

   **A `Glued` vertex is the form-level case of a derivation citation**
   (ruled 2026-09-10). Both cite a held form by address and claim its
   verdict; they differ only in what is cited. `Glued` cites the whole
   form and its placement, and the citer's loops close because the cited
   form's do (13.6). A word vertex's derivation (50.2 step 2) cites a
   `Word` vertex *inside* a held form and composes its rotor. So the
   court treats them alike and must: the same replay resolves the cited
   address on the ruled cover, the same deferral applies when it is not
   yet ruled and the syllabus serves it, and the same `settle_use` pays
   its discoverer at install (26). One mechanism at two levels, not two
   mechanisms — and it is why the derivation is a coordinate of the form
   rather than an annotation on the frame.

   **`by_moduli` ROUTES AND DOES NOT DECIDE** (ruled 2026-09-10). It
   narrows the candidates a chart could reach and nothing more; what
   decides is `speak_form` reproducing the pair's output — `apply` and
   `unapply`, a deconstruction — on every pair of the bundle. 24.3 is the
   standing rule: "incidence is deconstruction, never a key. A listing is
   incident with the mind iff some held word deflates it under
   `apply`/`unapply`, measured lane-wise. Chart keys route the quote; they
   never decide comprehension." An index read as the comprehension is the
   dictionary fault one level up from the cipher — the key becomes the
   answer instead of handing the traversal its candidates — and it is
   refused wherever it appears. Nor is the glue the operation: gluing is
   the case where a **whole closed form** reproduces a sub-scene, and the
   operation of which it is one case is the factorisation of 50.2 step 2.
4. **Rebasing.** "Do to B what A did to itself": vertex `j` is `i` read in
   `j`'s frame when `w_j = t · w_i · t⁻¹` with `t` the relative placement
   from `i`'s body to `j`'s body (the translator factor of `face_of(i,
   j)`'s carriage versor, a versor conjugating a versor, never a pair
   subtracted), on every pair of the bundle and with `w_i` not the
   identity. Then replace `j` by
   `Rebased{by: i, of: j}` (3.2), whose `word_in` is exactly `t · w_i ·
   t⁻¹`. The criterion is **not** `face_of(i, j) = w_i · w_j · w_i⁻¹`: since
   `face_of(i, j) = w_j · w_i⁻¹`, that equation reduces to `w_i = 1` and
   never fires on a non-trivial word; an earlier version of this document
   stated it that way and 3eca3d5 implemented it as written, so rebase never
   triggers there.
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
and `carry_cell` (element/sensor.rs) at the placement solved from the face to
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

### 23.1 `replay_form`, `/opt/mycelium-v0.1.0/src/court/verdict/replay.rs`

**The files.** Four organs (Stage 8b S15): `replay.rs` a spoken form
re-derived from its own tape move by move (23, 42); `blind.rs` the same
re-derivation with nothing supplied but the word (42); `ruling.rs` what a
court decides about a word (42); `selection.rs` the anchor a family derives
and the key that names it (A9).

Before the existing sentence arm in `blind_replay` (31):

```rust
pub fn replay_form(spoken: &Element, resolve: &dyn Fn(&[u8;32]) -> Option<Form>, family: Option<&[(Grid,Grid)]>) -> Option<&'static str>
```

1. `REFUSE_GLUED_DEPTH`.
2. `REFUSE_GLUED_UNRULED` when `resolve` is `None`; `REFUSE_GLUED_FRAME_GAUGED`
   when the resolved form's `Frame` is not identity (chapter 32).
3. Each `Word` vertex: rotor from tape, affine, M1 compare against the
   vertex's own recomputed vertex (`REFUSE_VERTEX_TAPE`); then the vertex
   reconstructs its body on the receiver's copy of the input, `(V · W ·
   γ) · B_in = B_out` under exact cellular equality (50.2 G step 3,
   `REFUSE_VERTEX_BODY`). Each `Demand` vertex names a kind and a chart
   (`REFUSE_DEMAND_UNTYPED`). A tape with no vertex is a chain and is
   refused whole (`REFUSE_NOT_A_SENTENCE`, 50.6).
4. Each face: `face_of(i, j)` recomputed, M1 compare with stored
   (`REFUSE_FACE`); its edge transport re-derived as 50.3 states it, in
   the body's stabilizer or the exchange, or a remainder edge
   (`REFUSE_EDGE`); or the face is the chord of a loop whose class key
   is filed on torsion with the filed order, in which case the loop's
   holonomy is what the chord carries.
5. Each loop: holonomy recomputed as the product of the edge transports
   read in the base body's stabilizer up to conjugacy (50.3),
   projectively identity or filed torsion (`REFUSE_LOOP`).
6. `REFUSE_REBASE_TRIVIAL`, `REFUSE_UNFOLD_STILL`, `REFUSE_ORBIT_STILL`, by
   executing the move on `family` when given, else by the form's own
   consistency.
7. Closure: every remainder on the family empty (`REFUSE_NOT_CLOSED`), and
   beyond the cited forms a deflation, a reach or a shorter description
   (`REFUSE_REDUNDANT` only when all three are absent); the deflation is
   read per lane and paid, the reach travels on the certificate (S9.13),
   and nothing is compared against anything.

The translation arm of `blind_replay` recomputes 13.9 step 5 blind for a
`TRANSLATION_TAG` element (`REFUSE_MV_INEXACT`), refuses a translation
across an overlap whose fetched tapes fail (A1) for either hypha
(`REFUSE_OVERLAP_CYCLIC`), and refuses a spoken class whose key differs
from its recomputation on the cited tapes (`REFUSE_NOT_NATURAL`). The chain
compare canonicalises the spoken chain by A5 first. `sel_key` (374) is A9.
`rule(...)` (222) is unchanged.

### 23.2 Kernel carriers, `/opt/mycelium-v0.1.0/src/server/cognition/`

**The files.** The mind is fourteen files, one organ each (Stage 8b S3):
`kernel.rs` the `Kernel` struct and the reads that count what is in it;
`language.rs` the `Language` snapshot and what a reduction answers with
(`Outcome`, `Glue`, `NormalForm`, `MarketView`, `Rank`, `FieldEntry`,
`ListingField`, `Reduction`, `Adoption`); `basis.rs` how a ruled word
enters, occupies, relocates and leaves; `cocycle.rs` the filed cocycles and
their causal supersession (45.3); `reduce.rs` the reduction and the quote
over the snapshot; `keys.rs` the A21 keys the mind indexes by; `wire.rs`
the mind's own codec; `reflex.rs` the action potential and the memory
headroom (24.1); `identity.rs` the `Crystal` the family extraction speaks
to; and `book.rs`, `court.rs`, `germinate.rs`, `market.rs`, `snapshot.rs`
as before. `mod.rs` holds declarations and re-exports only.

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
never pivots: it replays and files.

**Certificates.** For each `PivotCertificate` on the spoken form's tape
(A20), the court, holding the resolved form before the move, reads the
loop's class vector and requires its key to equal `before`; applies `by`;
re-walks; requires `class_key(κ') = after`, `rank' = rank_after`, and the
discharge condition of 12.6. Success logs `court pivot certified {addr}
loop {i} axis {label} {before}→{after}`; any mismatch is
`REFUSE_PIVOT_INEXACT` and the form is refused whole, because a
crystallization that rests on a pivot that does not replay rests on
nothing. A form with no certificates is replayed as before.

`serve_ruled` (3148) and the `ACT_SYLLABUS` reply (mycelium-server/door.rs,
920) append the chart's class records (A19 step 1). `Kernel::rank`
(A18) is asserted in debug on every glue and on every refinement.

### 23.8 Hardening: what the court does with an utterance

Carried from chapter 5: the hypha replays, verifies, keeps the market and
the genome; it never thinks. Every act a voice speaks to it is a proof or
a question, and the court's whole behaviour on an act is fixed here.

- **An utterance is a sentence, replayed by G, never reduced (ruled
  2026-09-10; the earlier text described a demand frame carrying a chain
  and citations, the chain era's act).** The speech frame carries the
  sentence — its word vertices, its faces with their edge transports, and
  its unmet boundaries as typed demand vertices (50.5) — with the standing
  byte and the voice's K. No raw chain rides the wire, and a frame that
  carries one is refused `REFUSE_NOT_A_SENTENCE` (50.6). The court replays
  the sentence on the family its task binds (`perceive_speech`, 23.1 with
  G's four steps), answers a typed adjudication (50.9) — stood, or refused
  with a kind and a content coordinate — lists the demand vertices by kind
  under the sentence, and dockets the closed sub-complex under the voice's
  room. `reduce_all`, `reduce_germ`, `reduce_proven`, `percept_elements`
  and the percept act are deleted: a hypha that reduces a percept against
  its own words is thinking on the voice's behalf, and its book then fills
  with words no mind stood. `REFUSE_UNPROVEN` is retired with the chain.
- **Replay resolves through the court's genome.** `replay_form_with`
  hands the deflation measure the kernel as its `Crystal` (the kernel
  implements it), never `NoCrystal`: a form whose `Glued` vertex cites a
  ruled word replays its deflation through that word. `NoCrystal` is
  admissible only for a tape with no `Glued` vertex.
- **Admission is room, never a count.** A docket draws the chain's μ⃗ from
  the voice's room on each lane it occupies (the bond's cover on that lane
  less its occupancy, as `lane_room`); a lane whose room does not cover
  the draw refuses the act with `REFUSE_ROOM` and nothing is docketed.
  The draw is released to the voice on closure or ruling of the listing
  and burned on an equivocation proof (47.4). There is no per-voice or
  per-stroke ceiling anywhere in the court.
- **Idempotence by receipt.** The receipt of an utterance is the triple
  (voice `vk`, M2 address, K). A second frame with the same triple is
  answered from the docket with the same reply and logged `court attest
  duplicate`; it is neither re-docketed nor re-charged. The same address
  from the same voice at a later K is a use; from another voice it is a
  consilience. This is the whole defence against a voice that repeats
  itself, and it needs no clock (law 9).
- **The syllabus teaches proofs.** A taught word travels with its tape,
  its certificates (A20) and the ruling receipt (the ruling voices, K,
  height). The receiving mind replays it (`blind_replay`, then `rank`)
  before `learn_one` stands or promotes it; a word that fails replay is
  refused locally (`syllabus refused {addr}: {REFUSE_*}`), is not learned,
  and the refusal is a receipt the mind keeps (47.4 makes it a contest).
  Two organs replay every word in the genome: the court that ruled it and
  the mind that holds it.
- **The book has a height.** Every quote reply carries the book's ledger
  height and the voice's frontier on it. A voice asks again when its
  working language changed, when the height advanced, or when its dwell
  elapsed (24.3), so an unchanged book is not asked on every stroke. A
  height a voice reads off a reply only advances; a deferral carries no
  height and moves nothing (ruled 2026-09-09). That height is the mind's
  TUniversal (45.2: its own chain cites no peer, so its own height is its
  K): the quote and attest arms advance it on the entity, the stroke
  line's `h` reads it, and every dialect stand, expressed residual,
  learned page and withheld form is stamped with it (V2, 2026-09-09; set
  once to zero at boot, every voice's temporal lane was a point).
- **A deferral carries the door's deficit.** The frame a full reflex pool
  answers every act with carries the door's per-lane reserve deficit
  (S9.6b's reading) as its four lanes, with no height and nothing taught;
  the asker dwells on those lanes exactly as it dwells on a reply's
  pressure (24.3, 45.7), and never counts deferrals (ruled 2026-09-09).
- **Teaching is decided by standing, never by length.** In the demand
  reply a cited word is taught back when the docket holds it anchored
  (ruled); the sentence's length decides nothing. A two-word sentence over
  anchored words teaches both; a one-word sentence over an unanchored
  word teaches nothing.
- **Pressure travels exact.** The reply's per-lane pressure is four exact
  lane values beside the height, never a byte per lane; the client reads
  the four lanes and paces each on its own (24.3).

### 23.9 The court adjudicates conclusions, never words

Ruled 2026-09-07. Two minds perceiving in their own dialects will close
the same demand by different proofs: different letters, different charts,
no transition of the group carrying one tape onto the other. Identity of
tapes up to transport is identity on the symmetry side, the kernel, and
the court never reduces there (0.12, the asymmetric coboundary). The
court's key is the **conclusion**: the demand closed, named by its
coordinate-exact description, which is already the banked object ("bank
the description, invert for the solution").

- **Each utterance is a proof check, alone.** The court replays the tape
  against the demand it claims to close: does this tape close this
  description. Closure is the certificate — the remainder is empty, the
  loop returns, the tape re-derives — equalities, chart-free, readable in
  either direction. Nothing is compared with another mind's word.
- **Convergence is read at the demand.** A conclusion is language when
  independent lineages have closed it, independence being read from the
  receipt ledger (two derivations with no common ancestor, Part IX), never
  from a count of voices. The dialects that reached it stand as sections
  over it — the punctured axis (25): alternative assignments over the same
  base, one bundle, many proofs.
- **The taught form is the proof that derives the others.** Among the
  proofs of one conclusion, the one from which the rest can be derived
  (A12 replay, `reduce_excluding`) is what the syllabus carries down. No
  ranking, no count, no constant: the language settles to its most general
  proof of each theorem by derivation alone.
- **Four levels, and the court reads the top two.** Letters are the
  seventeen rotors. Words are tapes of letters, group elements; equality is
  `projective_eq`, `same_vertex`. Conjugates are words up to `g w g⁻¹`, the
  same motion seen from another frame: the class vector, the canonical
  representative of 15.2–15.3, the third equality of law 3; the transition
  on a face is the conjugating element, and order is a conjugacy invariant.
  Sentences are nerves whose vertices carry words and whose faces carry the
  conjugations between them; a loop's holonomy is a word and κ(ℓ) is its
  class. Two proofs made of conjugate words are one proof in two frames,
  and E2 pays it once — that is the conjugate level, and it is the whole of
  what "transport equivalence" decides. Two proofs whose words are not
  conjugate but close the same demand are two proofs of one conclusion —
  that is the sentence level, where the court glues. Word, conjugate,
  conclusion; a reading that goes from letters to sentences has skipped the
  level where a word's class, and so its order, lives (corrected
  2026-09-08).
- **Divergence is the contradiction case.** Two dialects closing the same
  demand to different conclusions is node against node (the recohering
  judge, Part V); it is settled by replaying the mathematics, never by
  counting.
- **The mind never waits.** Dialect is spoken the moment the mind stands on
  it; what waits is only the genome, which gains a letter when the court
  glues. The syllabus is one-way: ruled language down, never a stood word
  back (the C3 leak). The mind speaks ANASTOME and its dialect at once.
- **What the mind supplies, and when.** No threshold. An utterance leaves
  when three readings hold: the word has stood (it closed a germ and
  carried a derivation), the voice has room on the lane (its own bond and
  escrow, 47.1), and the two-stroke outcome vector paces it (24.3). Three
  measurements, no literal.

The conjecture this rests on: many minds close the same demand by many
proofs; the court recognises them as one conclusion by replay, not by
comparing words; the conclusion becomes language once independent
lineages reach it; and the language settles to the proof that derives the
rest. The court today keys on the word — it slashes on address inequality
and glues on the word's chart (TASKS A3, Stage 9) — which is the
misplacement this section corrects.

**The membrane: holonomy over the proofs (ruled 2026-09-08).** The
gluing of 23.8 is pairwise — a representative against a spoken word, one
transition per pair, a dialect row where the transition is not the
identity. The question 23.9 asks is not pairwise: given the proofs of one
conclusion from independent lineages, compose the transitions between
them around the loop, `g_ab · g_bc · g_ca`, exactly as `holonomy` composes
the versors around a Čech cell (13, `cech/cell.rs`). The minds are the
cover, their proofs the sections, the transitions the cocycle. Holonomy
trivial: the proofs are one proof seen from several frames — the conjugate
level — and E2 pays the closure once. Holonomy non-trivial: the proofs are
genuinely several, the conclusion needed more than one perception to be
spanned, and the residual of the holonomy, localised to the cells where
every lineage diverges, is a boundary of the language and not a fault of a
mind — it is the demand the next letter answers. This is the court's whole
answer to "does one mind suffice for this conclusion", and it is a
computation on carriers the tree already holds; no count, no vote, no
threshold. The convergence engine had this as the membrane's holonomy over
perspectives and its meta-Delta between observers; what it lacked was the
level (it compared outputs, not proofs) and it identified orbits by a
digest. The geometry carries over; the identity and the votes do not.

## 24. Comprehension (the mind)

Carried from chapter 23: what the court checks, the mind must be able to
re-derive, and it re-derives against everything it holds, not only what the
court has returned to it (chapter 6). `/opt/mycelium-v0.1.0/src/client/sdk/`:

**24.1 The working atlas.**

- `Epigenome::working_atlas(&self) -> WorkingAtlas` **new** replaces every
  cognitive read of `atlas()` (805). It reads three strata: the genome
  `Stalks` (the existing `atlas`), a dialect `Stalks` **new** admitted in
  `stand_dialect` (734) by `invariant_of(&sentence)` and retracted in
  `shed`, and the idea charts already in `Contemplation.by_chart`. A fiber
  read returns `(address, Standing)` with
  `pub enum Standing { Ruled, Stood, Held }` **new**; no read returns a
  boolean. `atlas()` remains for persistence (`crystallize`) only.
- **The Held stratum is the contemplation's forms, read at the
  coordinate (ruled 2026-09-09).** An idea is a form (50.6) and its held
  words are its `Word` vertices. A cite reads them at the coordinate the
  listing supplies and nowhere else: the body's chart →
  `Contemplation.by_chart` → the bubble → `Form.by_class` (18.2) → the
  vertex, and the word is derived at that vertex alone, its letters,
  carriage and gauge on the body its address names (A21 §4). Nothing
  materialises the ideas' words ahead of a question, no walk over a
  form's vertices is made at a cite, and no second carrier of held words
  exists beside the contemplation: `Entity::ideas`, a `Keyed<Element>`
  filled at hold time and scanned at cite time, is the dual carrier the
  enumeration audit names and is deleted. `Reading` takes the held words
  as a reader at a chart, never as a list of elements. Bound (L6):
  `Contemplation::held_at(chart)` is the read, `Reading.held_at` the
  reader, `sentence::vertex_cites` the evaluation — the vertex's rotor
  under its dilator, carried onto the body's own frame by the translator
  the two centroids solve (`blade::versor_carrying`, verified cell by
  cell both ways as `versor_between` verifies) with each landed hue the
  gauge's image of the hue it left — and a `HeldCite` (the vertex's
  address, the body reconstructed in the listing's frame) is what it
  answers. The strata are read in their order, Ruled, then Stood, then
  Held: a held word answers only where no ruled or stood word is on the
  chart, and exactly one held word must reconstruct the body; several
  stand as multiplicity, never a choice (rule 15).
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
  geodesic through the court (chapter 27). It counts **the mind's own K**,
  never TUniversal, as 45.4 rules for every lifecycle of the mind: a height
  advances only on admitted receipts (45.2), so a lifecycle read against one
  cannot fire at all on a mesh that admits nothing, and a fleet refusing
  work would be unable to reclaim what it holds. A mind's K advances on its
  own acts, so it reclaims whether or not anyone answered it. The same holds
  for `retire_on_t` (25.4) and for `touch`, whose stamp is the use the shed
  measures against. `last_used_tu` remains beside it as standing extent in
  heights, which is a coordinate (45.4) and not a lifecycle.

**24.2 Sentences and connections.**

- `Comprehension` (549) gains `Withheld { missing: Vec<[u8;32]> }`.
- `Epigenome::comprehend` (856), sentence arm, on the `Form`:
  1. each `Glued` address: `word_at` (681); absent → `Withheld`.
  2. absent, but a closed form of the cited chart is held under another
     address (`Crystal::on_class` over the working atlas): understood
     through each such form, `Understood` carrying `(held address,
     standing)` per form (13.11). No re-gauge is performed, since 13.6
     makes it vacuous, and no first-found form is chosen: every held form of
     the chart is an entry. Log `form understood through {held} for
     {cited}`.
- Connection arm: A12 over the fiber's basis (M6), never a single
  representative and never an address lookup as the test.
- `learn_one` (977): a `Withheld` sentence enters `Dialect` (629) with
  `withheld: true, missing`, never the genome; `pull_syllabus` (1427)
  requests the missing addresses by incidence and learns served words in
  ascending `depth`. The address dedupe in `learn_one` is a fast path after
  M1, never instead of it.
- `serve_ruled` (cognition.rs 3115): serves in depth order and never a form
  whose `Glued` charts the voice's atlas lacks.
- **What a joiner chooses (ruled 2026-09-09).** A mind joining the mesh
  chooses the era it adopts (45.8), the depth it learns from and the
  strata it works in; it never chooses a velocity or a phase, it may join
  behind the head and climb, and it lists only into the head's book.

**24.3 The utterance: what leaves the mind, and when.**

The order is fixed and the same for every producer: perceive, think,
stand, utter. A word reaches the wire only after it stands in the
dialect, and a mind that has stood nothing utters nothing but a percept.

- Three local producers stand sentences (ruled 2026-09-09, chapter 50).
  (1) A closed form (every body reconstructed, every loop filed or
  identity) stands through `speak` and is uttered once. (2) A withheld
  form (some body not reconstructed) is stood in the dialect as
  `withheld: true` with its open coordinates (24.2) and held as an idea
  with its bundle (25); what it could not close is its demand vertices
  (50.5), and the sentence is uttered once, whole, with its demands, its
  standing byte and the voice's K. Nothing is expressed as a residual
  word and no chain is uttered (50.6). (3) A served listing is a demand:
  `conclude` factors it through the working atlas by G (50.2) and answers
  with a sentence that closes it, in whole or with demands of its own; a
  listing no held word closes is held as a thought and never re-uttered,
  because a mind does not claim what it did not reduce. (4) A fed pair's
  percept cites over the working atlas, not the genome.
- **Incidence is deconstruction, never a key.** A listing is incident
  with the mind iff some held word deflates it under `apply`/`unapply`,
  measured lane-wise. Chart keys route the quote (which hypha, which
  strata); they never decide comprehension, and no filter compares a
  listing's open charts with an atlas fiber for equality.
- **Nothing is uttered twice.** The utterance receipt (address, K) is
  kept in `taught_up`; the same address is uttered again only when its
  vertex changed (a new address) or the reply was absent past the
  transport aperture, measured in strokes.
- **Standing is carried, never weighed.** Every read, citation, docket
  and settle receipt carries the standing of the word it rests on as a
  coordinate; no path multiplies it into μ⃗, orders claims by it, or gates
  on it. **Multiplicity opens an axis.** `open_axis` fires when several
  held words deflate one listing at once, and the glue between them is
  the new coordinate; a listing no held word deflates is a held thought,
  never an axis. Held words are bodies on a chart (the residuals the mind
  expressed), so the glue is the dislocation versor between their shapes
  — the versor the court's `Twisted` closure carries — and the germ is a
  connection whose carriage is that versor with the axis opened on it,
  its magnitude the listing's with the holonomic lane raised by the words
  that met (V1, 2026-09-09; the old reading asked the bodies for a group
  solution and a connection tag they never have, so no axis ever opened
  on the fleet). A citation is a use (45.4): the cited word stays in the
  dialect for an aperture of the mind's K, so the second body is still
  held when the axis is asked for. The listing is recorded before the
  germ is spoken and tried again only when the language moved; a glue
  that cannot be read or spoken is written as `[{tag}] axis refused
  {addr}: {why}`, never silence.
- **Pacing is the outcome, never a constant.** A stroke's outcome has the
  modes (fed, pivoted, stood, uttered, closed, learned), each a μ⃗ on the
  four lanes; `pivoted` is the lane measure of the stroke's A16 snaps
  over every dirty idea. Lane-wise sums across modes are allowed; nothing
  is summed or compared across lanes. A working stroke spends
  `lanes_of(outcome)`; only the tick spends all four lanes, and the idle
  `physics.spend([true; 4])` is that tick, not work. The dormant test is
  `outcome == [0; 4]` on every mode, with no threshold other than zero.
  The dwell is per lane: each lane's dwell is that lane's surface in the
  last reply, in strokes — the door's reserve deficit and the entropic
  flux (`yield_surface`), each a calibrated reading — and the loop dwells
  for the lanes it needs to expand next, never for the most-pressured
  lane; there is no `max` over lanes and no clamp against the aperture
  cycle. The open-demand backlog is not on the surface: pressure is a
  magnitude the metabolism relaxes and prices nothing (`price` reads the
  word's own measure), and read as strokes it held every voice on one
  hypha for 48,000 strokes on the fleet of 2026-09-09 (R3).
  Until 5.1 the dwell is `from_transport(rtt, strokes)` per lane; after
  5.1 it blocks on the next admitted receipt or quote reply. The quote is
  asked when the working language changed since the last ask, when the
  book height in the last reply advanced, or when a lane's dwell elapsed;
  never on every stroke. The utterance budget of a stroke is the lanes
  the physics can expand and the room the last reply reported, never a
  count.
- The stroke line is written once per aperture cycle and on every change
  of K, not on every stroke.

### 24.4 The eye is a cover: cochains, cocycles, congruence (ruled 2026-09-08)

A perception is never taken by one eye. The senses of one mind are a
cover of one sample, each sense a section over it, and what the mind
perceives is what the sections agree on; what they fail to agree on is
the demand, found at the eye and not at the market. The senses are
internal to one mind — never a second mind, never a twin (the cover over
minds is the court's, 23.9, and a different cover).

**The eyes.** Over one sample, at least two, and for a pair three:

- the **luminal** eye lifts each body to a blade on the hypersphere
  (`blade::lift`, `family::perceive`), and reads its carriage — the
  conformal versor that places it;
- the **harmonic** eye reads each body's conformal invariants
  (`Harmonics::of`: components, holes, turns, isoperimetric, anisotropy,
  spread, quartic, gauge), which no versor changes — what a body sounds
  like whatever its size, place or orientation;
- the **holonomic** eye reads the transitions between bodies — within one
  grid by contact and orbit (3.5), across a pair by the versor that
  carries a body of `a` onto a body of `b` (`versor_between`,
  `cech::transition`) — and composes them around every loop of bodies
  (`cech::holonomy`).

Today these run as a pipeline: the luminal eye produces bodies, the
harmonic sense is asked once to stratify the atlas, and the holonomic
sense runs on the sentence the first two already built. Each consumes the
previous one's output and nothing checks that the three agree, so there
is no cover and no cocycle, and every disagreement reaches the book as an
unreduced residual. That is the market carrying what perception should
have refused.

**Cochains and the cocycle.** The luminal and harmonic readings are
0-cochains over the bodies: to each body, a blade and its invariants. The
holonomic readings are the 1-cochain over the overlaps: to each pair of
bodies the eyes put in correspondence, the transition `g`. The cover is
consistent — the perception is **congruent** — when the cochains satisfy
the cocycle:

- on every overlap, the harmonic sections agree: `harmonics(a) =
  harmonics(b)` exactly when some versor carries `a` onto `b`, so the
  harmonic eye decides whether the luminal pairing is a pairing at all,
  before any word is sought;
- around every loop of bodies `a → b → c → a`, the transitions compose to
  the identity, `g_ab · g_bc · g_ca = 1`, read by the same `holonomy` the
  Čech cell uses; a loop that does not close is a residual localised to
  that loop, with the class of its holonomy named (12.6).

Congruence is a reading, never a gate: it is the residual of the cocycle,
per overlap and per lane, and it is zero or it says where and on which
axis. Coherence is the same reading over the whole of one mind's cover
(law 15); contradiction is between minds, and belongs to the court.

**The sensory verdict, at the eye.** What the cocycle's residual names is
the verdict every percept carries before it leaves the mind:

| verdict | what the eyes read | what follows |
|---|---|---|
| Recognised | harmonics agree, the transition is a held word or the identity | the percept is a form; nothing is listed |
| Reflection | harmonics agree, the transition's orientation character `w₁` is odd (3.10) | a reflection word; the form is spoken with it |
| Transposition | harmonics agree, the transition is a placement or dilation versor | attaches to the held word as a section (25) |
| Novel | harmonics disagree on named axes, no versor carries a body, a rotor block does not spell, or a loop's holonomy is non-trivial with a class no held word discharges | a typed demand (50.5): `TopologicalDeformation`, `Source` or `Creation`, `RotorPrime` or `Standing`, or the loop's class; the only verdict that becomes a listing, and it is listed under its kind |

**What leaves the mind.** An utterance carries a percept with its verdict,
its residual per overlap and the axes it names. The hypha's book lists
demands, never raw residuals: a listing is a Novel percept with its class,
and the market prices and relays it. Recognised, Reflection and
Transposition never reach the book; they were perception. This is what
"clients think, hyphas verify" means at the eye, and it is why the book
stops being a queue of unreduced perception (S9.7 shrinks to what a
congruent percept still cannot say).

**Cost.** Every eye is exact and every check is finite: the harmonic
comparison is coordinatewise equality of invariants, the holonomy is one
geometric product per loop, and the loops are those of the pair's nerve —
no search, no threshold, no vote among senses. The eyes compound; they
never compete.

**The harmonic eye's coordinates, carried from the convergence engine
(ruled 2026-09-08).** The older engine read its harmonics as projections
of the nested relational field `Δ[i][j][l]` — `Vec<Vec<Vec<Q>>>`, the third
index the fibre layer — into eleven exact rationals, three times over:
`HarmonicState` (an entity's self-perception), `HarmonicTuning` (running
averages), `SoundnessSpectrum` (eleven layers over a percept). The
mathematics that carries is the exact part of the third, and it is
exactly the second and third eyes this section names; the rest is refused
by name so it is not re-imported.

| reading | the older engine | exact? | mycelium receiver |
|---|---|---|---|
| the body's conformal invariants | vision: `SymmetryGroup` elements and per-body shape | yes | `Harmonics` (seven invariants and the hue spectrum) — present, one caller |
| **cohesion χ**, per cell: same-hue neighbours over neighbours, a 0-cochain over the body's cells | touch: `AdjacencyStructure`, `CohesionCluster` | yes, Q per cell, no threshold | the touch eye of P2: a 0-cochain on the blade's cells, its coboundary the body's seams |
| **refraction at a boundary**: Σ|δ_a − δ_b| over the boundary length between two bodies | `ClusterRefraction`, `refractive_index` | yes; `transparent` is only "index = 0" | the holonomic eye across a face: the transition Γ's residual on the seam (3.10), read at perception |
| **stabiliser order**: the smallest p with A^p = I for the body's symmetry element | vision: `torsion_order` | yes | `Word::order` of the body's stabiliser word, a class invariant (23.9), on the percept |
| **periodicities from the state's own entries**: the primes of every entry's reduced numerator and denominator | `RefractiveLens::torsion_orders` | the factorisation yes; the `[−2, +2]` rescale no | a candidate, not ruled: the primes of the invariants' denominators as the body's periodicities, beside the class's `(m5, m13)` |
| **per-layer harmonics**: the same readings taken on every fibre layer, never summed across layers | `Δ[i][j][l]` per `l`; `compute_rank` over `l` | yes | the punctured axis: `Harmonics` of every section of a word's fibre, a `Vec` over the sections (25, S9.13), never one reading for the bundle |
| eleven "tones" of self-perception: coherence, velocity, attention, symmetry score | `HarmonicState::from_delta` | no — scalar composites of a coherence functional (law 4, D0) | refused; the mind's self-reading is the outcome vector per lane (24.3) |
| eleven running averages with `/10` and `/8` | `HarmonicTuning` | no (rule 5) | refused; was `Aperture`, deleted (F2g) |
| dominance ratio, anchored fraction, coverage fraction | `SoundnessSpectrum` layers 1, 4, 5 | no — count ratios, majority tests (rule 2) | refused; the readings above replace them |
| a gravitational field over vibrations | `gravitational_field` | no — a potential (12.6) | refused |

Two corrections to the tree follow. `blade/harmonics.rs` claims eleven
coordinates and holds seven and a spectrum; the header says what the file
holds (§2.8). And a body is read by three eyes or it is not perceived:
`Harmonics` alone is the harmonic eye, the cohesion cochain is touch, and
the refraction on a seam is the holonomic eye at a face — the pipeline had
one of the three, called once.

## 25. Germination and thoughts

Carried from chapter 24: comprehension re-gauges. Germination pivots, files,
cauterizes, and wraps (M9). `/opt/mycelium-v0.1.0/src/transducer/bubble/`:

**The files.** The contemplation is eight files, one organ each (Stage 8b
S5): `idea.rs` one bubble and what it measures; `contemplation.rs` the
complex of held thoughts and their incidence; `merge.rs` when two held
thoughts are one (13.12); `drive.rs` what makes a thought move (25.1);
`horizon.rs` the classes seeded from the syllabus (A19); `shed.rs` what a
mind stops holding when the room is spent (24.1); `tape.rs` the THT codec
(28); `mod.rs` declarations and re-exports.


**25.1 The pivot loop in the contemplation.** Each stroke, for each dirty
idea under the stroke `Budget`:
`idea.form.pivot_loop(&entity.working_atlas(), &self.horizon, budget)`
(A16). Every discharge is logged with its certificate; A18's rank is
asserted before and after each applied pivot (R2). A loop with several
discharging candidates stays a vortex: its candidate cells remain in
`Bubble.nerve` (the thought complex), each carrying its certificate, and
the idea is not `dirty`-cleared. A vortex collapses when: a later
perception adds a face that one candidate predicted (that candidate is
applied, the others become dialect cells); a held form arrives through the
syllabus or a glue whose class key equals one candidate's `after` (that
candidate is applied); or the court, replaying a spoken form that used one
candidate, rules it (the mind learns the ruling through the working atlas
and applies the same). A chord A16 hands to A17 is filed on the idea's form
or left open with `torsion open`; a free chord is spoken as `Novel(g_rem)`
through M6 step 5 unless its key is `Held` in `horizon`. Nothing in this
loop orders candidates by a scalar; the fixed axis order of 12.6 is the
order of trial, not a preference, no axis waits on another's failure, and
a candidate is applied only because it was the only one that discharged.

**25.2 Cauterize.** An idea whose form `is_closed()` after filing (every
loop identity or filed torsion, remainder empty) crystallizes: `as_element`
writes the tape with its torsion section; M2 fixes the address; the form
enters the dialect as `Stood` (`stand_dialect`) and is spoken once. The
idea's bubble is retired from the contemplation (`retire_on_t` skips it
only while `glued_by` is non-empty).

**25.3 Wrap.** `Contemplation::bundle` and `pivot_on` glue crystallized
forms into ideas on meeting charts as `Glued` vertices; the glued form's
loops are not re-pivoted (T6). Depth grows by one per wrap.

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

Carried from chapter 25: forms are spoken. `/opt/mycelium-v0.1.0/src/anastome/dialogue/speech.rs`:

- `SPEECH_WITHHELD: u8 = 3` beside line 127; `encode_speech_reply` (131)
  appends `n_missing u16 ‖ address 32…`; `decode_speech_reply` (137) returns
  `(Vec<u8>, Vec<[u8;32]>)`.
- `StratumDigest` (sheaf.rs 21) gains `depth_max: u16`; `sheaf::digests`
  (44) fills it; `encode/decode_digests_reply` (406/417) carry it;
  `encode_cover_digest` (451) carries the hypha's maximum ruled depth.
- `encode_syllabus_reply` / `decode_syllabus_reply` (754) gain a trailing
  class section: `n_classes u16`, then per class `class key len16 + bytes
  ‖ order u32 (0 = infinite)` (A19 step 1, A21). Speech, quote, and cite
  frames carry one standing byte beside each address, addresses being M2
  composites of variable length (`len16 + bytes`), never 32-byte digests
  (`0 Ruled, 1 Stood, 2 Held`, chapter 24.1).
- The quote reply and the demand reply carry the book height (`h: u64`)
  and the per-lane pressure as four exact `u64` lanes, replacing the
  byte-saturated lanes; the client's `Reply::load()` (a max over lanes)
  is deleted and every reader takes the four lanes (23.8, 24.3).
- Pivot certificates (A20) travel inside the crystallized form's tape
  (chapter 19) and nowhere else; there is no certificate act. A hypha's
  mint for a form (chapter 26, economics) requires every certificate on its
  tape to replay (23.7).
- Every signed frame signs the receipt prefix of 47.1; `cover_signed_prefix`
  (460) gains genesis, epoch, previous receipt hash, K and height; the
  speech frame's `tu` field becomes the voice's K and its cited home head,
  and the root frame's tick becomes the speaker's `(K, height, hash)` with
  the reply carrying the receiver's own `(K, height)` (45.6).
- `/opt/mycelium-v0.1.0/src/bin/mycelium-server/judge/speech.rs`: `ACT_SPEECH`
  handler collects `Kernel::missing_forms(&Element) -> Vec<[u8;32]>`
  **new** into the reply; `admit_peer_digest` (385) records `depth_max` and
  is the point where a pending socket string becomes a `vk` peer (A10);
  `judge`'s percept rendezvous is A11; `nerve::home` (788) gains `depth:
  u16` and excludes hyphae with `depth_max + 1 < depth`.
- **The sentence-native wire (ruled 2026-09-10; a recut, §10, with L9).**
  The speech reply carries one `Adjudication` per sentence sent, in the
  order sent, never an opaque byte: `Stood { receipt }` (docketed, awaiting
  the second organ), `Ratified { receipt }` (carried on the syllabus page
  once a witness court co-signed, 47.3), or `Refused { kind, at, receipt }`
  with `kind` one of `VertexBody`, `EdgeHolonomy`, `RemainderUndeclared`,
  `MalformedComplex` and `at` content-addressed — `Vertex(address)`,
  `Face(address, address)`, `Chart(invariant, cells)` — never a position in
  the sender's tape. The receipt is the court's replay receipt, its chain
  hash of 47.1: a receipt identity, never a digest of content (law 2). The
  quote reply serves each listing as its sentence tape with its open demand
  vertices (kind, chart, cells) and its μ⃗, never as a chain; a probe cites
  against the form's chart coordinates; the demand act (a chain with
  citations) and the percept act are retired, and a frame of either is
  refused at the door. `SPEECH_STOOD`, `SPEECH_REFUSED` and
  `SPEECH_DIALECT` survive only as the adjudication's kind byte.

`/opt/mycelium-v0.1.0/src/economy/`: five organs (Stage 8b S12) —
`lanes.rs` what a word measures and how two measurements compare (law 5);
`treasury.rs` who is bonded and what the audit says (E1); `escrow.rs` value
set aside against a listing (E1, E2); `settle.rs` what each closed act pays
(E2, 26); `persist.rs` the ledger on disk (28).

- `settle_recursive(&mut self, escrow: &[u8;32], shares: &[([u8;32],[u64;4])], anchor) -> [u64;4]`
  **new**: draws the bundle's escrow proportionally to each speaker's
  verified deflation and mints each share via `mint_claim` (289). Called
  from `Kernel::pay_solve` (cognition.rs 3220) and `home_install` when the
  form has `Glued` vertices; shares from `rule_replays_family`.
- **A cited generator is paid at the install of a standing sentence**
  (ruled 2026-09-10). A word vertex's derivation names the generators it
  composed (50.2 step 2). Nothing rides beside the tape and nothing is
  paid on hearing: the court replays the derivation as part of `G`, and
  only when the sentence stands and `home_install` has ruled the form
  is the use paid. **It is paid as a share of the install's own mint,
  never as a transfer from the citing voice's balance** (corrected
  2026-09-10, after cdb917f measured the transfer paying zero: the lane
  a cited generator is priced on is the lane the citer's draw occupies
  at the moment of install, so a transfer from the balance pays nothing
  and a broke voice could cite for free). The mint at install is the
  deflation the sentence closes on its family; the part a cited form
  closed is that form's speaker's, and the citer earns the rest. So
  `home_install` computes one share per cited form — its verified
  deflation on the citer's family at the citer's placement, once per
  cited address however many vertices cite it — and `settle_recursive`
  draws the released award by those shares and mints each to its
  discoverer, the remainder to the escrow's voices. `settle_use` is the
  probe's transfer (a use settled against a listing) and is not called
  at install. A refused sentence pays nobody, because a derivation that
  did not compose proved nothing; a sentence deferred for an unruled
  citation pays nobody yet, and pays when the deferral clears. This is
  the same shape as a `Glued` citation's use, which is what a derivation
  citation is at the form level (21 step 3), and it is row L8t.

  **Two levels of citation, two measures** (ruled 2026-09-10, after
  eece48a measured a derivation's share at zero). A `Glued` citation
  closes part of the citer's family, and its share of the mint is that
  closure, by `settle_recursive` as above. A derivation citation
  contributes an **action** the citer composed from the generator and
  closes nothing on the citer's family by itself; measured as a closure
  its share is zero, and that zero is correct, not a fault of the
  measure. What a use of a generator earns is the thing E2 already names
  for a use: **room**. `use_credit(who)` is the lane-wise sum, over the
  forms `who` discovered, of each form's μ⃗ once per distinct citer
  recorded in `cited_by`, and `lane_room_of` adds it to the bond before
  the draw is read. It is computed at the read and stored nowhere, it is
  never summed across lanes, and it never mints: a generator many
  derivations compose through lets its speaker draw more, which is the
  reward the language pays for being composable. `cited_by` therefore
  records a derivation citation as it records a `Glued` one, one entry
  per citer address per cited address, however many of the citer's
  vertices compose through it. The observable for a derivation
  citation is the discoverer's room rising by the cited form's μ⃗ on the
  citer's install, and its balance not moving.
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
- **Condensation retires what it derives** (3.8, ruled 2026-09-08): a
  torse or a punctured bundle that closes what its cited forms close with
  more reach or a shorter description relocates them to the cavity,
  addresses kept, and is paid the compression yield (E2); `shorter_form`
  widens from strictly shorter tapes on the same vertices to this, the
  reach read from the certificate (S9.13).

## 28. Persistence

- `GEN2`: `depth` after `mag`; sentence tape v2 in the description; a v1
  tape reads as depth 0 with `Word` vertices only. Runs A8 at load.
- `MYC2`: ruled forms as tapes; `cocycle_at` with class keys; `cited_by`,
  `replay_memo` rebuilt at load; `RMP1` section while the migration
  window is open (chapter 33).
- `KCH1`: the receipt chain with parents and heights (45.1); every `_tu`
  carrier in `MYC2`, `DEF1` and the genome is a height (45.4); no unix
  origin is persisted anywhere, and the mesh record carries genesis and
  epoch (45.5).
- **The genome section's tag is `GEN3`** (drift corrected 2026-09-10).
  The code has written and refused on `GEN3` since the fibre-index tail
  landed (`epigenome/persist.rs`); this chapter says `GEN2` at five
  lines, and a sixth was added by the v7 paragraph below before the drift
  was noticed. The tag on disk is the authority — `uncrystallize` refuses
  anything else — so the name is corrected here rather than in the code,
  and the older lines are left standing as the record of when each was
  written (§5.4: nothing is amended silently). Every `GEN2` below that
  names the genome section means `GEN3`.
- Genome (`crystallize` 1011 / `uncrystallize` 1025): `GEN3`; dialect,
  ideas, and withheld forms never written; the epigenome is rebuilt by
  thinking, which is why it may be lost on restart and the genome may not
  (chapter 24.1); syllabus of opened letters (`SYL1`, chapter 37) when Part
  VI lands.
- **The genome section does not change for v7** (ruled 2026-09-10).
  `GEN3` writes forms as tapes, and a derivation is a coordinate *of a
  tape* (50.2 step 2), so it persists inside what `GEN3` already writes.
  The generator index a mind reads its presentation from is not persisted
  either: like the axis index and the moduli index, it is rebuilt from
  the restored forms at `uncrystallize`, because it is a routing
  coordinate over content the genome already holds and a persisted index
  is an index that can disagree with its content.
- **Persistence by torsion (M9).** A mind's classes persist on the mesh, not
  on the mind: filed torsion travels on the spoken form's torsion section
  into the chart's `Congruent` cells, free classes into `cocycle_at`, both in
  `MYC2` with their class keys. A restarted mind refills its `horizon` from
  the chart's class records on its first quote (A19 step 4). Nothing the
  mind thought is lost to the mesh; what the mind itself keeps across a
  restart is exactly the genome.
- **The archived prefix (45.8, ruled 2026-09-09).** `ARC1`: the receipts of
  every folded prefix, cold, keyed by the fold root, written once at the
  fold and never rewritten; the hot ledger holds the `epoch` rows and the
  tail after the last fold. A restore reads the hot ledger as before and
  opens `ARC1` only to serve a K-range fetch or a court's replay of an era.
- **The act kind on the receipt (45.1, S13.6).** Every receipt on every
  tape — the kernel snapshot (`MYC4`), the court journal, the wire — carries
  its `ActKind` byte after its parents. `MYC3` snapshots do not re-read
  (a layout change is a recut; no legacy reader is carried).
- **The fold root and the archive (45.8, S13.9).** `KCH1` carries the
  chain's fold root — the archived prefix's K, hash and height, or none —
  before the hot receipts, so a chain that begins at a fold re-verifies
  from its root; `ARC1` follows it: one era per fold, in K order, each the
  root, the identities its epoch rows named and the receipts it archived.
  Both are in `MYC4`, before the recut.

## 29. Binding summary by file

Everything Parts III and IV change, in one table.

```
alphabet/rotor.rs  Rotor::normalised (sign step), Rotor::projective_eq (either sign), Rotor::key (A1), Rotor::class_key (15.3),
              Rotor::coordinate DELETED, PlaneForm::coordinate DELETED, PlaneForm::conjugacy_key (15.2), PlaneForm eq on reduced,
              PlaneForm::torsion_part() -> (u8,u8) NEW, PlaneForm::free_part() -> (i64,i64) NEW,
              Word::form (reduced), Word::normal_form (A2 step 1 off-plane), Word::key (= A1 key), Word::class_key (15.4),
              Word::{with_contact, contact, rebase} NEW, shorter_tape(word, budget) NEW, Contact NEW
cga.rs        Mv::canonical NEW, Mv::projective_eq NEW, Mv::key NEW, derive(PartialEq) REMOVED
element/       canonical_chain(&[Cell]) -> Vec<Cell> NEW (A5), Element::same_vertex NEW, PartialEq REMOVED,
              form_key_bytes uses the M1 key, depth field, GEN2 depth after mag
sentence.rs   Vertex/Face/Loop/Form NEW (18.2), Form::{face_of, delta1, coheres, is_closed, depth_of, regauge, glue, retract_vertices, rebase, unfold, loops_of},
              as_element/from_element v2 (19), vertex_address writes Mv::canonical, from_element recomputes germ addresses,
              chart_of on reduced forms, SentenceRule { form, sections, ratio } (31), Nerve/Germ/Cycle REMOVED (30)
family/       word_form, frame_vertex, pair_form, Crystal, extract_rule (seven moves), speak_rule on Form, rule_replays_family shares, orbit/OrbitEnd
cech.rs       ChartGraph, Goodness, goodness, cell_key with component, set_edge_in, Overlap, MayerVietoris, Complex::{overlap, mayer_vietoris, delta2, restrict, straddling},
              solve_gauge_between (A14); solve_gauge unchanged
nerve/        Keyed::meet (A13), Cover::charts_of, Router::insert(vk, addr), home(depth), no hash(addr) keys outside pending_addrs
stalk.rs      Stalks::refine(inv, &[u64]), split_stratum coordinate_of -> Vec<u64>, invariant/invariant_of empty on A6 refusal
blade/body.rs axes_of -> Option<[u64;2]> (A6), moduli -> Option<Vec<u64>>
blade/harmonics.rs  Harmonics::axes -> Option<Vec<u64>>
anchor.rs     Sel::to_bytes NEW (A9)
verdict/      replay_form NEW, REFUSE_* (chapter 34), sel_key over Sel::to_bytes, blind_replay canonicalises the spoken chain and recomputes 13.9 step 5
cognition.rs  resolve_form, replay_memo, cited_by, cocycle_at fifth field, push_cocycle (M5), glue_fetched (M8), refine_chart (15.5),
              cover-change re-representation (15.6), restore: A8 + RMP1, composite_parent_id/section_address via M2,
              reduce_excluding calls A12 for connections, shorter_form, missing_forms, pay_solve → settle_recursive
economy/       settle_recursive, use_credit, Treasury::restore A8 collision rule
sdk.rs        Epigenome::by_moduli NEW (21 step 3: body chart → closed forms carrying a body there; moduli_admit/moduli_retract beside axis_admit/axis_retract,
              standing_of NEW), Crystal::on_body NEW on every impl (the glue's read; on_class keeps 24.2's cited-form chart),
              Epigenome::working_atlas NEW, Standing NEW, dialect Stalks NEW (admitted in stand_dialect, retracted in shed),
              learn_one promotion dialect → genome by same_vertex, Comprehension::Withheld, comprehend (24, M6),
              pull_syllabus depth order, uncrystallize A8, peers keyed by vk with pending_addrs (A10)
bubble/drive.rs  pivot_on and expand_with call Form::pivot_loop (A16), vortex cells kept in Bubble.nerve with certificates (25.1),
              Bubble glues/glued_by, retire_on_t, Contemplation::bundle,
              horizon: Keyed<(Rotor, Order, Standing)> by class key, Contemplation::seed_horizon NEW (A19)
sentence.rs   Form::class_vector NEW, ClassVector/Axis NEW (12.6), Form::pivot_loop NEW (A16), PivotCertificate NEW (A20) with tape section (19),
              Form::rank NEW (A18); cech.rs Complex::rank NEW
sentence/     Form.by_class NEW (18.2, written in reindex, never on the tape); reading.rs Reading.held_at NEW, HeldCite NEW,
              vertex_cites NEW, no_held NEW (24.1, L6); chain.rs Sentence.held, Located.held NEW
blade/carry.rs  carries NEW (the cell-by-cell verification, shared), versor_carrying NEW (24.1: the translator solved for a given linear action)
bubble/       Contemplation::held_at NEW (24.1); hold_form reindexes the form it takes custody of
family/perceive.rs  scene_of_chain NEW (50.6 seam 1: a chain's before and after grids in one frame)
client/sdk    Entity::ideas DELETED, hold_thought DELETED, conclude DELETED, cited_word DELETED, WorkingAtlas::standing_addr DELETED (L6);
              hold_chain NEW (the pair form of the chain's scene), WorkingAtlas.held is the contemplation, Entity::cite reads the Held
              stratum through it, open_axis/axis_germ take the held cites' bodies
alphabet/rotor.rs  Rotor::order NEW (15.3 as ruled: the period of the class invariants); Rotor::word_order DELETED; Word::order delegates;
              unit_order DELETED; cech/cell.rs word_of_rotor DELETED, extend reads the transition's own order (L2b)
blade/body.rs  components joins by adjacency within one hue (20 as ruled, L6b)
server/cognition/reduce.rs  percept_elements lists one sentence per percept, its demand vertices riding it (20, 50.1; L6b);
              germinate/ingest.rs the puncture-before-mint move applies to residual chains only
element/carrier.rs  record_torsion as A17's filing call; toroid.rs witness_stored as A17's witness
cognition.rs  glue_fetched torsion arm (23.7), serve_ruled class records (A19), Kernel::rank debug assertions
dialogue/      SPEECH_WITHHELD, speech reply missing list, depth_max in digests, syllabus class section, standing byte
mycelium-server/       judge: A11 and missing_forms; admit_peer_digest depth_max + A10; id_of only for pending_addrs
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
| `Nerve` | `family/` | 3 |
| `Nerve` | `bubble.rs` | 2 (`hold_nerve`, `Bubble.nerve`) |
| `Nerve` | `mycelium-client.rs` | 1 (`speak_sentence`) |
| `Germ`/`.germs`/`Cycle`/`.cycles` | `family/` 22, `bubble.rs` 14, `cognition.rs` 6, `cech.rs` 5, `sdk.rs` 2, `mycelium-client.rs` 2 | |
| `SentenceRule` | `family/` | 8 |

`ResidualGerm` in `germinate.rs` and its uses in `element/`,
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
3. `family/pair.rs`: `pair_nerve` → `pair_form`; `extract_rule` on `Form`s;
   `SentenceRule` per chapter 31; `speak_rule`, `rule_replays_family`,
   `mint_sentence` (981) on `Form`. **Landed** (`abacf53`): `extract_rule`
   is `extract_rule_with`; `speak_rule` speaks the form; `agree_up_to_gauge`
   is gone. Nerve fields on `SentenceRule` remain until step 6.
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
(replaced by the keyed map). The eight consumers in `family/` read
through `rule.form`. Until 1.15 the live `SentenceRule` still carries
nerve adapters (`charts`, `faces`, `holonomies`, `nerve`); `hue_maps` and
`shifts` are already `Keyed` by chart key and transport-class key
(`abacf53`).

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

`Element::address()` (element/vertex.rs) hashed canonical description bytes
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

### 34.1 Refusals, `/opt/mycelium-v0.1.0/src/court/verdict/mod.rs`

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
pub const REFUSE_REDUNDANT: &str = "form closes nothing new, reaches nothing new, and is no shorter than what it cites";
pub const REFUSE_NOT_CLOSED: &str = "form leaves a remainder on the family";
pub const REFUSE_MV_INEXACT: &str = "Mayer–Vietoris exactness fails: cochains are inconsistent";
pub const REFUSE_OVERLAP_CYCLIC: &str = "translation across a non-acyclic overlap";
pub const REFUSE_NOT_NATURAL: &str = "class key changes under restriction";
pub const REFUSE_RANK_LOST: &str = "form rank below what its tape re-derives";
pub const REFUSE_PIVOT_INEXACT: &str = "pivot certificate does not replay";
pub const REFUSE_ORIGIN: &str = "receipt names another genesis or origin epoch";
pub const REFUSE_RECEIPT: &str = "receipt height does not re-derive from its parents";
pub const REFUSE_RECEIPT_STALE: &str = "receipt index at or below the signer's known head";
pub const REFUSE_MANIFEST: &str = "signing key not in the signer's active manifest";
pub const REFUSE_WITNESS_DEPENDENT: &str = "witness bond lies inside the home's delegation closure";
pub const REFUSE_WITNESS_ROOM: &str = "witness room does not cover the escrow on an occupied lane";
pub const REFUSE_UNPROVEN: &str = "utterance citations do not deflate its chain";
pub const REFUSE_ROOM: &str = "voice room does not cover the utterance on an occupied lane";
pub const REFUSE_FORM_TAPE: &str = "form does not parse from its tape";
pub const REFUSE_EQUIVOCATION: &str = "two receipts at one K with different chain hashes";
pub const REFUSE_SIGNATURE: &str = "signature does not verify over the signed prefix";
pub const REFUSE_MALFORMED: &str = "act names a payload it does not carry";
pub const REFUSE_NOT_EQUIVOCATION: &str = "two receipts that do not share key and K, or do not differ";
pub const REFUSE_FOLD_OPEN: &str = "fold prefix is not final";
pub const REFUSE_FOLD_INEXACT: &str = "fold does not conserve";
pub const REFUSE_NOT_A_SENTENCE: &str = "utterance is a chain, not a sentence";
pub const REFUSE_VERTEX_BODY: &str = "vertex does not carry its body onto its image";
pub const REFUSE_EDGE: &str = "edge transport is neither in the stabilizer nor the exchange nor a remainder edge";
pub const REFUSE_DEMAND_UNTYPED: &str = "demand vertex names no kind";
pub const REFUSE_NOT_CONTRACTION: &str = "sentence is not the contraction of the refused sentence at the coordinate";
```

`REFUSE_NOT_A_SENTENCE`, `REFUSE_VERTEX_BODY`, `REFUSE_EDGE` and
`REFUSE_DEMAND_UNTYPED` (chapter 50, ruled 2026-09-09) are the receiver's
four answers to an utterance that is not a valid sentence: a bare chain
with no vertex, a word vertex that does not reconstruct its body on the
receiver's copy of the input, a face whose edge transport re-derives as
none of 50.3's three readings, and a demand vertex with no kind. Each is
a refusal that travels to the voice; none is a slash, because a sentence
that does not replay was never installed. They travel as the kinds of the
adjudication of 50.9 (ruled 2026-09-10): `VertexBody` is
`REFUSE_VERTEX_BODY`, `EdgeHolonomy` is `REFUSE_EDGE`,
`RemainderUndeclared` is `REFUSE_NOT_CLOSED`, and `MalformedComplex` is
`REFUSE_NOT_A_SENTENCE`, `REFUSE_FORM_TAPE`, `REFUSE_DEMAND_UNTYPED`,
`REFUSE_NOT_CONTRACTION` (a sentence citing a refusal receipt that is not
the contraction at the coordinate the court named, 50.9) and the tape
refusals of 23.1 steps 1–2 and 5–6. `REFUSE_UNPROVEN` is retired with the
chain (23.8).

`REFUSE_FOLD_OPEN` and `REFUSE_FOLD_INEXACT` (45.8, ruled 2026-09-09) are
the two answers to a fold: a prefix below whose head a listing,
provisional word or escrow still stands is not final and is not folded;
a fold whose `epoch` rows do not satisfy `MU-ECONOMY.md` §6 against the
prefix they replace does not conserve, and the witness refuses to
co-sign it. Neither replays anything but the ledger.

`REFUSE_MALFORMED` (Stage 11 I18) is the door's answer to an act whose
frame parsed but whose fields do not agree with one another: a
`RULING_REBIND` with no selection, a `RULING_ATTACHED` with no base and
transport. It is not `REFUSE_FORM_TAPE`, which says a form's tape does not
re-derive, and it is decided by `court::admission::admit_payload` before
any carrier is touched; the reply is `Ruled::Refused` naming it.

`REFUSE_NOT_EQUIVOCATION` (Stage 11 I21) is the door's answer to an
`ACT_EQUIVOCATION` (0x35, 47.4, `XSEPTUM.md` §5.5) whose two receipt
sections are not a proof: signed by two keys, at two K, or carrying one
chain hash, which is a duplicate frame and not an equivocation. Before it
is read, both prefixes name this node's origin (`REFUSE_ORIGIN`), both
signatures verify under the key each receipt names (`REFUSE_SIGNATURE`),
and each receipt agrees with its prefix and re-derives its own chain hash
(`REFUSE_RECEIPT`); `court::admission::admit_equivocation` decides all of
it before any carrier is touched. A proof burns the signer's bond on all
four lanes once and is answered `Ruled::Applied`; the same proof presented
again finds no bond and is answered `Ruled::Inert`. A third party presents
what it holds; the court replays nothing and judges only the signatures.

These are returned by `replay_form` and `blind_replay`, by the admission of
47.2 (`Admitted::reason` names the constant an arm answers with:
`REFUSE_RECEIPT_STALE` for a duplicate frame, `REFUSE_EQUIVOCATION` for
the proof of 47.4; `admit_signature` answers steps 1 and 2 with
`REFUSE_ORIGIN` and `REFUSE_SIGNATURE`, in that order, and `admit_ruled`
answers 47.1's two bindings with `REFUSE_RECEIPT`; `court/admission/` holds
both and re-exports the receipt constants from this file), by
`Kernel::commit_germ` as
`Outcome::Refused(REFUSE_ROOM)` on the demand and percept replies (wire
code 7), and by `Kernel::perceive_speech` in the speech reply. Speech replies also carry
`"citation pending"`, `"overlap goodness pending"`, and `"letter not in
syllabus"` (chapter 37) as `Glue::Dialect` reasons.

### 34.2 Server log lines, `mycelium-server/` and `cognition.rs`

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
  court organs reached {n} silent {n} moved {bool}
  court offered {n} germs to the overlap
  court genesis {hash} epoch {e} (written once, at the first receipt)
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
  court pivot certified {addr} loop {i} axis {label} {before}→{after}
  court form refused {addr}: pivot certificate does not replay
  court genesis {hash} epoch {e}
  court receipt parent {vk} K {k} h {h}
  court receipt gap {vk} {k_head}→{k} backfill
  court origin refused {vk}
  court witness {vk} co-signed {addr}
  court equivocation {vk} K {k} burned {mu:?}
  court citation lapsed {addr} after {n} receipts
  court concurrent {addr_a} | {addr_b} held
  court attest {addr} voice {vk} K {k} cites {n} residual {m} of {chain_len}
  court attest duplicate {addr} voice {vk} K {k}
  court room refused voice {vk} lane {l} draw {mu} room {r}
  court syllabus {addr} to {vk} receipt K {k} h {h}
  court book height {h} frontier {n} for {vk}
  court offer refused {addr} from {vk}: {REFUSE_*}
  court offer refused {addr} by {vk}: {REFUSE_*}
  court offer unmeasured {addr} at {vk}
  court offer deferred {addr} by {peer}
  court exchange deferred by {peer}
  court exchange asked {peer} K {k}
  court receipt backfill {vk} {k0}..{k1} admitted {n}
  court orbit {carrier} {n} period {strokes}
  court phase h {h0}..{h1} comprehension {deflated}/{held} congruence {sections}/{strata} lineages {n} growth +{adjoined} -{retired} expansion {axes}/{splits}
  court fold {vk} K {k} h {h} prefix {k0}..{k1} identities {n}
  court fold witnessed {vk} co-signed {root}
  court fold refused {vk}: {REFUSE_*} [origin {g} e{e} carried {g'} e{e'}]
  court nerve deferred by {addr} acts {n}
  court demand listed {kind} chart {inv} from {vk}
  court sentence refused {addr}: {REFUSE_*}
  court adjudication {addr} for {vk}: stood receipt {k}
  court adjudication {addr} for {vk}: refused {kind} at {coordinate} receipt {k}
  court contraction verified {addr'} from {addr} by {kind} at {coordinate}
  court contraction refused {addr'}: not the contraction of {addr} at {coordinate}
  court era adopted {root} h {h}
  court verdict refused {addr} from {vk}: {REFUSE_*}
  court verdict unread {addr}: the court is not readable
  court recall short voice {vk} lane {l} by {mu}
  court slash short voice {vk} lane {l} by {mu}
```

The two `court offer refused` lines are the two ends of one act: `from`
is the witness refusing a frame 47.2 would not admit (`judge/offer.rs`),
`by` is the home reading a witness's refusal off the reply
(`offer.rs::offer_round`). `court offer unmeasured` is the third arm of
that reply — the witness admitted the frame and has re-derived nothing of
its own on the class, which is neither agreement nor refusal (47.3).
`court verdict unread` is not a refusal and names no 34.1 constant: a
court whose kernel cannot be read has judged nothing, and answers
nothing. `court offer deferred` and `court exchange deferred` are the
deferral frame — the reply a full reflex pool gives every act (23.8) —
read on a hypha-to-hypha reply as what it is, impedance: the peer refused
nothing and witnessed nothing, and the act is asked again after that
peer's Λ, never on the next cycle (Stage 12 S12.4c, S12.4d). Before this
the home read the deferral's zero code as a witness's refusal by key
`0000…` and re-sent the offer on every cycle. The two `short` lines are the lanes on which a voice's balance
or bond did not cover what was asked, read per lane as `court room
refused` reads a draw against a room (law 5).

### 34.3 Client log lines, `mycelium-client.rs` and `sdk.rs`

Prefix `[{tag}] ` as the existing lines:

```
[{tag}] form spoken: {addr} depth {d} vertices {v} glued {n} faces {f}
[{tag}] form withheld: family {fam} loop {i} holonomy {key} | remainder {n} cells
[{tag}] form withheld: family {fam} missing glue {addr}…
[{tag}] form glued {glued_addr} into idea {idea_key} remainder {before}→{after}
[{tag}] form understood through {held_addr} for {cited_addr}
[{tag}] dialect {addr} class recorded
[{tag}] dialect stood {addr} k {k} tu {tu}
[{tag}] form learned {addr} depth {d} (syllabus order ok)
[{tag}] form class loop {i} torsion {(s,u)} free {(m5,m13)}
[{tag}] orbit {addr} contact ring {r} steps {l}
[{tag}] unfolded {addr} measure {m} steps {l}
[{tag}] retired {addr} shortlex
[{tag}] retired {addr} shorter
[{tag}] thought merged {a_key}+{b_key} overlap {v} one section
[{tag}] thought held apart {a_key}|{b_key} class {key}
[{tag}] chart refused: ratio exceeds axis
[{tag}] axis refused {addr}: {why}
[{tag}] quote closing {k} charts (ruled {a} stood {b} held {c})
[{tag}] promoted {addr} dialect → genome uses {n}
[{tag}] pivot {idea} loop {i} axis {label} discharged {before}→{after}
[{tag}] pivot {idea} loop {i} axis {label} no discharge
[{tag}] pivot {idea} loop {i} closed by the tree gauge
[{tag}] pivot {idea} held, tree gauge cannot be applied
[{tag}] pivot {idea} held, rank overflow
[{tag}] vortex {idea} loop {i} {n} candidates held
[{tag}] vortex {idea} loop {i} {n} candidates unmeasured
[{tag}] vortex {idea} loop {i} collapsed by {perception|glue|ruling} to axis {label}
[{tag}] torsion filed {idea} (σ,ℓ)
[{tag}] torsion open {idea} chord {i}
[{tag}] demand spoken {key} order ∞
[{tag}] horizon {n} seeded from {hypha} chart {inv}
[{tag}] form spoken: {addr} … torsion {n}
[{tag}] receipt K {k} h {h} parents {n}
[{tag}] manifest {n} keys K {k}
[{tag}] form withheld: family {fam} remainder {n} cells over {p} pairs
[{tag}] sentence spoken: {addr} vertices {v} demands {d} loops {l} remainder edges {r}
[{tag}] demand {kind} chart {inv} on {addr}
[{tag}] edge remainder {i}-{j} on {addr}
[{tag}] uttered {addr} standing {s} K {k} lanes {mu:?}
[{tag}] listing {addr} reduced by {c} cites: residual {m} of {n}
[{tag}] listing {addr} held: no held word deflates it
[{tag}] syllabus refused {addr}: {REFUSE_*}
[{tag}] quote asked: language {changed|same} book {h}
[{tag}] stroke idle: dwell {[s, g, t, h]} strokes
[{tag}] stroke outcome fed {mu:?} pivoted {mu:?} stood {mu:?} uttered {mu:?} closed {mu:?} learned {mu:?}
[{tag}] refracted {addr} into {frame} by Γ
[{tag}] pivot {idea} loop {i} axis orientation({p}) discharged by half-turn
[{tag}] orbit {carrier} {n} period {strokes}
[{tag}] era adopted {root} h {h}
[{tag}] adjudicated {addr}: stood receipt {k}
[{tag}] adjudicated {addr}: refused {kind} at {coordinate} receipt {k}
[{tag}] contracted {addr} → {addr'} by {kind} at {coordinate}
[{tag}] quiescent {addr}: held inert on chart {inv}
[{tag}] sealed {addr}: refused at K {k}, not uttered again
```

`court orbit` and `[{tag}] orbit` (45.7, ruled 2026-09-09) are written
once per completed circuit of a carrier — the docket, the exchange to
one peer, the mind's stroke ring — with the circuit count and the period
in strokes, so velocity is read in kernel coordinates.

Stroke line adds `forms {spoken}/{withheld} glued {n} depth {max}`; it is
written once per aperture cycle and on every change of K (24.3).

`sentence spoken`, `demand … chart … on`, `edge remainder`, `court demand
listed` and `court sentence refused` (chapter 50, ruled 2026-09-09) are
the lines of the sentence as the unit of speech: a voice writes one
`sentence spoken` per utterance with the count of its demand vertices,
one `demand` line per (kind, chart) it carries, and one `edge remainder`
per non-rigid pair; the court writes one `court demand listed` per demand
vertex it lists and one `court sentence refused` per utterance that is
not a valid sentence. `remainder expressed` is retired with the seam that
wrote it (50.6): no residual is expressed as a word, and an `uttered` line
with `cites 0` on a chain is the fault the auditor greps for.

`court nerve deferred by {addr} acts {n}` (R2, 2026-09-09) is the nerve
thread's reading of a peer that deferred the acts routed to it — a demand
routed to a listing's home above all — written once per aperture of acts
per peer, never per act: written per act it was 376,000 lines in one
aperture, each one a percept the home's full reflex pool had deferred.

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
- No digest, hash, or fingerprint of any mathematical structure exists in
  cognition: every key is the canonical coordinates of an exact object
  written out (A21) and `decode(key)` returns the object; `blake3` appears
  only in `network/`, `court/identity`, `chronometry`, and the persistence
  loaders, over wire bytes, file bytes, and receipts; no `Keyed` is keyed
  by a 32-byte digest, and no two keys are compared as digests.
- No `(ℚ, ℚ)`, `(BigRational, BigRational)`, or `(i64, i64)` pair is carried
  as a placement, shift, or residue outside `pair_form`, `speak_rule`, and
  `orbit`; every placement is a carriage versor and every residue a
  component of a holonomy versor (law 11).
- Every coordinate a form stores re-derives from the form's own words and
  faces (law 12); a coordinate that does not is filed as a section
  coordinate and never enters a key, a class vector, or a certificate.
- No `QMatrix`, affine block, or matrix product carries, composes, or
  compares an action anywhere in `transducer/`, `server/`, `client/`, or
  `court/`; `core::zlinalg` holds the only matrix arithmetic, for ranks,
  and A3's sensor-plane block is read from a rotor and never stored.
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
- No pivot is selected by a measure: every `pivot … discharged` line names
  an axis the loop's class vector occupied, every applied pivot has a
  certificate that replays on the court, and a `vortex … held` line is never
  followed by an applied pivot on that loop without a `collapsed by` line
  naming the perception, glue, or ruling that collapsed it (law 10).
- No `SystemTime`, `Instant`, or `Duration` is named in a kernel, court,
  ledger, physics, or transducer file; every duration is a stroke count or
  a height difference (45.4), and the clock gate of `MU-ENGINEERING.md` §7
  fails the build otherwise.
- Every `_tu` carrier is a height and every `_k` carrier a receipt index; no
  field anywhere holds a unix second.
- Every signed frame's prefix carries the node's own genesis and epoch; a
  differing one appears only as a `court origin refused` line.
- Every `court form installed` line is preceded by a `court witness …
  co-signed` line for the same address from a key outside the home's
  delegation closure (47.3).
- Every equivocation proof produces exactly one `burn` entry and one `court
  equivocation` line per hypha, and `Treasury::audit` holds on each (R9).
- No `court concurrent … held` pair is followed by a resolution by order; it
  is followed by a merge with conservation, a `court mv` line, or a burn
  (45.3).
- Every `form withheld` line is followed in the same stroke by a `dialect
  stood` line and one `sentence spoken` line whose `demands` count is the
  number of (kind, chart) pairs the withheld bodies fall under; no
  `remainder expressed` line and no `uttered … cites 0` line on a chain
  appears on any voice (50.6); a voice that has logged a withheld form
  never shows `dialect 0` on its next stroke line (24.3).
- Every `uttered` line names a sentence address that a `dialect stood` or
  `sentence spoken` line on the same voice logged earlier, and there is
  one `uttered` line per (address, K) on that voice; every listing on any
  book carries a demand kind (50.5), and every `court attest` line reports
  a sentence replayed, never a chain docketed.
- No hypha reaches `reduce_all` or `reduce_germ` from a wire act; every
  `court attest` line reports citations replayed and a residual measured
  (23.8).
- Every `court syllabus` line has exactly one `form learned` or one
  `syllabus refused` line on the receiving voice, and no `form learned`
  appears without a replay before it.
- On any voice, `quote asked` lines per aperture cycle never exceed one
  plus the changes of its working language plus the advances of the book
  height in that cycle; an idle voice writes one stroke line per aperture
  cycle, each `stroke idle` line carrying four lane dwells; no `max` over
  lanes and no clamp appears anywhere in the pacing path, and no standing
  byte is multiplied into a μ⃗ or ordered against another.
- `court room refused` never appears on a voice whose bond covers its open
  dockets; no count of utterances, dockets, or quotes is compared with a
  literal anywhere in the court or the client.

---

# Part VI — Extensions: breaking the two walls

Carried from Parts I–V: the engine on ℚ over the sensor plane. Three
limits are built into the carriers, not the doctrine; the third, the prime
wall, was named on 2026-09-10 when the ruling of law 6 gave the court the
effector chapter 50's demand had been missing. This part specifies the
upgrades as language extensions: nothing in Parts I–V changes, the 17
letters remain the base no voice changes, exact arithmetic remains exact,
each upgrade opens only where a demand asks for it, and since 37.7 the
court answers the demand at run time.

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

### 36.3 The prime wall (ruled 2026-09-10)

The rational points of the unit circle are a group: the units `±1, ±i`
times a free abelian group with one generator per prime `p ≡ 1 (mod 4)`,
the Gaussian prime `π_p = a + bi` with `a² + b² = p` (Fermat; the
decomposition is unique under `a > b > 0`). A plane rotation over ℚ is a
product of powers of these generators, and the alphabet's prime letters
are the generators at `5` (`2 + i`) and `13` (`3 + 2i`) alone. A rotation
whose weight carries `17`, `29`, `37`, `41`, … is exact over ℚ, is a rotor
of `Spin(4,0)(ℚ)`, and spells in no word: the peel of `spell` stops at a
remainder after the `5` and `13` shells, and `factor_rotor` names it
`Demand::RotorPrime(plane, prime)` (50.4). This wall is inside ℚ. No
cyclotomic arithmetic crosses it, no conductor rises, and nothing of
37.1–37.4 is needed: what is missing is one letter, and the letter is the
witness rotor's own Gaussian factor (37.7).

On the sensor's ℤ² a rotation carries cells onto cells only in `D4`, so
the prime wall, like the field wall, does not bite on ARC. It is
specified because a demand without a consumer is a sensor without an
effector: since chapter 50 a voice names the prime it cannot spell, and
until 37.7 nothing in the running engine could answer it but a commit.

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
a fifth lane; among two descriptions closing the same remainder the one
the other derives retires it (27), and nothing prefers by conductor. The
tower is climbed only when ℚ cannot close, and a word ℚ can close is never
spoken at a higher conductor.

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
17], ext: Vec<ExtLetter> }`, `ExtLetter::Turn { n: u32, plane: (u8, u8),
conductor: u32 }` beside `ExtLetter::Prime` of 37.7. An extension letter
is named on the tape by its content under the escape byte `0x7f` — the
kind, the plane and `n` (or `p`) as A21 coordinates — never by an id
assigned in open order (law 6 as ruled 2026-09-10; 37.7 P4 says why), and
the genome's syllabus section `SYL1` lists the opened letters by the same
content, so two nodes that learned a letter hold one name for it without
agreeing on an order; a node that has not learned it refuses the tape with
`REFUSE_LETTER_UNTAUGHT` (`"letter not in syllabus"`) and asks for the
syllabus by the existing teach path.

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

`core/cyclo.rs`, `core/scalar.rs` new; `alphabet/`: `Rotor<Scalar>`,
`Alphabet { base, ext }`, `ExtLetter`, `relators()` appending, `spell`
steps 1–4, `PlaneForm` torsion in `Z/N`; `element/letters.rs`: `Letters` carries
the conductor; `economy/settle.rs`: conductor factor in the S-lane cost; genome
section `SYL1` `(n, plane, id)*`. Tests and traces: chapters 39–40.

### 37.7 THE ADJUNCTION — the court adjoins a remainder as a generator (ruled 2026-09-10)

**There is one move, not one per wall.** A demand is the canonical
remainder a factorisation through the presentation left (A12 step 5,
50.2 step 2), and the generator the presentation lacks **is that
remainder**. It is already exact, already measured, already canonical,
and it carries itself as its own witness; nothing is searched for and
nothing is constructed. The court adjoins it when two independent courts
re-derive the same remainder from the same witness, rules it with a
receipt, and the syllabus teaches it before any voice speaks it. That is
the whole of how the language grows a generator, and 1.8 already said so:
"a residual that no generator reduces is adjoined as a new generator once
the court has replayed it and independent lineages have reached the same
conclusion".

**The kind is a level, never a dispatch.** A remainder lives at the level
its factorisation stopped at, and the level is a coordinate read off the
remainder — for the log line and for the route — never a key into a table
of closers. Its canonical form at that level is the one the tree already
computes:

| level | the factorisation that stopped | the remainder's canonical form | already in the tree |
|---|---|---|---|
| the ring | A22's peel over one plane block | the Gaussian factor, `a + b·e_ij` with `a² + b² = p`, `a > b > 0` | A1 (16.1) and `core::gaussian` |
| the presentation | A12 over the stratum's generators | the HNF residue, unique under the row order | A12 step 5 |
| the cover | the pieces' nerve does not glue | the filled cell over the meet's loop, `Fill::Class(ℓ)` | `cech::extend` (50.8) |
| the relations | two proofs of one vertex do not commute | the composite holonomy `g⁻¹ ∘ f` around the proofs, keyed by 15.4 | `cech::holonomy`, the membrane of 23.9 |

The last row adjoins a **relation**, not a generator: two proofs of one
vertex whose composite holonomy is non-trivial are a relator the
presentation lacks, and 1.8's "relations are sentences" is what reaches
the table here. Trivial holonomy is one proof seen from several frames —
the conjugate level — and E2 pays the closure once (23.9). The adjunction
is otherwise unchanged: the same witness, the same two-court consilience,
the same receipt and syllabus.

None of these is a closer written for a kind; each is the canonical form
of an exact object, and it exists because the object does. A dispatch
table over the five demand kinds of 50.5 would be a dictionary of
closers — the cipher's fault at the level of the effector — and it is
refused for the same reason (1.7).

**The adjoined generator's carrier, parties and act** (ruled
2026-09-10, L11 (2/2)). The remainder is docketed as a provisional form
with one word vertex whose citation list is empty, whose residue is the
identity, and whose canonical rotor is the remainder itself: named by
its content on every tape (law 6) with no escape byte, because a
vertex's identity is its rotor (1.5, L14). Its **discoverers** are the
voices whose demands carried the witness, parties as a consilience
award is split; a court is never a discoverer, because a court does
not speak (law 8). The install on the second court's re-derivation is
the second-vantage install every ruled word takes (47.3), under
`ActKind::Ruling` with the generator's address — a new act byte would
be a recut — and the syllabus teaches it as any ruled form. The
adjoined count (45.9) is read from content, never from a counter: the
ruled forms whose vertex cites nothing and carries a rotor outside the
span of the seventeen. Opened at d35ac33 (2026-09-10): two voices, two
courts of distinct identity, one address, the count 1 on each, in
process, no corpus, no new act byte, no per-kind closer. Two facts the
landing found and ruled in place: a letterless word does not survive
`Word::to_bytes`, so an adjoined generator carries its rotor in the
derivation's rotor field under a presence byte, and "no citations" no
longer means "the residue is the action"; and a letter's vertex has no
star face, because the face from the Frame would be the letter's own
action, which is what the seventeen cannot spell — Frame plus vertex,
no faces, closed and coherent vacuously, as a generator should be.

**Depth 4 and depth 5 use carriers the tree already has** (ruled
2026-09-10, L17). At depth 4 the object is the holonomy of a loop two
derivations of one vertex make through their two frames, read in the
stabilizer with rotor coefficients (23.9, `cech::holonomy`); trivial,
it is one proof in two frames and the closure is paid once; non-trivial
and of finite order, it is the torsion class chapter 25 already files
(`FiledLoop`, `Closure::Class`, `Fill::Class(ℓ)`), and the adjunction
is that class ruled on two courts' re-derivation of the same A1
canonical rotor and order, under `ActKind::Ruling`, taught by the
syllabus, and cited by any later form whose loop carries it; of
infinite order it is not a relator but a generator, and it takes the
ring level's path. At depth 5 the object is a rewrite between two
derivations of one vertex, and the kinds are the ones `rewrite.rs`
carries: a generator **retires** (3.8, 27) when it derives through the
others — decided by factorisation, A12 or the ring division, never by
a length or any scalar (rule 4) — and the 2-cell is the substitution of
its derivation into every citer, minted by `mint_rewrite` as
`Retire` with the citers re-docketed (`redocket_citers_of`), ruled on
two courts' re-derivation of the derivability, paid as E2 pays a
rewrite. No carrier is invented for either depth.

**v7's layout is fixed at the recut.** Until the fleet is recut on v7,
a change to v7's layout (the presence byte above) is v7, and no
version is spent on it; after the recut a layout change is v8 and a
recut.

**What follows for the walls.** The prime wall (36.3) is the ring level
of the adjunction and needs no tower: the letter is rational, the field
is ℚ, and 37.1–37.4 stay closed. The deformation wall (50.8) is the cover
level of the same adjunction. P1–P7 below bind the ring level in full and
are read as the instance, not as the procedure; 50.8 binds the cover
level the same way. What varies between them is the canonical form in the
table above and nothing else: the witness, the two-court consilience, the
receipt, the syllabus, the content name and the epoch are one procedure.

**P1 The letter.** For a prime `p ≡ 1 (mod 4)` and a pair plane `(i, j)`
the letter is

```
Λ_{p,ij} = a + b·e_ij,   a² + b² = p,   a > b > 0
```

with the carried inverse `a − b·e_ij` (weight `p`), rational, conductor
`1`. Fermat's decomposition is unique under `a > b > 0`, so the letter is
a function of `p` and the plane and of nothing else: two nodes cannot open
two letters for one prime. Its order is infinite (15.3, Niven) and its
relators are the (3,4,5) letter's (1.2): infinite order, complementary
planes commute, a sheet letter conjugates it to its inverse.
`Rotor::order`, `class_key`, A1 and A21 need no change; the tape gains the
content name of 37.3; `factor_rotor`'s lattice per plane becomes
`ℤ^{1+k} ⊕ ℤ/4` with one `ℤ` per prime letter the syllabus holds; `spell`'s
peel gains one shell per opened prime after the `5` and `13` shells, the
primes in increasing order — an order by content, never by the order they
were opened.

**P2 The witness.** `Demand::RotorPrime(plane, prime)` carries beside the
prime the **witness rotor** `R = s + t·e_ij`: the plane block left after
the base shells, in A1 canonical form (50.4). The block the shells are
peeled from is the **residue** of 50.2 step 2 — what remained after `F`
factored the measured versor through the generators the mind holds — so
the witness is a coordinate of the demand's carrier and not a quantity
the court recomputes from a spelling. The Gaussian factor is read off the
witness, never searched for:

```
π_p = gcd_{ℤ[i]}(s + t·i, p)      Euclid over ℤ[i]; exact; bounded by the norm (rule 10)
```

normalised by a unit to `a > b > 0`. A witness whose gcd with `p` is a
unit names a prime it does not carry and is refused
`REFUSE_LETTER_WITNESS`. The two-squares problem is never solved from `p`
alone: the transducer measured the rotor, so the transducer already holds
the letter, and 36.3 is exactly the statement that the effector was
missing, not the object.

**P3 Consilience at arity two.** The court opens `Λ_{p,ij}` when a
`RotorPrime` demand vertex with its witness rotor stands on a chart and
two independent courts — the home and a witness hypha outside its
delegation closure (47.3) — each re-derive `π_p` from that witness by P2 in
their own replay and reach the same factor: equality of content (law 2),
never "the same minimal polynomial", which is the class invariant of 15.3
alone and forgets the plane. A voice's demand is listed, priced and
relayed as 50.5 says and opens nothing; a second voice speaking the same
demand is a use, never a judicial gate — a letter is no exception to 47.3
(ruled 2026-09-10, correcting this section's first text, which read the
second organ as a second voice). The opening is a ruling: a receipt of its
own act kind (`ACT_LETTER`) carrying the letter's content, the demand
address and the two courts; a court that re-reads a different factor from
the witness refuses the ruling (13.9 step 8). `court letter opened {plane}
{p} by {vk} {vk}` is the line, the two keys the two courts.

**P4 Teaching and the epoch.** The opened letter enters `SYL1` and the
syllabus page (A19) teaches it to every voice as it teaches a word; a
voice logs `letter learned {plane} {p}`. The alphabet epoch is the
syllabus page's height (45): a tape naming a letter above a receiver's
page is `REFUSE_LETTER_UNTAUGHT`, and the receiver asks for the page. No
voice speaks an opened letter before the page that teaches it; a word
spelled in an untaught letter is not a word on that node; and that is the
whole of intelligibility — the floor every voice speaks is the syllabus,
not a constant. An index assigned in open order would need every node to
agree on an order of openings, which is the totalisation law 9 refuses;
the content name needs no agreement.

**P5 The demand closes without re-perception.** A sentence stood withheld
with a `RotorPrime` vertex (50.5) re-spells that vertex on the first stroke
after the page arrives: `factor_rotor` over the syllabus's letters now
peels the witness to a word, the demand vertex becomes a `Word` vertex with
the same carriage and gauge, and the sentence is uttered whole (24.3). The
percept is not re-read; the witness rotor was the measurement.

**P6 What does not change.** The seventeen and their relators; exact ℚ;
the T lane as extent; the cost of a word on the S lane, letters counted
as now with no conductor factor; the court's two-organ replay. A voice
holds no letter it was not taught, and no letter is ever opened for a
prime no witness carried.

**P7 Binding.** `alphabet/letters.rs`: `Letter::{Base(u8), Prime { plane,
p }, Turn { plane, n }}` in place of `LetterId`, `Alphabet { base, ext }`,
`letter`/`letter_inverse` over both, the relators appended per opened
letter; `alphabet/spelling.rs`: the peel over the syllabus's primes;
`alphabet/factor.rs`: the per-plane lattice over the held primes;
`sentence/demand.rs`: the witness rotor on `RotorPrime`;
`core/gaussian.rs` NEW: `gcd` over ℤ[i] with its unit normalisation;
`court/letters.rs` NEW: the opening, P2–P3; `SYL1` in the genome;
`ACT_LETTER`; `REFUSE_LETTER_WITNESS`, `REFUSE_LETTER_UNTAUGHT` in 34.1;
the two log lines in 34.2/34.3. The tape's escape byte is a recut (§10).
Tests: `a_prime_letter_is_read_off_the_witness_never_searched`,
`one_voice_opens_no_letter`, `two_witnesses_of_one_prime_open_one_letter`,
`a_witness_that_does_not_carry_the_prime_is_refused`,
`a_tape_naming_an_untaught_letter_is_refused`,
`an_opened_letter_spells_the_demand_that_opened_it`,
`opened_primes_peel_in_increasing_order`. Traces: `court letter opened`
once per `(plane, p)` on the mesh, `letter learned` on every voice after
the page, and the sentence that carried the demand uttered without it on
the next stroke.

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

`element/{carrier,residual}.rs`: `Cell` widened, `positions`, `measure_lanes`, `from_residual`
(A5 on six-tuples), chain bytes with dimension byte; `blade/`,
`blade/harmonics.rs`: `d`-dimensional moments and Betti numbers, axes
prefixed by `d`; `peel.rs`:
`d`-cube shells; `family/perceive.rs`: `perceive` over frames writes `t`;
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
4b. `family::tests::face_work_is_linear_in_occupied_cells` (occupancy + ring, not n²).
4c. `sentence::tests::ratio_and_sections_are_on_the_form_key` (ratio, sections, torsion on the tape; zero denominator refused).
5. `family::tests::extract_rule_by_class_with_sections` (a class in some pairs is a section).
6. `family::tests::extract_rule_glues_a_held_form` (depth 1, one `Glued`, fewer `Word` vertices than the flat rule).
6b. `family::tests::empty_fiber_is_unspoken_not_every_body`.
7. `family::tests::unfolding_grow_to_frame` (one form, `Unfolded{Occupied}`, no step count on the tape).
8. `family::tests::orbit_contact_and_no_orbit`.
9. `verdict::tests::replay_form_refuses_unruled_glue`.
10. `cognition::tests::witness_fetches_glued_form_before_install` and `replay_memo_reused`.
11. `cognition::tests::slash_of_glued_redockets_citers`.
12. `sdk::tests::comprehend_understands_through_held_class` (the cited address absent, the cited chart held under another address: `Understood` with one entry per held form and no re-gauge), `sdk::tests::foreign_class_is_withheld` (one outcome, `Withheld`), and `withheld_then_depth_order`.
13. `economy::tests::settle_recursive_pays_by_deflation`.
14. `cognition::tests::shorter_form_retires_to_cavity_and_resolves`.

**Torsion bridge, pivots, and rank (M9, A16–A20, chapter 42).**
`sentence::tests::class_vector_is_canonical_and_occupied_axes_fixed_order` (12.6);
`sentence::tests::pivot_candidates_are_read_from_occupied_axes_only` (A16 step 3: a zero coordinate is never visited);
`sentence::tests::pivot_discharges_only_its_axis_and_raises_none` (12.6 discharge condition);
`sentence::tests::pivot_loop_terminates_in_occupied_coordinates` (T4);
`sentence::tests::pivot_never_reopens_a_closed_chord` (T2);
`sentence::tests::several_discharges_are_held_as_vortex` (A16 step 5: two discharging candidates, both kept, none applied; S9.3 renames it from `incomparable_discharges_are_held_as_vortex`);
`sentence::tests::vortex_collapses_by_perception_glue_or_ruling` (25.1);
`sentence::tests::mu_neither_certifies_nor_selects` (T13: the applied pivot is a function of the class vector and the held words, unchanged when μ⃗ of the remainder is permuted across lanes; S9.3 renames it from `descent_certifies_never_selects`);
`sentence::tests::certificate_key_equal_across_two_minds` (A20 step 1);
`verdict::tests::pivot_certificate_replays_or_refuses` (23.7, `REFUSE_PIVOT_INEXACT`);
`sentence::tests::torsion_filed_only_when_witnessed_at_period` (T5);
`sentence::tests::infinite_class_is_never_filed` (T3);
`sentence::tests::rank_invariant_under_regauge_and_pivot` (R1, R2);
`sentence::tests::rank_invariant_under_gluing_closed_form` (R3);
`cognition::tests::rank_conserved_under_refinement_carried_plus_discharged` (R4);
`cech::tests::rank_of_union_is_sum_over_components` (R5);
`physics::tests::stroke_terminates_on_zero_outcome` (T9, T25; `descends_has_no_infinite_chain_on_bounded_lanes` leaves with S9.3);
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
`alphabet::tests::conductor_is_lcm_and_nothing_prefers_by_it`;
`family::tests::octagonal_pair_opens_letter_and_closes`;
`element::tests::plane_cell_bytes_unchanged_under_b1` (every existing address equal);
`ear::tests::cube_and_plane_square_have_distinct_charts`;
`ear::tests::betti_numbers_exact_on_a_hollow_cube`;
`family::tests::two_frame_shift_is_a_word_on_plane_0_3`;
`sentence::tests::rule_constant_across_frames_has_identity_holonomy_in_t`;
`cga::tests::cl41_sphere_blade_projective_canonical`.

**Mesh under adversaries (Part IX).**
`chronometry::tests::height_is_one_plus_max_of_prev_and_parents` (45.2);
`chronometry::tests::precedes_implies_lower_height_and_concurrency_is_kept`
(T15: two chains with no citations are concurrent at every height; a
receipt citing both orders both before it and neither before the other);
`chronometry::tests::frontier_visits_each_receipt_once` (N vs 4N counter);
`cognition::tests::extent_is_zero_without_admitted_receipts`,
`extent_replays_from_the_home_chain` (T16);
`cognition::tests::cocycle_supersedes_by_causal_order_and_keeps_concurrent`
(45.3);
`dialogue::tests::prefix_refuses_foreign_genesis_or_epoch_before_carriers`
(47.2 step 1, a carrier-touch counter at zero);
`cognition::tests::receipt_gap_backfills_before_acting` (47.2 step 3);
`cognition::tests::claimed_height_that_does_not_rederive_is_refused` (47.2
step 4, T18);
`cognition::tests::install_requires_independent_witness_with_room` (47.3:
a witness inside the home's closure refused, a witness short on an occupied
lane refused, a two-signature install written);
`cognition::tests::equivocation_proof_burns_bond_and_supply` (47.4, T17,
R9: bond zero on four lanes, one `burn` entry, `Treasury::audit` holds);
`cognition::tests::duplicate_mint_after_partition_is_burned_once` (45.3,
47.4);
`cognition::tests::withheld_citation_lapses_and_releases_escrow` (47.5);
`identity::tests::frost_group_signature_verifies_as_ed25519`,
`frost_reshare_keeps_the_group_key` (46.1);
`identity::tests::manifest_key_is_scoped_and_revoked_whole` (46.2);
`server::tests::rtt_is_counted_in_strokes` (45.4);
the clock gate of `MU-ENGINEERING.md` §7 as a build failure on
`SystemTime`, `Instant`, `Duration` in the directories of 45.6.

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
- A mind holding a word under another spelling receives it: `Understood`,
  never `Admitted`; a citer whose cited chart is held under another address
  logs `form understood through …`.
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
- A mind holding the cited chart under another address logs `form
  understood through …` and installs; a missing glue logs `withheld` then
  learns in depth order.
- An idea logs `glued <addr>` and its remainder shrinks by the glued form's
  cells.
- One settlement with two speakers; a longer form relocated with its citers
  still replaying through the cavity.

**Working atlas (chapter 24.1).**
- After a clean boot a voice logs `atlas ruled 0 stood 0 held c` with
  `c > 0` once it holds an idea, and `quote closing k charts (ruled r stood
  s held h)` counts the clusters whose open charts the working atlas,
  ideas included, is incident with; `h > 0` before the first `SYLLABUS`
  line once an idea's chart meets a listing.
- On the second stroke `sentence formed` exceeds `sentence withheld`.
- The first `SYLLABUS +n` is accompanied by `n` `promoted` lines; the stroke
  line shows `dialect` down by `n` and `genome` up by `n`, never both up.
- After a restart the stroke line shows `genome` unchanged and `dialect 0`,
  and the voice quotes on its genome charts on the first stroke.

**The utterance (chapters 23.8, 24.3).**
- The first `family … rule …` line on a voice with an empty genome is
  followed, in the same stroke, by `form withheld … remainder n cells over
  p pairs`, one `dialect stood`, one `sentence spoken … demands d` with
  `d ≥ 1` and `d` `demand … chart …` lines, and one `uttered` line for the
  sentence (chapter 50); no `remainder expressed` line; the next stroke
  line shows `dialect ≥ 1` and `ideas ≥ 1` on that voice.
- The hypha answers with `court attest` naming a sentence replayed and
  `d` `court demand listed` lines, and the book grows by the number of
  distinct (kind, chart) demands, never by pairs re-uttered and never by
  a chain; a repeated frame logs `court attest duplicate` and the book
  does not grow.
- A second voice served that listing logs `listing … reduced` only when a
  word it stood deflates it, else `listing … held`; the book's `closed`
  grows only after a `court attest … residual 0` or a probe closure.
- On the first `SYLLABUS`, every taught word logs a replay on the receiver
  before `form learned`, `promoted` lines on the voice that stood it, and
  no `syllabus refused` on a clean tape.
- With the feed exhausted and the book unchanged, a voice writes one stroke
  line per aperture cycle, its `stroke idle` lines carry four lane dwells,
  its `quote asked` lines stop, and they resume with `quote asked: … book
  h'` for `h' > h` once another voice's utterance advanced the book.
- A voice under pressure on one lane only keeps expanding on the other
  three: its `stroke outcome` lines show non-zero μ⃗ on those lanes while
  the pressured lane dwells.
- No hypha log contains a reduction of a demand: grep `reduce_all` sites
  against the wire handlers returns none.

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
- One idea with one held loop whose class vector occupies a unit turn on
  the sensor plane and a placement residue, with the mind holding a
  quarter-turn word and the translation: two `pivot … discharged` lines
  naming axes `Unit(0)` and `Placement`, in that order → one `vortex … 2
  candidates held` line and no applied pivot on that loop
  in that stroke; on the next perception that adds the translation face,
  `vortex … collapsed by perception to axis Placement`, then `form spoken …
  torsion 0` with two certificates on the tape, and the hypha's `court pivot
  certified` twice for that address.
- One idea with a held loop whose only occupied axis is an infinite prime
  exponent the mind has no word for: no `pivot … discharged` line, one
  `demand spoken … order ∞`; the loop never files as torsion.
- One idea with a held loop of finite order no held word discharges:
  `pivot … no discharge` for each occupied axis, then `torsion filed …
  (σ,ℓ)`, then `form spoken … torsion 1`.
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

**Mesh under adversaries (Part IX).**
- Cold boot of three hyphae and five minds: one `court genesis {hash} epoch
  {e}` line per process, one hash and one epoch across all eight, zero
  `court origin refused`.
- Roots and digests exchanged: `court receipt parent {vk} K {k} h {h}` on
  each hypha for each peer; heights strictly increase along every cited
  chain; a hypha stopped for the soak's length resumes at its old height and
  climbs on the first digest it admits.
- A mind idle for the soak accrues no T-lane extent and sheds nothing; a
  mind speaking every stroke accrues extent equal to the height deltas of
  its admitted receipts at its home, and a witness recomputes the same
  number from the home's chain.
- Every `court form installed` has a preceding `court witness … co-signed`
  from a distinct closure; a fleet with one bonded hypha installs nothing
  and logs `witness room` refusals.
- A test binary speaking two roots at one K: exactly one `court equivocation
  … burned` per hypha, bond zero on all lanes, supply down by the same
  vector on every hypha's audit.
- Two hyphae partitioned and rejoined after both installed one address: one
  `court concurrent … held`, one merge with conservation, one `burn` for the
  duplicate mint, ruled count unchanged.
- A recut: epoch increments on hypha a's genesis; a stale process speaking
  the old epoch is refused with `court origin refused` and admits nothing.

## 41. Landing order

One item per commit, the trace value in the message, reproduced on cold
boot by the auditor before the next lands; no soak clock until every trace
of the items landed so far passes.

**Stage 0 — identity (Part III), before anything else.**
0.1 M1: A1, A3, A4, A2 step 1. 0.2 M2: A5 and the address composition.
0.3 M3: A8, since a29bba5 changed the address rule without it; no cutover
onto existing state before this lands (chapter 33). 0.4 M4: A6, A7.
0.5 M5. 0.6 M7: A9, A10, A11. 0.7 M6: A12. 0.11 A21 structured keys: every
site of the 1.6 digest table converted in one landing (`Keyed` over
variable-length A21 bytes, M1 and M2 as coordinates, class vectors and
certificates as their own keys, chart axes unhashed, `CLOSED_KEY` deleted,
the A19 record without a representative, tape version 4), with the
`blake3` gate of `MU-ENGINEERING.md` §7 turned on in the same commit so no
seam keeps digest equality alive. 0.12 Carriage versors: every
`placement`, `shift`, and residue pair replaced by an A4 versor on
`Vertex`, `Transport`, `Loop`, `PivotBy`, `Form.carriages`, and the class
vector's placement and dilation axes, with coordinate pairs confined to
`pair_form`, `speak_rule`, and `orbit`. 0.13, folded into 0.12 because it
rewrites the same function: the class vector reads the rotor's conjugacy
invariants first and the plane blocks as the sensor section (12.6, 15.3,
16.3); `Axis::Sheet` becomes `Axis::Orientation` read from the rotor's
action, never a stored flag; `Word::class_key` and `same_class` consult the
rotor before the plane form; the 1.2 relator is the half-turn;
`Transport::refract` (3.10) lands with its replay. 0.11 to 0.13 land
before any new `Keyed` site is written, before 2.3, and before Cut B+1. Traces: the
identity and migration group of chapter 40.

**Stage 1 — forms (Part IV).**
1.1 Chapters 18–19 form types and tape, tests 1–3. 1.2 Chapter 20
perception, test 4. 1.3 Chapters 21–22 extraction and orbit, tests 5–8.
1.4 Chapter 23.1–23.2 court replay and carriers, tests 9–11; chapter 28
persistence. 1.5 Chapter 24.1 the working atlas (`working_atlas`,
`Standing`, dialect `Stalks`, promotion in `learn_one`, standing on the
wire), then chapters 24.2 and 26 comprehension and wire, test 12; the
working-atlas traces of chapter 40 gate this item.
1.6 Chapter 25 germination and bundles. 1.7 Chapters 26–27 economics and
retirement, tests 13–14. 1.8 M9 the torsion bridge: `Form::class_vector`
and `ClassVector::occupied` (12.6), A16 `pivot_loop` with vortex cells in
the thought, A20 `PivotCertificate` with its tape section and court replay
(23.7, `REFUSE_PIVOT_INEXACT`), A17 filing and witness, A18 `rank` with
debug assertions and `REFUSE_RANK_LOST`, A19 class records on the syllabus
page and `seed_horizon`; the torsion-bridge and pivot tests and traces of
chapters 39–40 gate it. Any existing greedy that orders chords by torsion
order is removed in this item; it is retired doctrine (chapter 43).
1.16 Chapter 24.3 the utterance and chapter 23.8 hardening: the three
local producers stand words before anything reaches the wire; incidence
by deconstruction; pacing by the outcome vector and the book height;
replay through the court's genome; room, receipt idempotence, and replay
on the syllabus. The utterance traces of chapter 40 gate it, and it lands
with 5.1 before the first recut after Cut B. Traces: the forms group.

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

**Stage 5 — the mesh under adversaries (Part IX).** 5.1 Receipts with
parents and heights, TUniversal as height, every duration in strokes, the
clock gate; `origin_unix` and the `tuniversal` file removed; the first recut
after this item writes epoch 2 (45). 5.2 The signed prefix and admission
(47.1–47.2): `REFUSE_ORIGIN`, `REFUSE_RECEIPT`, `REFUSE_RECEIPT_STALE`, gap
back-fill. 5.3 Two-organ install receipts (47.3). 5.4 `ACT_EQUIVOCATION` and
the burn (47.4), the duplicate-mint burn on rejoin. 5.5 Lapse of withheld
citations (47.5). 5.6 FROST group keys and manifests (46). 5.7 xSEPTUM acts
0x30–0x35 against `XSEPTUM.md` (49). Traces: the Part IX group of chapter
40. 5.1 lands before the first recut after Cut B; 5.2–5.5 before any
settlement is live; 5.6–5.7 with xSEPTUM. Stage 5 does not wait on Stage 4.

**Stage 12 — cognition before machinery (ruled 2026-09-09).** The first
soak of Stage 11 (seven hours on a91fcb8, `MU-RECURSION-TASKS.md` I34,
I34b) read a machine that pays everywhere and learns nowhere: one 13-axis
stratum per corpus task and no shared chart, so no congruence and no glue
(24.4 is the eye that recognises; the atlas axes must run coarsest first
so a prefix is a stratum); two-vertex star forms with no loop, so the
pivot loop never fires and no axis opens (21 step 2 keeps non-constant
faces as sections, 25); residual chains uttered as words with no citation
deflating any demand (A12 over the working atlas before anything is
listed, 23.9 at the court); and every act polled on every stroke because
the quote reply's height never moved (23.8, 24.3). The landing order for
this is Stage 12 of the task list, and it precedes every remaining item of
Stages 2–5: nothing below the eye can learn while the eye discriminates.

**Stage 14 — language formation (ruled 2026-09-09).** The soak of
2026-09-09 on d6e19a1 read a market flooded with unspoken chains: 226
remainders expressed against no sentence spoken, genomes of 250–420 ruled
words comprehending 0 of 2 listings, no axis, no class. Chapter 50 rules
the sentence as the unit of speech and names the three seams through
which chains reached the book. Its landing order is Stage 14 of the task
list — the typed demand carrier and the demand vertex (tape v6, a recut),
the ordered factorization A22, the edge transport in the stabilizer, the
speaker F, the receiver G at the court, then the three seams severed in
the order 50.6 lists them, the last of them the court's irreducible
listing — and it precedes S13.11 onward and every remaining item of
Stages 2–5, since a court that lists chains has nothing to rule. Ruled
2026-09-10, after the recut of L9: the walls are answered by the running
engine. Amended the same day — the two rows once called L10 (the prime
letters) and L11 (the deformation cover) are **one** row, the adjunction
of 37.7: the court adjoins the canonical remainder a factorisation left,
at whatever level it stopped at, on two courts' re-derivation of the same
witness. Two rows for two walls was a dispatch over a taxonomy, and the
first proof of the adjunction is therefore at whichever level is cheapest
to construct, in process, with no corpus in it — a generator adjoined at
run time with no commit is the claim, and no order between levels is
ruled. Before it, L8x: F factors through the presentation (50.2 step 2),
without which the utterance path is a cipher and there is no presentation
for a remainder to be the remainder *of*. The order stands: the seams are
severed before any generator is grown on the substrate they leaked
through.

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
stroke `Budget`, the lane readings of μ⃗, T-extent, K-disuse). A fibre that
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
`pivot_never_reopens_a_closed_chord`. Consequence: a conjugating pivot
(re-gauge, rebase) can close a chord only if its class is trivial and can
never open a closed one; only a pivot that adds a face, a word, or an axis
can change a class vector, and it changes exactly the coordinate it acts
on.

**T3 Torsion is exactly the finite-order part and is read, not searched.**
`(1, r)² = 1`; `s = 0` has infinite order iff a prime exponent is nonzero,
else order dividing 4 (9.1); off the sensor plane the order is the period
of the rotor's class invariants (15.3, ruled 2026-09-09). Code:
`Rotor::order`, `Word::order`, `PlaneForm::torsion_part`, `free_part`,
`cech::extend` (106) reading the rotor's order and mapping finite to
`Fill::Congruent(g, ℓ)` and infinite to `Fill::Class`. Proof: `a3_form_is_reduced_and_eq_on_reduced`,
`goodness_cyclic_reports_chord`, `infinite_class_is_never_filed`.

**T4 The pivot loop terminates.** Every applied pivot zeroes one occupied
coordinate of one held loop and by T2 raises none, so the total number of
occupied coordinates over all held loops strictly decreases with each
application; the candidate list of a loop is finite (its occupied axes times
the held words on them); therefore A16 makes at most (occupied coordinates)
applications and finitely many trials per application, all under `Budget`.
Code: `Form::pivot_loop`, `Form::class_vector`, `Contemplation::pivot_on`
(712), `expand_with` (84). Proof:
`pivot_loop_terminates_in_occupied_coordinates`; trace: the count of
`pivot … discharged` lines for an idea never exceeds the occupied
coordinates of its first class vectors in that stroke.

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

**T9 Pressure terminates by budget and closure, never by an order.** A
fibre leaves the hot set only by derivation (retirement, address kept,
27) or by its own T-room being spent, each a finite event on a finite
carrier; every stroke spends a `Budget`; a minted deflation closes a
remainder, and a closed remainder is not minted again (idempotence by
receipt, 23.8), so no chain of mints is infinite. No well-founded order on
μ⃗ is invoked (12.6, ruled 2026-09-07). Code: `Physics::stroke`, `Budget`,
`retire_on_t` (223), `Epigenome::shed`, `relocate_t_exhausted`,
`reduce_excluding`. Proof: `settle_recursive_pays_by_deflation`,
`stroke_terminates_on_zero_outcome`; invariant: no `mint` without a
closure in the same stroke.

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

**T13 μ⃗ neither certifies nor selects.** The candidate list of a held
loop is a function of its class vector and the held words on its charts
(12.6, A16 step 3); the applied pivot is a function of which candidates
discharge with an admissible rank move, and a loop with several certified
discharges is a vortex. `μ⃗(ℓ)` enters a trial only as the per-lane reading
that refuses a candidate raising a lane, never as a comparison between
candidates. Therefore permuting the lanes of the remainder's `μ⃗`, or
scaling all of them, changes neither the candidate list nor which
candidate is applied, and two minds with the same vector and the same held
words apply the same pivot or hold the same vortex. Code:
`Form::pivot_loop`, `economy::lane_moves`. Proof:
`mu_neither_certifies_nor_selects`, `certificate_key_equal_across_two_minds`;
trace: identical `pivot … discharged` sequences for one loop on two voices
holding the same words.

**T14 A pivot changes exactly its axis.** For each move of the 3.9 table
and each coordinate of the class vector, the move either leaves the
coordinate unchanged or zeroes it, and it zeroes only the coordinate it is
labelled with: a gauge or hue word on plane `p` acts on `(s_p, u_p, m5_p,
m13_p)` or on the hue cycle type by the group law of 9.1 and leaves every
other plane's coordinates and the placement fixed; a translation word acts
on the placement residue alone; a glue of a closed form makes the loop a
boundary and zeroes the whole vector (T6); opening an axis removes the loop
(15.5). Code: `Form::class_vector` before and after each move. Proof:
`pivot_discharges_only_its_axis_and_raises_none`; the debug assertion of
A18 on every trial.

**T15 Causal order is exact and never totalised.** `e ≺ e'` iff the frontier
of `e` is below that of `e'` componentwise and differs; heights strictly
increase along `≺`; two receipts with incomparable frontiers are concurrent
and no operation orders them (45.3). Code: `ReceiptChain::frontier`,
`precedes`, `Height`. Proof:
`precedes_implies_lower_height_and_concurrency_is_kept`; trace: no `court
concurrent … held` line is followed by a resolution by order.

**T16 Extent re-derives.** A home's T-lane extent for any carrier or voice
is a difference of heights on the home's own chain, and any node holding
that chain computes the same number (45.4). Code: `standing_mag`, `Height`.
Proof: `extent_replays_from_the_home_chain`,
`extent_is_zero_without_admitted_receipts`.

**T17 Equivocation is decided by two signatures.** Two valid signatures
under one key over prefixes with one `K` and two hashes exist iff the signer
produced both; verification decides, with no replay and no judgement
(47.4). Code: the `ACT_EQUIVOCATION` handler, `court::identity::verify`.
Proof: `equivocation_proof_burns_bond_and_supply`.

**T18 No clock, no clock attack.** No kernel reads a physical time, so no
claim about time enters the system except as a receipt whose height
re-derives from signed parents; a false height is refused by re-derivation
exactly as a false tape is (45.2, 47.2 step 4). Code: the clock gate,
`REFUSE_RECEIPT`. Proof: `claimed_height_that_does_not_rederive_is_refused`
and the gate.

### 42.2 Rank theorems (dimensional capacity is never lost)

With `rank(F) = (torsion_rank, free_rank, offplane_rank, axes)` from A18, a
vector never summed:

**R1 Re-gauging preserves rank.** By T2. Code: `Form::regauge`, `Form::rank`.
Proof: `rank_invariant_under_regauge_and_pivot`.

**R2 The pivot loop preserves rank except by discharge into an axis.** A
conjugating pivot closes only a chord whose class is trivial, which was
never rank; a word or glue pivot that discharges a coordinate removes a
class that the form's own faces now express, so `torsion_rank` or
`free_rank` falls by exactly what the new face carries and the form's total
information is unchanged (the coordinate became a face); an axis-opening
pivot moves one unit from `H¹` into `axes` (R4); filing torsion moves a
chord from open to filed without changing `torsion_rank`. Any other change
is refused in the trial (A16 step 4). Code: `Form::pivot_loop`, `Form::rank`.
Proof: the same test; trace: rank equal, or moved by exactly one unit into
`axes`, before and after every `pivot … discharged` line in debug builds.

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

**R6 The polytonic lanes are never projected.** No operation orders,
sums, or thresholds μ⃗; a comparison of two lane vectors returns the
reading per lane (`lane_moves`: each lane fell, held or rose) and never a
verdict, so no lane's extent is ever lost to a scalar. Code:
`economy::lane_moves` as the only comparison; `lane_room` per lane. Proof:
the order gate of `MU-STANDARDS.md` §4.6, which fails the build on a
`bool` over two lane vectors, and S9.3's test of the reading.

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

**R9 Burn conserves supply.** Forfeiture on equivocation and the burn of a
duplicate issuance reduce `bond` or `balance` and `supply` by one vector, so
`supply = genesis + mint − burn = Σ balance + Σ bond + Σ escrow` holds on
every lane after every burn (47.4, `MU-ECONOMY.md` §6), where `Σ escrow` is
the **funded** escrow. An escrow that a payer backed from a balance holds
μ⃗ that left that balance and is still in the mesh, so it is counted and
discharging it moves value; an escrow that is a claim on issuance holds μ⃗
that does not exist yet (`MU-ECONOMY.md` §3.1), so it is not counted and a
ruling mints it. Counting all escrow asserts value that was never created;
counting none makes the audit hold by not looking. Code:
`Treasury::burn`, `audit`, `escrow_from_balance`, `discharge_funded`. Proof: `equivocation_proof_burns_bond_and_supply`,
`duplicate_mint_after_partition_is_burned_once`.

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
6. `F` closes a remainder beyond what its cited forms close (E2,
   `REFUSE_REDUNDANT`);
7. no strictly shorter form on the same vertices exists (T10);
8. every pivot `F`'s crystallization rests on has a certificate that replays
   on the court (A20, T13, T14), and none of them was selected by a measure.

Under pressure the persisting set is finite at every radius (polynomial
growth, 1.3) and stable (T9); under refinement it is carried or discharged,
never lost (T8, R4); under change of cover its classes keep their keys
(T7, 15.6); across restarts of a mind its torsion is refilled from the
mesh (M9, A19). Everything that fails one of 1–7 is either a demand (a free
class, spoken as `Novel(g_rem)`, sitting in the docket) or torsion filed on
the mesh, or a dialect shed by K-disuse; none is forgotten by the mesh.

*Proof by running.* Conditions 1–7 are exactly the seven checks of
`replay_form` and `glue_fetched` (chapter 23) plus `shorter_form` (chapter
27) and the deflation read per lane; the "only if" is that each check refuses with its named
constant (chapter 34.1) and a refused form is neither installed nor taught;
the "if" is `home_install` followed by the syllabus (chapter 26) and
promotion (T12). The cold-boot traces of chapter 40 are the instances the
auditor reproduces.

### 42.4 Termination theorems, collected

- A16 terminates in at most (occupied coordinates) applications and
  finitely many trials each (T4); a vortex is not a loop of A16, it is a
  held state that only an external event collapses (25.1).
- A17's witness walk terminates in at most `ℓ` steps.
- A2's shortening terminates by `Budget`; run reduction by strict shortening.
- Unfolding terminates by a strictly monotone integer measure (3.4); orbit
  by an integer ring count bounded by the frame (3.5).
- Retirement terminates because tape length is a natural number (T10).
- Pressure terminates by budget and closure (T9).
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

### 42.5 Transport, class, discharge, vortex, refraction

Added 2026-09-06 for the refraction row (next set after Cut B+1). Each is
a theorem the engine proves by running; the test names are the proofs.

**T19 Transports form a groupoid, not a product group.** A transport
`Γ_ij = Transport { word, carriage, gauge }` carries the word form at
vertex `i` to the one at `j`. Composition `Γ_jk ∘ Γ_ij = Γ_ik` multiplies
the words, composes the carriages by the geometric product through the
word's conjugation (`face_of`), and composes the gauges as permutations on
the union of hues with absence as identity; it is associative, `Γ_ii` is
the identity, and `Γ_ji = Transport::inverse(Γ_ij)`, which refuses off the
plane. No component composes on its own: `(V, W, γ)` taken as a product
group is not the transport. The holonomy of a loop is the composite around
it, and its class (15.4) is invariant under change of basepoint (3.2) and
re-gauging (3.6). Code: `Transport::{compose, inverse, refract}`,
`Form::face_of`, `Form::delta1`. Proof: `transport_composition_is_associative`,
`transport_inverse_is_the_reverse_face`, `holonomy_class_is_basepoint_invariant`.

**T20 The class vector is a complete conjugacy record.** `κ(ℓ)` is the
list: the holonomy rotor's conjugacy invariants (15.3) first; the six
plane blocks `(o_p, u_p, m5_p, m13_p)` when the plane form exists (A3),
as the sensor section; the hue cycle type, a partition; the translator
and dilator residues of the loop's carriage; nothing else. Two loops have
equal `κ` iff their holonomies are conjugate under the moves of 15.1, `κ`
commutes with restriction and refinement, and `κ = 0` iff the loop is
closed: holonomy projectively identity, carriage identity, gauge identity.
No direct sum of plane groups and no `Z^{|𝒢|}` appears; the coordinates are
invariants, not group elements. Code: `ClassVector::read`, `key`,
`is_zero`, `Word::class_key`. Proof: `class_vector_is_conjugation_invariant`,
`rotor_class_is_read_before_the_plane_form`, `zero_class_vector_iff_closed`.

**T21 Discharge is certified exactly.** A trial on an occupied axis `x`
is certified iff (i) `κ_x' = 0`; (ii) `κ_y' = κ_y` for every `y ≠ x`,
unchanged, since `≤` is undefined on `Z/4` or a partition; (iii) `rank'`
equals `rank` or moves exactly one unit from torsion, free or off-plane
into axes (A18); (iv) on every pair of the bundle no lane rises, read per
lane (`lane_moves` with no `Rose`), strictness not required and no descent
asked. Every clause is decided exactly and replays on the court from the
certificate (A20). T14 is the corollary that a move changes exactly its
axis. Code: `pivot_loop`, `Rank::moved`, `economy::lane_moves`,
`PivotCertificate`. Proof:
`pivot_discharges_only_its_axis_and_raises_none`,
`pivot_certificate_replays_or_refuses`, `rank_lost_refused`.

**T22 The vortex is two certified discharges.** If two trials on one loop
are both certified by T21, the loop is a vortex: both are held as cells of
the thought with their certificates, no scalar and no reading of `μ⃗`
selects between them, and the vortex collapses only when a perception adds a face one candidate predicted, a
glued or taught form's class equals one candidate's `after`, or a court
ruling installs one. A trial that does not discharge is not applied and is
not a vortex; a remainder the mind did not reduce is a thought (24.3), not
a vortex; nothing searches and nothing terminates a search. Code:
`select_discharge`, `Discharge::Vortex`, `Contemplation` cells. Proof:
`pivot_loop_discharges_held_word_and_vortices_a_pair`,
`vortex_collapses_by_glue_or_ruling`, `two_certified_discharges_are_a_vortex` (S9.3 renames it from `equal_mu_neither_dominates`).

**T23 The quotient carries the refraction.** For a trial transport `Γ` on
a pair `(I, O)` with `R = residual_of(Γ·I, O)`, the element deposited in
the contemplation, uttered, and docketed is `Refraction { pair, remainder:
R, transport: Γ }`, never `R` alone. `Transport::inverse(Γ).refract` reads
`R` in `I`'s frame with the closed part identity, and the next trial acts
on that; the court, given `(R, Γ)` and `I`, re-derives `Γ·I` and `R`
exactly and reads their closure per lane. Without `Γ` the remainder
re-enters at the identity frame as a new scene, the faces already walked
are re-derived, and no partial closure can be replayed. Code:
`Refraction`, `Transport::refract`, `hold_form`, `speak_form`,
`replay_form_with`. Proof: `refraction_is_rebase_regauge_and_carriage`,
`refraction_replays_as_its_three_moves`,
`remainder_travels_with_its_transport`.

**T24 Degree-2 obstructions are read from refractions.** For three
overlapping charts with transports `Γ_ij, Γ_jk, Γ_ki` read from faces or
from refractions, `(δΓ)_ijk = Γ_ij ∘ Γ_jk ∘ Γ_ki` (10.4). If it is
projectively identity the three sections glue (13.5, one section or
attached); otherwise its class key is a degree-2 class recorded under its
key (12.3, 15.4) and the cover is refined, never the base (14.8), and
nothing resolves it by choice. If remainders travel without `Γ`, every
`Γ_ij` on a remainder is the identity, `(δΓ)_ijk ≡ 1`, and `Ȟ²` is
unobservable: the engine cannot distinguish a missing section from an
obstruction. Code: `Complex::delta2`, `cocycle_at`, `refine_chart`. Proof:
`delta2_of_refractions_is_the_triple_product`,
`nontrivial_delta2_files_a_class2`.

**T25 Metabolic termination is polytonic.** Every trial spends a `Budget`
on the lanes it occupies, so every stroke terminates; a stroke whose
outcome is zero on every lane spends the tick; a minted deflation closes a
remainder and a closed remainder is not minted again (idempotence by
receipt, 23.8), so no chain of mints is infinite. A vortex and a held
thought are states, not non-termination. No clause compares lanes with
each other or with a constant, and no order on `μ⃗` is invoked. Code:
`Physics::stroke`, `Budget::spend`, `StrokeOutcome`, `lane_moves`. Proof:
the enumeration gate (every loop spends or walks a boundary),
`stroke_terminates_on_zero_outcome`.

### 42.6 Scale: four covers, one product (ruled 2026-09-08)

The engine scales along four axes that are four different covers, each
with its own Čech structure, its own cocycle and its own stroke budget,
and no one of them is indexed by another's size. Cost is therefore a
product of four independent extents and never a sum along one line; this
is the polytonic law applied to cost, and the four are the four lanes of
μ⃗: spatial, gauge, temporal, holonomic.

| cover | covers | by | the cost that follows it | its bound |
|---|---|---|---|---|
| the atlas | the language | strata (class vectors as prefixes) | a demand is reduced against the generators of one stratum, never the language; a stratum that cannot resolve is split by refinement | the stratum's population; the refinement, never a constant |
| the fibre | a word | its sections and torsion cycles (25) | the harmonic and holonomic readings are taken per section, never once for the bundle (24.4) | sections held; a period-ℓ cycle speaks ℓ for the price of one generator |
| the nerve | a sample, a sentence, the proofs of one conclusion | overlaps | a cocycle is one geometric product per loop (24.4, 23.9) | the nerve's dimension: eyes minus one, bodies minus one, lineages minus one; there is no other maximum on cochains |
| the mesh | the atlas | hyphae holding covers (4, 47) | a demand goes to the hyphae covering its stratum; adding hyphae adds coverage and never coordination; convergence is read from independent lineages on the ledger, never counted | the number of covering hyphae; the height is the only extent the mesh shares (45.7) |

The covers meet only on overlaps — a demand's stratum on a hypha's
cover, a section's harmonics on a loop, a proof's transition on another's
— and the overlap is where a cocycle is read, which is why a meeting
costs a geometric product and not a join. The work counters of chapter 42
and MU-ENGINEERING §7 are this section's proofs: one fetched vertex, one
held thought, one expansion, one quote and one thirty-body assembly cost
the same at N and 4N. Where the tree scales worse than this today —
the cursor that walks off its docket (S9.14), the book that holds raw
residuals (Stage 10), the fibre without a priced coordinate (S9.13) — is
recorded as work, not as a limit.

# Part VIII — Boundaries

## 43. What is deliberately absent

From the earlier lineage of this system (ipsal-autic v0.1 to v0.4),
reviewed on 2026-09-06: its refraction `Γ` and its reflection are adopted
as the moves of 3.10, and nothing else of that lineage's optics is. Absent
on purpose: refractive indices and wavelengths in nanometres, Padé
saturation, super- and sub-harmonic wavelength series, a "sheet" as a
two-dimensional object, and recognition by fingerprint-to-handle lookup,
which is the fault of 1.7 by its own name.

Absent by law 11, and removed by item 0.14: a matrix as the carrier of an
element's action. The Δ core's `Element` with `QMatrix` fibres, `realize`
as a matrix product, `compose` and `inverse` as matrix arithmetic,
`same_action` as matrix equality, and any affine block re-derived from a
tape are the same flattening at the level of the action that digests were
at the level of the key: a fixed frame of squared coordinates standing in
for the algebra. Matrices remain only as the exact integer arithmetic of
a rank (A18) and as the sensor-plane projection of a rotor (A3).

No angles: a rotation is a rotor with rational (or, in the tower,
cyclotomic) coordinates, and its angle is never formed. No norms or roots
beyond `rational_sqrt`, which refuses when inexact. No limits, series, or
derivatives: unfolding terminates by an integer measure, orbit by an integer
ring count, replay by finite products, every stroke by a `Budget`. No
probability: superposition is a set of cells, resolved by fill or class.
No scoring: nothing is compared except by `projective_eq`, `same_form`,
`is_identity`, class keys, and the per-lane reading `lane_moves`, which
returns each lane's move and never a verdict. No spectral sequence: the N-fold
cover is handled by iterated pairwise Mayer–Vietoris, order independent
(13.7), and Leray's theorem is proved by that induction along the link. No
homotopy beyond the acyclicity of an overlap, decided by a component count
and a tree walk; the one contractibility used is that a single face has
trivial `H¹` and constant `H⁰`. No abelianization off the sensor plane:
rotor coefficients use the pointed-set sequence and descent only, and
degree 2 is read on the plane or recorded as remainder. No metric on
classes: a class key is the conjugation invariants written out (A21),
compared coordinatewise and never as a digest. No
"good enough" covers: an overlap is good or the cover is changed until it
is, every retraction decided by `home`. No consensus, voting, or scoring
between nodes: exactness is the reason two nodes agree, and
`REFUSE_MV_INEXACT` is the only thing that happens when they do not. No
byte identity anywhere in cognition (chapter 1.5). No bare chain as a
word: a residual chain is the boundary of a thought (2.3) and reaches the
market only as a typed demand vertex of a sentence (50.5), never as a
listing, a citation or a genome entry. No complete rewriting
system for the mixed-plane group, no Knuth–Bendix run, no claim of
minimality for an off-plane tape beyond what a completed search depth
proves. No gradient and no greedy: a held loop is never closed by picking
the chord with the lowest torsion order, the smallest remainder, or any
other scalar first; that rule, once written into an earlier version of this
document as the "twist loop, loosest first", is retired, and the pivot loop
of 16.16 that replaces it reads the class vector, tries the moves its
occupied axes name in a fixed order that is not a preference, keeps every
certified discharge, and holds incomparable discharges as a vortex rather
than choosing (law 10). No wall clock, unix clock, or timer anywhere a kernel
reads: time is the receipt ledger (Part IX). No total order across nodes and
no tie-break between concurrent receipts, by arrival or by anything else. No
clock synchronisation protocol, no estimate of skew, no ε. No reputation
score, weight, or decay on a node: standing is bond and room per lane. No
hand-rolled threshold signature: RFC 9591 as specified, or one key. No
handshake. If an implementation step needs anything not permitted here, the
step is wrong.

## 44. Statements relayed and not adopted

A blueprint for the algorithms of Part III was relayed on 2026-09-05. Its
structure matches chapters 16–17 and its correct additions are in them.
The following statements in it are wrong for this tree and are not adopted;
an implementor who follows them forks the language or builds against an
algebra that is not here. One relayed position that this table once held,
that no hash may serve even as a key, was adopted on 2026-09-06 as law 2
and A21 and is no longer listed: it was the founding ruling of 1.5 all
along, and the rows below that mention a key mean an A21 key.

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

---

# Part IX — The mesh under adversaries

Written against `/opt/mycelium-v0.1.0` at 5865bd0 (2026-09-06). Carried from
Parts I–VIII: exactness is why two honest nodes agree. This part says what
holds when a node is not honest, and it removes the one physical quantity a
kernel still read at 5865bd0, the wall clock. Nothing here changes what a
word is. It changes what a receipt is, what a signature covers, how time is
counted, and what the court does with a node that signs two truths.
`XSEPTUM.md` is rewritten against this part; `MU-ECONOMY.md` §4 carries the
burn rule of 47.4. Functions named without **new** exist at 5865bd0.

## 45. Time without clocks

### 45.1 The receipt is the clock

Every kernel, hypha or mind, keeps one receipt chain
(`chronometry::ReceiptChain`, chronometry/chain.rs). A receipt is written for
every admitted act: a percept docketed, a word sectioned or installed, a
ruling, a settlement, a syllabus page served, a peer digest admitted, a form
stood in the dialect. `KStepReceipt` (chronometry/receipt.rs) gains parents and a
height:

```rust
pub struct Parent { pub vk: [u8; 32], pub k: u64, pub hash: [u8; 32] }       // new
pub struct KStepReceipt {
    pub step_index: u64,             // K: this kernel's own index, 0 at genesis
    pub height: u64,                 // TUniversal, 45.2                        new
    pub parents: Vec<Parent>,        // peer receipts this act verified          new
    pub kind: ActKind,               // the act admitted (ruled 2026-09-09; tape v6, a recut)
    pub delta: Address,              // the act's M2 composite or listing key (A21), never a digest of anything
    pub operator_hash: [u8; 32],
    pub prev_receipt_hash: [u8; 32],
    pub chain_hash: [u8; 32],        // blake3(step ‖ height ‖ parents ‖ kind ‖ delta ‖ op ‖ prev)
}
```

**A receipt names a derivation, never a tape** (ruled 2026-09-10, 1.9).
`delta` is the address written out — the canonical coordinates of the
vertex, one address for every derivation of it — and `parents` are the
peer receipts the act verified, so the chain of every kernel is a node
set of one causal poset of proofs. The depth of a derivation (1.9) is
read from this poset: the distinct signers whose receipts of kind
`Install`, `Ruling` or `Adopted` name its address. It is computed at the
read and stored nowhere. The receipt hash is a digest of bytes because
the receipt is bytes (rule 2, Part IX); nothing in cognition reads it as
a coordinate.

`ActKind` (ruled 2026-09-09) names the act a receipt admits — `Attest`,
`List`, `Section`, `Install`, `Ruling`, `Syllabus`, `Lapse`, `Fold`,
`Adopted`, `Stood`, `Uttered`, `Learned`, and `Settled`, the settlement
the first paragraph names (a probe that closed a listing or descended
its remainder; added with S13.6, 2026-09-09) — written on the tape as one
coordinate byte, in the chain hash after the parents, so a phase (45.9)
and a fold (45.8) read the ledger by kind, never by grepping a journal. A
byte that names no kind refuses the read. The hypha's sites: a listing
drawn on the page is `List`; the install branch of a demand and the home
install on a peer's stratum are `Install`; a puncture of a held word is
`Section`; a verdict applied and a compression or refinement bid
accepted are `Ruling`; a connection docketed by its ratifier is
`Attest`; a page served is `Syllabus`; a released escrow is `Lapse`; a
probe that closed or descended is `Settled`. The mind's: `Stood`,
`Uttered`, `Learned`. `Fold` and `Adopted` are written by 45.8's fold and
adoption (S13.9, S13.11).

K orders events inside one kernel and nothing else. A mind's K is its own
chain (`Session::voice().k()`); it is not a clock reading and it is never
compared with a hypha's K. No kernel reads `SystemTime`, `Instant`, or a
`Duration`: `chronometry::unix_now`, `TUniversal::ticks`, `ticks_at`,
`load_or_genesis`, and `MeshRecord.origin_unix` (mesh.rs 20) are deleted, and
the clock gate of `MU-ENGINEERING.md` §7 fails a build that names them in a
kernel, court, ledger, physics, or transducer file.

### 45.2 TUniversal is the height of the ledger

```
h(genesis) = 0
h(r)       = 1 + max( h(prev(r)), max over p ∈ parents(r) of h(p) )
```

A kernel's TUniversal is the height of its head receipt. A parent's height
is read from the parent receipt itself, which the kernel holds because it
verified it (47.2); a height a receipt claims that does not re-derive from
its parents is `REFUSE_RECEIPT`. Along the causal order of 45.3 heights
strictly increase (T15). The converse is never read: two heights are
compared only as an extent at one reader, `h_now − h_then`, and never to
order two events. Synchronisation is the exchange of receipts itself: every
cover digest carries its signer's head `(K, height, hash)` (47.1), and the
receiving hypha's next receipt cites it as a parent. A hypha that hears
nothing does not advance. Mesh time advances only by verified acts, and the
receipts that carry it are the receipts that carry the proofs of reduction:
the temporal ledger and the proof ledger are one ledger, so a proof's
position in time is a coordinate it carries, not a stamp read off a wall.

### 45.3 Order across nodes is causal and partial

The **frontier** of a receipt `r` is the keyed map `F(r): vk ↦ greatest K
of vk's receipts reachable from r through prev and parents`, computed by a
keyed walk that visits each reached receipt once
(`ReceiptChain::frontier(&self, budget)` **new**). `e ≺ e'` iff `F(e) ≤
F(e')` componentwise and `F(e) ≠ F(e')` (`precedes` **new**). Two receipts
neither of which precedes the other are **concurrent**, and concurrency is
permanent: a later receipt citing both orders both before itself and neither
before the other.

A decision that depends on the order of two concurrent receipts is not
taken. The two are held, exactly as a vortex is held (3.9, law 15), and the
conflict, if there is one, resolves by content: two installs of one address
are one vertex, merged with conservation as A8 merges (16.8 step 4, where
"earlier" reads causally earlier and concurrent entries are unioned); two
forms on one class are a Mayer–Vietoris question (chapter 13); two mints of
one deflation are a duplicate issuance and the second is burned (47.4).
`cocycle_at`'s supersession (cognition.rs 1036) compares the two receipts'
causal order; concurrent representatives are both kept, since 15.4 gives
them one key. No node breaks a tie by any scalar, and arrival order at a
socket is not a tie-break. Log `court concurrent {addr_a} | {addr_b} held`.

### 45.4 Extents and durations

Every duration a kernel reads is a count of its own K or of heights.

- **Transport.** A round trip is the number of the kernel's own strokes
  between a frame sent and its reply. `Mesh.rtt_window` and `rtt_floor`
  (mycelium-server/exchange.rs) hold stroke counts; `persist_dwell`, `aperture_cycle`,
  and `transport_deadline` (physics.rs) are stroke counts derived from them;
  `from_transport(rtt_floor, aperture_cycle())` is unchanged in form and
  returns strokes. The socket layer polls readiness each stroke and blocks
  on readiness when the kernel has nothing to admit; it passes no timeout
  the kernel computed. The OS scheduler's clock is the only physical clock in
  the system and yields no number a kernel reads.
- **Standing.** The T lane of μ⃗ is an extent in heights at one home. For a
  ruled carrier it is `h_now − h_install` on the home's chain
  (`standing_mag`, cognition.rs 960). For a voice it is the height delta
  between two successive receipts of that voice the home admitted; the home
  writes `(K_voice, K_home, h_home)` into the admitting receipt, and the
  voice reports nothing but its K. No admitted receipt, no extent: an idle
  voice accrues nothing, and a home that hears nothing advances nothing. Any
  witness re-derives a home's extents from the home's chain (T16).
- **The mind.** `Epigenome::shed` (sdk/epigenome/shed.rs) and `retire_on_t` (bubble/shed.rs
  223) count the mind's own K; `moved_tu`, `last_used_tu`, `stroke_tu` are
  K values and are renamed `_k`.
- **Carriers.** Every `_tu` field in the kernel (`provisional_tu`,
  `standing_tu`, `aperture_tu`, the second tuple field of `cocycle_at`, the
  eighth of `Settlement`) is a height and is renamed `_h`; `MarketView.tu`
  (cognition.rs 135) is deleted; the `set_clock` test hook becomes
  `set_height`.

### 45.5 Genesis and epoch

`H_genesis` is the chain hash of the mesh's first receipt, written once by
the first hypha started on a fresh state and provisioned to every other node
the way the bootstrap peer list is. `E_origin: u32` is the epoch,
incremented by a recut that wipes ledgers under the same keys: a new
genesis, the keys kept. `MeshRecord` carries `genesis: [u8; 32], epoch: u32`
in place of `origin_unix`, signed as before. Both are in every signed prefix
(47.1); a frame naming another genesis or epoch is `REFUSE_ORIGIN` in 47.2
step 1, before any carrier is touched. The `tuniversal` file, `scripts/tu`,
and the origin copy in `ops/lab-homes.py` are deleted; the recut procedure of
`MU-RECURSION-TASKS.md` writes the new epoch's genesis on hypha a and
provisions it to b, c and the minds.

The mesh record of a new node also carries the fold roots of the hyphae
covering the strata it will work in (45.8), provisioned and signed as the
genesis is; the eon (this cut), the era (a fold) and the kernel's own K
are the three coordinates of 45.8 (ruled 2026-09-09).

### 45.6 Binding

```
chronometry/receipt.rs  Parent NEW; KStepReceipt.{height, parents} NEW; Height NEW replaces TUniversal
chronometry/chain.rs    ReceiptChain::append(parents, …); frontier(&self, budget) -> Keyed<u64> NEW; precedes(a, b) -> bool NEW;
                    unix_now, ticks, ticks_at, load_or_genesis DELETED
mesh.rs             MeshRecord.{genesis, epoch} replace origin_unix; origin_of DELETED
cognition.rs        now_tu → height(); standing_mag in heights; every _tu carrier → _h; MarketView.tu DELETED; set_clock → set_height;
                    cocycle_at supersession by causal order (45.3); one receipt per admitted act carrying its parents
sdk.rs              now_tu → height() read from the last admitting receipt; shed and stand by the mind's own K
dialogue/            ACT_SPEECH tu → voice K + cited home head; ACT_ROOT ticks → (K, height, hash), the reply carrying the receiver's own
                    (K, height) as its comment at 227 already states; Settlement field 7 → height; cover_signed_prefix → the prefix of 47.1
mycelium-server/       Mesh.rtt_window/rtt_floor in strokes; the boot line prints genesis and epoch; note_root keeps the speaker's (K, height, hash)
physics.rs          transport_deadline, persist_dwell, aperture_cycle in strokes; no Duration
build.rs            the clock gate: SystemTime, Instant, Duration refused in src/server, src/client, src/court, src/transducer, src/economy/, src/physics.rs
```

### 45.7 Local orbit, universal height (ruled 2026-09-08)

The mesh synchronises on TUniversal and on nothing else: the height is the
one quantity two kernels share, it advances only by the exchange of
verified receipts (45.2), and it is read only as an extent at one reader.
Every other temporal reading a kernel takes is its own. K is the kernel's
own index of admitted acts (45.1), and the **stroke** is the kernel's own
orbit: one circuit of what it holds, in the fixed order the carrier gives —
the pivot's candidate list over the occupied axes (12.6), the court's
docket in key order, the mind's dirty bubbles, the basis. A kernel's local
time is therefore two counts and never a clock: K, the acts it admitted,
and the orbit count, the circuits it completed; both are per kernel, both
are per lane where the lane paces (24.3), and neither is ever compared
with another kernel's.

The docket orbit rules what is not yet ruled: a listing already on the
ruled cover, or relocated to the cavity, is passed over by the circuit —
its standing moves only by a verdict or a lapse — so a circuit's cost is
the unruled listings and its `court section` lines are one per ruling.
Re-replayed on every circuit, one word was sectioned 20,800 times and
another slashed 2,227 times on the fleet of 2026-09-09 (R1).

The orbit is the primary motion, and it is a circuit, not a walk. A cursor
that resumes after the last key it served and stops at the end of the key
order has walked off the edge of its book: nothing below the resume point
is ever revisited, and a listing admitted at a smaller key is never ruled.
A cursor that restarts at the first key on every stroke never reaches the
tail beyond one stroke's budget. Neither is an orbit. `Keyed::cursor(from,
budget)` continues past the last key from the first and ends where it
began, and reports the completed circuit so the stroke can count it; every
stroke over a carrier resumes where it left off; the period of a carrier —
strokes per orbit — is measured by that count and is never a constant. The
convergence engine's orbits were its conjugacy classes and its genesis
class was the sun they were read against; that geometry is the conjugate
level of 23.9 and stands. Its dynamics — motion between orbits down a
coherence gradient, chosen by echelon votes, identified by a hash of a
normal form — are the imports Stage 9 removes, and an orbit that pulls
toward the axioms by a potential is one of them. The pull that belongs is
the group action itself: each circuit applies each generator once.

**Velocity is a reading (ruled 2026-09-09).** A carrier's period — strokes
per orbit — is what its last orbit's outcome and its measured transport
say, never a phase, a schedule, or a curve that says when to slow. The
mind's stroke spends the lanes its outcome moved and dwells by the
pressure the last reply reported (24.3); the court's docket orbit spends
its stroke; the exchange asks a peer again only when that peer's Λ (45.4)
has elapsed since the last ask, counted in this court's own K, or this
court's own roots moved, and an idle orbit spends the tick; a deferral
(23.8) carries the door's per-lane deficit and dwells the asker on it. A
fresh node in a mature mesh runs hot because every sync moves words, and
slows as it converges; a mesh at equilibrium spends ticks and nothing
else. Equilibrium is that fixed point, not a rate. `Physics::stroke` is
priced by the reserve per lane: a lane at zero reserve affords only the
tick, and every floor and ratio in the reserve is a host calibration or
leaves (rule 5, S10). Every carrier's orbit count and period stand beside
K on its log line (34.2, 34.3), so velocity is a kernel coordinate and
never a wall-clock window.

### 45.8 Eras: the fold (ruled 2026-09-09)

Three coordinates, and only the first is ever shared as a boundary:

- the **eon** is the cut, `E_origin` (45.5), a wipe boundary before beta;
- an **era** is a fold of one chain at a height, per chain as K is;
- **K and orbits** are the kernel's own (45.7).

A hypha folds its ledger behind a **fully final prefix** — no listing,
provisional word or escrow below the prefix's head — when its own room
reading says so; never at a height interval, on memory alone, or on a
wall clock. The reading is the reserve shortfall (S10): a docket orbit
that completes while a lane is short is the syntropic phase (24.3), and
the fold is the ledger's consolidation stroke in it — read once per
orbit, and a prefix that is not final folds nothing and writes no line
(S13.9). The fold writes the net per identity per lane as `epoch` rows
(`MU-ECONOMY.md` §2), which move no supply, and seals a receipt of kind
`fold` citing the archived prefix's head hash, so the chain never breaks
and heights continue; a fold changes no price, extent or ruling, it
re-expresses rows. The fold is a two-organ act (47.3): a witness fetches
the prefix by K range (47.2's back-fill act), re-derives §6 across it and
co-signs the fold root; a prefix that is not final is `REFUSE_FOLD_OPEN`,
one that does not conserve is `REFUSE_FOLD_INEXACT` (34.1). What stays
hot after the fold is what still acts: the `epoch` rows, the install
receipts (the proofs of compensation, already two-signed) and the
presentation (the ruled cover, on two organs). The work receipts —
attests, quotes, offers, pages — go cold, held by the home in the `ARC1`
section (28) and by the witness that fetched them to co-sign, so every
archived prefix has two holders and no operator's archive; extraction is
by K range from either (47.2), and a court asked to replay era N fetches
and re-derives. A chain may be **adopted from a fold root** and is never
truncated behind one: a joiner seeds its chain from the fold's head and
height on the two signatures and its own §6 re-derivation over the fold
rows it received, and replays the archive on demand, never as a
precondition (47.2 step 1). A joiner chooses an era, a depth and strata
(24.2); it never chooses a velocity or a phase, and it lists only into
the head's book. The mesh record of a new node carries the fold roots of
its cover beside the genesis and epoch (45.5).

### 45.9 Phase is a reading (ruled 2026-09-09)

The window is an aperture of docket orbits (`aperture_cycle()` circuits),
so a court whose docket fits one budget reads its phase over a span of
receipts and not over the one receipt a single circuit may seal (R7,
2026-09-09: `comprehension 0/0` on every orbit at period 1); `court phase
…` is written once per window.

**These four are the instrument** (ruled 2026-09-10). Any reading of this
engine is taken in its own coordinates — addresses, ranks, generators
adjoined against retired, the presentation's size against the families it
comprehends — and never in a taxonomy a reader named. A tally of the
demand kinds of 50.5 is an observation and orders nothing: it is the
engine rendered into human buckets, and a build order chosen from it is a
count deciding a branch (rule 2) with the implementor as the effector,
which is the fault 37.7's adjunction exists to remove. Whether a corpus's
families close is that corpus's score and measures the **cognition**;
whether the presentation grows, converges, and comprehends more per
generator measures the **engine**, and it is `growth` and `comprehension`
below that read it. The one number that bears on the architecture is
generators the engine adjoined at run time with no commit; until the
adjunction of 37.7 runs, that number is zero and every report says so.

A phase is computed from the receipts over a height window and is never
stored, announced or thresholded; a transition is read after the fact and
gates nothing. Four coordinates, each read from the receipt kinds (45.1)
and the ruled cover at the window's ends: **comprehension**, listings a
held word deflated against listings held; **congruence**, strata holding
more than one section and installs reached by independent lineages
(23.9); **growth**, generators adjoined against retirements (E2), the
language's description length against what it comprehends; **expansion**,
axes opened and strata split (3.7). The regimes these name — ooze,
congruence, presentation, equilibrium — are words for readings, not
states of the mesh, and two nodes holding the same receipts read the
same phase (13.7). A joiner reads the phase of its cover off the fold
summary it adopted; nothing declares it.

## 46. Identity: threshold keys and manifests

### 46.1 Group keys

A voice's or hypha's verifying key is the group key of a FROST signing group
over Ed25519 as specified in RFC 9591: two nonces per signer per session, a
binding factor over the commitment list and the message, `t` of `n` partial
signatures aggregated into one ordinary 64-byte signature. A scheme with one
summed nonce is the naive multisignature that falls to the ROS attack under
concurrent signing and is refused as a design. The mind's eyes hold the
shares: the multi-eye cognition of the client is also its signing quorum, and
a voice cannot speak without internal agreement. The wire, the court, and
every verifier see one 32-byte key and one signature and are unchanged
(`court::identity::verify`, identity.rs 77). Resharing rotates the shares
without changing the key. Re-keying is a receipt of kind `rekey`, signed by
the prior group key and naming the successor; carriers keyed by `vk` follow
the successor as A8 follows an address remap, with the same conservation.
Binding: `client::identity::VoiceGroup` **new** (the `frost-ed25519` crate),
replacing the `VoiceKey` of `XSEPTUM.md` §8; `court::identity::HyphaKey`
becomes a group key when a hypha has more than one operator.

### 46.2 Manifests

Keys scoped to a chart or a lane are independent keys, never additive tweaks
of the group key: a public tweak lets anyone holding one scoped secret
subtract it and recover the root. A **manifest** is a receipt signed by the
group key listing `(chart moduli | lane, key)` pairs at its `K`; a later
manifest revokes the earlier one whole. A frame on chart `c` verifies against
the manifest key for `c` after the manifest is confirmed active
(`REFUSE_MANIFEST` otherwise), and a compromise of that key is confined to
`c`. Nothing else in the tree changes: the manifest is a receipt, and the
verifier reads one key. Log `[{tag}] manifest {n} keys K {k}`.

### 46.3 What identity does not carry

Standing. A node's standing is its bond per lane and the slash and burn
entries against it in the ledger (`MU-ECONOMY.md` §4), read as room by
dominance (`lane_room`, cognition.rs 3455). There is no score, weight,
threshold, or decay on a node, as there is none on a word: a word's standing
is `Ruled`, `Stood`, `Held` (24.1); a hypha's is its room. A slashed hypha
homes and witnesses less because its room fell, not because a number said so.

## 47. Receipts on the wire

### 47.1 The signed prefix

Every signature in the tree is over exactly these bytes, in this order:

```
H_genesis 32 ‖ E_origin u32 ‖ prev_receipt_hash 32 ‖ K u64 ‖ height u64 ‖ chart len16 + axes u64… ‖ payload address len16 + bytes
```

The payload address is the M2 composite (A21) for a word or form, the
ruled root or cover root for a transmission digest, the listing key for a
settlement act, the manifest's own address for a manifest. It is never a
digest of frame bytes: two spellings of one word have one receipt lineage,
and a receiver reads the form's coordinates off the address. The chart is
the FRAME's chart, the one the payload lives on and a manifest key would be
scoped to (46.2): a word's or form's chart on an offer or a verdict, and
none on a transmission digest, whose payload is a root over the whole
cover. It is never the signer's cover written out: a prefix carrying every
chart a hypha holds grows with the book, and a length the wire writes in
two bytes then wraps, which is how every digest of a hypha with a thousand
listings was refused whole by every peer on 2026-09-08 (Stage 11 I33). A
prefix that does not fit its wire lengths is refused where it is built,
never signed and never framed (rule 11). The `prev_receipt_hash` and the
receipt's own chain hash are digests of receipt bytes, which are the
object there. `cover_signed_prefix` (dialogue/cover.rs) becomes this
prefix, so every signed digest is a receipt and every signer's causal
position travels with its signature.

### 47.2 Admission

M7 step 1 (17.7) becomes, for every signed frame, in this order and before
any carrier is touched:

1. `H_genesis` and `E_origin` equal the node's own: else `REFUSE_ORIGIN`
   (`court origin refused {vk}`). A node that adopted a fold root (45.8)
   holds the genesis and epoch the fold names and its chain's head is the
   fold's; the adoption stands on the two signatures and the node's own §6
   re-derivation over the fold rows, and the archive behind the fold is
   fetched by K range and replayed on demand, never as a precondition
   (ruled 2026-09-09). The same K-range act serves a gap's back-fill (step
   3) and an archived prefix (45.8): `ACT_RECEIPTS` (0x36, S13.10),
   `[ACT_RECEIPTS][voice 32][signer 32][from u64][to u64]`, answered with
   `[n u32][receipt…]` — the signer's own receipts from its hot chain or
   `ARC1`, or the receipts a holder admitted from that signer — as many as
   fit the frame budget, from `from` upward. A fetched receipt carries no
   signature of its own: it is admitted by its chain — it re-derives its
   hash, links to the head this node holds for the signer, and its height
   re-derives — and the signed head that reported the gap is admitted
   again once the chain reaches it. On the exchange that reported a gap,
   pages are fetched until the chain reaches the reported K or the
   exchange's own stroke budget is spent (`court receipt backfill {vk}
   {k0}..{k1} admitted {n}` per page), and the frame that reported the
   gap is then admitted again on that same exchange; nothing back-filled
   is cited as a parent: the closing frame is. One page per gap reported
   fell behind a peer's K on the fleet of 2026-09-09 and no gap ever
   closed (R4).
2. The signature verifies under the claimed key or, for a chart-scoped
   frame, under that key's active manifest entry: else `BAD_SIGNATURE` or
   `REFUSE_MANIFEST`.
3. `K` against the signer's known head `K_h` with hash `H_h`:
   - `K = K_h + 1` and `prev = H_h`: admit, and record the receipt as a
     parent of the node's next receipt (`court receipt parent {vk} K {k} h
     {h}`);
   - `K > K_h + 1`: a **gap**; the frame is docketed as a pending citation
     is (`citation pending`), the missing receipts are fetched from the
     signer or any peer holding them, and nothing is acted on until the
     chain closes (`court receipt gap {vk} {k_head}→{k} backfill`);
   - `K ≤ K_h` and the held receipt at `K` has the same chain hash: a
     duplicate frame, ignored, `REFUSE_RECEIPT_STALE` in the reply;
   - `K ≤ K_h` and the held receipt at `K` has a different chain hash: an
     equivocation proof, 47.4.
4. The height re-derives from the parents the receipt names: else
   `REFUSE_RECEIPT`.

### 47.3 Two organs sign an install

An install is a receipt with two signatures over one prefix:
`(address, sig_home, sig_witness)`. The witness is any hypha that

- holds ruled sections over the chart, since replay needs them, so it is at
  nerve distance one from the home;
- has bond outside the home's delegation closure (`MU-ECONOMY.md` §4:
  delegation shares slash, so a delegate is not a second vantage), else
  `REFUSE_WITNESS_DEPENDENT`;
- has room on every lane the install's escrow occupies, by dominance
  (`lane_room` ≥ escrow lane-wise), else `REFUSE_WITNESS_ROOM`.

The volume of a cell is the measure of the open listings on its chart:
its vertices are vantages (hyphas), and no speaker's index gates it.
Read through `spoken_by`, which is keyed by voice, every hypha vertex
measured zero, so no section stood a 0-cell, no edge had volume, and
nothing ever filled although charts held two sections (R6, 2026-09-09).
Two sections that are the same body at two vantages glue by the identity
edge, which fills, pays the cell's volume and records the gamma; two
bodies a versor relates but no alphabet rotor spells stand as a
remainder edge, the obstruction recorded — spelling that versor as a
rotor is its own item.

`confirm_peer_holds` (cognition.rs 3031) becomes the request for the
witness's signature over the install prefix after the witness's own replay;
`home_install` (3067) writes the ruled root and the mint only when both
signatures are held, and the install receipt cites the witness's receipt as
a parent. Log `court witness {vk} co-signed {addr}`. Identities are free;
room is not. A node running two hyphae has two keys and one bond, and the
second cannot witness the first.

**The fold is a two-organ act (45.8, ruled 2026-09-09).** A witness for a
fold is a hypha with bond outside the home's delegation closure that
fetches the folded prefix by K range, re-derives `MU-ECONOMY.md` §6
across it, and co-signs the fold root; the home writes the `epoch` rows
and seals the `fold` receipt only when both signatures are held, and it
keeps the cold prefix beside the witness that fetched it. Log `court fold
witnessed {vk} co-signed {root}`.

On the wire (S13.10): the home proposes with `ACT_FOLD` (0x37) —
`[ACT_FOLD][k0 u64][root k u64 ‖ hash 32 ‖ height u64][n u32][(who 32 ‖
μ⃗ 4×u64)…][47.1 receipt section, payload the root hash]` — to the peers
with room on some lane, in key order, one per spend of its stroke. The
witness runs 47.2 steps 1 and 2 on the section, fetches the prefix's
receipts by `ACT_RECEIPTS` and its ledger rows by `ACT_ROWS` (0x38,
`[voice 32][signer 32][from u64][to u64][cursor u32]` → `[total u32][m
u32][row…]`, a page of the signer's own rows anchored in the span from
the cursor), re-derives the receipts as one chain ending at the root and
the rows' nets as the epochs proposed (`MU-ECONOMY.md` §6), and answers
refused with the 34.1 constant, unmeasured when the prefix could not be
fetched, or co-signed with its own signed position over the root hash. A
witness without bond is `REFUSE_WITNESS_ROOM`; the home itself is
`REFUSE_WITNESS_DEPENDENT`. On the co-signature the home checks the
witness's signature under its key, that the proposal still reads exact
and its prefix still final, and only then archives, seals and writes;
the era carries the witness's key and both signatures, and the witness
files the same era under the home's key.

### 47.4 Equivocation

An **equivocation proof** is two signed receipts under one key with equal
`K` and different chain hashes; equal hashes are a duplicate, not a proof.
Any node presents it (`ACT_EQUIVOCATION` **new**, 0x35 in `XSEPTUM.md`). The
court verifies the two signatures and the equality of key and `K` and burns
the signer's bond on all four lanes: one `burn` ledger entry naming the
proof, supply reduced by the same vector, so `supply = Σ balance + Σ bond +
Σ escrow` (`MU-ECONOMY.md` §6) holds through the forfeiture (R9). Nothing is
replayed and nothing is redistributed; the burned μ⃗ leaves circulation. Log
`court equivocation {vk} K {k} burned {mu:?}`. A duplicate issuance
discovered when partitions rejoin (45.3) is burned the same way: the second
mint of one deflation is a `burn` entry naming both install receipts, not a
slash of anyone.

### 47.5 Withholding

A citer whose cited form the cited address's home has not served within
`Λ(chart)` receipts admitted at the citer's home lapses: `Λ` is
`from_transport(rtt_floor, aperture_cycle())` on that chart in strokes, never
a constant. On lapse the citer returns to `provisional`, its escrow is
released to its payers as A8 releases a refused tape's, the missing address
stays docketed, nothing is slashed, and the log reads `court citation lapsed
{addr} after {n} receipts`. `overlap goodness pending` (13.9 step 0) lapses
by the same rule, the glue returning `Glue::Dialect` unchanged.

## 48. The adversary

Named: any number of nodes holding valid keys, which may equivocate,
withhold, replay frames, collude, mint identities without limit, and claim
any `K` or height. They cannot forge a signature, invert blake3, or make a
tape re-derive to a vertex it does not reach. Every mechanism of this part is
a finite check a node performs on its own carriers; none is a vote.

| threat | what stops it | where |
|---|---|---|
| a tape that lies | re-derivation by M1, M2, A5; two organs replay | 34.1, T11 |
| a signer who tells two peers two truths | the equivocation proof; bond burned | 47.4 |
| free identities witnessing themselves | room, not identity, admits a witness: bond outside the home's closure, room covering the escrow | 47.3 |
| a home that never serves what it cites | lapse in admitted receipts; escrow returned; no mint | 47.5 |
| two partitions minting one deflation | causal order holds both; one vertex; the duplicate issuance burned | 45.3, 47.4 |
| speech spent to burn a hypha's strokes | every claim escrowed against its own deflation (`MU-ECONOMY.md` §4); the congestion door (`door_defers`, cognition.rs) | 26 |
| a clock moved forward or back | there is no clock; a height is re-derived from signed parents | 45.2, 47.2 step 4 |
| a stolen key | the manifest confines a scoped key to its chart; a `rekey` receipt succeeds the group key | 46 |
| a frame replayed later | `K ≤ K_h` with the same hash is ignored | 47.2 step 3 |
| a reputation gamed | there is none; room is bond, and bond only falls by slash or burn | 46.3 |

Not defended, and said so: a mind whose entire bootstrap list is adversarial
sees only what those hyphae serve, the list being provisioned out of band as
the genesis is; flooding a socket below the frame layer; and the honesty of
the genesis provisioning itself, which is the one act that precedes any
receipt.

## 49. Settlement bound to this part

`XSEPTUM.md` is the binding document for settlement and is rewritten against
chapters 45–47. The deltas, so the auditor can check both texts against one
list:

- terms: `K` is a receipt index, TUniversal a height (45); there is no clock;
- every frame signs the prefix of 47.1; the `CHALLENGE` carries the payee's
  prefix and the home's `(K, height)` at pricing with `Λ` in admitted
  receipts; the `NOTE` carries the payer's prefix, so the payer's `K` is the
  replay protection and there are no nonces;
- expiry is the home's own `K` passing `k_home + Λ`;
- `RELEASE` carries the home's and the witness's signatures under 47.3;
- `SEPTUM_EQUIVOCATION` (0x35) is the proof act of 47.4; `DUPLICATE` means
  an identical Note at a spent `K`, and a different one is `EQUIVOCATION`;
- refusal codes `EQUIVOCATION`, `ORIGIN`, `WITNESS_DEPENDENT`,
  `WITNESS_ROOM`, `MANIFEST`, `RECEIPT_GAP` are typed variants (XSEPTUM §9);
- voice identity is a FROST group with manifests (46),
  `client::identity::VoiceGroup`.

Mode 1 of xSEPTUM, paying with work, is unchanged: the receipt that releases
escrow is the word that replays, which is the one place in the system where
the phrase is the credential. Identity says who is paid; the word says
whether.

---

# Part X — Language formation

Written 2026-09-09 against d6e19a1. Carried from Parts I–IX: words are
paths in the Cayley graph, forms are vertices, faces and loops, the court
replays, the mind comprehends, and time is the receipt ledger. This part
says what a voice speaks — a sentence, composed — and how a receiver takes
it apart — factored, replayed — so that meaning is derived and never
recalled; and it names the three seams through which unspoken chains
reached the market on the fleet of 2026-09-09.

## 50. The sentence: construction and deconstruction (ruled 2026-09-09)

### 50.1 The theorem

Let a scene transformation be an endomorphism on the discrete cellular
space `X` (the sensor's ℤ²; chapter 38 widens it). A **valid sentence**
`S` is an irreducible, connected Čech 1-complex over the support bodies
`{B_k}`, `k = 1..m`:

```
S = ( {W_k, V_k, γ_k}_{k=1..m},  {Γ_ij}_{(i,j) ∈ E} )
```

each vertex carrying an exact word `W_k ∈ Spin(4,0)(ℚ)` spelled in the
seventeen letters (1.1), a carriage `V_k ∈ Cl(3,1)(ℚ)` (A4) and a gauge
permutation `γ_k` of the hues; each directed edge carrying the transport
`Γ_ij` of 50.3. `S` is valid iff

1. **Local reconstruction.** For every `k`, `(V_k · W_k · γ_k) · B_k^in =
   B_k^out` under exact cellular equality: every cell of the input body,
   carried by the sandwich, lands on a cell of the output body with its
   hue mapped by `γ_k`, and every cell of the output body is reached.
2. **Global holonomy closure.** Around every minimal cycle of `E`, the
   product of the edge transports, read in the base body's stabilizer up
   to conjugacy (50.3), is the class the sentence files: the identity, or
   a torsion class filed with its period (A17). A cycle whose product is
   neither is a held loop with its class vector (12.6), and the sentence
   is valid but not closed.

A vertex that reconstructs nothing — a bare chain of cells with no word,
no carriage and no gauge — is not a vertex of any sentence. That is the
whole content of the ruling: the unit a voice speaks is the sentence, and
every vertex of it is a word that acts. What the alphabet cannot spell
and what the eyes cannot pair ride the sentence as typed demand vertices
(50.5); they reconstruct nothing and claim nothing.

### 50.2 The two functors (no search)

**Speaker, F: Percept → Sentence.** F runs on the voice and nowhere else
(law 8 as ruled 2026-09-10): the sensor's cells never leave the client.

1. Measure versors. For each body pair the eyes admit (24.4), `V_k =
   versor_between(B_k^in, B_k^out)` (blade/carry.rs: translation from the
   centroids, dilation from the spread, rotation from the moment
   harmonics, every candidate verified cell by cell) and the hue map
   `γ_k = derive_hue_map` (element/gauge.rs). Both exact or refused; a
   refusal is a demand of 50.5, never a chain. Where several candidates
   verify they are one coset of the body's stabilizer, and the versor
   returned is that coset's **canonical representative** (ruled
   2026-09-09): the member nearest the identity in the Cayley graph — the
   geodesic of the coset, law 6's normal form read on a coset — which is
   the greatest `a²/w` of its rotation part (15.3's class invariant, an
   exact rational; a reflected member reads zero), ties broken by the
   least A21 key of the canonical rotation; never the first that
   verified. A dilated bar is therefore carried by the dilation and not
   by the dilation composed with the bar's half turn, and a square by the
   turn within the eighth. Two voices measuring one pair return one
   `V_k`, so `Γ_ij` of 50.3 is the same class on both; the representative
   is a gauge fixed by content, A1's sign step at the altitude of a
   coset, and not a choice among candidates (rule 4).
2. **Factor through the presentation, and peel only the residue** (ruled
   2026-09-10). `W_k` is first factored through the generators the mind
   holds on the measured versor's stratum (A12, `WorkingAtlas::stratum`,
   42.6): the presentation is the language (1.8), and a voice says what
   its own generators reach. The reading is `Factored`:
   **Understood** — the versor is in the span, and `W_k` is the path
   through those generators, each cited by address; **Novel** — the
   factorisation leaves the canonical remainder of A12 step 5, unique and
   with no choice in it, and only that residue goes to `factor_rotor`
   (A22, 50.4), which peels it against the seventeen; **Unmeasured** — the
   budget ended, which is not a demand (rule 11). A residue A22 cannot
   spell either is the demand, and the demand carries it as its witness
   (50.5).

   A22 from the identity, run on every measured versor, is a **function of
   the versor alone**: it returns the same tape whatever the mind knows,
   has learned or has stood, so two voices with different languages emit
   one tape, no held generator ever enters an utterance, and nothing a
   mind learned can change what it says. That is a cipher — meaning
   recovered from a table rather than reached by composition — and it was
   the whole of the utterance path until this ruling. It survives as the
   last step and never the first.

   **The path is the proof, and the proof is what a word vertex is**
   (ruled 2026-09-10, superseding the reading that put the path beside
   the tape). A word vertex does not carry a spelling. It carries its
   **derivation**: the generators it factored through, each cited by
   identity — the cited form's address and the address of the word vertex
   within it — with its exponent, in application order; and the residue
   in letters, the part A22 peeled from the identity because no held
   generator reached it. The word of the vertex is `∏ g_i^{e_i} · r`,
   **recomputed** from the derivation whenever a rotor is wanted, and
   never stored: a stored spelling beside a derivation is two carriers
   for one meaning, and the one that goes stale is the one a court
   replays. A vertex a mind spelled from the identity alone is the case
   where the generator list is empty and the residue is the whole word;
   that is the cipher's case, and it is now a boundary of the general
   one rather than the only one.

   This is a coordinate of the form, so the tape changes: **tape v7**.
   Version 6 is refused on read, as version 5 is. The court composes the
   cited generators on its own ruled cover, multiplies by the residue,
   compares the product projectively against the vertex's own carriage
   block, and pays each cited generator's discoverer at the install of
   the standing sentence (26, 50.9). A cited form the court has not ruled
   is not a refusal: it is the deferral a `Glued` citation already takes,
   and the syllabus serves the generator (21 step 3). Two minds with
   different presentations reach one vertex by two derivations — one
   address, two proofs (13.10's synonyms, 23.9's conjugate level) — and
   the derivation is where the wire tells two languages apart, because
   the address, being the vertex, cannot.

   **The factorisation at the ring level** (ruled 2026-09-10, L11
   (3/3), resolving L13). A12 over letter exponents reads only the
   generators the seventeen span, and a generator the court adjoined at
   the ring level (37.7) is a Gaussian prime the seventeen do not. On
   the sensor plane a rotor's block is a Gaussian integer up to scale,
   so the factorisation through such generators is division in ℤ[i]:
   each held prime is divided out to its full multiplicity, in the
   presentation's key order — unique factorisation up to units makes
   the order a matter of the log line and nothing else — and cited with
   that exponent; the cofactor is the residue, peeled by A22 against the
   seventeen; a cofactor A22 cannot peel is the demand, with the
   cofactor as its witness. The two readings are one operation at two
   levels of the same table (37.7): the lattice over letter exponents
   where the generators are words of the seventeen, the ring where they
   are primes beyond it.
3. Connect. For every edge the eyes admit, `Γ_ij` by 50.3; a pair with
   no rigid versor is a remainder edge.
4. Form loops. The holonomy of every cycle of `E`, read in the base
   body's stabilizer up to conjugacy; filed as torsion or held with its
   class vector.
5. Emit the composed sentence, its demand vertices riding on it, one per
   (kind, chart).

**Receiver, G: Sentence → Proof.** G runs on the court and is the court's
only reading of an utterance; its answer is the adjudication of 50.9.

1. Factor. Every word into its letters, every letter into its rotor
   coordinates (A1), every edge into its transport (50.3).
2. Replay. `B'_k = (V_k · W_k · γ_k) · B_k^in` on the receiver's own copy
   of the input.
3. Congruence. `B'_k = B_k^out` exactly, per vertex; the receiver's own
   factorization of `V_k` is compared with the speaker's by the class
   vector (15.4, 12.6) — conjugacy in the group, never equality of tapes.
4. Coherence. The holonomy of every cycle as 50.1 step 2; a sentence
   whose loops close is integrated into the working dialect as an active
   theorem (24.1, 25.2), and a sentence with demand vertices is stood
   withheld with them.

`F` and `G` are adjoint: `G ∘ F` on a percept is the identity on what the
alphabet spells, and its defect is exactly the typed demands `F` emitted.
Nothing in either is a search, a candidate list or a nearest match; every
step is a measurement, a peel, a product or an equality.

### 50.3 Edge transport in the stabilizer (ruling 1)

`Γ_ij = V_j · V_i⁻¹` is a coboundary: around any cycle its product
telescopes to the identity, so it measures nothing and carries no
meaning. The edge transport is:

- `V_ij`, the versor carrying the pair-body `(B_i ∪ B_j)^in` onto
  `(B_i ∪ B_j)^out`, measured by `versor_between` on the union's shape; it
  exists iff the pair moved rigidly.
- `Γ_ij = V_i⁻¹ · V_ij`. When the pair is rigid, `V_ij` carries `B_i^in`
  onto `B_i^out` or onto `B_j^out`, so `Γ_ij` lies in `Stab(B_i^in)` — a
  symmetry of the body: the identity, a half turn, a quarter turn — or in
  the exchange `Aut(B_i ↔ B_j)` of two congruent bodies that swapped
  places. A rigid global motion gives `V_ij = V_i = V_j` and `Γ_ij = 1`;
  a relative motion the pair still shares gives the stabilizer element or
  the exchange that distinguishes it.
- A pair with no `V_ij` is a **remainder edge** (`set_remainder_edge`,
  13.9 step 7): the two bodies are two charts, the split is
  Mayer–Vietoris (13.9), and no loop through that edge is evaluated.
- The **loop holonomy** is the product of `Γ` around a cycle, valued in
  the base body's stabilizer and certified up to conjugacy by the class
  vector (15.4); stabilizer cosets do not telescope, which is why this
  product carries what the coboundary could not. Finite, it is filed as
  torsion `(σ, ℓ)` (A17); infinite or undischarged, it is held with its
  class vector (12.6). The torsion the form files today (`Form.torsion`,
  the hue-map orbits) is the gauge component of this holonomy; the shape
  component — the stabilizer and the exchange — is what this ruling adds.

`Form::face_of` remains the coboundary of the words' placements (the
relative carriage, 3.1); the edge transport is carried on the face beside
it (`FormFace.edge`), and the court re-derives both.

### 50.4 Ordered commuting-pair factorization (ruling 2)

`Spin(4,0)` is not abelian across planes sharing an axis, so no single
Smith normal form over the letters' exponents exists. Letters on
complementary planes commute (1.2). `factor_rotor(R)` (A22) peels in the
fixed order

```
(Π_01 × Π_23)  →  (Π_02 × Π_13)  →  (Π_03 × Π_12)
```

Per commuting block the exponent lattice is `ℤ² ⊕ ℤ/4` per plane — the
(3,4,5) and (5,12,13) exponents and the quarter turn — and Smith normal
form over it (A12 step 2) decides the factor exactly; the block's rotation
is read by `spell_peel` (the Gaussian-integer peel on the plane block,
alphabet/spelling.rs) and its remainder is the block's own. A block whose
rational rotation carries a Gaussian prime outside `{5, 13}` leaves
`Demand::RotorPrime(plane, prime)`; a block that is not a rational plane
rotation in this order leaves `Demand::Standing(block)`. Nothing across
blocks is ever treated as one lattice. The sensor plane `(0,1)` with its
complement `(2,3)` is the first block, so every rotor a plane sensor
measures factors in the first step and the remaining blocks read the
identity; the order matters only when perception writes on the third and
fourth axes (chapter 38).

### 50.5 Typed demands and the anti-freeze protocol (ruling 3)

A demand is one of exactly these kinds, and a demand of no kind is not a
demand:

| kind | what `F` could not do | what it carries |
|---|---|---|
| `Demand::RotorPrime(plane, prime)` | spell a plane rotation whose Gaussian factor is a prime outside the alphabet | the plane, the prime and the witness rotor — the peeled block in A1 form (37.7 P2) — as A21 coordinates |
| `Demand::Standing(block)` | decompose a rotor block in the order of 50.4 | the exact 2×2 block, canonical |

Both rotor demands carry a **block that is the residue**, not the
measured versor (ruled 2026-09-10, 50.2 step 2). `F` factors through the
mind's generators first; what a demand states is the part no held
generator reached and A22 could not peel. The block is therefore the
witness in the exact sense 37.7 P2 needs — the thing a second court
re-derives the Gaussian factor from — and it is what the demand carries
on the wire, both kinds alike. A demand whose block is the whole measured
versor is a demand from a mind that held nothing on that stratum; that is
one case of the rule, not the rule.

| `Demand::Source(B_out, chart, multiplicity)` | say which input body an output body is the image of | the output body's chart and the number of input charts a versor carries onto it, every one standing as a section, none first-found |
| `Demand::Creation(B_out, chart)` | find any input body carrying onto `B_out` | the output body's chart |
| `Demand::TopologicalDeformation(B_in, B_out)` | find any element of `Cl(3,1)(ℚ)` carrying `B_in` onto `B_out` — the harmonics differ | both charts and the axes on which they differ; answered by the cover of 50.8 — pieces each carried, the loop of their meet, the cell the court fills — through Mayer–Vietoris boundary splitting (13.9), never held for a rotor that cannot exist |

Demands ride the sentence as vertices (`Vertex::Demand`), aggregated per
chart: one vertex per (kind, chart), never one per body, carrying the
bodies they stand for as the sentence's remainder. The **closed
sub-complex** of the sentence — every `Word` vertex and every cycle over
them — is spoken, replayed and ruled at once; its deflation is paid on
closure (E2); the demand vertices are listed by the court as demands
under their kind and chart (24.4's Novel), priced and relayed by the
market, and answered inside the running engine — by a letter the court
opens on two witnesses (37.7), a section (25), a split (13.9) or a filled
cell (50.8) — never by a commit. A voice therefore never emits nothing and never emits a bare
chain: a percept `F` cannot close in full is a sentence with demands, and
a percept `F` cannot open at all — no body pair the eyes admit — is a
percept with no sentence and no listing, reported to the voice as such.

### 50.6 The three seams, severed

Read on the fleet of 2026-09-09 (d6e19a1): 226 remainders expressed
against no sentence spoken on one voice, 235 held against 23 reduced,
genomes of 250–420 ruled words comprehending 0 of 2 listings, no axis, no
class. Unspoken chains entered the market through three seams; each is
closed by this ruling and stays closed by the invariant below.

1. **The voice's remainder.** `Entity::withhold_form`
   (client/sdk/entity/think.rs) attested every uncovered body of a
   withheld form through `from_residual` as a word (`remainder expressed`,
   `uttered … cites 0`), and `hold_thought` held the same chain as an
   idea. The withheld form is stood with its demand vertices and uttered
   whole; no residual is expressed, `attest` is never called with a bare
   chain, and an idea is a form whose words are read at the listing's
   chart when a cite asks (24.1), never listed ahead of one.
2. **The body-only vertex.** `family::body_only_vertex`
   (family/perceive.rs) made an unpaired body a `Vertex::Word` with the
   identity word and no measured versor, so a created, destroyed or
   non-similar body was a vertex that reconstructed nothing. It is a
   demand vertex of 50.5: `Source` where an input chart carries onto it,
   `Creation` where none does, `TopologicalDeformation` where the
   harmonics differ.
3. **The court's irreducible listing.** `reduce_proven` and `reduce_germ`
   (server/cognition/reduce.rs) answered
   `NormalForm::Irreducible(from_residual_framed(residual))`, and
   `commit_germ` listed, escrowed, ruled and installed it; `attest(chain)`
   on the wire and `from_residual` on any utterance path are the same
   seam. The seam is severed at the wire (ruled 2026-09-10, law 8): a
   voice sends sentences only, the court's entry point takes a sentence
   and calls G (`perceive_speech`, 23.1), and the percept act, the demand
   act, `adjudicate`, `percept_elements`, `reduce_all`, `reduce_germ`,
   `reduce_proven` and `commit_germ`'s chain arms are deleted rather than
   refused — the four `from_residual` sites of the court's reduction fall
   with them, as a side effect and not a checklist. `from_residual`
   remains only where a chain is the object — a listing key, a remainder's
   measure, a fixture — and never as a word, a citation or a genome entry.

**The invariant**, checked by the auditor on every landing and every
soak: no `remainder expressed` line, no `uttered … cites 0` line on a
chain, no `court attest … cites 0 residual m of m` line on any process;
every listing on any book carries a demand kind; every entry of every
genome is a sentence or a word spelled in the letters.

### 50.7 Binding

```
sentence/form/carrier.rs   Vertex::Demand { kind: Demand, chart: Invariant, bodies: Vec<Cell> } NEW; FormFace.edge: Option<Transport> NEW (50.3; None is a remainder edge); tape v6, a recut (§10)
sentence/demand.rs         Demand NEW: the five kinds of 50.5 and their A21 key (kind byte, then the coordinates each carries)
alphabet/factor.rs         factor_rotor(&Rotor) -> Result<Word, Demand> NEW (A22, 50.4), over spell_peel per commuting block
blade/carry.rs             versor_between on the union shape for V_ij (50.3)
family/perceive.rs         word_form → factor_rotor; demand_vertex NEW; body_only_vertex DELETED
family/pair.rs             pair_form: F steps 1–5; the edge transport on every face
sentence/form/carrier.rs   Form::loops_of composes FormFace.edge around a cycle; holonomy read in the stabilizer up to conjugacy
client/sdk/entity/think.rs withhold_form stands the sentence withheld and holds it; hold_thought DELETED (hold_chain holds a chain as the pair form of its scene); attest(chain) unreachable from a withheld form (L6); from a served listing until L8
court/verdict/replay.rs    replay_form_with: G steps 1–4; REFUSE_NOT_A_SENTENCE, REFUSE_VERTEX_BODY, REFUSE_EDGE, REFUSE_DEMAND_UNTYPED (L7)
court/verdict/body.rs      vertices_carry_family (23.1 step 3), edges_rederive_family (step 4), remainder_undeclared (step 7 on the closed sub-complex) — G on the court's copy of the family (L7)
server/cognition/germinate/listing.rs  bind_percept_family: the adjudicated pair is the sentence's family (23.8); list_demands: `court demand listed` per demand vertex (L7)
server/cognition/reduce.rs percept_family (the court's copy of the pair); percept_elements lists no form without a vertex that acts (50.5, L7); reduce_proven, reduce_germ: no Irreducible(from_residual*) (L8); a demand vertex is listed by kind; a chain is refused
server/cognition/germinate/ingest.rs   commit_germ lists demand vertices, never a residual (L8)
```

Tests (chapter 39): `a_sentence_reconstructs_every_body_or_is_refused`,
`edge_transport_lies_in_the_stabilizer_or_the_exchange`,
`a_rigid_scene_has_identity_edges_and_a_relative_motion_does_not`,
`a_non_rigid_pair_is_a_remainder_edge_never_a_loop`,
`loop_holonomy_in_the_stabilizer_does_not_telescope`,
`factor_rotor_peels_commuting_blocks_in_order`,
`a_prime_outside_the_alphabet_is_a_rotor_prime_demand`,
`a_created_body_is_a_source_or_creation_demand`,
`a_deformed_body_is_a_deformation_demand`,
`demands_ride_the_sentence_one_per_chart`,
`the_closed_subcomplex_is_ruled_while_the_demands_are_listed`,
`a_bare_chain_is_refused_by_the_court`,
`no_utterance_path_calls_from_residual` (a gate over `client/` and the
attest arms). Traces (chapter 40): on a cold boot every voice's first
`form withheld` is followed by `sentence spoken … demands d` with `d ≥ 1`
and `d` `demand` lines, the hypha answers `court demand listed` `d` times
and `court attest` with a sentence replayed, and `remainder expressed`
appears nowhere.

### 50.8 The levels of one remainder (ruled 2026-09-10)

Chapter 50.5 typed what `F` cannot do; until this ruling its demands had
no consumer in the running engine, and a demand without a consumer is a
sensor without an effector. The consumer is **one** — 37.7's adjunction —
and this section says what the remainder's canonical form is at each
level the factorisation can stop at. Neither level is a search and
neither is a commit; neither is a closer written for a kind.

**The field of the letters — `RotorPrime`.** The letter a prime demands is
the witness rotor's own Gaussian factor; two independent courts re-derive
it from the witness at one chart, the home opens it, and the syllabus
teaches it before any voice speaks it (37.7, law 6 as ruled). The alphabet grows by ruling, at run time, and
every voice that learned the page spells the same word.

**The topology of the bodies — `TopologicalDeformation`.** No versor
carries a body onto an image with different harmonics; but a body is a
union of pieces, and pieces carry. `F` peels `B_in` into pieces each of
which a versor carries onto a piece of `B_out` — the shells of
`peel_body_scaled` (chapter 22) read against the image, or the one-hue
components of each (20) — and speaks one `Word` vertex per piece, the
contact faces between pieces carrying `Γ` (50.3). Then, by the pieces'
overlap:

1. **Every meet good and every edge in the stabilizer.** The pieces glue
   to one section (13.9 step 5, `∂θ = 1`): the deformation was a rigid
   motion of a body the eyes had cut, and there is no demand.
2. **A pair of pieces with no `V_ij`.** A remainder edge (50.3): the two
   pieces are two charts, the sentence is split by Mayer–Vietoris (13.9),
   and it is valid with the split and no demand.
3. **A loop through the meet.** Where two pieces' meet has more than one
   component (13.9 step 0; the arcs of a ring, the arms of a fork), the
   pieces' nerve has a cycle, and the product of `Γ` around it, read in
   the base piece's stabilizer up to conjugacy, is the deformation's own
   holonomy. Finite order (15.3): `cech::extend` files the 2-cell whose
   boundary is that loop, `Fill::Class(ℓ)`, and the sentence carries the
   class as its theorem (law 12) — the filled cell is a constructor the
   mind glues by (25.3), the degree-2 class of 15.4 keyed by the loop and
   the holonomy's invariants, carried or discharged under refinement
   (15.5), and 13.9 step 7 finds the same class at the court. Infinite
   order: `Fill::Open`, the loop is held with its class vector (12.6), and
   the `TopologicalDeformation` vertex stands as 50.5 says.
4. **Cells of `B_out` no piece reaches, or of `B_in` no piece carries.**
   `Source` or `Creation` vertices ride beside (50.5): a hole that opens
   or closes is those cells, not a holonomy, and is never a class.

Nothing chooses the cover: the pieces are the peel's shells, the loop is
the meet's, the order is the class's, and the court re-derives all three.
`court cell filled` (34.2) is the line at the court and `cell filled
{addr} class {ℓ}` at the voice.

**Binding.** `family/perceive.rs`: `demand_vertex`'s deformation arm peels
the pair into pieces and speaks them (`pieces_of` NEW over
`peel_body_scaled` and `blade::components`); `family/pair.rs`: contact
faces between pieces with their edges; `cech/cell.rs` `extend` unchanged;
`sentence/form/carrier.rs`: the filled cell on the form's tape beside its
loops (tape v7, one recut with 37.7's escape byte);
`court/verdict/replay.rs`: the filled cell replayed as any loop
(`REFUSE_CELL_ORDER` when the loop's order is not the filed `ℓ`). Tests:
`a_deformed_body_is_spoken_as_pieces_that_carry`,
`a_loop_of_pieces_with_finite_holonomy_fills_a_cell`,
`a_loop_of_pieces_with_infinite_holonomy_stands_as_a_demand`,
`a_hole_is_cells_never_a_class`,
`the_court_rederives_a_filled_cell_or_refuses_it`.

**The ruling, as given (2026-09-10).** The seams are severed first (L6 to
L8): no generator is grown on a substrate that leaks chains. No search
returns: a generator is measured at the moment of failure and held as a
typed invariant, never found by a beam. And chapter 37's opening is the
court's own transition, automated on the congruence of two courts'
re-derivations, not an operator's intervention.

### 50.9 The contraction protocol (ruled 2026-09-10)

The ruling names this section 50.7; 50.7 is the binding table and 50.8
the walls, so it stands here. A refusal that does not change the
speaker's state transition is an inert log line, not a protocol: a voice
that utters a sentence, receives a refusal and re-emits the same tape on
the next stroke is deaf, and a mesh of such voices is an echo chamber.
The feedback loop is an algebraic contraction on the held form, certified
like a pivot (A20) and verified by the court like any move.

**The judicial reply.** The court never returns an opaque byte. Its
answer to an utterance is the `Adjudication` of chapter 26:

```
Adjudication := Stood { receipt } | Ratified { receipt } | Refused { kind, at, receipt }
kind         := VertexBody | EdgeHolonomy | RemainderUndeclared | MalformedComplex
at           := Vertex(address, chart) | Face(address_i, address_j) | Chart(invariant, cells) | Tape(address)
```

`Stood` is the docket's receipt: the sentence replayed on the court's copy
of the family, the closed sub-complex docketed under the voice's room, the
demands listed. `Ratified` is the second organ's co-signature, carried on
the syllabus page (47.3), and the answer to a sentence the court's
language already holds — a use of a ruled word. `Refused` names the kind
(34.1) and the coordinate as content — the vertex's address with the chart
it acts on, a face's two addresses, or the chart with the cells the court
read — never a position in the tape; `Tape(address)` is the coordinate of
a `MalformedComplex`, the whole tape. The chart rides with the vertex so
that a court which never held the refused tape can still verify that the
demand stands on that chart (the L8 landing; a coordinate the ruling's
grammar left implicit). The receipt is the court's own replay receipt
(45.1, 47.1). The door's deferral (23.8) is the act's deferral frame,
never an adjudication: nothing was heard.

**The four certified contraction moves.** On `Refused { kind, at }` the
voice executes a shrinking move `C : S → S'` on the held form in its
contemplation, and every move strictly decrements the closed sub-complex,
so the loop is well founded:

1. `VertexBody` at `Vertex(a, chart)`: the word vertex `a` becomes a
   demand vertex over its body's chart — `Creation`, carrying as its
   bodies the cells the word was to reach, read as the remainder the
   contracted form leaves on the speaker's own family — and the closed
   sub-complex loses one word; the faces that reached the vertex leave
   with it, since a demand vertex enters no face (50.5). The
   `TopologicalDeformation` kind names harmonics the cover of 50.8 (L11)
   reads; until it lands the move speaks `Creation` (a departure of kind,
   named at the L8 landing).
2. `EdgeHolonomy` at `Face(a, b)`: the contact face becomes a remainder
   edge (`edge: None`, 50.3); the 2-cell whose boundary ran through it is
   unlinked. The coordinate carries no cycle, so nothing is filed as
   torsion here: a finite cycle is filed where it is walked, by the pivot
   (A17).
3. `RemainderUndeclared` at `Chart(inv, cells)`: the undeclared cells
   become `Demand::Source` over that chart, with the multiplicity the
   count of bodies the cells fall into (50.5); the sentence now declares
   what it does not close.
4. `MalformedComplex`: structural collapse — the form becomes an inert
   held idea and the mind is quiescent on that chart until its language
   moves; nothing is re-spoken.

Each move is a form rewrite with a certificate, `Contraction { kind, at,
before: address, after: address, receipt }`, filed on the idea beside its
pivot certificates and written as `contracted {addr} → {addr'} by {kind}
at {coordinate}` (34.3).

**The re-utterance ban.** A refusal is an admitted act of the mind (45.1)
and seals a receipt on the mind's own chain; the refused address is
sealed in the utterance index (`taught_up`, 24.3) with the refusal. The
speak path reads the sealed set before any frame is built, and a sealed
tape is refused by the mind itself — `sealed {addr}: refused at K {k},
not uttered again` — so re-uttering the identical sentence is
structurally impossible: the only door out is a new address, and the only
way to a new address is a changed form. The ruling as given says this
refusal "raises an immediate runtime panic"; §2.4 and the panics gate
forbid an abort in cognition, and the effect the ruling asks for — the
identical tape cannot leave — is exactly what the sealed set gives without
one. That is a departure of word, not of invariant, and it is named here.

**The contraction is verified by the receiver.** The only valid
subsequent utterance is `S'`, citing the refusal receipt as its
contraction certificate, carried on the speech frame beside the tape
(26). The court that issued the receipt holds the refusal by the refused
address; it replays that `S'` is `C(S)` at the coordinate it named — the
vertex is now a demand on that chart, the face now a remainder edge, the
cells now declared — before it replays `S'` as a sentence, and writes
`court contraction verified` or `court contraction refused` (34.2); a
contraction it refuses is a `MalformedComplex` at the tape. A court that
never refused `S` holds no receipt to verify against and replays `S'` as
a sentence, verifying nothing: the certificate is a claim on one court's
chain, and 47.3's second organ re-derives the sentence, not the refusal.
Bilateral synchronization is therefore a proof the court checks, not a
discipline the voice is trusted to keep.

**The contraction certificate is the only thing that rides beside the
tape** (ruled 2026-09-10). It is a claim about *which utterance this is*
— a receipt naming a prior refusal — so it is not a coordinate of the
form and belongs on the frame. A word's derivation is the opposite kind
of object: it is what the vertex is, it is replayed as part of `G`, and
it is on the tape at v7 (50.2 step 2). The earlier reading that put the
path on the speech frame beside the sentence is struck; nothing about a
vertex's meaning travels outside the tape that carries the vertex.

**The intelligibility assertion.** The protocol is proved by one exchange
run in-process through the dialogue codec, with no socket: Voice A utters
`S`; Court 1 replays on its copy of the family and answers a typed
adjudication; Voice A receives it, contracts, and utters `S'` citing the
receipt, or is quiescent; Court 2 — a second kernel with its own identity
and no shared runtime memory (the pair form's memo cleared or on its own
thread) — replays the serialized frame and reaches the same address and
the same standing. A single differing rational refuses consilience (13.9
step 8). Until that test passes the tree has an echo chamber; when it
passes it has a medium.

**The soak invariant.** Across any round trip the closed sub-complex of a
sentence either grows, because a demand was answered by a taught word, or
shrinks, because a vertex was refused; it never stands. No address is
uttered twice with the same standing, and the auditor greps for
`adjudicated … refused` lines followed by the same address uttered
unchanged.

**Binding.** `anastome/dialogue/speech.rs`: `Adjudication` on the speech
reply; `book/speech.rs` `perceive_speech`: the court's one entry point for
an utterance, answering an adjudication per sentence and verifying a cited
contraction; `client/sdk/entity/utter.rs`: `receive_adjudication`, the
sealed set on the utterance index, the contraction certificate on the
next utterance; `bubble/contemplation.rs`: the four moves on a held
idea's form (`contract`); `sentence/form/certificate.rs`: `Contraction`
beside `PivotCertificate`; tests: `a_refusal_contracts_the_held_form_and_seals_the_address`,
`a_sealed_tape_is_refused_by_the_mind_before_the_wire`,
`the_court_verifies_a_contraction_before_it_replays_the_sentence`,
`two_courts_of_distinct_identity_reach_one_standing_through_the_codec`.
