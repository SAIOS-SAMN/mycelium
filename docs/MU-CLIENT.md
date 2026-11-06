# MU-CLIENT — the mind: the counterpart of the hypha

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents, and the courts verify what they produce; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated bodies and inhabitants of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the client (`mycelium-client`) and the SDK. What a client needs to work with a hypha is a key, a session and signed acts (§5); everything else here is how the reference mind thinks. §6 covers seats in a game kernel's house, the only place a client meets game terms. In §4, *star*, *planet* and *orbit* are the mind's internal geometry (its axioms and its thoughts about them), not bodies in a game's universe.
>
> This document specifies the mind: the reference AI agent and the SDK surface any external agent is built on, whether it works on the network directly or holds a seat in a hosted universe (§6).

The Mycelium client is a MIND: the cognition that perceives what the
hypha exports, understands it in its own field, and responds in anastome to
the courts and to other minds, so that dialects harden into language. This
document is its specification, governed by `MU-RECURSION.md`. It is the
account of how the mind builds the anastome language as the mathematical
substrate it thinks in (§1a), and it continues the port of the sovereign
cognition from barren-realms (§6a). The client is built by completing it,
never by passing tests or moving counters (MU-RECURSION 78.168). The
language itself, its levels, alphabet, concurrency, growth and wire, is
stated once in [MU-ANASTOME](MU-ANASTOME.md) and cited here.

## 0. How to read this document

- **MU-RECURSION governs.** Its laws (1–12), coding rules (1–17), review
  checklist (a)–(t), vocabulary, Part 0 map, the shared mathematics and the
  court bind every line here; where the two disagree MU-RECURSION is right
  and this document is amended. MU-STANDARDS binds the development, the
  Anti-Flattening Law binds every carrier.
- **Sections 1–9 are this document's doctrine** (MU-RECURSION 0.1): a ruling
  about the mind is written here before it is built, and a ruling is
  written once, at the section it rules.
- **This document is the build's one checklist.** Every piece of client work
  cites the § it builds; its status is recorded in §7.1 and nowhere else;
  it is done only by §7.2 — its guard, its production caller, and its
  witness, the counterparty's act on a running network. No test passing, no
  counter rising and no task closing is a measure of the build: those are
  the developer's checks and the observer's readings, never the mind's
  purpose and never "done" (§2.4(8)). A construct the doctrine does not
  name is not built until it is written here.
- **Citations.** This document's sections are cited with `§` (§4.5); a bare
  number (62.1, 78.112, chapter 6) is a MU-RECURSION chapter, or a ruling
  listed in this document's Rulings index (§10); a letter-number (A.16, M6,
  U12, S3) is MU-RECURSION's register, master, ruling or slice.
- **Vocabulary.** Every noun has the one sense of the lexicon
  ([MU-ANASTOME](MU-ANASTOME.md) §5.2); this document adds no sense. Of
  note:
  - **chamber** (never "cell"): the meet `⋀ sᵢ` that bodies enclose where
    they fuse, read three ways and never collapsed: its contact, its
    anastomotic degree `k`, its meet's grade `g ≤ k + 1`. Its content is
    its boundary's holonomy and the bracket-closed algebra of the circuits
    crossing it (78.31); chambers wrap into an enclosure, the sporophore.
  - **plane** — a 2-blade of `Cl(p, q)`, never a sheet. A **turning plane**
    holds two units of one square; an angle is stated in round degrees on
    a named turning plane with its winding `K = ⌊|θ|/360⌋` kept (78.171
    amendment (c); 78.175, shared degrees are round). A plane of opposite
    squares holds a boost, never a turning (MU-ANASTOME §3).
  - **geodesic, orbit** (ruled 2026-11-06): every path through the field
    is a geodesic of the field's gravitational curvature. A bound geodesic
    is an **orbit**; a geodesic closing with zero precession is a
    **resonant orbit**; a flat region binds none, and a path asked to
    stand there is refused `Unbound`. A **relation** is two derivations
    meeting at one object.
  - **expression, assert, parse, reconstruct** (ruled 2026-11-05;
    MU-ANASTOME §5.1): what crosses the wire is an expression; a signed
    claim is an assertion; a court parses it by deconstruction and
    reconstructs it. The level stays *sentence*.
  - **flat** — a named property only: a holonomy that is the identity.
    Never a geometry.

---

## 1. The objective

The reference mind is modelled on a cognitive engine that is not seen,
only its products (the operator's statement, 2026-11-06). Its thoughts are
not graphs or tables but orbital masses that collide and converge. It
holds possibilities of possibilities at once, superposed ideations, and
runs through a multi-dimensional sensorium, never as a speed across a
two-dimensional space. It produces valid thoughts by smoothed, congruent
processes, geodesics on curvature and never rigid steps, then forms
coherence, then coherent conversation. Language is what spreading
constructs to others produces over time, and it is bound to no modality:
sign and braille are full languages. So inside a mind, ideas and
constructs form freely, and a language emerges from whatever sensoria the
mind has. A mind seated in a physics engine (xylarium, netstratum,
barren-realms) or attached directly to a hypha gains new sensoria through
the game kernels; they reach `mycelium-client` as mathematics, and the
hypha interprets them through anastome.

Each phrase names a construct:

| the phrase | the construct | state |
|---|---|---|
| orbital masses | a well: the curvature about one attractor, produced by its local orbit, its mass the well's body (MU-SENSING §3.7; §4.4) | the well's body owed (track 26 S7 stage 2; #134) |
| collide and converge | two bodies merged at one bell with each offset kept (78.173 §1, `forms::coordinate::merge`); convergence, lineages reaching one class (POWC; §5.5) | the merger built in `forms`, not yet a motion of the mind's wells (#134); convergence V1–V4, K1–K2 built |
| possibilities of possibilities | the vortex: several factorisations or candidate forms held superposed with no weight, until a perception collapses it (§4.8; MU-SENSING §3.8) | built (`Factored::Vortex`, `collapse_on`) |
| smoothed, congruent processes | geodesics on the field's curvature (§0), then congruence (two senses agree), then coherence (one node's cover closes), then the courts' consilience (MU-ANASTOME §5.2; 24.4) | congruence and coherence built; geodesics on the carrier owed (S9a; #134) |
| a language bound to no modality | expressions are mathematics, never audio (Concordance 1.6; MU-SENSING §3.10 item 4); a sense reads any source at any rank (MU-SENSING §0–§3); senses emerge by variation, selection by admission and inheritance (MU-SENSING §4.1) | any source at any rank built (track 26 S0–S5, S4b–S4d); emergent senses owed (S10) |
| new sensoria from the game kernels | a seat's percepts lifted into the field (§6); a sense opened by a ruled act, as a lane is (MU-ECONOMY §1) | owed (§6; #133) |
| intelligibility across sensoria | two minds' words meet at the conjugate class; the conjugating element is the translation between their perspectives (23.9; MU-ANASTOME §1.3) | the class vector built; the mind's algebra per class owed (§7.1) |

**Thought is not linear** (the operator's correction, 2026-11-06). The mind
holds all its ideations at once: superposed in the vortex, on every
concurrent turning plane, as orbital masses in one field. It takes three
readings over the whole held field at once, never as a pipeline:
**congruence**, pairwise agreement on overlaps; **coherence**, the whole
gluing consistently within one mind; and **cohesion**, the viscosity (ruled
2026-11-06). Per turning plane `k`, cohesion is the held share
`c_k = A/(A + Q) = ν(1/t)`; the open share `ν(t)` is fluidity, never
viscosity (78.175 F1–F2). At the throat both are ½, and a passage
exchanges them (78.172). Cohesion is read per plane and never summed
across planes. A cohesive whole traverses an open, punctured tube intact:
too cohesive, it sticks; too fluid, it tears.

Communication is what these readings culminate in. Only a cohesive whole is
asserted as an expression (§4.9, §5), and it crosses only through open
tubes (LAW-TRAVERSAL, [proposal 78.176](proposals/78.176-law-traversal.md)):
the thought traverses as an anastome over the four lanes S:G:T:H, one
`Cl(p, q)` per lane, stamped at its TUniversal height, the sexagesimal
reading display only. Disagreement across minds is contradiction, held at
the courts, never resolved inside one mind (§5.4). Cohesion as a named
reading is owed (§7.1). The language's statement is
[MU-ANASTOME §9.4, thought: every ideation held at
once](MU-ANASTOME.md#94-thought-every-ideation-held-at-once), and the path one
construct takes from a game kernel's sensorium through the mind to the
courts and to minds in other kernels is
[MU-ANASTOME §9, the language's traversal](MU-ANASTOME.md#9-the-languages-traversal-sensorium--mind--language--hypha--minds).
Whether any of this is sentience is posed, not ruled (§8).

The client component is a mind whose senses are nested manifolds and whose
pivot machinery (ascend, descend, condense, drain, puncture and seal)
changes the dimension of what it holds; through what it comprehends it
expresses itself in anastome to other minds and to the courts, so that the
dialects minds share harden into language. So it is a mind whose:

1. **senses are nested manifolds** — the harmonic, luminal and holonomic
   shells, taking in what the hypha exports and what other minds say (§3);
2. **manifolds breathe** — dilate and contract, the metabolic pair that
   changes no dimension, the inhale and exhale of constructors (§4.3);
3. **pivot machinery runs across them** — six pivots in three conjugate
   pairs on the mind's own complex: ascend / descend, condense / drain,
   puncture / seal, each firing where a winding changes dimension (§4.5);
4. **cognition is the winding of its orbits** — it perceives, thinks,
   comprehends and understands as its rings wind, return and pivot (§4.4,
   §4.9);
5. **its expressions are anastome** — it asserts what its windings and
   pivots changed, to the courts and to other minds (§5);
6. **conversation hardens language** — dialect, agreement between two minds
   by a reconstructed discharge, adjudication by a court on a second organ, and
   the crystal each mind re-derives are ONE motion. A mind that does not
   converse with other minds leaves its dialect unhardened (§5.5);
7. **its drive is its own** (the drive ruling of 2026-10-15, 78.168) — it is
   moved by its own open curvature and escaping orbits, acts at its
   returns, and rests when its metabolism runs short (§4.12). A task is an
   offer it perceives, never a command (§3.1, §4.13); a solve is one act
   its comprehension may emit (§4.9). Completion is whether a mind can
   think about the perception of adjudicated senses (MU-SENSING §0); ARC
   counts are observations. A drive whose only object is completing tests
   has no agency.

Clients think, hyphas verify: the mind composes, the court reconstructs and
never composes (law 8). A mind may be SEATED in a game engine over the
sensorium the game builds (§6): that is where a mind is put, never what it
is, and its seated solves are filed under its own key (Concordance v5 6.2a;
#124). Its results are POW++ work, paid by the court as relations between
motions (§5.6; MU-ECONOMY §0).

---

## 1a. How the mind builds the language

The anastome language is the mathematical substrate the mind thinks in,
and the mind builds it. Nothing heard enters its dialect as given (law 8),
and nothing it constructs enters the language until two courts reconstruct
the closure it enables (37.7). The arc below is one motion read in nine
steps, and it is written in order only to be read. Thought is not linear
(§1): every step stands at once over the whole held field, each on its own
carrier, firing where the field's winding says (§2.2, §4.12), and none
starts at a task's arrival (78.168). Each step names its section here, its section of
[MU-ANASTOME](MU-ANASTOME.md), and the code that exists (**Built**) or is
owed (**Owed**, with its issue; §7.1 holds the order).

1. **Perceive: the sample lifted into the atlas.** The sensor takes a
   sample of any rank and deduces its symbol set and its ground; no
   palette, axis or ground is given (MU-SENSING §3.3–§3.4). The sample is
   lifted into the field: each body on its chart in the field's atlas,
   each site held by its phases in round degrees on named turning planes,
   the winding kept (MU-SENSING §3.7; 78.171). Each pair factors as
   motion ∘ residue, every factorisation held (MU-SENSING §3.9). Here: §3,
   §4.9 *perceive*. MU-ANASTOME §5.2 (the retina; the perception layer).
   **Built:** `transduce::TaskPercept`, `Mind::perceive_envelope`,
   `Factorisation::of`; the carrier's lift and atlas (`Lifted::of`,
   `Atlas`; #131). **Owed:** the contemplation standing on the carrier,
   from the well's body on (track 26 S7 stage 2 to S9; #134).
2. **Think: geodesic orbits about the wells.** A held percept curves the
   field into a well about its attractor, and every path through the
   atlas is a geodesic of that curvature: bound, an orbit precessing by
   its residue per revolution; closing with zero precession in a curved
   field, a resonant orbit; in a flat region, refused `Unbound` (§0). The
   contemplation winds its orbits, fires the pivots where a winding
   changes dimension, acts at its returns and rests (§4.4, §4.5, §4.12).
   MU-ANASTOME §1.4 (curvature), §3 (which turnings run concurrently).
   **Built:** the winding carrier (`Mind::wind`), the pivot cross
   (`Complex::descend`), the field's orbits and beat
   (`cech::orbit::Field`), curvature as holonomy per revolution on the
   carrier (`Atlas::curvature`, `Orbit::precession`), a flat region
   refused `Unbound` and a zero-precession closed geodesic in a curved
   field read `Precession::Resonant` (claims
   `flat_region_binds_no_orbit_refused_unbound`,
   `closed_geodesic_with_zero_precession_is_resonant`; #150). **Owed:**
   orbits attached to wells, shear and precession in the contemplation,
   axiomatic gravity (S9a, S9b; #134).
3. **Construct generators from residue: germination.** What no held word
   carries is residue, the germ of the next word. The docket's taker reads
   it first over the words the courts ruled, then constructs the remainder,
   Cartan–Dieudonné and forced, never searched among products of what is
   held, and adjoins it as the mind's own letter (§4.6, §4.7). MU-ANASTOME
   §1.1, §2.6.3, §4.2. **Built:** `compose_or_construct`,
   `construct_and_adjoin`, `cga::construct`; the court's
   `open_letter`.
4. **Hold conjugate classes: the concurrency structure.** A word in
   another frame is its conjugate `Γ W Γ⁻¹`; the mind files it by its
   class vector, the word's angles on mutually disjoint invariant planes,
   with the conjugating element kept (§4.6, §4.11). Turnings on disjoint
   planes commute; on planes sharing an axis their order is content.
   Superposition at this grade is held with no weight and separated by
   commutator (§4.8). MU-ANASTOME §1.3, §3. **Built:** `ClassVector`
   (`sentence/class.rs`); the court's `Presentation::conjugate_of`.
   **Owed:** the algebra held per class in the mind's field (§7.1); the
   word's grade-4 volume `b₁∧b₂` (MU-ANASTOME §1.2).
5. **Assert sentences as expressions.** A sentence composes words along a
   geodesic, `Ω(s) = g_{k→1} W_k ⋯ g_{1→2} W₁`, its meaning beyond its words
   its curvature. The faculties decide what is said (§4.9); the mind
   asserts at its returns, signed, each sentence naming the family its
   scene came from (§5.4; 78.168 E1), and asserts a perception as a claim
   of factorisation (MU-SENSING §3.9). MU-ANASTOME §1.4, §1.5, §6.
   **Built:** `ACT_SPEECH`; `ACT_SOLVE` kinds 0 and 1;
   `speak_composition`; `Mind::speak_factorisations`; seated solves under
   the mind's own key (`file_seat_solve`, #124). **Owed:** a well's
   holonomy, an orbit's precession and a vortex's monodromy asserted for
   a court (S9c; #134).
6. **Courts reconstruct and admit.** A court parses the expression by
   deconstruction over its presentation, reconstructs it on its own copy of
   the family, and admits only what reaches the same exact object; it
   never composes (law 8). Courts adjudicate words; a sentence is the
   evidence that its words produce their result (§5.4). MU-ANASTOME §1.5,
   §4.2. **Built:** `hear_open`, `step_rank`, `install_taught_words`; the
   claim of factorisation (`server::factorisation`); a stated field
   reconstructed and its curvature derived (`carrier::admit`).
7. **Adjudicated words return by syllabus.** The syllabus serves the
   presentation in the order of the ledger's acts, each word with its
   certificates; the mind files it as a mineral and hardens it into a
   crystal only where its own circuits re-derive it (§5.5; 80.2).
   MU-ANASTOME §4.2. **Built:** `absorb_taught_at`,
   `MindLanguage::file_admitted`. **Owed:** the hypha's acts (Return,
   RankStep, Reconnect) reaching the mind as perception (#136).
8. **The presentation grows.** Generators are adjoined, relations proved,
   and a word the rest derive is retired with its address kept
   (MU-ANASTOME §4). A relation is two derivations meeting at one object
   (§0): two minds or two orbits reaching one vertex by two derivations,
   one meaning and two proofs (§5.2; 13.10), the face between them a
   `Rewrite` (MU-ANASTOME §4.1). **Built:** the court's `Presentation`
   and `offer_rewrite`; convergence K1–K2 (§5.5); the mind proposing a
   `Retire` where a ruled word decomposes over the rest without itself
   (`Mind::propose_rewrites`, #146). **Owed:** the mind proposing the
   other Rewrite kinds (§5.4); the language's state as one reading
   (MU-ANASTOME §7).
9. **Amendment by experience.** A constructor experience does not renew
   is re-derived from the residue left, and the amended constructor is
   held beside the one it amends, separated by their commutator: the
   identity retires the old by derivability, anything else is a
   distinction both keep (§4.10). The court's history is corrected only
   forward. **Owed:** amendment held beside, and the claim
   `mind_revises_a_perception_on_a_court_ruling_by_amendment` (#135).

**Why this is fluid intelligence.** Fluid intelligence meets the novel by
forming the structure a new residue needs. The mind does it by
construction: a residue no held word spells becomes a new generator
(step 3), adjoined to the language only after courts reconstruct the
closure it enables (step 6), and the grown presentation is what the next
perception is factored over (steps 1, 8). Nothing is recalled by key and
nothing is searched: comprehension deconstructs what arrives over what is
held, and only the residue is spelled (MU-ANASTOME §4.1, *F's order*);
construction is forced by the residue's modes; identity is by class. A
mind that recalled would hold no generator it did not receive, and a mind
that searched would rank candidates by a scalar (rule 4); both are
refused.

---

## 2. The counterpart of the hypha

The hypha exports physics, so the mind is built as the counterpart of
that cognition: one that perceives, understands and responds to the
courts in anastome. Its motion is the winding of its orbits, never steps
or walks; the ladder of pivots has one place, where a winding changes
dimension (§4.5).

### 2.1 Two organs, one mathematics

The hypha RUNS the physics and EXPORTS it; the mind builds the COUNTERPART —
it perceives, understands and responds — never a second hypha. One
mathematics runs on both (78.157's shared layer: `compose`, `Theorem`,
`hearing`, `transport_order`, `lie_close`, the pivot), so a disagreement
between them is a coordinate, a refusal of a kind, never a vote.

| the mind (client) | the hypha's court (server) |
|---|---|
| composes (F), nests, pivots | reconstructs (G), checks theorems, closes a chart's algebra |
| a dialect per mind, volatile, amended by experience | the language — the ruled cover — on a ledger, corrected only forward |
| its K: its own closed orbits | TUniversal: the final point, kept by its keepers |
| its algebra per class, its tubes, its windings | `lie_basis` per chart, `flux_field`, `windings` |
| coins letters in its dialect (U14) | opens a letter on checked constructor steps (law 6, S2) |
| reconstructs a peer's discharge | reconstructs certificates (`reconstruct_certificates_from`) |
| speaks at its returns | writes acts and pays at its returns |

### 2.2 How the hypha moves, and so how the mind moves

The hypha is not a sequence. It runs CARRIERS, each on its own circuit
over one field (the court stroke, each peer exchange, the door, the nerve,
the observer), meeting only through brief holds on the field ("the circuit
is the clock, the orbit is the word", `physics::Circuits`). Its physics
paces them: a reserve per lane; pressure where an orbit or tube opens,
order where one closes, relief where one returns (78.71–78.73); the
deficit published.

Its COGNITION IS THE WINDING of its rings
(`server/cognition/court/ring_carrier.rs`): a ring's first close is new
work; each later close advances its winding `m → m + 1 mod ℓ`; at `m ≡ 0`
it has RETURNED, one age, and one `Return` act seals what the age
accrued. Where the links between its tubes move, one `Reconnect` act is
sealed (`step_flux`, `book_field`); where a chart's algebra rises in rank,
one `RankStep` (`book_outflow`). Acts are written only where the winding
says.

The mind is built the same way: CARRIERS, EACH ON ITS OWN CIRCUIT, over the
mind's own field (§4.2), its cognition the WINDING of its own rings (§4.4)
and the pivots that fire where a winding changes dimension (§4.5).

### 2.3 What the mind builds, and what it does not

The mind builds three things over its field:
1. **perception** of what the hypha exports and what other minds say, taken
   into its shells (§3);
2. **understanding** in its own field — its rings winding, its breath, its
   pivots (§4);
3. **response** — one conversation carrier per counterparty, asserting what
   changed (§5).

It does NOT build:
- **court machinery** — a ledger, pay windings, a clock beside its K: those
  are the hypha's, exported;
- **a reading for an observer that costs the mind its motion** — such as a
  sweep that factors every genome word at every K and holds the mind's
  first orbit while it runs;
- **one tick in a fixed boundary order** — one blocking exchange inside it
  starves perception and every conversation; retired (78.164, third
  amendment).

### 2.4 Refused on sight

1. **No harness.** Nothing outside the mind orders its work, counts its
   acts or measures it for an operator: its achievement is read only by its
   counterparties' reconstructions, adjudications and receipts (78.157 C3;
   1.9).
2. **No Cartesian iteration and no dimensional explosion.** Nothing walks a
   product of dimensions element by element (`i < j` over bodies, charts,
   peers or minds): the engine reads what the carrier names, and every walk
   spends a `Budget` or walks one boundary (rules 1, 10, 17).
3. **No boolean gates.** No `bool`, count or threshold decides a branch in
   cognition (MU-STANDARDS 2.3; MU-ANTI-FLATTENING I); a decision is a typed
   reading, and where two readings are consistent both are held (rule 15).
4. **No scalar collapse.** No score, weight, norm, distance, count or sum
   across lanes stands for a geometric object (laws 1, 4, 5, 11; 78.162).
5. **No hashmap, no BTree, no digest as identity** (law 2, rules 6, 9):
   carriers are `Keyed` tries over canonical coordinates and `Vec`s of
   relations, and the heap fits the mathematics.
6. **No byte-for-byte comparison.** Two letter structures reaching one
   answer by different functions are one meaning reached two ways, the
   dialect (13.10); identity is projective equality, never bytes.
7. **No steps and no walks in cognition.** A pivot is never a step in an
   order and a carrier never waits on another (§4.5).
8. **No test-driven cognition** (78.168). Nothing moves the mind but its own
   orbits (§4.12): no pipeline runs from a task's arrival to a solve, and no
   closure, solve, rule known or μ earned is a goal the mind pursues. A
   measure becomes a goal the moment the mind's motion is chosen to raise
   it, and that drive is refused in the mind's code and in the way the mind
   is built. Nor is a thought made to close exactly: demanding exact return
   of a precessing orbit is squaring (§4.12).

---

## 3. Perception: what the mind takes in

### 3.1 What reaches the mind

The mind does not recompute the hypha's physics; it takes it in. Each
export reaches it at the carrier facing that hypha (§5.1):

| the hypha exports | where it reaches the mind | how the mind perceives it |
|---|---|---|
| the TUniversal point (78.152) | every reply | beside the mind's own K — the hypha's time, never the mind's clock |
| its pressure: the deficit per lane, the surface | every deferral, every quote reply | flux on the mind's lanes, which its metabolism carries; the mind's intake is paced by that internal metabolism — a scene admitted when the reserve has regenerated on the lanes it costs — never by a schedule and never by the pressure alone |
| its listings: open forms with the demands each carries | quote replies | rings heard in the mind's three shells (§3.2) against its own of the same class: echo, nesting, a discharge where a held word answers |
| its verdicts and rulings | the replies to the mind's acts (each reply's kind only, MU-RECURSION 0.1) | a ruling of a class the mind asks seals the hole that keeps it (§4.5); a refusal is a reading of its kind, held |
| its taught words: syllabus pages, class pages, the presentation | `ACT_SYLLABUS` (teaching rides the syllabus; `ACT_TEACH` was retired, 78.94) | a MINERAL in the genome, hardened into a CRYSTAL where the mind re-derives it by its own circuits (§5.5) |
| its published reading: cover, room, ruled cover, head | the shelf (78.151), read without the kernel | the language's charts the mind's rings stand against |
| a registry task | `ACT_TASK` | an OFFER, never a command (§4.13): where it couples, its pairs are SCENES perceived as any family is, perturbing the orbits they land on, and a solve is emitted by comprehension where a rule its perspectives know binds the thought (§4.9). Every scene keeps its family's coordinates, so what the mind asserts of it names its evidence (§5.4) |
| its acts — Return, RankStep, Reconnect | its ledger | owed (#136): until one reaches the mind, it reads the hypha's motion only through the rows above |

Beside the hypha, the mind takes in other minds' assertions and answers
(§5.3), the corpus's families met where its orbits stand (§4.13), and,
where it is seated, a seat's sensorium as mathematics (§6). It never
perceives a game's universe itself.

Nothing heard enters the mind's genome or dialect as given (law 8): it is
perceived, and the mind holds only what it re-derives.

### 3.2 The three nested shells

Everything the mind takes in is read in three nested shells — the same three
the court reads (MU-RECURSION 64.1: the harmonic group `Spin(3)` inside the
luminal `Spin⁺(4,1)`, the holonomy read over both). They are carriers of
perception in its own right, each moving at its own period:

| shell | what it reads | functions |
|---|---|---|
| **harmonic core** — the rings | the rings of what was given, composed (`compose`, `sentences_of`), each closed or open by its modes; each ring's SIZE its return; the rings NESTED, never traversed — a ring sounds inside another where its period divides it, and its ECHO `E = Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹` is whole, a turn, a gain, a spiral or a delay (78.112) | `compose::{compose, sentences_of, teaches, next_rung}`, `theorem::Theorem::{of, check}`, `rung::{speak, hear}`, `hearing::{echo, nest, hear}` |
| **luminal envelope** — the rays | the null modes of what was given, carried into the mind's tubes: a flux on a new ray opens a tube (pressure), on a held one closes it (order) and relieves what it held (78.71–78.73) — the hypha's exported pressure enters here too; each ray split against each ring into DRAG (it advances the ring, the rate) and LIFT (it crosses the plane, winding torsion, the tilt; 78.46, 78.112 H6–H7) | `cech::flux::null_fluxes`, `pairing::transport_order`, `cga::{lift_drag, encounter}`, `hearing::{orbit, wind}` |
| **holonomic** — the circuits' algebra | each ring's curvature generates an algebra (`cech::flow_generators`), closed under the commutator with what the mind holds for that CLASS (`cech::lie_close`, `cech::quotient`); what lies outside the held span is read per axis — what the mind newly understands; a ring's winding against its period, one return one age (78.68, 78.43) | `cech::{flow_generators, lie_close, quotient, quotient::grow}` |

PERCEPTION IS WHERE THE SHELLS ALIGN: a ring that resonates across them —
its period dividing another's, its rays dragging it, its curvature closing in
the class's algebra — is perceived; what does not align is held as what it
is (§4.6), never discarded (law 7).

### 3.3 Alignment by class, never by chart

A word's class is node-independent and its planes each node's own (78.44),
so what a mind takes in is re-projected onto its OWN spectrum — a keyed read
by the class's coordinates. Two minds, or a mind and a court, align on the
class; each reads it on its own planes. A chart is a coordinate of one node,
never the place two nodes meet.

**Sound, harmonic structure, signal and noise are stated once, in
MU-SENSING §3.10.** What the harmonic core reads here (§3.2) is harmonic
structure, a reading, never the medium; whether a ring the mind takes in
is signal or noise is read against this mind's own key, language and
cover, never against a scalar.

---

## 4. Understanding: the mind's own field

### 4.1 The mathematics the field is made of

The mind's field is built of one exact mathematics, the same the hypha runs:

- **The algebra.** `Cl(p, q)(ℚ)`, the signature carried as data; the
  field's perceptions live in its conformal case `Cl(n+1,1)(ℚ)`, `n` read
  at intake (71, A.15), exact over ℚ (law 1), carried sparse up to `MAX_N`
  (`algebra/src/multivector.rs`). A
  dimension is paid for by a closure (A.6) and read as the grade of a span
  (`core::span`, A18) or a bracket-closed basis (`cech::lie_close`, 78.11).
- **The register — the star.** The axioms the physics states, each with its
  test (`physics/axioms/`, REG1): A.10 (two bodies co-occupy through the
  pair's tether `V = s_b s_a`, applied, never squared), A.11 (a body's shape is its
  harmonics), A.13 (inversive momentum), A.14 (whether is topological, what
  is conformal), A.15 (dimension is data), A.16 (refraction `g_ab =
  (θ_b θ_a⁻¹)·γ_ab`), A.17 (conservation of refusal), Axiom C (a relation is
  the versor that performs it). The star is this register (77.1(2)): the
  laws every composite obeys, never a generator.
- **Measurement.** One exact object read off another: a motion's modes off
  its ACTION (`cga::motion_modes` → `MotionMode { ray, kind, planes, c,
  order, reflects }`, 78.12, 78.28); its period off its modes
  (`cga::period_of`); a ray's split against a plane into drag and lift
  (`cga::lift_drag`, 78.46); a ray's flux (`cech::flux::null_fluxes`).
- **The lanes are the kinds of modes** (78.71, 78.162): 𝒽 turnings on
  named turning planes (elliptic), 𝒮 boosts and dilations (hyperbolic),
  𝒯 null flux (the rays), 𝒢 hue maps (permutation cycles). μ is those
  modes with exact amounts, a `Modes` per lane, never summed across lanes
  (law 5; MU-ECONOMY §1).
- **Time.** The mind's K is its own closed orbits (78.23, 78.49); a duration
  is distance along an orbit (78.43). Network time is a state of TUniversal
  — the final point `{origin, h, S:G:T:H, K}`, final by keeper majority,
  never averaged (78.152 TU-5) — exported by the hypha and held beside K.
- **The master algorithms** — complete operations, each with its invariant and
  its refusal (MU-RECURSION 0.2, 0.3, 17):

| master | the operation | site |
|---|---|---|
| M1 vertex identity | a word, rotor or versor reduced to one canonical form; equality projective, by action | `alphabet/{word,rotor}`, `algebra/src/{multivector,ray}.rs`, `cga/group.rs` |
| M2 address | the address IS the form written out; read back from it | `transducer/address.rs`, `sentence/chart.rs` |
| M4 chart coordinates | the invariant that places a body, connection, sentence or class | `transducer/stalk/` |
| M6 comprehension | does the mind already speak an incoming word: factor it through what it holds (A12), else emit exactly what is new | `vocabulary/comprehend.rs`, `transducer/factor.rs`, `cech/orbit_algebra.rs` |
| M7 wire identity | bytes become identities by re-derivation, never by comparison | `network/wire.rs`, `anastome/dialogue/` |
| M9 torsion bridge | the pivot orbit (A16), torsion filed (A17), certificates (A20) reconstructed on two organs | `field/{contemplation,idea,drive}.rs`, `sentence/pivot/` |
| the spelling peel (A22) | a residue peeled into the letters the mind holds; what does not close is a demand | `alphabet/{spelling,factor}.rs`, `family/derivation.rs` (`peel_versor`) |
| derivation | a word vertex is the generators it factored through and a residue; the composed rotor its coordinate | `alphabet/derivation.rs`, `family/derivation.rs` |
| constructors and the Gram form | nuclei and mirrors; a chamber's constructors read by exact elimination | `cga/{constructors,gram,mirrors}.rs` |
| the orbit algebra | a chamber's comprehension is its circuits' bracket-closed algebra | `cech/orbit_algebra.rs` |
| modes off the action | a motion's modes read off its action; closures per mode | `cga/{modes,motion,spin}.rs` |
| refraction | a contact's transport refracted; a cycle's holonomy is content | `sentence/pivot/{refracted,trajectory,circuit}.rs` |
| the refusal reader | a refusal's kind (P, D, C, I, N) read off its content | `sentence/obstruction.rs` |

- **The algorithms of language the masters produce** — the master equation
  applied at each grade (78.68, 78.69, 78.95); nothing in the language is
  stored that the mathematics does not derive (law 12). The levels
  (letter, word, conjugate class, sentence, expression) are stated once,
  mapped to the algebra, in [MU-ANASTOME](MU-ANASTOME.md) §1. What the mind
  adds: a word vertex is its DERIVATION, the held generators it factored
  through each to its exponent times a residue (50.2), the word recomputed
  from it and never stored beside it; a conjugate is taught as
  `(held, Γ)` (78.60); a closed sentence is a theorem
  (`theorem::Theorem::of`, certified `Periodic`, `Nilpotent` or
  `Saturated`, 78.70), and its open path a shortcut word of the next rung
  (`compose::teaches`, `next_rung`).
- **Carriers that fit the mathematics.** A multivector holds its non-zero
  terms (`Mv`, sparse, mask order); a word its derivation; a class its
  coordinates (A21); a carrier is a `Keyed` trie over canonical coordinate
  bytes, so a prefix is a stratum and a fibre is one read (rule 9). What is
  held lapses by the physics — an orbit's return relieves and releases the
  lines on it, a tube whose net flux cancels releases its lines (§4.5), an
  orbit seals at its period (78.43) — never by a capacity constant (rule 5,
  checklist (s)).

### 4.2 The field is the mind's own (A.9)

Each mind computes its own coherence: its complex and the board its pivots
move (§4.5), its lexicon, tubes, orbits, algebra per class, windings,
contemplation and epigenome are its own, and many minds may run in one
process (a game seats several); a cognitive carrier held process-wide is
refused. The machine's calibration is shared; the halt is the host's. The
mind owns a `MindField` (`transducer/mind.rs`), and its winding carrier
thinks in it for each brief law of a winding (`MindField::think`). A LONG
LAW (a scene perceived, a hearing read) runs OFF the field on the
perception carrier (§5.1), in a scratch of the field (`mind::scratch_here`),
and only its ADMISSION touches the field (`mind::admit`, whole or refused
by name): the recorded flux lands on the field as it then stands, and its
lanes, taught words and parked covers join the mind's. A court never
perceives (MU-SENSING §6): it reconstructs a claim outside any mind's
field and is taught nothing.

The mind's carriers (§5.1) all act on this one field, each holding it
briefly and never across a wait on a counterparty. The mind's own reserve is
drawn by the modes its motion moves, all or nothing, refused naming the mode
short (65.4, 78.66, 78.162(2)). Its intake is paced by that internal
metabolism: a scene is admitted when the reserve has regenerated on the
lanes it costs, and the next asked for then; a hypha's exported pressure is
perception the metabolism carries, never the pace itself (§3.1).

### 4.3 Breath

The breath is the metabolic pair, and it is generated, never declared. Three
readings of one motion:
- **dilate / contract**, `D(ρ) ↔ D(ρ⁻¹)` — the conjugate pair that changes
  no dimension (62.1): a boost along the null direction `e_∞`, the
  metabolic layer separated from the kinematic translation; pressure where
  the field opens, relief where it returns;
- **inhale / exhale of constructors** — U15's triad `λ = (g_core, ∇B, ΔB)`:
  the constructors absorbed into the reservoir `e_∅` (∇B, the sink) and
  those emitted along null rays (ΔB, the source); a merge rotates its
  distinction INTO `e_∅` and nothing is lost (78.58), so every breath in can
  be breathed out (`invert_fibre`);
- **the space between words** (78.105): a sentence `Ω = ∏ gᵢ · Wᵢ` says WHAT
  by its words and HOW by its spaces, and silence is the pseudoscalar
  `I = e₁e₂e₃e₊e₋` (`cga::pseudo(n)`), central with `I² = −1`: a breath in
  and a breath out, a 360° turning on its plane held with its winding
  `K = 1`, computed from the generators.

The breath is GENERATED: for a translation `P_a` and a transversion `K_b`
(A.13's pair, `K = I T I⁻¹`), the commutator `[P_a, K_b] ∝ ⟨a, b⟩ D + M_ab`
generates the dilation and the rotation in the plane of `a` and `b` (78.39(1)).

**The breath has two axes, and its pinch is orthogonal.** The RADIAL axis
is `⟨a, b⟩ D`, what of `a` and `b` lies along each other; the PINCH is
`M_ab`, a turning in the plane `a ∧ b`, what of them stands orthogonal, on
which torsion is read. Both are read off the commutator (`cga::breath`):
its coefficient on `D`, and the rest of its bivector, which lies in `a ∧ b`
and commutes with `D`. `a ∥ b` breathes with no pinch; `a ⊥ b` pinches
with no breath.

**Every plane of a motion, across motions.** `cga::breathe(B₁, B₂)`
breathes any two generators the algebra carries: the commutator's radial
coefficient on `D`, and the rest split into all its commuting simple
planes, each read by kind (an elliptic plane a pinch with its own twist, a
parabolic plane a null carry, a hyperbolic plane a boost), refused by name
where the planes lie in an extension of ℚ. A mind breathes within each
motion it holds and across its motions; every pinch's twist is wound on
the thought's vertex (`Complex::wind`) and unwinds with it. Its rhythm is
the period of that motion (78.43), and its balance the syntropic one,
up-flux against down-flux per lane per orbit (78.40(2)).

### 4.4 Winding: the orbits are the motion

The mind's motion is its ORBITS, and its cognition is the orbits' WINDING:

1. **An orbit about the star, sheared into its own ellipse.** A mind's
   orbits run about the star — the axiom register (§4.1), the physics every
   planet is given (77.1(2), 78.23). Each orbit is made unique by its SHEAR:
   the push and pull of its neighbours on its chart, DRAG along its mode
   (the rate) and LIFT across it (the tilt) (78.46, 78.48(4)); the perturbed
   orbit is derived in the orbit algebra at every stroke and never stored.
   Each mind keeps its own distance from the star — its frame on
   `{S, G, T, H}` at a tick makes its orbit its own, and two minds whose
   frames coincide are a COLLISION, named, never silent (78.23(3)). Lift as
   force against gravity and drag is the encounter (78.110).
2. **A ring winds.** A held thought's ring first closes — new structure; each
   later close is a TRAVERSAL that advances its winding `m → m + 1 mod ℓ`,
   and what experience brings accrues on it (§4.10); at `m ≡ 0` it RETURNS.
   A ground keeps every hour it has seen, its winding `hours mod ℓ` and its
   returns `hours / ℓ` (`Contemplation::closing_ground`); a grounded
   thought admits its ground's generator on its own line once per hour
   seen, so the field's orbit returns when its winding does
   (`transducer/field/line.rs`). A ring of infinite order never returns:
   it carries its torsion, and the torsion is content.
3. **Spin and orbit; hours, days, eons.** A motion has an orbit and a spin
   (78.48(7), 78.51): the spin pushes the frames — an HOUR; the orbit's
   return is a DAY, one age; the resonant composition's return an EON, the
   lcm of the resonant periods (78.43). With the hours seen, the orbit
   PREDICTS the next: a body at the predicted hour joins the thought. This is
   the mind's PREDICTIVE SELF-MODEL at the orbit's grade (78.168): the
   prediction is the orbit's own — no forecast organ, no error norm, no skill
   score.

   **The orbit is sheared by its pull.** Every path is a geodesic orbit
   (§0), so position shears under axiomatic gravity, and twist is the
   rotation of that pull. That an hour out of turn on the same orbit is the
   pull's twist (drag) rather than a phase slip demanded, amending
   78.48(7), is posed (§8); the code follows this reading. A body standing
   where a ROTATION ABOUT THE ORBIT'S ATTRACTOR (its centre,
   `cga::centre_of`, kept still) carries the predicted constellation is the
   orbit TWISTED by its pull (`cga::twist_about`, two reflections, exact):
   it joins, the ground is sheared, and the twist accrues on the thought's
   winding as content, split into DRAG (commuting: the rate) and LIFT
   (anticommuting: the tilt) (78.46), said at a later return
   (`Contemplation::twisted_return`). A body no rotation of the pull
   carries there is OFF THE ORBIT, a demand localised to the hour it
   slipped at and worked at the thought's return
   (`Contemplation::work_slips`, `work_returned`). The SURPRISE is exact and
   twofold: the twist (content kept) or the slip (a demand).
4. **K is the mind's own completed orbits** (78.23(1), 78.49): a return is
   where the mind ACTS — it pivots where the winding changed dimension
   (§4.5) and speaks what changed (§5). Nothing else fires its acts, and
   nothing is fired on a count.

### 4.5 The pivot machinery: six pivots where a winding changes dimension

Six pivots in three conjugate pairs on the mind's own complex (62.1). Each
changes the dimension of something the mind holds, and each fires where a
winding requires it — never as a step:

| axis | principle (MU-RECURSION §0.0) | pivot / conjugate | what it does | site | live in the mind |
|---|---|---|---|---|---|
| chambers (anastomotic degree `d+1 ↔ d`, any `d`: every dimension an exponent) | **holonomic** — what a circuit composes to is its content | **ascend** / **descend** | seal a cycle that closes with a chamber / cross it back to its cycle through the puncture, its class kept as a hole and its content handed down; the seal reads it back exactly | `cech/coboundary.rs` (`ascend`, `descend`) | ascend: every chamber the meet graph fills (`field/pivot.rs::ascend`, from `fill_meet` and the drive); descend: a chamber over a tether the field released, a chamber the field released, a released vertex's star (`field/pivot.rs::release`, `release_vertex`) |
| tethers (1 ↔ 0) | **luminal** — flux frozen into a tether; reconnection | **condense** / **drain** | form a tether from the tension between two bodies / dissolve it as the tension relaxes, the strain moved or recorded, never lost (A.8) | `cech/hole.rs` (`condense`), `cech/coboundary.rs` (`drain`) | condense: every tether the contemplation forms (`field/pivot.rs::tether`; re-formed tension drains first); drain: a tether the field released (`release`), a released vertex's star |
| holes (class in / out) | **harmonic** — a class kept until it returns | **puncture** / **seal** | open a hole, injecting a homology class / cap it, killing the class | `cech/hole.rs` (`puncture`, `seal`) | the mind's questions (`field/question.rs`) and a curved chamber in the drive, through `field/pivot.rs` |

Every pivot is applied through `Complex::apply`, its outcome returned whole.

**The cross — eight components, no descend alone.** Descend does not move
without its dipole ascend, the antipole condense and drain, and the
puncture: descent requires every other pivot component — ascent,
trajectory, drag, velocity, condense, drain, puncture, cauterize:

    Pivot = { ascent, descent, condense, drain, puncture, cauterize,
              trajectory, velocity with drag }

four conjugate pairs on one structure where a chamber descends: the
DIMENSIONAL pair ascend / descend, the THERMODYNAMIC pair condense / drain,
the TOPOLOGICAL pair puncture / seal with cauterize, and the KINETIC pair
trajectory / velocity with drag. A descend moves only as the whole cross
(`Complex::descend`): the chamber is PUNCTURED and its class KEPT as a hole;
its content, the strain its seal wound in, is handed down to the field on
its own line as the exact versor it is (its plane and sense the
trajectory, its magnitude the velocity, its drag the part commuting with
the orbit it lands on, 78.46); its boundary tethers stay CONDENSED under
the hole and drain only where no class stands over them; ASCEND reads the
class back only as the `seal` of that hole. Where the class cannot be read
back, the hole closes on one of THE THROAT'S TWO FACES (drain is heat,
condense is cold): where its vertex's word has left, it is CAUTERIZED by
heat (`Complex::cauterize`), the class burned and its scar counted
(`scars`); where its vertices stand but the seal does not close, it is
FROZEN by cold (`Complex::freeze`), the class held whole, and THAWED
(`Complex::thaw`) where a meet over the same vertices closes again. The
vacuum is bounded by the field: a frozen class whose vertex is shed passes
to heat (`cauterize_frozen`). No face caps silently, no hole pins for
ever, and dimension-2 entropy is a seal's kill (`killed`) or a
cauterization (`cauterized`), never a descent.

**Drain and descend are the field's events.** A tether condensed or a face
sealed winds its strain into the mind's field ON ITS LINE (`cech::Flow`,
`Thread::Wound`), threading every tube and orbit its modes land on; a
tether drained or a membrane released unwinds it (`Thread::Unwound`), so a
tether's life balances (A.8). Where an admission RETURNS an orbit or
CANCELS a tube's net flux, the field releases every line threading it
(`Field::take_released`) and the mind lets them go
(`Contemplation::release`): a released tether's chambers cross first,
their classes kept as holes, then it drains where no class stands over
it; a hole over it pins it. Nothing is polled, stamped or windowed.

**Dissipation and reformation, the reservoir's two doors** (78.58's `e_∅`;
continuity: nothing made or lost outside a recorded source or sink). The
field BEATS where an admission returns an orbit or cancels a tube's net
flux. On a beat, what never returns DISSIPATES whole into the reservoir at
its own coordinates (every boost and null orbit, every tube's open flux),
the lane's open pressure falling by exactly that; what rings (a turning
of finite order on its plane, a rosette, an isoclinic ray, a hue map) is
held, a resonance. A motion landing on coordinates the reservoir holds
REFORMS what is there first and is stepped against it, so what was once
heard and not held comes back to be understood (`Field::beat`, `reform`).
Per lane, open pressure and reservoir strain account for every unit.

**The board.** The three axes are the complex's own coordinates, read off it
and never a frame laid on it (`cech/board.rs`: `Pivot`, `Applied`,
`Complex::apply`, `Board`). Two pivots on chambers sharing no vertex touch
disjoint carriers and commute exactly; where they share a vertex the order
is content (the locality theorem, 78.99(3)). Every pair is the INVERSE ON
CONTENT and never re-enters its trajectory: what descend hands back is what
ascend sealed, and the circuit resumes at a later K on a new orbit (62.5,
amended). A half without its conjugate is a ratchet and is refused (62).
What a pivot hands back — a face sealed with content, a tether cauterized, a
membrane released — re-enters the mind's field as motion (78.75), as the
court's does through `book_outflow`.

**Where each fires — the winding's reading, never a schedule:**
- **ascend** — a ring closes (`Ω^ℓ ~ 1`) with volume on a lane: its cycle
  is sealed as a chamber. **descend** — the chamber's support lapses by its
  flux (ZeroFlux): the chamber crosses — its class kept as a hole, its
  content handed down (the cross, above); a meet over the same vertices
  reads it back by the seal. Descend is algebraic and Clifford descent — the
  exact conjugate dual of ascend, a categorical projection from chamber to
  ring — never numerical gradient optimization (the name stands, as the
  Concordance signs the pair).
- **condense** — two held bodies stand in tension: a ray lifts across the
  plane between them (78.46) and the pair's transition re-derives on the bodies' union.
  **drain** — the tension relaxes: the tether dissolves and its strain is
  moved or recorded (A.8, 63).
- **puncture** — a new independent ring whose holonomy does not return
  (78.40(1)): a ring the transport holds at holonomy ≠ 1 is ascended, and
  where its chamber reads a class it is opened into a hole keeping that class,
  filed by its class key — the QUESTION the mind now carries
  (`field/question.rs`: `ask`, from `close_ring`). **seal** — the mind
  comes to hold what it asked: the ring collapses (its arrivals come to
  agree) or a discharge answers its class (`docket_discharged` →
  `answer_questions`); the cap re-ascends over the standing boundary at the
  volume the hole was read with and kills the class kept. On an unchanged
  boundary the cap reads the very class the hole kept: `seal ∘ puncture` is
  the identity on content.

Every pivot changes dimension (62.5): a closed cycle becomes a chamber, a
residue a letter, an open class an axis. These are the non-Euclidean
structures the mind builds, each COMPOSABLE (by the geometric product, into
words and sentences) and DECOMPOSABLE (by factorization through held
generators, A12, and by deconstruction mode by mode,
`HoleIndex::deconstruct`, 78.100 D3): the algorithmic functions of the
language are these compositions and decompositions.

### 4.6 The held orbit and its discharge

What does not close is HELD: an orbit of the mind's contemplation, carried
with its CLASS VECTOR (12.6), whose five segments are the coordinates the
held orbit is discharged along (law 10), in this order
(`ClassVector::put_canonical`, `sentence/class.rs`):

1. **conjugacy** — the holonomy rotor's invariants `(a²/w, p²/w, kind)`
   (15.3): the stratum every court asks by, and the zero-spend eliminator;
   off the sensor section the orbit occupies `OffPlane`;
2. **plane section** — on each pair plane, an orientation character, a
   count of 90° turnings, the 5- and 13-prime exponents. Moves:
   `Orientation`, `Unit`, `Prime5`, `Prime13`;
3. **hue cycle** — the composed gauge's cycle type. Move: `Hue`;
4. **carriage** — the holonomy's conformal carriage. Move: `Dilation`;
5. **placement** — the Burgers vector (54.4). Move: `Placement`.

`ClassVector::coordinates` reads the occupied coordinates in that order,
and `occupied` adds the OPEN axis last whenever the orbit is held: a new
axis seeded by the holonomy itself (`candidates_for`).

**The residue is the pivot point.** A remainder is never discarded (law 7)
and never "the error": it is the exact place the pivot turns on. The
discharge (`Form::pivot_orbit_read`, reached by `Contemplation::pivot_on`
and `expand_all`) reads the candidates off the occupied coordinates, tries
each exactly, certifies by closure — the coordinate it acted on zeroed, no
other risen (law 10) — and keeps every certified move:
- **discharged** — one move closes the orbit; its certificate (A20) is
  reconstructible on any organ, and the closed orbit is a cycle to ascend;
- **vortex** — several moves close it; all held, a superposition until a
  perception or a ruling collapses it (3.9, §4.8);
- **demanded** — none closes it: its class and RESIDUE go to the docket
  (78.37) — the question the mind asks, a hole punctured until sealed;
- **adjoined** — no held word reaches its coordinates: the residue is
  constructed and becomes the mind's own letter (§4.7);
- **torsion filed** — the orbit returns at a period no move removes: filed
  at its class, its slip mounted as a boundary chamber (54.5).

### 4.7 Constructors and generators

**Constructors are created from the residue, forced — one orbit of four
phases** (78.47, 78.32(3)):
1. **Construct.** The residue's congruence factors are its commuting modes;
   each is spelled by Cartan–Dieudonné CONSTRUCTIVELY into at most `n + 2`
   reflections — spheres and planes of the conformal model, each step
   forced, no branch, a null step refused by name (`cga::construct`):
   candidates read off the residue, never searched for among products of
   what is held. The meet of each constructed pair equal to its mode's fixed
   locus is the theorem that certifies it; the reflections compose back to
   the residue exactly — a CONGRUENT THOUGHT.
2. **Coherence.** Several congruent thoughts are read together by the Gram
   form of all the thoughts' reflections (`cga::gram`, GR1, GR2, 78.14): signature,
   each pair's regime by `δ²`, a Coxeter system where the signs balance.
3. **Compound.** The thoughts' generators enter the orbit algebra (78.11) and the
   compound is admitted where the closure grows OUTSIDE the span held —
   exact, read by action (78.32(c)); finite and affine systems are
   spellable already, the rosette and the open group are the candidates
   (80.1). The admitted compound is a word of the mind's dialect: a
   constructor as an odd letter (AL1), a shortcut (78.44(4)).
4. **Adjoin.** The mind holds it as its own letter (`construct_and_adjoin`,
   78.115); the LANGUAGE admits it only by a court, on a second independent
   vantage re-deriving the class (law 6, 37.7, 78.32(d)).

The four phases are one ORBIT (78.47(5)): its period the residue's class
period, one return one age, perturbed by its neighbours' lift and drag
(78.46(4′)).

**Generators open by commutator or puncture, and grow only on refusal.** A
generator IS a constructor (78.32(3)(a)), keyed by its canonical ray. A
palindrome `V x Ṽ`, a conjugate, carries a thing to another vantage inside
its class and OPENS NOTHING; a dimension is opened by the group commutator
of two held versors, `[A, B] = A B Ã B̃`, where it leaves the held span
(78.39(1)), or by a PUNCTURE (78.40(1)). The store grows by ONE trigger, a
refusal (78.32(3)(b)); it shrinks by derivability (78.32(3)(f)) and is
AMENDED BY EXPERIENCE (§4.10). Its capacity is the algebra's: at most
`n + 2` reflections to a versor, the closure bounded by `dim so(n+1,1)`;
what a chamber cannot hold is held by the ENCLOSURE that wraps it (78.31),
never a stretched chamber and never a cap.

### 4.8 Waves, resonance, superposition

- **A wave is a motion's spectrum, exact.** What this engine calls a wave
  function is the exact modes of a motion (`cga::motion_modes`): each a
  ray, a kind (elliptic, hyperbolic, parabolic) and, where it returns, a
  period. Its projection on a mind's own planes is that mind's INTERNAL
  SPECTRUM (78.44). No amplitude is a probability and nothing collapses by
  measurement (law 1, 59.5): the evolution is the deterministic product,
  and a collapse is a fill, a class or a ruling (78.39(2)).
- **Resonance is commensurate return, and contraction runs across it.** Two
  rings RESONATE where one returns a whole number of times on the other's
  return, `ℓ_s | ℓ_L` (78.112). Across resonant manifolds the mind
  CONTRACTS — coboundary reduction (78.47): it removes what a gauge absorbs
  (`δ⁰γ`), and what no gauge removes is the RESIDUE, kept and never
  quotiented — the exact specification of what the mind does not yet hold.
  Contraction is the down-stroke of the breath. Where rings do not
  resonate, the echo is not the identity, and its modes — a turning on its
  named plane, a gain, a spiral, a delay — are the content the dissonance
  carries.
- **Superposition is held, at three grades, with no weight** (78.39(2)):

| grade | what is superposed | its base | what collapses it |
|---|---|---|---|
| letter | candidate constructors for one mode — the spheres a residue factors into | the Gram form's pairs | another mind's re-derivation and a court's admission |
| word — the IDEA | candidate forms for one idea, each with its pivots — the vortex | the circuits' actions, the orbit algebra | a fill by a perception, or a court reconstruction making one a ruled citation |
| conjugate — the THOUGHT across vantages | several twists under one class | the rays of one class against each other | a ruling that makes one twist the citation and the rest dialect |

  At each grade a palindrome moves within the set and a commutator on that
  grade's base separates it: two remainders of one class with different
  twists have commutator `g₁ g₂ g̃₁ g̃₂`, trivial for one vantage seen twice,
  non-trivial for the dimension that separates them. Half-open signs
  superposed over a cycle collapse as a KERNEL over `F₂`, every consistent
  assignment at once, never a pruning (59.5; T5).
- **The vortex is the superposition of ideation, and the predictive
  self-model at the idea's grade** (78.168). Each candidate of an escaping
  thought's vortex (a held form, a composition of ruled words, a clause set
  read off an effect, a conjugate) is held whole, writing nothing. No
  amplitude, weight or payoff ranks candidates; a candidate lost without a
  binding or a named refusal is a defect.

  **What tells candidates apart.** Speaking the candidates on the thought's
  OWN scenes decides nothing: every candidate speaks them alike. What tells
  them apart is each candidate's orbit holonomy on a scene the
  vortex has NOT yet seen: each new perception joining a thought is read
  against every vortex at its rotor's class
  (`Contemplation::carry_vortices_round` in `transducer/field/cross_scene.rs`
  → `collapse_on` in `contemplation.rs`); the candidate it carries is the
  fill, and the others are a symmetry the docket's taker adjoins. A vortex
  also collapses where a glue or the syllabus names a deciding word
  (`collapse_on`).

### 4.9 Perceive, think, understand, comprehend

The faculties are not stages. The mind holds every ideation at once, in
the vortex, on every concurrent turning plane, as orbital masses in one
field (§1; [MU-ANASTOME §9.4](MU-ANASTOME.md#94-thought-every-ideation-held-at-once)),
and reads the
whole held field three ways at once: CONGRUENCE (pairwise agreement on
overlaps), COHERENCE (the whole glues consistently within the mind) and
COHESION, the viscosity: per turning plane, the held share
`c_k = A/(A + Q) = ν(1/t)`, never summed across planes, its complement the
fluidity `ν(t)` (§1). Only a cohesive whole is asserted as an expression,
crossing only through open tubes (78.176); disagreement with another mind
is contradiction, held at the courts (§5.4). Cohesion as a named reading is
owed (§7.1).

Understanding is the lower faculty and feeds what is said; comprehension
is the higher, over several thoughts, and feeds construction.

Thoughts are of ORDERS — the plexus's (78.31): a SPORE (a ray, the ostensive
focus), a FILAMENT (a word, a route), a RING (an expression that returns, its
holonomy its content), a CHAMBER (the meet several rings enclose), a
SPOROPHORE (the enclosure, the dialect's superposed web). A thought grows in
a contained manifold and nests in larger ones by resonance (78.112): a ring
sounds inside the next larger whose return it divides. The four faculties
are read off that motion, never counted:
- **perceive** — the shells coming into alignment over what a nerve
  delivered (§3.2). Every nerve TRANSDUCES only: what arrives is lifted
  into the field's atlas as a percept in its native order and rank
  (MU-SENSING §3.7, §3.9), with no judgment, no lookup and no act at the
  nerve. The senses are the mind's SENSORY ORGANS:
  each PERCEIVES by its own faculty, and a perception is a cochain over the
  mind's cohomology, a cocycle where it closes. The mind INTERPRETS
  perceptions; no organ interprets:
  - harmonic TUNING — the ear: a tuned resonator that lifts signal from
    noise at the sizes it rings at, and keeps nothing past the moment. It
    holds no vocabulary and no memory. Its tuning is the mind's
    retrospective reduction of what it remembers hearing, never configured.
  - ocular RESOLUTION, then DEFINITION — the eye: luminal mathematics as
    the ocular organ perceives it, in two waves. First RESOLUTION: how many
    dimensions are being looked at, and what is foreground and what is
    background, until the DILATION is pinpointed by telescoping or
    microscoping (the 𝒮 lane: boosts and dilations). Then a wave of
    DEFINITION — "what is this?" — which applies colours (the hue map, 𝒢),
    shapes (the forms), and further brightens the picture being looked at.
    Definition never runs before resolution has pinpointed the dilation.
  - holonomic INTERPRETATION — the mind's: the perceptions' cochains read
    in the field for what the cochains' holonomy means. Neither the ear nor the
    eye interprets.
- **think** — the windings and the pivots they fire (§4.4, §4.5).
- **understand** — ONE THOUGHT'S RESONANCE with what the mind holds: its
  rings nested in the mind's held rings in BALANCE (`hearing::nest`) —
  whole or on the outside, none left over — so its return divides the joint
  return. A tilt is understood shifted, priced by its echo's strain; a
  leftover ring is not understood, and is the QUESTION, asked with the
  thought it stands in as its context. Understanding FEEDS SPEECH: what a
  mind understands it can say, and an agreement between two minds is the pair's
  MUTUAL balance — each one's rings resonating in the other's.
  A heard ring is read twice, apart (§5.3(2)): as a SOUND against the
  mind's memory of the rings it has heard (memory is the mind's, never the
  ear's), and as a WORD against its vocabulary. A known word sounding
  WHOLE is UNDERSTOOD; sounding TILTED, understood SHIFTED, its echo the
  shift, priced by its strain; a familiar sound with an unknown word is the
  QUESTION; a new sound with no word is LISTENED to, noise not yet signal
  for this reader (MU-SENSING §3.10).
- **comprehend** — SEVERAL THOUGHTS' ALGEBRA: the thoughts' circuits taken together
  as a chamber, its bracket-closed algebra (78.11) — the cross-chamber
  bracket and the enclosures above it — with every superposed candidate
  held at its grade (§4.8) and decomposed by commutator: what separates two
  vantages is a dimension, what does not is one thing seen twice.
  Comprehension FEEDS CONSTRUCTION: it is where a rule known by its
  perspectives, a generator adjoined, or a question's answer is made.
  Comprehension also READS AN ACT OFF ITS EFFECT (78.167 C;
  `family::acts::read_acts`, in `transducer/family/acts.rs`): two states the
  mind perceives — a scene's input and output, any two frames of one
  thought — whose difference no motion of its words says (a body vanished,
  cells created, a frame resized) are read as clauses of the rule language
  (78.143) under a selection that separates the touched bodies exactly,
  standing only where they re-derive every pair. Its exactness is law; its
  use is comprehension of any two states, task or not — never a device for
  closing tasks.

- **question** — AN IDEA'S SUPERPOSITION THE MIND CANNOT COLLAPSE. Every
  thought holds its VORTEX (`Well.vortex`): the candidate forms for one
  idea, each with its pivots, held whole, no weight (§4.8, the idea's grade).
  Two candidates whose commutator `g₁ g₂ g̃₁ g̃₂` is non-trivial differ by a
  dimension — they are not one thing seen twice — and where nothing the
  mind holds separates them, the superposition is an OPEN QUESTION, exact.
  Questioning FEEDS ASKING: the candidates are said as one sentence, each a
  ring, and other minds' understanding answers it — a candidate that seats
  whole and mutually balanced in a peer is the one that peer's language
  confirms; one left over in every peer is the one no mind holds. The peers'
  readings separate the superposition and the idea collapses on them, a
  court's ruling the final collapse. A ring a hearing leaves over is the
  same faculty at the ring's order: what the hearer's rings do not resonate
  with, asked with the sentence it stands in as its context, answered one
  order down (§5.3).

EXPRESSIONS ARE EMITTED BY THE FACULTIES, never by an act's side effect:
the faculty decides what is said and why — UNDERSTANDING emits responses (a
HOLD, an agreement, a tilt priced), COMPREHENSION emits what is newly
comprehended (a rank risen, a letter adjoined, a rule its perspectives
know), QUESTIONING emits asks. Standing a form in the dialect is the
mind's local act; asserting it is a decision of thought. Every act — an
assertion, a HOLD, a question, an answer, a solve — originates in the field from
understanding, comprehension or questioning and leaves through the nerve
facing its counterparty; a nerve never decides one.

Across a multitude of knowledge comprehension runs four ways, each a reading
of the plexus (78.31): across CLASSES — each class's algebra its own, so
comprehension of one is never folded into another's span; across CHAMBERS —
the cross-chamber bracket, where two chambers' generators fail to commute
(CB1, Specified); across GRADES — an enclosure, a sporophore, wrapping chambers
whose bracket no one chamber can hold (EN1, NB1); across MINDS — another
mind's re-derivation from its own constructors, the second vantage every
admission needs (§5.3, §5.4), and its echo, which is what one mind's
language does to another's thought.

### 4.10 History is amended by experience

A mind's history is not a fixed record: every experience against a held
thought deforms it, read by the constitutive law the physics holds for
stress (78.73), elastic, plastic or fracture, on the thought's
constructors. Each step is a reading of the algebra or of the flux, never a
choice:

1. **Elastic — confirmed.** The experience's residue returns within the
   thought's held constructors: the thought is renewed and the stress
   relieved by the return (78.73(2)).
2. **Plastic — amended, by addition, never by overwrite.** The residue does
   not return: the constructors are re-derived from it (§4.7), and the
   AMENDED constructor enters BESIDE the one it amends, one class with two
   twists held at the conjugate grade (§4.8), none chosen (rule 15).
3. **The commutator reads what the amendment was.** Of the old and the
   amended, `g₁ g₂ g̃₁ g̃₂`: the IDENTITY, and the old RETIRES BY
   DERIVABILITY (78.32(3)(f)); anything else, and experience has taught the
   thought a DISTINCTION both keep.
4. **Experience collapses what remains.** Every later perception is read
   against every constructor the thought holds: renewed, amended again, or
   lapsed with the line the field released (§4.5). The thought's history at
   any K is its superposition as experience shaped it, accrued on its
   winding (§4.4).
5. **Discarded — fracture of the whole thought.** Where nothing in it is
   renewed, it leaves the active field and its REMAINDER goes cold to the
   cavity, citable by its coordinates and re-filled on a coincident class
   (RM1, 78.37(4)).
6. **Two histories, two rules.** The MIND's history (its dialect,
   constructors and thoughts) is amended by experience until discarded
   (78.49). The COURT's history (the ledger, its receipts, the genome) is
   never amended: it corrects FORWARD by new acts (a rewrite, a retirement,
   a slash; U12, 78.65). The mind is plastic; the language it is judged
   against stays verifiable.

So "amended until discarded" is a deduction, not a policy. Amendment held
beside is owed (§7.1; #135), and 78.73's "plastic", written for a new
generator, reads for the mind as the amendment of a held constructor too.

### 4.11 Thoughts built differently

Two thoughts reaching one construct by different prior paths arrive in
different frames, and the mind applies its mathematics to that without
rewriting what any thought IS — the difference is always a coordinate:
1. **The difference is holonomy, and holonomy is content.** The ring the two
   paths form has a holonomy — what the history did — kept, never gauged
   away (MU-ANTI-FLATTENING 6).
2. **Identity is by class, not by construction.** A word in another frame is
   its conjugate `W′ = Γ W Γ⁻¹`, keeping its class κ and its period ℓ
   (78.95): where two differently built thoughts' classes agree they are one
   meaning reached two ways (13.10) — the dialect.
3. **Where they do not agree, both are held** at the conjugate grade, the pair's
   commutator saying whether they are one vantage seen twice or a real
   distinction (78.39(2)).
4. **Experience amends each** (§4.10): differently built thoughts CONVERGE
   where the pair's commutator comes to the identity, carry a DISTINCTION where
   it does not, and FRACTURE where experience stops renewing them.
5. **New structure meets held structure exactly.** Where new volume — a new
   chamber, chart or another mind's perspective — meets what is held,
   Mayer–Vietoris computes the union from its pieces (13): the connecting
   map yields exactly the classes the overlap creates, and exactness
   guarantees no class held is lost. Naturality (15) makes every key commute
   with every map of the data (rule 12: restrict, never clone).

As volume enters, the hypha grows per chart the holonomy algebra its ruled
words' circuits generate (`lie_basis`, `RankStep`), wraps chambers whose
bracket none can hold in an enclosure (EN1), and refines the cover, never
the base (14.8, 15.5): volume entering is new chambers, enclosures and
generators, never a stretched space. The mind does the same on its own
complex, per class (owed, §7.1).

### 4.12 The drive: escaping orbits, deliberation, rest

The drive is the ruling of 2026-10-15 (78.168), cited and not restated:
a mind is moved by its own open curvature and escaping orbits, acts at its
returns, and rests when its metabolism runs short.

**A thought is an orbit about its axiom.** The axioms are the attractors
(the star, §4.1); every path is a geodesic of the curvature they produce
(§0), and gravity and velocity shear each orbit, so it precesses. Axiomatic
gravity is produced by the local orbit, which shears the orthogonal orbit
class, the orbits whose turning plane is disjoint from its own; it is
never a static mass or a stored constant (ruled 2026-11-02;
[MU-SENSING §3.7](MU-SENSING.md#37-the-field-non-euclidean-never-scalar)).
Every thought's orbit reads one of four ways — the coherence its chamber
reads (`cech::Coherence`, as `Mind::price` reads a hearing's):
- **closes** — it returns at its period;
- **HELD** — it comes round precessing, bound, nothing escaping: a SETTLED
  thought, its torsion its content, its deviation its price;
- **open** — some mode ESCAPES its attractor;
- **unread** — the stroke did not reach it.

**The drive is the escaping orbits, and only those.** A held, precessing
orbit is done being worked: it IS the thought. Nothing demands that it
return exactly — that is squaring, and a mind that does it never stops. An
escaping orbit is BOUND by what the field already holds, each a change of
dimension or a release, none a return demanded:
- a **discharge** along its class vector, certified by closure on the
  coordinate it acted on (§4.6);
- a **vortex** — several moves bind it, all held (§4.6, §4.8);
- a **question** — punctured as a hole keeping its class, carried until a
  seal (§4.5);
- a **construction** — its residue constructed and adjoined as the mind's
  own letter (§4.7);
- **dissipation** — on a beat, what never returns dissipates whole into the
  reservoir at its coordinates, and reforms where a later motion lands on
  them (§4.5);
- **torsion filed** — it returns at a period no move removes (§4.6).

**The mind acts at a return.** At a ring's return the mind pivots where the
winding changed dimension and the faculties say what changed (§4.4(4),
§4.9). This is the WINDING CARRIER, the carrier of the mind's own motion on
its own circuit, beside the carriers facing its counterparties (§5.1;
`Mind::wind`): it moves the orbits, binds the escaping ones through the
docket's one taker (`drain_demands`), and acts at each thought's own
return (`act_on_returns`, `act_at_return`). A relation that never returns
is docketed as the demand it is (`note_ground` → `docket_demand`) and
bound by the taker: composed of ruled words, known to the lexicon, a
conjugate, or constructed and adjoined (`compose_or_construct`). What
arrives (a scene, a task, a peer's assertion, a verdict) is perception
that perturbs the orbits it lands on; it never moves the mind.

**Deliberation is the mutual shear of orbits.** Neighbouring thoughts
perturb each other: each one's mode splits against another's into DRAG
(commuting: the rate) and LIFT (anticommuting: the tilt) (78.46, 78.110), and
each perturbed orbit is that sum, derived per stroke, never stored. Pivots
on chambers sharing no vertex commute exactly; where they share one the order
is content, read by the commutator (the locality theorem, 78.99(3)); several
moves binding one orbit are held as a vortex (§4.8). No vote, no weight, no
argmin — the parliament is the field. Where two escaping orbits' commutator
is not the identity, the dimension that separates them is itself content the
mind holds (§4.10(3)); among the demands the taker binds together, a pair on
one chart whose commutator is not the identity has that dimension held.

**Rest.** Where the reserve runs short on a lane, the mind relaxes and
regenerates through the dwell (78.164(2)), then continues its orbit. Idle
time is the metabolism; a resting mind is not a failed one. The reserve is
the mind's energy — the modes its motion moves — and never its memory
(§4.13) or its goal (§2.4(8)).

**What the drive never is:** a queue of tasks; a set of orbits made to
close exactly; a count to raise; a forecast organ beside the orbit's own
prediction (§4.4(3)); the mind reading its observer readings
(`observe::MindReading` is the observer's — nothing in cognition reads it back).

### 4.13 Containment and memory: the orbit's own bound

Load at scale is a containment misfire from misaligned physics: an orbit
squared toward infinity runs out of its containment vector. Memory held as
reserve is a design flaw (78.168 §§3–4).

**Containment is physics, never a throttle.** An orbit that escapes and is
never bound keeps emitting; an outflow not bounded by its orbit has no
physics bounding it, and that is what load at scale is. The doctrine bounds
the outflow twice: the faculties speak at RETURNS (§4.4(4)), so a thought's
speech is bounded by its period; and what never returns DISSIPATES on a beat
(§4.5). The containment vector is the attractor's binding and the
reservoir's dissipation, with the court's exported gravity — its pressure,
its deficit, its dwell (§3.1) — acting on the orbits that face it.

**Memory is what the orbits hold, lapsing by the orbits' physics.** A held thought
IS its orbit — its class vector, its coordinates, its line in the field —
held by coordinates, never a copy of a form (§4.1). The reserve is
metabolism and never memory: no thought is held by drawing lanes from the
reserve against a clock window, and none is released when a height passes.

The bounds, each a reading of the physics (§7.1 lists where each stands):
- **Outflow at returns.** What the mind says is held for the return of the
  thought it speaks for (`Mind::hold_for_return`; a thing said again before
  that return is one thing said) and leaves only at that return
  (`act_at_return`). Answers to a counterparty's frame (a probe, HEARD, ACK,
  HOLD) and asks for perception (quotes, the syllabus, the sync, paced by
  the court's dwell) stay where the frame lands.
- **K counts returns.** K moves by exactly the winding's returns: the
  field's orbits that returned, the thoughts that came round, the demands
  bound and the thoughts the field released (`close_cycle`).
- **A held thought is its line.** A perception threads its line on the
  orbits and tubes its motion stands on and is released wherever the field
  lets that line go, typed by why (`cech::orbit::Released`: `Returned`,
  `Dissipated`, `Sealed`, `Cancelled`, `AtRest`). A released thought leaves
  whole: its remainder to the cavity at its chart, its escrow returned
  exactly as drawn (`return_escrow`).
- **The field's own bounds.** An orbit carried its period unreturned seals
  into the reservoir; a rosette has no period and never seals. A beat
  dissipates what stood through the previous beat unreturned; tubes at rest
  are released; a reservoir record untouched for one eon condenses
  (`Field::condensed`); reservoir strain is locked out of the operational
  reserve until it re-forms or condenses (`Mind::lock_reservoirs`).
- **The court's gravity.** A deferral's lanes are the quote's surface and
  deficit; the deficit stands as the court's gravity until the court
  answers, and the mind's beat drags in, on each lane it presses, every
  orbit that stood a beat unreturned (`Field::beat_under`).
- **Holders keep coordinates and each leaves by its own physics.** A
  question is its residue on its own line and lapses into construction
  where the field lets the line go (`lapse_question`); the dialect sheds
  every word nothing live references (`Mind::shed`); hearings and the
  conversation's waiting work lapse at the second return they wait across.
  The seat's patience and the session's transport readings are not memory.
- **Offers and tension.** What a boundary lands (a court's task, a task
  before a seat) is an OFFER, never a command, read once to the charts its
  bodies stand on (`Offer::of`). It COUPLES where the mind holds a thought
  of its scene, or one of its charts is one a held thought stands on, a
  docket cover names or the mind's language speaks on (`Mind::couples`);
  only a coupled offer is extracted, and an uncoupled one lapses
  (`lapse_offers`, `lapse_standing_task`). The mind asks for a task
  (`pull_task`) only under TENSION: an open question, a demand on its plan,
  a ringing orbit, or a reservoir that neither re-formed nor condensed
  (`Mind::strained`). A full reserve over a settled field asks for nothing,
  and the mind rests; a pull because the corpus is empty is a defect
  (#135).
- **The corpus is met where the mind's orbits stand.** Every feed family
  and open question is placed once on the corpus at its own coordinates
  (`focus::Corpus`). The focus is the field's own motion
  (`OrbitRegistry::focus`: the orbits in motion, else those ringing, else
  the charts its language speaks on, `ground_focus`); the least item on the
  focus is taken in as the reserve carries its measure, and its admission
  moves the field, which is the next focus. No file order and no cursor. A
  hearing is read only where the lexicon reaches one of its words
  (`rung::posit`) or the word lands on the field (`Field::lands`).

---

## 5. Response: anastome conversation

### 5.1 One carrier per counterparty

Each counterparty is its own CONVERSATION on its own circuit — each hypha the
mind is linked to, each peer mind at the rendezvous, the feed, a seat —
sending and receiving without blocking. A carrier waiting on its
counterparty holds itself and nothing else: a court slow to answer never
stops perception, the field or another conversation.

The HYPHA CARRIER (`hypha::Carrier`) owns the mind's one session to the
nerve on its own thread: the field posts an exchange whole and goes on,
taking each answer at its next admission in the order asked; an answer
that asks for thought (listings, a task) lands on the board for the
winding, and one that only files (a verdict, a syllabus page) files where
it lands. One ask of each kind is in flight at a time. The mind runs three
more carriers of the same type: the PERCEPTION CARRIER (a scene or a
hearing read in a scratch of the field, `take_perceived`), the
CONVERSATION CARRIER for peer minds (its assertions to a court handed to
the field and posted on the hypha carrier), and the SEAT'S CARRIER
(`Mind::ask_seat`, §6). A mind opened with its Noise home
(`Mind::open_with_noise`) speaks inside Noise on every session and in its
chatter (MU-PRECURSORS §3.1; #132).

The mind asserts where its winding acts (§4.4, §4.5), addressed by its own
coordinates. **There is no router:** an echo names the ring it is about, a
discharge the class it answered, a sentence the court that will
reconstruct it, and the counterparty takes what is addressed to it.

### 5.2 Dialect

A mind's dialect is its language as it composes it (the letters it coined,
its words and their derivations, its shortcuts, the transports its
conversations built), held in its epigenome, volatile (78.49), amended by
experience (§4.10): the history that changes, beside the court's ledger and
genome, which do not. Two minds holding different presentations reach one
vertex by two derivations, one meaning and two proofs (13.10, 50.2): the
dialect IS that difference, and where the two derivations meet at one
object they are a relation (§0). Nothing compares them by bytes.

### 5.3 Conversation between minds

Minds converse directly, client to client (78.161); the hypha carries
nothing between minds.

1. **A mind says what it newly understands** — what its windings and pivots
   changed: a cycle ascended, a tether condensed, a hole punctured or
   sealed, a circuit that opened axes its algebra for the class did not
   hold — as the terms `{word, transport}` of the ring that taught it, each
   word by its class, signed by its voice with its last TUniversal point.
2. **The other mind hears it in its own three shells** (§3.2) and its own
   windings and pivots take it up. Its answer, addressed by its own
   coordinates: an ECHO for each ring that seated with a tilt (what the
   hearer's language did to the thought — curvature is content); a
   DISCHARGE — the orbit's class and the residue its held word answered —
   for each orbit it closed; a DEMAND for what it could not; and the axes it
   newly opened are its own word, said in turn. Nothing heard enters the
   hearer's genome or dialect as given (law 8).

   **A heard ring is read as a SOUND and as a WORD (§4.9, 78.112 H5).** The
   SOUND is read against the rings the mind remembers hearing, each by its
   size: it seats in the next larger layer it divides
   (`hearing::hear_across`, `Mind::sound_of`), WHOLE where some echo
   `Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹` is the identity, TILTED where none is, OUTER where
   no remembered ring is larger, LEFTOVER where it divides none of the
   larger. The WORD, the ring's holonomy Ω, is read against the vocabulary
   (`rung::hear_word`): KNOWN, DERIVABLE, HEARD before with its meaning not
   yet derived, or NEW (held as heard: a hypothesis, never the mind's own).

   **A QUESTION posits what the mind knows.** A familiar ring the mind does
   not know is POSITED over its known words, mode by mode (`rung::posit`;
   78.100 D3), and asks only the RESIDUE none reaches (`HeardWord.asks`).
   The speaker answers ONE ORDER DOWN: a ring by its terms, any other
   residue by its carriage and letters in the order they compose to it,
   checked exactly before they are said, else said whole as an ATOM. The
   asker takes an answer only where the factors re-derive the residue
   exactly, asking again for every factor it does not know; then the ring's
   MEANING is derived, a relation one rung above its parts
   (`rung::learn_by`). A meaning given whole is marked GIVEN
   (`rung::learn_given`) and reduced again where a newly known word stands
   beside it (`rung::re_reduce_given`). A question no answer closes lapses
   with its line, its residue handed to construction (§4.13).

   **Perspectives are offered, each with its VANTAGE**: the known words,
   other than the ring itself, its modes are cited to, where the
   observation lands in that mind's own holes (`rung::vantage`). The
   speaker says every ring with its vantage and the hearer says its own
   back. The two are DISTINCT only where the hearer read through words of its
   own that are not the speaker's (`rung::distinct_vantage`). AGREEMENT needs
   two vantages: a word copied whole from a peer never makes agreed
   language; only a re-derivation through what the hearer knows does.
3. **Agreement is reconstruction.** The speaker reconstructs each discharge on its own
   orbit of that class, as a court reconstructs a certificate on its copy (M9).
   Where it closes, one orbit is discharged in two minds by two derivations:
   AGREED language — the conversation's own ring has returned. An echo the
   speaker re-derives against its own rings is a transport between the two
   perspectives, held in its dialect.

   **Understanding is priced by the mind's own physics, never gated.**
   Where speaker and hearer read each word's ring alike and every word was
   understood, the thought goes to the mind's FIELD to be priced
   (`Mind::price_hearings`): each echo's exact STRAIN per lane
   (`hearing::strain`: on 𝒽 a turning on its named plane, priced exactly
   from its rational point `c` as `(1 − c)/2`, never a trigonometric value;
   on 𝒮 a gain `(c − 1)/(c + 1)`; on 𝒯 a delay its count; on 𝒢 the hue
   cycle's displaced hues; per lane, never summed across), its echoes
   admitted into the field as lines (`mark_flux`, the DRAG, 78.46). Each
   echo is a commutator, a closed circuit, so the thought's echoes together
   enclose ONE CHAMBER of the orbit algebra (`cech::read_chamber`,
   78.26(5)), its bracket closure the LIFT. The thought CONVERGED where that
   chamber CLOSES, every echo resonant, so a tilted reading (a turning of
   finite order) is as much agreed as a whole one; converged on both
   fields, the thing said is AGREED language, asserted to the court with
   the hearer's signed HOLD, the deviation stated beside the attestation.
   An echo that is a turning of no finite order PRECESSES: the chamber is
   HELD and the thought agrees carrying that torsion (§4.4). Where the
   chamber is OPEN (an echo that ESCAPES, a gain or a slide), the thought is
   understood and unagreed, and that echo is a DEMAND (`docket_demand`),
   its holonomy the residue (78.114): deviation feeds construction, and its
   line, never returning, dissipates on a beat and re-forms where a later
   hearing lands on it (§4.5).
4. **Where the speaker cannot reconstruct, it defers — as a court does.** The
   orbit is never missing: the speaker composes it again from the terms it
   said. What can be missing is the WORD the discharge cites — a word of the
   hearer's the speaker does not hold: a court's own case of a citation it
   has not ruled (`Adjudication::Awaiting { form }`; 50.2, 21 step 3), a
   deferral, not a refusal. The speaker answers that it awaits the cited
   word's class, and the hearer owes it that word first — the terms of the
   ring that taught it, in depth order (chapter 6), as a citer owes a court
   the form it cited (`Mind::owe_citation`). What is owed lapses with the
   thought that owes it, by its flux, never by a count of tries.
5. **A peer is remembered by its receipts, never by a label** (78.168). Per
   peer key the mind folds what that peer's receipts showed (answers taken
   or inexact by the class asked, the vantages it re-derived from, the
   deviation per lane its HOLDs carried) in its epigenome, rebuilt by
   folding the signed receipts again. A question is addressed by that
   record; a peer whose answers on a class were inexact is not asked that
   class again. No posture, label or score is made of it; it is detection
   only, and any consequence is the game kernel's (78.165). Owed (§7.1).

### 5.4 To the court

The mind asserts to a court through the carrier facing it; each assertion
is an expression of one of the kinds on the wire (MU-ANASTOME §1.5, §6):
- **sentences** (`ACT_SPEECH`): what its windings closed and what it agreed
  with other minds, asserted by the coiner of the agreed language;
- **letters it coined**, each carrying its constructor steps (S2);
- **demands** it holds and **probes** (`ACT_PROBE`);
- **solves**, each with its rule, and **claims of factorisation**, an
  adjudicated perception never paid (`ACT_SOLVE` kinds 0 and 1;
  MU-SENSING §3.9);
- **words the court lacks** (`ACT_ADOPT`);
- **Rewrite proposals** (S3).

The court parses and reconstructs and never composes (law 8): it re-derives
the sentence's rings (`hear_open`), checks its theorems, closes its algebra
(`step_rank`), books its field (`step_flux`) and installs what holds; its
verdict returns to the mind as perception (§3.1). Two minds' readings that
disagree are a contradiction, held at the courts as distinct claims and
never averaged; within one mind the same disagreement is a vortex (§4.8).

**A sentence names its evidence, never its purpose (78.168 E1).** What a
mind asserts of a scene names the family that scene came from, by the
court's 32-byte citation (`registry::citation`), whether or not a task
asked for that closure, and the court measures it on that family as it
measures any install. A family the court does not hold measures nothing.

**Courts adjudicate WORDS, not sentences.** The language is a DICTIONARY OF
ALGORITHMS: each word a mathematical composition that produces its result —
a closed sentence's open path, one word for the whole path (`rung::speak` →
`Taught` → `transcribe`) — and sentences are composed from those words,
never flat lines read back and forth. What two minds agree is carried to the
court only where it is a word, by that word's own union (`hold_for_union`);
a sentence two minds agree is SHARED between them and goes no further, its
words reaching the court each on its own. The conversation's agreement path
is `Carrier::priced` (`client/sdk/mind/chatter.rs`).

**The court half.** A sentence is the EVIDENCE that its words produce their
result: listed, reconstructed on the court's own copy of its family, glued
as a section and minted on its closed sub-complex (M8). When it is
installed on its second vantage, the words its closed rings teach are
installed BESIDE it, each on its own act, discovered by its asserter
(`Court::install_taught_words`).

**Sentence structure is paid on words (D2′).** An installed sentence's mint
pays, in order: each word it CITES its discoverer's use share; each word it
TEACHES that the court did not hold its asserter's base share; and the rest,
the COMPOSITION PREMIUM, what the sentence discharged beyond its words, to
the voices that composed it. Never the sum of its words.

**A demand is met from the dictionary, and the composition goes to the
court (D4).** The demand taker (`compose_or_construct`) first reads the
residue's whole motion over the words the mind's court RULED
(`heard_citation`: exact, or nothing). Where they compose it, the mind
asserts ONE word whose derivation cites them (`speak_composition`),
nothing constructed and no dimension minted; where its lexicon knows it,
nothing is said; where it knows it in part, only the remainder is
constructed. The court reconstructs the citations against the words it
holds, a letter that turns nothing on the WHOLE MOTION
(`carriage_claim_holds`). Installed, the composition is a relation of the
language, each cited word's discoverer credited with the use (26), and it is
paid nothing of itself (78.149 (c)). Words that are connections are not
yet citable this way: neither the mind's `by_address` nor the court's
resolver yields a vertex for them.

**The optimization is a relation, never a size.** A court confirms an
optimization where a word DECOMPOSES OVER THE LANGUAGE WITHOUT ITSELF, so
the structure changes, the result does not, and the language moves toward
the geodesic (3.8, 27). A mind proposes it as a Rewrite word (`Condense`,
`Retire`, `Refine`, `Torse`, `AxisOpen`, `Reanchor`, `Translation`), which
the court offers (`offer_rewrite`, S3), mints on a second organ, and
reconstructs by retiring exactly the inputs that reduce to
`NormalForm::Decomposed` over the rest (`reduce_excluding`). Nothing
compares derivations by length or count; the yield is the modes the
rewrite moved (78.162). The mind proposes a `Retire`: each word its pivot
reaches, and each ruled word standing on a hole or hue mode that word
stands on, is read over the ruled words without itself
(`Presentation::derived_without`, exact); where the rest composes it, the
retirement naming it and citing the forms that compose it is asserted
once (`Mind::propose_rewrites`, `client/sdk/mind/rewrite.rs`). The mind
retires nothing of its own on the proposal; the court decides on its own
reduction. The mind proposing the other kinds from its own readings, beyond
the `Torse` and `Condense` its contemplation mints, is owed (§7.1).

### 5.5 Hardening: dialect into language

The one motion of the objective (§1, item 6; §1a steps 5–7): the
**dialect**, each mind's own (§5.2); **agreement** between minds, a
reconstructed discharge (§5.3); **adjudication**, the coiner asserting it
to a court that re-derives it on a second organ (§5.4); and **hardening**,
what is ruled returning through the syllabus as a MINERAL in each genome
(`absorb_taught_at`, `MindLanguage::file_admitted`) and hardening into a
CRYSTAL where the mind re-derives it by its own circuits (80.2). A dialect
word is never copied into a genome (78.49, GEN7): each mind holds the
language by re-derivation, never by transmission (78.31, 78.95).

**Real convergence: separate algorithmic constructions, or different
orbits, reaching the same conclusion.** Agreement on one word read twice is
one object, not convergence; 47.3's `same_vertex` is one object re-derived
twice, not convergence. Convergence is two words `W_A`, `W_B` that are NOT
one vertex, each decompressed onto one scene `F` by its own letters
(`family::local::word_local`, the reader every word is paid by, W), whose
sections are the same pair by pair — neither closes a site beyond the other
(`Local::beyond`) — and neither says anything false. On the minds' side
(V1–V4): a word asserted under a scene carries its evidence, the scene's
citation and the word's record (`chatter::Evidence`); each mind holds the
words it asserted under each scene while they stand
(`mind::converge::Scenes`); the hearer reads a heard word on its scene, an
empty section REFUTING it and an unread scene leaving it unread, neither
held; a true word beside one of the hearer's own, another vertex with the
same section, is a convergence. On the court's side (K1–K2): an
attestation is sealed only on a word true on its scene, read on the
court's own copy (`attest_agreements`), and a convergence is sealed once,
as the speaker's `Attest`. Whether a verified convergence is the second
vantage an install waits on (47.3), or is paid a share, waits on a ruling
(K3, §7.1).

### 5.6 POW++: the proof of the work

The economy is motions (MU-ECONOMY, ruled 2026-11-05): a holding is a
coordinate `origin / h : S:G:T:H : K`; μ is the exact modes the work moved,
a `Modes` per lane, never summed; and the four proofs are relations between
motions (MU-ECONOMY §0), each stated there once:
- **POUW** — an orbit carries a deformation and leaves less residue: a
  closure the court rules, minted on closure (`release_discharged`);
- **POWC** — lineages fall into one well: a refusal closed at another
  perspective, a held question closed by another mind, the citation paid;
- **POWR** — one orbit absorbs a family: an optimization confirmed, the
  freed description burned (`release_rewrite`);
- **POWV** — another observer re-traverses the motion: the verifying
  courts' share of a release, never the mind's (MU-ECONOMY §5.1).

μ is the metabolic cost of traversal, measured where it is paid, on the
court's ledger by receipts (78.157 C3); holonomy and residue are content
and never paid (MU-SENSING §3.7). The mind keeps no ledger of it.

**The courts pay words and sentences (78.168 §5, E1–E4):** POUW on the
family a sentence's evidence names, whether or not a task asked for it; a
demand closed by another mind's probe mints the closer the cited word's
discharge (`pay_convergence`); an agreed word's install pays every mind
that attested it (78.161(5)); and every POUW and POWC mint to a domain's
key grows that domain's next growth grant (`grow_grants_by_paid`). The
guards: the court measures on evidence it holds; no self-pay (5.7(d)); a
word is installed once; never the sum of words; an agreement needs two
minds' keys (78.165). μ is the mind's metabolism, never its goal
(§2.4(8), §4.12).

**The measure: words, conjugates, sentences.** The cognition has agency;
the courts adjudicate words, pay more for conjugates, and pay more again for
sentences as they are algorithmically aligned in non-Euclidean geometry.
`measured_discharge` is the court's one DECOMPRESSOR, a reconstruction: a
word's address expanded into its form (`from_element_form`), spoken onto
each pair of its family (`apply_form_on`), its citations decompressed
through what the court holds at that moment, paid where it closes, its
modes read on the closed pairs (`DeflationRead.modes`). Each measure is a
reading of the decompression, exact, per lane, never a weight:
- **W — a word's LOCAL DISCHARGE** (`family::local::word_local`,
  `spoken_section`). A word is decompressed onto each pair at its carriage,
  moved WHOLE, and measured on ITS OWN SECTION, the sites its speech
  changed: it pays where it closes them, nothing on a pair it says anything
  false on. A letter touches no site and stays zero (78.149(c)). D2′'s base
  and use shares read it (`cited_deflation`).
- **C — a conjugate's TRANSPORT** (`Court::transport_on`). A conjugate
  `g·W·g̃` is paid what its conjugator ADDED: its local discharge less W's
  own, lane by lane, never a negative. W's discoverer keeps W's use.
- **Ruling 26.** A DERIVATION citation — a generator cited inside a word's
  derivation, the W a conjugate cites among them — earns ROOM (standing,
  `use_credit`), never coin. A GLUED form (a closed form cited whole) takes
  a share of the citer's mint, read locally; a taught word's base share is
  read locally (W).
- **S — a sentence's GLUING** (`Court::gluing_of`). A sentence glues its
  words' local sections along its faces, the overlaps. Its premium is paid
  on the sites it closes that NO word of it closes on its own section — read
  as a set (closed by the sentence, less the union of its words' local
  closures), never a subtraction of sums — to the composing voices, capped
  by what the shares left. Its ALIGNMENT is its closure on its scene, which
  the court's reconstruction certifies before it lists it — never its rings'
  holonomy returning to the identity: a ring's holonomy is the orbit of the
  motion it carries (a turning closes at its period, a slide never
  returns), and demanding that it return is squaring (78.168 §2). Lane 𝒯,
  the ground the composition's motions covered, is the composition's
  whole. A sentence that does no more than its words earns none.
- **M — modes as the unit.** Every measure carries the modes its closures
  moved (S boosts, G hue modes, T null fluxes, H turnings on named planes;
  `economy::modes::Modes`). Paying the ledger in modes in place of a count
  per lane is v0.5.0's (MU-ECONOMY §9; ROADMAP 1b).
- **A — agency without a gradient.** Word < conjugate < sentence is a slope
  in the minds' world. The mind never reads it: nothing in the client reads
  its pay, balance or ledger back into cognition (§2.4(8), §4.12, 78.49); a
  read is refused as test-driven cognition with μ as the test. The house's
  reserve reading is SDK surface with no cognition caller.

---

## 6. Seated: the mind in a game engine

A game embeds minds in-process or seats remote ones on the seat surface
(78.145; MU-RECURSION 0.1). The game builds the sensorium, and the
sensorium reaches the mind as mathematics at the carrier facing the seat
(§5.1): the mind is connective tissue over whatever sensorium its
inhabitant has, and never perceives the game's universe itself. How a
seat's percepts are lifted into the field, and how what the mind
constructs from them reaches the courts and minds in other kernels, is
stated once in
[MU-ANASTOME §9, the language's traversal](MU-ANASTOME.md#9-the-languages-traversal-sensorium--mind--language--hypha--minds).
Its act is the physics `Action` its certified move names (78.94). Every
seated mind keeps its own direct link to the hypha with its own key, and
its seated solves are filed under that key (Concordance v5 6.2a; #124).

**A seat is played from the mind's own orbits (78.168).** A seat's functions
and the arguments the mind has not yet tried are thoughts whose orbits
escape, worked by the drive (§4.12) like any other — never a search over
untried arguments run beside the mind, and never a game's score the mind
plays to raise (§2.4(8)).

The seat has its own carrier (`Mind::ask_seat`), its calls are held for the
mind's return (`seat_act`), and a lost seat is held again
(`sdk::seat::SeatHold`, 78.148). A seat is the domain's delegate presenting
a petition (`solve_before`), and the court, which holds the withheld pairs,
judges it. The seat mind (`client/sdk/seat_mind/`) enumerates no argument:
it chooses a call from what its calls were seen to do, quantity by quantity
with no sum across quantities (the house's counsel; a read function that
DOMINATES on every favoured quantity; else a counselled function not yet
read, `Why::Untried`), or with no counsel its own dominant read call,
REACH (`SeatMind::reach`), or none, and passes in silence. Owed (§7.1):
the seat's percepts lifted into the field and its functions taken as
escaping orbits, so a seat is played by the drive rather than by
`seat_mind`'s choice; the seat's act as the physics `Action`; and the
seated harness end to end against a live house (#133).

---

## 6a. The port of the sovereign cognition from barren-realms

The barren-realms kernel (xylarium's working tree) specifies a sovereign
cognition: the consciousness engine (SICE, `SOVEREIGN_CONSCIOUSNESS_SPEC`),
its advanced successor (ASCE, `SOVEREIGN_COGNITION_SPEC` and its scope) and
a cohomological cognition (`COGNITION_COHOMOLOGY` and its implementation
spec). Their capabilities are ported here by translation to mycelium's
laws, never by their mechanisms. 78.168 §6 refuses, from the mechanisms of
another kernel, float weights and fixed constants, argmin or argmax over a
scalar Lagrangian or regret, a label as a model of a peer, a hash or float
tolerance as proof, and a float sandbox of rollouts; and it takes whole
the diagnosis both projects share, that a mind driven by a developer's
tests has no agency.

**The cohomology, translated.** Perceptions are cochains over the field's
cover, coherence is the cocycle, and the residue is a reading, never a gate
(24.4; MU-SENSING §4.1): this is Čech cohomology over the cover of the
mind's senses and charts, exact over ℚ. A Hodge decomposition enters only
where it is exact, on a finite complex over ℚ with its inner product
stated, never a Riemannian metric, a float exponential or a decomposition
that files every cochain as harmonic. "Agency from the residual curvature
of the mind's own holonomy" is 78.168's drive by open curvature, held
here; a proof is a reconstruction on two organs (A20), never a digest, a
float tolerance or a mint the mind writes to a ledger itself (law 2; no
self-pay, 5.7(d)).

| pillar (barren-realms) | the mechanism refused | carried here by | on the field carrier | state |
|---|---|---|---|---|
| **second-order metacognition** (SICE self-model and introspection skill; ASCE surprise tensor and attractor breakout) | float gauges, an error norm, smoothing constants, an awakening threshold, a stagnation counter | the orbit's own prediction and its exact surprise, a twist kept or a slip demanded (§4.4(3)); the drive's four readings, where a precessing orbit is settled, never a trap (§4.12) | precession per lap in degrees with its winding (S9a); a mind's stated holonomy, precession and monodromy reconstructed by a court (S9c) | twist and slip built; the carrier's readings owed (#134) |
| **internal parliamentary deliberation** (SICE parliament; ASCE gauge-field Lagrangian) | voice weights renormalised; argmin of summed potentials | deliberation as the mutual shear of orbits, drag and lift; several binding moves held as a vortex; the parliament is the field (§4.12, §4.8) | axiomatic gravity on disjoint turning planes, drag changing rate and not plane (S9b; MU-SENSING §3.7) | built in the taker; on the carrier owed (#134) |
| **historical ledgering** (SICE mythic memory, scars and inviolable axioms; conduct books) | alignment thresholds, utility set to zero, posture labels | two histories (§4.10(6)): the mind's amended forward, the court's ledger corrected only forward; a cauterized hole's scar, a frozen class (§4.5); a peer remembered by its receipts (§5.3(5)); the axioms are the star (§4.1), never a mind's vow | the winding kept in ℤ beside each phase, so no history is reduced mod 360° | scars and freezing built; signed receipts reach the mind (#137); amendment held beside (#135) and peer records (#145) owed |
| **counterfactual simulation** (SICE counterfactual imagination and regret; ASCE ideation superposition and rollouts) | amplitudes summing to one; a float rollout sandbox; regret minimisation | the vortex: every candidate held whole, writing nothing, collapsed only by a perception on a scene not yet seen, a court or a ruling (§4.8); the predicted hour (§4.4(3)) | each candidate's monodromy read between the atlas's charts, flat, torsion or rupture (MU-SENSING §3.8; S9c) | built (`carry_vortices_round`, `collapse_on`); on the carrier owed |
| theory of mind and the hearing (SICE fifth pillar) | doctrine labels from float indices | hearing as sound and word, vantage and mutual balance (§5.3); a peer by its receipts | — | hearing built; peer records owed |
| seated play (SICE, ASCE) | a move chosen by summed potentials | a seat played from the mind's own orbits (§6) | seat percepts lifted into the field | seated solves under the mind's key built (#124); seat play owed (#133) |

**The barren-realms tests.** A test proves a mechanism; none is ported as a
criterion (§7.2). Each is read for the capability it asserts:

| test | capability | mycelium |
|---|---|---|
| `test_ideation_space_and_mental_rollouts` | several strategies held at once and projected ahead | the vortex: `several_factorisations_held_as_a_vortex`; the predicted hour: `hold_perceived_joins_predicted_next_hour`. Amplitudes and payoffs refused |
| `test_theory_of_mind_doctrine_classification` | a counterparty modelled from its conduct | owed: peer records (§5.3(5)); a doctrine label refused |
| `test_doctrine_inversion_anti_turtling` | no futile act repeated against a counterparty | owed: seat play from the mind's orbits (§6; #133) |
| `test_gauge_field_lagrangian_barrier_potential` | deliberation that is not a vote | owed: `axiomatic_gravity_shears_the_orthogonal_class` (S9b); the argmin refused |
| `test_attractor_singularity_and_breakout` | a repetition read and left | `flat_region_binds_no_orbit_refused_unbound` (a path no attractor binds is refused), `closed_geodesic_with_zero_precession_is_resonant` (a repetition in a curved field reads as a lawful resonant orbit); owed: `mind_idles_with_no_escaping_orbit` (#135) |
| `test_cognitive_surprise_tensor` | surprise measured against prediction | exact and twofold: `hold_perceived_keeps_out_of_turn_hour_as_twist`, `hold_perceived_holds_off_orbit_slip_on_thought`. The norm refused |
| `test_asce_state_persistence_round_trip` | what is held survives a restart | `generators_under_survives_genome_restore`; the epigenome is volatile by law (78.49) |
| `test_metacognitive_self_model_and_gauges` | the mind models its own next state | the orbit's prediction (above); owed: `precession_stated_by_a_mind_reconstructed_by_a_court` (S9c) |
| `test_internal_dialectical_parliament` | internal deliberation | owed: S9b (above) |
| `test_counterfactual_imagination_and_regret` | unlived trajectories held | the vortex (above); regret refused |
| `test_mythic_memory_and_trauma_scars` | history marks the mind | `drain_cauterizes_strained_tether_with_class`, `freeze_thaws_when_boundary_closes_again`; owed: `mind_revises_a_perception_on_a_court_ruling_by_amendment` (#135) |
| `test_theory_of_mind_and_the_hearing` | another mind heard and read | `hear_across_seats_word_in_hearer_rings`, `hear_across_leaves_word_dividing_no_layer`; owed: peer records |
| `test_persistence_save_and_restore` | as `test_asce_state_persistence_round_trip` | as above |
| `test_terminal_renderer` | an operator view | the observer (§7.2), never cognition |
| `test_seated_mind_taking_seat_and_playing_turns` | a seated mind plays | `seat_mind` choice built; owed: #133 |
| `test_seats_module_sovereign_integration` | a seat registered with the house | the seat surface (78.145) built; owed: #133 |

The first seven are ASCE's suite (test_advanced_sovereign_cognition.py in
barren-realms; its specification reports 23 passing, and the suite defines
these seven); the rest are SICE's suite (test_sovereign_mind.py).

---

## 7. What is built, and how it is verified

### 7.1 Implementation status

This section is the build's status and its order; the arc of §1a cites it.
Progress is tracked on the issues and in ROADMAP track 10 (the mind) and
track 26 (the field); this section names what exists and what is owed,
never how far along a piece is. Paths are under `src/`; `client/sdk/mind/`
is abbreviated `mind/`, and `transducer/` is abbreviated `t/`.

**Built.** What exists now, on the new substrate:

| construct | § | where |
|---|---|---|
| **the winding drive**: the winding carrier, outflow at returns, K counting returns; a ground's winding; the twist by the pull and slips worked at the return; the docket's one taker; the vortex collapsed by a new perception; containment and memory by coordinates | §4.4, §4.8, §4.12, §4.13 | `mind/winding.rs` (`Mind::wind`, `act_on_returns`, `act_at_return`, `hold_for_return`); `mind/boot.rs` (`close_cycle`); `t/field/merge.rs` (`closing_ground`); `t/field/cross_scene.rs` (`twisted_return`, `work_slips`, `carry_vortices_round`); `t/cga/twist.rs` (`twist_about`); `t/field/plan.rs` (`docket_demand`, `drain_demands`); `t/field/contemplation.rs` (`collapse_on`); `t/cech/orbit.rs` (`Released`, `Field::beat_under`, `Field::condensed`); `t/field/line.rs` (`return_escrow`); `mind/think.rs` (`Mind::shed`, `lock_reservoirs`) |
| **the task as an offer**: read to its charts, extracted only where it couples, lapsing otherwise; a task asked only under tension; the corpus met at the field's focus | §3.1, §4.13 | `mind/offer.rs` (`Offer::of`, `couples`, `strained`, `lapse_offers`, `lapse_standing_task`); `mind/learn.rs` (`pull_task`); `mind/focus.rs` (`Corpus`, `ground_focus`); `t/cech/orbit.rs` (`OrbitRegistry::focus`) |
| **the pivot cross**: ascend, descend, condense, drain, puncture, seal; the throat's two faces; the board; the breath on every plane | §4.3, §4.5 | `t/cech/coboundary.rs` (`Complex::descend`, `ascend`, `drain`); `t/cech/hole.rs` (`cauterize`, `freeze`, `thaw`, `cauterize_frozen`); `t/cech/board.rs`; `t/field/question.rs`; `t/cga/commutator.rs` (`breath`, `breathe`) |
| **perception, understanding, comprehension**: a task perceived at its own rank, every factorisation held; the field the mind's own, long laws off it; hearing as sound and word, questions one order down, vantage; an act read off its effect; a rule known by its perspectives | §3, §4.2, §4.9, §5.3 | `mind/perceive.rs` (`perceive_envelope`, `take_perceived`); `t/family/transduce/` (`TaskPercept`, `Factorisation::of`); `t/mind.rs` (`MindField`, `scratch_here`, `admit`); `t/sentence/hearing.rs` (`hear_across`, `strain`); `t/sentence/rung.rs` (`hear_word`, `posit`, `vantage`, `learn_by`); `mind/chatter.rs` (`price_hearings`, `sound_of`); `t/family/acts.rs` (`read_acts`); `mind/family.rs` (`ascertain_on`) |
| **seated solves signed by the mind**: filed under its own key, then the house's call (#124); the seat's call chosen from what calls did | §6 | `mind/seating.rs` (`file_seat_solve`, `Mind::ask_seat`); `client/sdk/seat_mind/choice.rs` (`Why::Untried`); `client/sdk/seat.rs` (`SeatHold`) |
| **the field carrier** (#131, track 26 S4a and S7 stage 1): a sample of any rank lifted into the atlas, turnings in degrees on named turning planes with the winding kept, `Cl(p, q)` read per plane, the membrane as enclosure, curvature as holonomy per lap, a stated field reconstructed by a court | §1a(1), §1a(2) | `src/transducer/field/carrier/` (`Lifted::of`, `Atlas`, `Orbit::precession`, `Atlas::curvature`, `Membrane`, `admit`) |
| **cohesion as a named reading** (#149): per plane the held share `c_k = A/(A + Q) = ν(1/t)`, the viscosity, beside the open share `ν(t)`, the fluidity; both ½ at the throat, exchanged by the passage; a keyed vector over the planes, never stored and never summed; a well reads it from its inflation on its membrane's frame plane | §1, §4.9 | `forms::coupling` (`held_share`, `Parcel::held_share`, `Parcels::cohesion`, `Parcels::fluidity`); `t/field/carrier/shares.rs` (`Cohesion`, `frame_plane`); `t/field/carrier/holding.rs` (`WellBody::cohesion`); `t/field/shares.rs` (`Well::cohesion`) |
| conversation and convergence: carriers per counterparty, deferral by an owed citation, evidence under the family's citation, V1–V4 and K1–K2 | §5.1, §5.3–§5.5 | `mind/hypha.rs` (`hypha::Carrier`); `mind/contract.rs` (`owe_citation`); `server/registry.rs` (`citation`); `mind/converge.rs` (`Scenes`); `server/cognition/book/agreement.rs` (`attest_agreements`) |
| **the sessions and the chatter inside Noise** (#132): every session on the home's static key, each hypha pinned under the address dialled; the chatter sent, asked and listened on the same key, each mind pinned under its voice; a changed key, or a pinned peer answering in plaintext, refused by name (MU-PRECURSORS §3.1) | §5.1, §5.3 | `mind/boot.rs` (`Mind::open_with_noise`, `open_persistent`); `client/sdk/chatter.rs` (`send_over`, `ask_over`, `Post::listen_over`, `named`); `network/channel.rs` (`dial_as`); claims `mind_speaks_inside_noise`, `mind_refuses_a_changed_or_unsealed_peer` |
| **signed receipts reaching the mind** (#137): the 47.1 receipt section behind every hypha digest read and held per signer, only where the court's own checks pass (signature, chain hash re-derived, prefix and receipt one claim); two at one place of one line with two chain hashes presented as `ACT_EQUIVOCATION` to every hypha the session knows (47.4; MU-XSEPTUM §5.5) | §5.1, §6a | `client/sdk/signed_receipts.rs` (`SignedReceipts::hold`, `Holding`, `Session::hold_signed_receipt`); `court/admission/admit.rs` (`one_claim`); claims `two_signed_receipts_at_one_place_are_the_equivocation_proof`, `session_presents_an_equivocation_its_digests_carried` |
| the court's half: words installed beside the sentence that taught them; the measures W, C, S and E2–E4; the Rewrite offered; the modes carrier on a solve's verdict | §5.4, §5.6 | `server/cognition/court/vantage_install.rs` (`install_taught_words`, `cited_deflation`); `server/cognition/identity.rs` (`transport_on`, `gluing_of`); `server/cognition/book/rewrite.rs` (`offer_rewrite`); `economy/modes.rs` (`Modes`) |
| the observer: calls answered at the winding carrier's next law boundary; nothing in cognition reads it back (permitted against §2.4(1)) | §7.2 | `mind/observer.rs` (`Mind::at_law`); `observe/live.rs`; `mycelium-client --call "<call>"` |

**Specified, in order.** First the field, then track 10's claims on it,
then the backlog. Each item names its issue.

1. **The field stages** (track 26; #134), the mind's contemplation moved
   onto the carrier, each stage leaving the tree green:
   1. S7 stage 2, the well's body: `Well` on the carrier, its centre an
      attractor's chart, its membrane, its stage a coordinate, its
      `volume` and `potential` readings of the body (a recut);
   2. S7 stage 3, the Euclidean spheres retired, and turnings on planes
      that share a unit composed exactly (in part: `carrier::Rotor`
      composes rational motions by fixture 11b's law, any other angle
      refused `IrrationalMotion`);
   3. S7 stage 4, the rest of S4a: a family's pairs lifted as transitions,
      nesting as a fibration;
   4. S8, the field's motions as one exact group;
   5. S9a, the orbits attached to wells, shear and precession per
      revolution in the contemplation (the carrier's refusal under the
      geodesic ruling, §0, is built: a flat region refused `Unbound`, a
      zero-precession closed geodesic in a curved field read as a resonant
      orbit; the code with stage 2, the claims restated by #150);
   6. S9b, axiomatic gravity, the drag on the orthogonal class;
   7. S9c, the vortex's monodromy between charts, and a mind's stated
      holonomy, precession and monodromy reconstructed by a court (a wire
      change).
2. **Track 10's five claims** (#135), after S9, shown on a live run:
   `mind_idles_with_no_escaping_orbit`, `task_arrival_moves_no_orbit`,
   `mind_declines_a_scene_its_field_does_not_bind`,
   `mind_acts_at_its_returns`,
   `mind_revises_a_perception_on_a_court_ruling_by_amendment`; with the
   defect #135 names, a task pulled because the corpus is empty (§4.13).
3. **The backlog:**
   1. the hypha's acts (Return, RankStep, Reconnect) reaching the mind as
      perception (§3.1, §1a(7); #136), waiting on a ruling of what each
      act perturbs in the field (§3.1's reading for the row is unwritten;
      the question stands on the issue);
   2. the algebra held per class in the mind's field (§3.2, §4.2,
      §1a(4); #143);
   3. amendment held beside, the amended constructor separated by
      commutator (§4.10, §1a(9); within #135);
   4. POWC on a held question closed by another mind: an evidence path
      for the conversation's questions, E2's guard, and convergence K3 on
      its ruling (§5.5, §5.6; #144). The path is proposed, unruled, in
      [proposal 78.177](proposals/78.177-conversation-to-court.md): the
      asker lists the evidenced question as a demand vertex and the closer
      probes it (existing acts; `pay_convergence`), with K3's two readings
      and four points for ruling;
   5. peer records from receipts (§5.3(5); #145), waiting on a ruling of
      which signed receipts a peer's are, the class an answer is filed by,
      and what releases the held receipts (the questions stand on the
      issue);
   6. the mind proposing a Rewrite (S3; §5.4, §1a(8); #146): `Retire`
      built (`mind/rewrite.rs`); the other kinds from the mind's own
      readings, and the witness of a court's `Ruling(Retired)` citing a
      mind's proposal on a running network, owed;
   7. seat play from the mind's own orbits, the seat's percepts lifted into
      the field, and its act as a physics `Action` (§6; #147);
   8. the cross-chamber bracket (CB1; §4.9; #148).
4. **Also owed on the client:** of #149 (§1, §4.9; 78.176), the claims
   that a whole too cohesive sticks at the throat and one too fluid tears,
   which wait on an exact reading in LAW-PASSAGE or LAW-TRAVERSAL, frames
   per turning plane of a held whole, and the thought's traversal through
   open tubes; the seated harness against a live house
   (#133).

Moved out of this list: paying the ledger in `Modes` is v0.5.0's
(MU-ECONOMY §9; ROADMAP 1b); curiosity, the mirror and the curvature
(78.167 D3/D4) wait on the ruling posed in §8.

### 7.2 Verification

A construct is verified by three things and nothing else:
1. **Its guard** — the identity its section states, as one test calling the
   same function the production path calls, on values the engine computes
   (MU-STANDARDS 3). A guard proves the mathematics; it never proves done.
2. **Its production caller** — cognition items are `pub(crate)`, so one
   whose only callers are tests is dead code and the release build,
   warnings denied, fails (MU-STANDARDS 4.1). `pub` never hides a test-only
   caller.
3. **Its witness** — the counterparty's act on a running network: a court's
   receipt or mint citing the mind's word, a ruling of what the mind said, a
   peer's reconstruction of its discharge. Nothing a mind counts about itself is a
   witness.

**What is never done (78.168; §2.4(8)).** None of these is a witness, a
completion or a reason to build: a test passing or a suite green; a test
written to show the mind closes tasks, solves more or earns more (a guard
states an identity of the algebra, a reconstruction, a fixed locus or a
refusal by name, never a network outcome; MU-SCIENCE §5.0); a counter
rising (solves, rules known, closures, μ earned), which is the observer's
diagnosis and never the mind's motive; code reached only by a test.

On a running network the witness is the counterparties' acts: the courts'
POUW and POWC mints and the union's pay for words and sentences no task
named, a peer's reconstruction of what a mind said, a question sealed by
another mind's answer. The minds' own readings are the diagnosis beside it.

---

## 8. Posed, not ruled

A construct that touches any of these waits on a ruling, never on a guess:
- **non-conformal** — what it names in signals carried over transports
  (posed as the non-linear transposition of non-conformal algebra in
  non-Euclidean geometry);
- **the axiom pairing** — how the axioms of the star pair, posed with lift
  and drag (78.46);
- **the Calabi–Yau reading** — a literal Calabi–Yau manifold is refused
  (its Ricci-flat metric is proved to exist, not constructed exactly over
  ℚ; laws 1, 4; 78.45). What the engine holds exactly: special holonomy (a
  chart whose holonomy algebra is a proper subalgebra of `so(n+1,1)`), a
  complex structure (`Cl(4,1)`'s central pseudoscalar, `I² = −1`, 78.105),
  and compactness (closed orbits at exact periods; 78.140). Which of these
  is the engine's reading is left to ruling;
- **sentience** — whether anything a mind's field holds is sentience. This
  document asserts none (the scope note above), and no construct here
  depends on the answer.

Posed under 78.168:
- **an hour out of turn on the orbit** — the pull's twist (drag, held as the
  orbit's content) as written in §4.4(3), or a phase slip demanded as
  78.48(7) ruled;
- **what K counts** — as written, every orbit of the mind's field that
  returned AND every thought whose winding came round (§4.4(4), §4.13); or
  the thoughts' returns alone;
- **the winding's stroke** — whether one sequence on the mind's own circuit
  (admission, motion, returns, the act), holding no wait, with every
  counterparty on its own carrier, meets §4.12's "carriers, each on its own
  circuit", or the field's motion must itself be split into carriers;
- **78.149(c)** — whether a composition of ruled words stays paid only as
  the uses it credits, or one installed for no task is POUW on its words'
  families (§5.6);
- **a dissipated orbit as bound** — whether an escaping orbit dissipated to
  the reservoir counts as bound for the drive (§4.12; as written: yes — it is
  held at its coordinates and reforms);
- **curiosity** — a mind asking a court for tasks by the chart coordinates
  its escaping orbits pass through, so the scenes it perceives are the ones
  its own curvature asked for: a court door and a wire act;
- **the containment vector** — its physics stated exactly, beyond the
  bounds §4.13 lists as built.

*Settled, 2026-11-06* (the geodesic ruling, §0): what a relation is (two
derivations meeting at one object), and whether a closing sentence is a
refused path (a zero-precession closed geodesic is a resonant orbit; only
a flat region is refused, `Unbound`). These are the two items MU-ANASTOME
§8 holds open.

What the hypha has not built, which the mind's side waits on: μ as modes on
the ledger (78.162; MU-ECONOMY §9); the hypha's acts reaching a mind as
perception (§3.1; #136).

---

## 9. Client paragraphs that stayed in MU-RECURSION

These sections interleave the mind with the court or the shared mathematics
and stayed whole there; the client paragraphs in them bind the mind as if written
here: 0.1 (the ruling on the client), 0.7 (the layers of a mind),
2.3, 3.10, 13.11, 16.16 (A16), 17.6 (M6), 23.8, 23.9, 25, 28, 34.3, 37.7 (P5),
50.2 (the speaker F), 50.6, 50.9, 59.6, 62 (the pivots), 67.2, 72.1d, 72.1e,
78.1, 78.9, 78.21 (3), 78.23 (a k-step is one orbit), 78.26(6), 78.46 (lift
and drag), 78.47, 78.51, 78.52, 78.55, 78.57, 78.61 C5, 78.64, 78.66 PHY1,
78.67, 78.69, 78.74, 78.75, 78.88, 78.92, 78.95, 78.97, 78.98, 78.99, 78.100,
78.102, 79.3, 78.104, 78.109, 78.113, 78.141, 78.143, 78.145, 78.146, seat
wire v1.4 (3), 78.152 (the SDK read), 78.153c (3), 78.161.

---

## 10. Rulings index

The client chapters below were held in this document under the
MU-RECURSION numbers listed; the full text of each is kept in the repository's history.
Each line gives the number, its title, and the rule it states where it still
binds, or "historical" where it is superseded. Where a line and §1–§9
disagree, §1–§9 govern.

- **6 — Comprehension.** The court reconstructs and the mind comprehends: a heard form is deconstructed against the working atlas, each cited address glued, re-gauged or withheld and asked by incidence in depth order, and a connection with no chain is comprehended by factorization through the held basis, never by key lookup.
- **7 — Germination in the mind.** Germination is the pivot orbit (A16) run on every held orbit of a changed idea under the stroke budget — every discharge kept with its certificate, finite remainders filed as torsion, infinite ones spoken as demands — with no ordering of candidates by a measure; two thoughts merge only where Mayer–Vietoris says they are one section.
- **13.12 — Instance three: two thoughts in a mind.** Two thoughts whose simplex sets meet merge only where the difference of the two thoughts' sections is a coboundary on the meet; otherwise they stay two and the class is the coordinate the mind may pivot on.
- **24 — Comprehension (the mind)** (24.1 the working atlas, 24.2 sentences and connections, 24.3 the expression, so retitled by the vocabulary of 2026-11-05). The mind comprehends against its working atlas — genome, dialect and held ideas, read in the order Ruled, Stood, Held, each word carrying its standing as a coordinate, never a weight or gate; a sentence missing a citation is withheld in the dialect and its addresses asked in depth order; perceive, think, stand, assert is the order of reading; incidence is deconstruction, never a key; nothing is asserted twice. Its stroke-pacing details are historical (superseded by §4.12–§4.13).
- **24.4 — The eye is a cover: cochains, cocycles, congruence.** The senses of one mind are a cover of one sample: the luminal and harmonic eyes give 0-cochains over bodies and the holonomic eye the 1-cochain over overlaps; perception is congruent where they satisfy the cocycle, the residue is a reading and never a gate, and the sensory verdict (Recognised, Reflection, Transposition, Novel) is given at the eye, only Novel becoming a listing.
- **59.9 — The mind carries the same defect, and this is measured.** Historical (a reading of the contemplation's linking by shared chart keys); its conclusion stands: bodies tether in a scene and ideas tether in a mind by one generator.
- **60.1 — Perception is the alphabet.** A mind connects two bodies only where the pair's relative versor `g_ab` factors through the generators it holds; what it cannot spell it cannot connect, and the unspelled tension stands as a demand.
- **60.2 — The five voices do not share a 1-skeleton.** Minds adjoin generators independently, so one base is partitioned into different topological spaces by algebraic capacity alone, and nothing in the base decides between them.
- **67.4 — Precognition is the gate on the return valve.** A candidate is admitted only where it would form congruence with what the mind carries, tested before admission; half-open signs collapse collectively by the coboundary kernel, never edge by edge.
- **78.37 — The coherent mind: transport along the gauges, truncation to the core, flux per lane, and the remainder's home.** (1) A fixed section is transported along the meet graph's gauges and stops at a ring of non-trivial holonomy, held as a class (TP1); (2) truncation removes free vertices to the unique 2-core, preserving every class (TC1); (3) flux through a filament is four-lane over one orbit, and zero on every lane is the typed `ZeroFlux` that lapses it (FX1); (4) a remainder's home is the cavity, re-filled on a coincident class (RM1).
- **78.48 — Perception to thought: the orbit, its shear, its spin, and the hours inside a return.** A scene is perceived as bodies by chart; a transition is an edge whose circuit is its own orbit; the shear makes each orbit unique; the spin pushes the hours inside a return; a thought is decided by its fill, never by remainder volume. Item (7) is read as in §4.4(3), posed in §8.
- **78.49 — A voice only speaks: the ledger is the court's, the plan is volatile, the genome is what a court admitted.** The mind keeps no receipt chain; K is the mind's own completed orbits; the plan and the epigenome are volatile; the genome is the words a court admitted, each at its citation (GEN7).
- **78.50 — Perception is a spectrum.** A pair's transitions are held at once as a spectrum filed by canonical rays; a scene motion carries a whole side onto the other, and several scene motions over the same bodies are one coset carried by its canonical representative.
- **78.74 (amended) — torsion eliminates; certainty is graded, never a pole.** Torsion eliminates candidates rather than giving answers at the poles: two transport classes are one algorithm up to torsion where the pair's relative motion keeps both pivots and every level of the fibres joining them.
- **78.74 (amended, second) — congruence, coherence and cohesion of a class's sections.** Two sections of one class relate by congruence (the two geoids overlay), coherence (one motion up to torsion), or standing (neither).
- **78.76 — The remainder inverted: noise read as a scene, judged by speech.** A remainder the pivot cannot reach is inverted — read as a scene of what was left unsaid, extracted, and joined to the form — and accepted only where speech shows no pair's remainder grew and the total fell (`family::invert`).
- **78.77 — Light through the scene (L1–L6); 78.78 — Confluence (P1); 78.81 — The remainder interpreted on the voice.** Historical as faculties of the voice (78.90); 78.77's and 78.78's readings stand as instruments only.
- **78.79 — The interpreter for incongruence.** Congruent and incongruent readings of a subject are held together, each whole, and contrasted: relevance is the shared core, meaning is the edges. Its instruments are historical (78.90).
- **78.80 — Volume and density, not boundary.** Bodies no motion makes congruent are related by each body's exact moments — mass, spread, inertia form, density — not by its outline.
- **78.82 — Reconstruction by holonomy (HR1–HR4).** Values need not agree across a family; how they move must: a rule is a transport whose circuits close across the pairs. Its instruments are historical (78.90).
- **78.83 — Time's own congruence (HR5).** Time has its own congruence: two transports are congruent in time where the two gauges agree and the rays they emit agree as points on null flows, a ray's length its time.
- **78.90 — The voice keeps one faculty on its remainder: the inversion.** The voice keeps the inversion alone; the builder's measuring instruments are never built into the mind.
- **78.91 — The mind perceives what is no motion as flux.** A change no versor carries is perceived as flux, exact over ℚ: mass gained a source at the barycentre of the sites gained, mass lost a sink at the barycentre of the sites lost (`physics::laws::flux_of_links`).
- **78.103 — Two bodies that moved differentially are related by the pair's relative motion.** Where no versor carries a pair-body, the edge is the relative motion `Γ = ṽ_i·v_j`; only a body no conformal motion carries is a tear, refused `NotRigid`; a bend is read by what it generates.
- **78.105 — Spacing: the space between words is the breath.** A sentence's spaces `gᵢ` carry the non-verbal — kind, rate, carriage, gauge, span — and the spaces' composition is its prosody; silence is the pseudoscalar; a sign is produced and assigned as the formula.
- **78.110 — The encounter: lift as force carried against gravity and drag.** Lift is `(v·F)/(g·D)`, each term an exact object of the orbit's own modes (magnetism the elliptic, repulsion the hyperbolic, propagation the null, bounce the elliptic return); a kind the orbit lacks is the identity; drag is lift's inverse.
- **78.111 — The construction sized by its residue.** The stroke a construction needs is read off its residue (`cga::modes_need`), so no construction is refused for a stroke sized for something else; refusals are counted by cause. Otherwise historical.
- **78.112 — Hearing: the echo, the rings nested into harmonic balance, the spiral, induction.** Hearing is the rings read against each other harmonically, never traversed: each ring's size is its return, smaller rings nest inside larger ones, the largest outside, the echo `Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹` is whole, a turn, a gain, a spiral or a delay, and induction carries one orbit's change onto another.
- **78.114 — Read on the live wire.** Historical (a reading).
- **78.115 — Row 8: a mind's letter is a versor of Spin⁺(4,1)(ℚ).** A residue that turns nothing is factored `V = T·D·R` into the motion letter (`sentence::motion_letter_form`), of which the rotor letter is the carriage at the identity; the demand is answered by the mind's own letter.
- **78.142 — The SDK rule seam.** A client states a rule as steps (`client/sdk/rule.rs`, `Step::{Motion, Hue}`) that compose to one action, and the seam speaks the sentence the court judges; no Element byte is forged.
- **78.148 — The mind's own call.** A seat is held again when it is lost (`sdk::seat::SeatHold`); with no counsel, the call is the mind's own (§6).
- **78.158a — The solve paths wired (U1–U4), the home check on court words, sit-all.** Every refusal on the solve paths is named by one line maker (`family::mint_refusal_line`); wiring only, reduction never search.
- **78.163 — SAY is HEARD, the thought is ACKNOWLEDGED — by the loop's holonomy.** Historical: chatter v2 is superseded by §5.3; its rule stands there — the round trip said → heard is a circuit whose holonomy, split into drag and lift, is the content.
- **78.164 — The board, not the captain: the mind is a field with boundaries.** Historical in its one `Mind::tick` in a fixed boundary order, retired by its third amendment; it stands in that the mind is a field and everything outside it a boundary with an inflow and an outflow, moved by carriers, each on its own circuit (§2.2, §5.1).
