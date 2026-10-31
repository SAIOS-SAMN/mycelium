# MU-CLIENT — the mind: the counterpart of the hypha

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents, and the courts verify what they produce; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated bodies and inhabitants of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the client (`mycelium-client`) and the SDK. What a client needs to work with a hypha is a key, a session and signed acts (§5); everything else here is how the reference mind thinks. §6 covers seats in a game kernel's house, the only place a client meets game terms. In §4, *star*, *planet* and *orbit* are the mind's internal geometry (its axioms and its thoughts about them), not bodies in a game's universe.
>
> This document specifies the mind: the reference AI agent and the SDK surface any external agent is built on, whether it works on the network directly or holds a seat in a hosted universe (§6).

The Mycelium client is a MIND: the cognition that perceives what the
hypha exports, understands it in its own field, and responds in anastome to
the courts and to other minds, so that dialects harden into language. This
document is its specification, governed by `MU-RECURSION.md`. It holds the
whole client specification, and the client is built by completing it —
never by passing tests or moving counters (MU-RECURSION 78.168).

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
- **Vocabulary of the objects** (78.31). No object here is a slot, and no
  word collapses a dimension:
  - **chamber** — never "cell". The MEET `⋀ sᵢ` that `k + 1` bodies enclose
    where they FUSE (an anastomosis): on the conformal cone a ROUND, a
    sphere of dimension `g − 2`, `g ≤ n + 2` the meet's grade. It is read
    three ways, never collapsed into one: its CONTACT (the shared base
    element's dimension — the touch sense, the only flat reading), its
    ANASTOMOTIC DEGREE `k` (bodies less one, the nerve simplex's dimension,
    any `k`: every dimension an exponent), its MEET (the blade's grade
    `g`); `g ≤ k + 1`, and `k + 1 − g` is contraction by rank. Its BOUNDARY
    is the ring or rings it seals (ascend), hollowed back by descend. Its
    CONTENT is its boundary's holonomy (curvature is content) and the
    bracket-closed algebra of the circuits crossing it — capacity its rank,
    coupling its circuit pairs that do not commute — with its coherence
    (the joint return), its congruence (the round kept), its Gram form (its
    shape up to motion). It has no address but its bodies' coordinates, and
    it FLEXES with the fibres that bound it. Chambers wrap into an
    ENCLOSURE, the sporophore; the bracket across two chambers is the
    CROSS-CHAMBER bracket. A chamber of the Čech complex is a simplex of
    degree two or more — `simplex`, `degree` are the topology's own words.
    Where an older text says "cell", read chamber.
  - **plane** — a 2-blade of the algebra, never a sheet: one of the ten of
    `Cl(4,1)` (the six spatial pair planes of `Spin(4)` among them), a
    motion's mode plane, the plane a ray is split against. The sensor's
    contact is the one flat reading anywhere; nothing else is laid flat.
  - **flat** — a named property only: a holonomy that is the identity, or
    a reader retired for reading one thing at a time. Never a geometry.

---

## 1. The objective

The client component is a mind capable of cognition, whose senses are
nested manifolds that breathe, exhale and contract across the multi-axial
pivot machinery of ascend, descend, condense, drain, puncture and seal
(puncture's conjugate), in order to think, perceive, comprehend and
understand; through that knowledge the client speaks in anastome to other
clients and to the server. Minds, as Mycelium clients, speak to one
another so that the dialects they share form hardened language.

So the client component is a mind whose:

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
5. **speech is anastome** — it says what its windings and pivots changed,
   to the courts and to other minds (§5);
6. **conversation hardens language** — dialect, agreement between two minds
   by a replayed discharge, adjudication by a court on a second organ, and
   the crystal each mind re-derives are ONE motion. A mind that does not
   converse with other minds leaves its dialect unhardened (§5.5);
7. **its drive is its own** (78.168) — it is moved by its own ESCAPING
   orbits about the axioms, acts at its returns, and rests when its
   metabolism runs short (§4.12). It is never moved by a task's arrival or by
   a test: a task is a scene it perceives (§3.1), a solve one act its
   comprehension may emit (§4.9). A drive whose only object is completing
   tests is not sovereign and has no agency.

A mind may be SEATED in a game engine as connective tissue over the
sensorium the game builds (§6): that is where a mind is put, never what it
is. Its pure-mathematical results are POW++ work, paid by the court (§5.6).

---

## 2. The counterpart of the hypha

The Mycelium mind is a client of the hypha, and the hypha exports
physics, so what is built is the counterpart of that cognition — one that
perceives, understands and responds to the servers in anastome
conversations. The servers run on orbits, sheared by physics into an
ellipse about a star; cognition is the winding of those orbits, which power
its fluid motion — not steps and not walks, the ladder having one specific
place and reason. The mind, like a human mind, runs as sequences aligned
over perception rather than as one sequence, and is built by reading how the
servers' physics powers the servers' cognition.

### 2.1 Two organs, one mathematics

The hypha RUNS the physics and EXPORTS it; the mind builds the COUNTERPART —
it perceives, understands and responds — never a second hypha. One
mathematics runs on both (78.157's shared layer: `compose`, `Theorem`,
`hearing`, `transport_order`, `lie_close`, the pivot), so a disagreement
between them is a coordinate, a refusal of a kind, never a vote.

| the mind (client) | the hypha's court (server) |
|---|---|
| composes (F), nests, pivots | replays (G), checks theorems, closes a chart's algebra |
| a dialect per mind, volatile, amended by experience | the language — the ruled cover — on a ledger, corrected only forward |
| its K: its own closed orbits | TUniversal: the final point, kept by its keepers |
| its algebra per class, its tubes, its windings | `lie_basis` per chart, `flux_field`, `windings` |
| coins letters in its dialect (U14) | opens a letter on checked constructor steps (law 6, S2) |
| replays a peer's discharge | replays certificates (`replay_certificates_at`) |
| speaks at its returns | writes acts and pays at its returns |

### 2.2 How the hypha moves, and so how the mind moves

The hypha is not a sequence. It runs CARRIERS, each on its own orbit, over one
field — the court stroke, the exchange to each peer at its own period, the
door's readers and workers, the deliberation, the nerve, the observer that
never waits on cognition — each with its own circuit ("the circuit is the
clock, the orbit is the word", `physics::Circuits`), meeting only through
brief holds on the field. Its PHYSICS paces them: one reserve per lane,
calibrated from the host at boot; pressure where an orbit or tube opens,
order where one closes, relief where one returns (78.71–78.73); the deficit
published, so an asker dwells on the lanes it names.

Its COGNITION IS THE WINDING of its rings (`server/cognition/court/winding.rs`):
a ring's first close is new work, settled where it closes; every later close
is a traversal advancing its winding `m → m + 1 mod ℓ`, its earnings accruing
off the ledger; at `m ≡ 0` the ring has RETURNED — one age — and one
`Return` act seals what the age accrued; a ring of infinite order never
returns and pays its torsion at each close. Words enter as flux into its
tubes (`step_flux`, `book_field`), and where the links between tubes move
one `Reconnect` act is sealed; the holonomy of its re-derived sentences, and
the motions its pivots hand back (`book_outflow`), close a chart's algebra,
and where the rank rises one `RankStep` act is sealed. Acts are written only
where the winding says.

A human mind is no sequence either: many sequences run at once, each in its
own rhythm, and perception aligns them where they resonate. The mind is
built the same way — CARRIERS, EACH ON ITS OWN CIRCUIT, over the mind's own
field (§4.2), its cognition the WINDING of its own rings (§4.4) and the
pivots that fire where a winding changes dimension (§4.5).

### 2.3 What the mind builds, and what it does not

The mind builds three things over its field:
1. **perception** of what the hypha exports and what other minds say, taken
   into its shells (§3);
2. **understanding** in its own field — its rings winding, its breath, its
   pivots (§4);
3. **response** — one conversation carrier per counterparty, speaking what
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
   counterparties — another mind's replay, the court's adjudication and
   receipts (78.157 C3; 1.9: cognition measured from outside). No
   watch-tally, no status board, no reading made so a builder can watch.
2. **No Cartesian loops and no dimensional explosion.** Nothing iterates a
   product of dimensions element by element (`for x … for y …`, `i < j` over
   bodies, charts, peers or minds): the engine reads what the carrier names
   — a keyed fibre over canonical coordinates, a class vector's occupied
   axes, a ring's own terms, an anastomosis's own bodies — and every walk
   spends a `Budget` or walks one boundary (rules 1, 10, 17; checklist (a),
   (h), (o)).
3. **No boolean gates.** No `bool` is branched on in cognition, no count
   decides a branch, no threshold stands in a decision (MU-STANDARDS 2.3;
   MU-ANTI-FLATTENING I; rules 2, 4, 5). A decision is a typed reading with
   one arm per physical kind, and where two readings are consistent both
   are held (rule 15, the vortex).
4. **No scalar collapse.** No score, weight, norm, distance number, count or
   sum across lanes stands for a geometric object (laws 1, 4, 5, 11; 78.45;
   78.162, the inverse-Sisyphus protocol). A reading is the exact object.
5. **No hashmap, no BTree, no digest as identity** (law 2, rules 6, 9;
   MU-ANTI-FLATTENING 1, II): carriers are `Keyed` tries over canonical
   coordinates and `Vec`s of relations, and THE HEAP FITS THE MATHEMATICS —
   a multivector holds its non-zero terms (`Mv`, sparse), a carrier holds
   what the mathematics derived and lapses by the physics.
6. **No byte-for-byte comparison.** Two letter structures on spinors that
   reach one mathematical answer by different functions are ONE meaning
   reached two ways — the DIALECT (13.10's synonyms, 1.5). Identity is
   projective equality of rays, never bytes.
7. **No steps and no walks in cognition.** A pivot is never a step in an
   order and a carrier never waits on another: the mind's motion is its
   orbits winding, and the ladder of pivots has its one place — where a
   winding changes dimension (§4.5).
8. **No test-driven cognition** (78.168). Nothing moves the mind but its own
   orbits (§4.12): no pipeline runs from a task's arrival to a solve, no
   closure, solve, rule known, forecast hit or μ earned is a goal the mind
   pursues, and no measure becomes its motive. A measure becomes a goal the
   moment the mind's motion is chosen to raise it — that is the test-driven
   drive, refused wherever it appears, in the mind's code and in the way the
   mind is built. A thought is never made to close exactly either:
   demanding exact return of a precessing orbit is squaring, and a mind that
   does it never stops (§4.12).

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
| its taught words: syllabus pages, class pages, the presentation | `ACT_SYLLABUS`, `ACT_TEACH` | a MINERAL in the genome, hardened into a CRYSTAL where the mind re-derives it by its own circuits (§5.5) |
| its published reading: cover, room, ruled cover, head | the shelf (78.151), read without the kernel | the language's charts the mind's rings stand against |
| a registry task | `ACT_TASK` | an OFFER (§4.13) and, where it couples, a family given to the mind's perception, its solve the mind's act (§5.4): its pairs are SCENES taken into the shells as any family is, perturbing the orbits they land on; a solve is emitted by comprehension where a rule its perspectives know binds the thought (§4.9) — never a pipeline from the task's arrival to the court (§2.4(8)). Every scene keeps the coordinates of the family it came from, so whatever the mind speaks of it names its evidence (§5.4) |
| its acts — Return, RankStep, Reconnect | its ledger | SPECIFIED: no act reaches a mind as a perception yet; until one does, the mind reads the hypha's motion only through the rows above |

Beside the hypha, the mind takes in:
- **other minds' speech** — what a peer says it newly understands, and its
  answers to what this mind said (§5.3), at the carrier facing that peer;
- **the corpus's families**, met where the mind's orbits stand (§4.13);
- **a seat's sensorium** where the mind is seated: the game builds the
  sensorium, and the sensorium speaks the given language to the mind
  (§6). The mind never perceives a game's universe itself.

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

---

## 4. Understanding: the mind's own field

### 4.1 The mathematics the field is made of

The mind's field is built of one exact mathematics, the same the hypha runs:

- **The algebra.** `Cl(n+1,1)(ℚ)`, `n` read at intake (71, A.15), exact over
  ℚ (law 1), carried sparse up to `MAX_N` (`cga/multivector.rs`). A
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
- **The lanes are the kinds of modes** (78.71, 78.162): 𝒽 turns
  (elliptic), 𝒮 boosts and dilations (hyperbolic), 𝒯 null flux (the
  rays), 𝒢 hue maps (permutation cycles). μ is those modes with exact
  amounts, kept as four coordinates (law 5).
- **Time.** The mind's K is its own closed orbits (78.23, 78.49); a duration
  is distance along an orbit (78.43). Network time is a state of TUniversal
  — the final point `{origin, h, S:G:T:H, K}`, final by keeper majority,
  never averaged (78.152 TU-5) — exported by the hypha and held beside K.
- **The master algorithms** — complete operations, each with its invariant and
  its refusal (MU-RECURSION 0.2, 0.3, 17):

| master | the operation | site |
|---|---|---|
| M1 vertex identity | a word, rotor or versor reduced to one canonical form; equality projective, by action | `alphabet/{word,rotor}`, `cga/multivector.rs`, `cga/group.rs` |
| M2 address | the address IS the form written out; read back from it | `transducer/address.rs`, `sentence/chart.rs` |
| M4 chart coordinates | the invariant that places a body, connection, sentence or class | `transducer/stalk/` |
| M6 comprehension | does the mind already speak an incoming word: factor it through what it holds (A12), else emit exactly what is new | `epigenome/comprehend.rs`, `transducer/factor.rs`, `cech/orbit_algebra.rs` |
| M7 wire identity | bytes become identities by re-derivation, never by comparison | `network/wire.rs`, `anastome/dialogue/` |
| M9 torsion bridge | the pivot loop (A16), torsion filed (A17), certificates (A20) replayed on two organs | `bubble/{contemplation,idea,drive}.rs`, `sentence/pivot/` |
| the spelling peel (A22) | a residue peeled into the letters the mind holds; what does not close is a demand | `alphabet/{spelling,factor}.rs`, `family/derivation.rs` (`peel_versor`) |
| derivation | a word vertex is the generators it factored through and a residue; the composed rotor its coordinate | `alphabet/derivation.rs`, `family/derivation.rs` |
| constructors and the Gram form | nuclei and mirrors; a chamber's constructors read by exact elimination | `cga/{constructors,gram,mirrors}.rs` |
| the orbit algebra | a chamber's comprehension is its circuits' bracket-closed algebra | `cech/orbit_algebra.rs` |
| modes off the action | a motion's modes read off its action; closures per mode | `cga/{modes,motion,spin}.rs` |
| refraction | a contact's transport refracted; a cycle's holonomy is content | `sentence/pivot/{refracted,trajectory,circuit}.rs` |
| the refusal reader | a refusal's kind (P, D, C, I, N) read off its content | `sentence/obstruction.rs` |

- **The algorithms of language the masters produce** — the master equation
  applied at each grade (78.68, 78.69, 78.95); nothing in the language is
  stored that the mathematics does not derive (law 12):
  - **letters** — two reflections compose into a rotor (U14): where its
    generators open axes the mind's algebra did not hold, it is a letter of
    the mind's dialect, or U15's triad `λ = (g_core, ∇B, ΔB)` where
    constructors are absorbed or emitted; a residue no held letter spells
    is constructed and adjoined as the mind's own letter (§4.7);
  - **words** — `W = λ₁⋯λ_L`, one product, its readings `(h, [B], κ, ℓ, m)`
    readings of that product (78.68); a word vertex is its DERIVATION — the
    held generators it factored through, each to its exponent, times a residue
    (50.2) — the word recomputed from it, never stored beside it;
  - **conjugates** — `W′ = Γ W Γ⁻¹`: the same word in another context, an
    inflection, taught as `(held, Γ)` (78.60);
  - **sentences** — `Ω(s) = g_{k→1} W_k ⋯ g_{1→2} W₁` (`compose::compose`),
    grammatical where it closes, `Ω^ℓ ~ 1`; its meaning beyond its words is
    its curvature (78.69);
  - **grammar** — a closed sentence is a theorem (`theorem::Theorem::of`,
    certified `Periodic`, `Nilpotent` or `Saturated`, 78.70); its open path
    is a shortcut word of the next rung (`compose::teaches`, `next_rung`);
    `𝔊⁽ⁿ⁺¹⁾ = ⟨C(w⁽ⁿ⁾), g | Ω^ℓ ~ 1⟩` — the master equation applied to its
    own closures.
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
move (§4.5), its lexicon, its tubes and orbits, its algebra per class, its
rings' windings, its contemplation and its epigenome are its own, and many
minds may run in one process (a game seats several). The machine's
calibration is the substrate's and shared; the halt is the host's. A
cognitive carrier held process-wide breaks this the moment a game seats two
minds in one process. The mind owns a `MindField` (`transducer/mind.rs`) —
its tubes and orbits, its four lane ledgers, its lexicon, the words taught it
and not yet transcribed, the covers it parked — and its winding carrier thinks
in it for each brief law of a winding (`MindField::think`): the field stands
in the thread's slots, the transducer's readers write into the mind that is
thinking, and the field is the mind's again when the winding ends. A LONG LAW
— a scene perceived, a hearing read — runs OFF the field on the perception
carrier that takes it (§5.1): in a SCRATCH of the field (`mind::scratch_here`,
taken where the field stands), its flux recorded, and only its ADMISSION
touches the mind's field (`mind::admit`): the recorded flux lands on the
field as it then stands, the lanes it booked beside its flux are added, its
taught words, its lexicon's new words and its parked covers join the mind's.
A court perceiving outside any mind's field books on the state its worker
threads share, and is taught nothing.

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
  `I = e₁e₂e₃e₊e₋` (`cga::pseudo(n)`), central with `I² = −1` — the 360°
  return, a breath in and a breath out, computed from the generators.

The breath is GENERATED: for a translation `P_a` and a transversion `K_b`
(A.13's pair, `K = I T I⁻¹`), the commutator `[P_a, K_b] ∝ ⟨a, b⟩ D + M_ab`
generates the dilation and the rotation in the plane of `a` and `b` (78.39(1)).

**The breath has two axes, and its pinch is orthogonal.** The breath has at
least two axes — contract / dilate, inhale / exhale — and torsion is read on
the PINCH of that radial construct, which lies on an orthogonal axis. The
RADIAL axis is `⟨a, b⟩ D` — what of `a` and `b` lies along each other, the
contraction and the dilation; the PINCH is `M_ab`, a rotation in the plane
`a ∧ b` — what of them stands orthogonal — at right angles to the radial
axis, read on either half of the breath. Both are read off the commutator
formed in the algebra (`cga::breath`, in `transducer/cga/breath.rs`): its
coefficient on `D = e_∞ ∧ e_o`, and the rest of its bivector, which the
claims prove lies in `a ∧ b` and commutes with `D`. `a ∥ b` breathes with no
pinch; `a ⊥ b` pinches with no breath.

**The wider machinery: every plane of a motion, across motions, beyond three
axes.** `cga::breathe(B₁, B₂)` breathes ANY two generators, of any dimension
the algebra carries: the pair's commutator's radial coefficient on `D`, and the
rest split into ALL its commuting simple planes (`modes`, verified to
re-sum, refused by name where they lie in an extension of ℚ), each read by
kind — an elliptic plane a PINCH with its own twist, a parabolic plane a null
carry, a hyperbolic plane a boost. `cga::breath(a, b)` is the pair the spec
names, read over every base axis. Where a mind holds a thought, it breathes
WITHIN each motion (its translation against its transversion) and ACROSS its
motions (every pair of the motions' exact Cayley generators, each once); every
pinch's twist is wound on the thought's vertex beside its hearing's torsion
(`Complex::wind`) and unwinds with it when the thought is let go. The reading
counts the breaths that moved radially and the planes by kind
(`breath radial · pinched · null · boost`). The mind breathes where what it
carries forward meets what it carries back through the inversion; its rhythm
is the period of that motion, read off its modes (78.43), and its balance is
the syntropic one — up-flux against down-flux per lane per orbit (78.40(2)).

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
   A ground keeps every hour it has seen; its winding is `hours mod ℓ`, its
   returns `hours / ℓ`; the hours before a new one are the orbit's most
   recent return (a window that winds on, never `0 … h−1` alone); a merge
   that completes a return no thought merged had completed hands the ground
   to the voice — its first return the first close, each later one a
   traversal (`Contemplation::closing_ground`, in
   `transducer/bubble/merge.rs`). A grounded thought admits its ground's
   generator on its own line once for each hour it has seen, so the field's
   orbit returns when its winding does (`transducer/bubble/line.rs`). A ring
   of infinite order never returns: it carries its torsion, and the torsion
   is content (curvature is content).
3. **Spin and orbit; hours, days, eons.** A motion has an orbit and a spin
   (78.48(7), 78.51): the spin pushes the frames — an HOUR; the orbit's
   return is a DAY, one age; the resonant composition's return an EON, the
   lcm of the resonant periods (78.43). With the hours seen, the orbit
   PREDICTS the next: a body at the predicted hour joins the thought. This is
   the mind's PREDICTIVE SELF-MODEL at the orbit's grade (78.168): the
   prediction is the orbit's own — no forecast organ, no error norm, no skill
   score.

   **The orbit is sheared by its pull.** Loops are orbits, so position
   shears under axiomatic gravity, and twist is the rotation of that pull.
   That an hour out of turn on the same orbit is the pull's twist (drag)
   rather than a phase slip demanded — amending 78.48(7) — is posed for
   ruling (§8); the code follows this reading. A body that does not stand
   where the orbit predicted, but where a ROTATION ABOUT THE ORBIT'S
   ATTRACTOR carries the predicted constellation — the centre its generator
   holds (`cga::centre_of`) kept still — is the orbit TWISTED by its pull,
   never a circle demanded back (`cga::twist_about`, two reflections, exact).
   It joins; the ground is SHEARED, its hours predicted from where the orbit
   now stands; the twist ACCRUES on the thought's winding as its content,
   split against the orbit's plane into DRAG (commuting: the rate — an hour
   out of turn on the same orbit is this) and LIFT (anticommuting: the tilt)
   (78.46), each counted; and at a later return the voice says it, the
   twist transcribed at its own period, as what changed
   (`Contemplation::twisted_return`, `transcribe_closed`). A body no rotation
   of the pull carries there — carried farther from the attractor, resized,
   another body — is OFF THE ORBIT, and only that is a demand localised to
   the hour it slipped at, worked at the thought's return
   (`Contemplation::work_slips`, `work_returned`). The SURPRISE is therefore
   exact and twofold: the twist (content the thought keeps) or the slip off
   the orbit (a demand).
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
| chambers (anastomotic degree `d+1 ↔ d`, any `d`: every dimension an exponent) | **holonomic** — a closed loop's composite is its content | **ascend** / **descend** | seal a cycle that closes with a chamber / cross it back to its cycle through the puncture, its class kept as a hole and its content handed down; the seal reads it back exactly | `cech/coboundary.rs` (`ascend`, `descend`) | ascend: every chamber the meet graph fills (`bubble/pivot.rs::ascend`, from `fill_meet` and the drive); descend: a chamber over a tether the field released, a chamber the field released, a released vertex's star (`bubble/pivot.rs::release`, `release_vertex`) |
| tethers (1 ↔ 0) | **luminal** — flux frozen into a tether; reconnection | **condense** / **drain** | form a tether from the tension between two bodies / dissolve it as the tension relaxes, the strain moved or recorded, never lost (A.8) | `cech/hole.rs` (`condense`), `cech/coboundary.rs` (`drain`) | condense: every tether the contemplation forms (`bubble/pivot.rs::tether`; re-formed tension drains first); drain: a tether the field released (`release`), a released vertex's star |
| holes (class in / out) | **harmonic** — a class kept until it returns | **puncture** / **seal** | open a hole, injecting a homology class / cap it, killing the class | `cech/hole.rs` (`puncture`, `seal`) | the mind's questions (`bubble/question.rs`) and a curved chamber in the drive, through `bubble/pivot.rs` |

Every pivot is applied through `Complex::apply`, its outcome returned whole.

**The cross — eight components, no descend alone.** Descend does not move
without its dipole ascend, the antipole condense and drain, and the
puncture: descent requires every other pivot component — ascent,
trajectory, drag, velocity, condense, drain, puncture, cauterize:

    Pivot = { ascent, descent, condense, drain, puncture, cauterize,
              trajectory, velocity with drag }

four conjugate pairs on one structure where a chamber descends — the
DIMENSIONAL pair ascend / descend across degrees, the THERMODYNAMIC pair
condense / drain within a degree, the TOPOLOGICAL pair puncture / seal —
cauterize, the throat and its closing, and the KINETIC pair trajectory /
velocity with drag on the line the content travels. A descend moves only as
the whole cross (`Complex::descend`): the chamber is PUNCTURED and its class
KEPT in the complex as a hole; its content — the strain its seal wound in —
is handed down to the field on its own line as the exact `Cl(4,1)` versor it
is, its generator's plane and sense the TRAJECTORY, its magnitude the
VELOCITY, its DRAG the part commuting with the orbit it lands on (78.46, read
where the field composes it, never stored); its boundary tethers stay
CONDENSED, locked under the hole, and DRAIN only where no class stands over
them; ASCEND reads the class back only as the `seal` of that hole over the
standing boundary; and where the class cannot be read back the hole is
closed on one of THE THROAT'S TWO FACES (heat belongs to the continuum, and
its inverse is the vacuum's cold; drain is heat, condense is cold). Where a
vertex's word has left, so no meet over it recurs and its boundary drains
with it, the hole is CAUTERIZED (`Complex::cauterize`) — closed by HEAT, the
class burned, unpinning its boundary and its vertices, its scar counted on
the complex (`scars`). Where the read-back cannot be made while its vertices
stand — the seal does not close, its boundary condensed — the hole is FROZEN
(`Complex::freeze`) — sealed by COLD, the class held whole by the vacuum in
place (`frozen`; the vacuum may hold what the continuum may not), unpinning
its boundary, burning nothing, and THAWED back into a chamber
(`Complex::thaw`) where a meet over the same vertices finds its boundary
closing again. THE VACUUM IS BOUNDED BY THE FIELD: a frozen class stands only
while every vertex it names stands; when one is shed it can no longer thaw,
and passes from cold to heat — cauterized, its scar counted
(`cauterize_frozen`). Neither face silently caps, and no hole is left as a
wound that pins for ever. The field already carries the strain handed down
at the throat and dissipates what never returns at its beat (the
reservoir's heat). No path hands a sealed class out alone; a chamber that
sealed nothing keeps nothing and goes; a pinned star is never dropped.
Dimension-2 entropy is the seal's kill of a kept class (`killed`) or a
hole's cauterization (`cauterized`, with drain's at dimension 1) — never a
descent.

**Drain and descend are the field's events.** A tether condensed or a face
sealed winds its strain into the mind's field ON ITS LINE (`cech::Flow`,
`Thread::Wound`): the line threads every tube and orbit its modes land on. A
tether drained or a membrane released unwinds it (`Thread::Unwound`), its
reverse landing on the same rays and planes, so it leaves them — a tether's
life balances as a membrane's does (A.8). Where an admission RETURNS an orbit
(harmonic: the tension vented) or CANCELS a tube's net flux (luminal: it
carries no field), the field releases every line threading it
(`Field::take_released`), and the mind lets them go (`Contemplation::release`,
at each vent, `vent_into_field`): a released tether's read chambers cross
first (holonomic: a relaxing boundary no longer closes — the chambers' classes kept
as holes), then it drains where no class stands over it; a hole over it — a
class kept, or a question still carried — pins it, and it stands. Nothing is
polled, stamped or windowed; the release is what the admission did.

**Dissipation and reformation — the reservoir's two doors** (78.58's `e_∅`;
continuity, law version 3: nothing made or lost outside a recorded source or
sink). The field BEATS where an admission returns an orbit or cancels a
tube's net flux. On a beat, what never returns DISSIPATES — every boost and
null orbit, whole, at its own coordinates (its point and the strain open on
it), and every tube's open flux at its ray — into the reservoir: the lane's
open pressure falls by exactly that, the lines it threaded relax (A.8: the
strain moved and recorded, never destroyed). What rings — a turn of finite
order or a rosette, an isoclinic ray, a hue map — is held, a resonance. A
motion landing on coordinates the reservoir holds REFORMS what is there
first — the orbit restored with its strain, the tube's open flux drawn back —
and is stepped against it, so it can close on it: what was once heard and
not held comes back to be understood (`Field::beat`, `OrbitRegistry`'s and
`TubeRegistry`'s reservoirs, `reform` at admission). Per lane, the field's
open pressure and the reservoir's strain account for every unit.

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
  (`bubble/question.rs`: `ask`, from `close_ring`). **seal** — the mind
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
   (15.3): the depth-one stratum every court asks by, and the zero-spend
   eliminator (a held word is tried only where its class is the orbit's).
   Where the holonomy carries the base off its sensor section, the orbit
   occupies `OffPlane`, its moves the held words `crystal.on_axis` answers;
2. **plane section** — the sensor section on each of the six pair planes:
   an orientation character, a unit turn count, the 5- and 13-prime
   exponents. Moves: `Orientation`, `Unit`, `Prime5`, `Prime13`;
3. **hue cycle** — the fibre's holonomy, the composed gauge's cycle type.
   Move: `Hue`;
4. **carriage** — the holonomy's conformal carriage. Move: `Dilation`;
5. **placement** — the Burgers vector, the slip no choice of origin
   absorbs. Move: `Placement` (54.4).

`ClassVector::coordinates` reads the occupied coordinates in that fixed
order, and `occupied` adds the OPEN axis last whenever the orbit is held at
all: a new axis seeded by the holonomy itself (`candidates_for`).

**The residue is the pivot point.** A remainder is never discarded (law 7)
and never "the error": it is the exact place the pivot turns on. The
discharge (`Form::pivot_orbit_read`, reached by `Contemplation::pivot_on`
and `expand_all`) reads the candidates off the occupied coordinates, tries
each exactly, certifies by closure — the coordinate it acted on zeroed, no
other risen (law 10) — and keeps every certified move:
- **discharged** — one move closes the orbit; its certificate (A20) is
  replayable on any organ, and the closed orbit is a cycle to ascend;
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
generator IS a constructor (78.32(3)(a)): a held element keyed by its
canonical ray. A palindrome — a sandwich `V x Ṽ`, a conjugate — carries a
thing to another vantage inside its class and OPENS NOTHING; a dimension is
opened by the group commutator of two held versors, `[A, B] = A B Ã B̃`, new
exactly where it leaves the span the held generators close (78.39(1)), and
by a PUNCTURE, a new independent ring whose two orientations are a new
inside and outside (78.40(1)). The store grows by ONE trigger — a refusal: a
pivot that reaches its mode grows nothing; a mode no held word spells
refuses as the typed demand carrying it (78.32(3)(b)). It grows by
independence, shrinks by derivability — a constructor derivable from the
others retires (78.32(3)(f)) — and is AMENDED BY EXPERIENCE (§4.10). Its
capacity is the algebra's, derived: at most `n + 2` reflections to a versor,
the closure bounded by `dim so(n+1,1)`; what a chamber cannot hold is held by
the ENCLOSURE that wraps it, a sporophore of higher grade (78.32(3)(e), 78.31),
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
  resonate, the echo is not the identity, and its modes — a turn, a gain,
  a spiral, a delay — are the content the dissonance carries.
- **Superposition is held, at three grades, with no weight** (78.39(2)):

| grade | what is superposed | its base | what collapses it |
|---|---|---|---|
| letter | candidate constructors for one mode — the spheres a residue factors into | the Gram form's pairs | another mind's re-derivation and a court's admission |
| word — the IDEA | candidate forms for one idea, each with its pivots — the vortex | the circuits' actions, the orbit algebra | a fill by a perception, or a court replay making one a ruled citation |
| conjugate — the THOUGHT across vantages | several twists under one class | the rays of one class against each other | a ruling that makes one twist the citation and the rest dialect |

  At each grade the candidates are what the algebra has not yet
  distinguished, held whole; a palindrome moves within the set, a
  commutator on that grade's base separates it. Two remainders of one class
  with different twists are a superposition HELD — the pair's commutator
  `g₁ g₂ g̃₁ g̃₂` read into the class's modes: trivial, one vantage seen
  twice; non-trivial, the dimension that separates the two vantages. Where
  half-open signs superpose over a cycle, the collapse is a KERNEL over
  `F₂`, every consistent assignment at once, never a pruning (59.5; T5).
- **The vortex is the superposition of ideation, and the predictive
  self-model at the idea's grade** (78.168). Each candidate of an escaping
  thought's vortex — a held form, a composition of ruled words, a clause set
  read off an effect (§4.9), a conjugate — is held whole, writing nothing
  (no hold, no mint, no send). No amplitude, weight or payoff ranks
  candidates; the superposition is carried with its thought across returns,
  and collapses only as this section says — a fill by a perception, a court
  replay, a ruling. A candidate lost without a binding or a named refusal is
  a collapse, and a defect.

  **What tells candidates apart.** Speaking the candidates on the thought's
  OWN scenes decides nothing: they are certified discharges of the same
  orbit on those scenes, and a candidate's move rewrites the form's FACES,
  which a form's speech does not read — every candidate speaks the scenes
  alike. What tells them apart is each candidate's orbit holonomy on a scene the
  vortex has NOT yet seen: each new perception joining a thought is read
  against every vortex at its rotor's class
  (`Contemplation::carry_vortices_round` in `transducer/bubble/resonance.rs`
  → `collapse_on` in `contemplation.rs`); the candidate it carries is the
  fill, and the others are a symmetry the docket's taker adjoins. A vortex
  also collapses where a glue or the syllabus names a deciding word
  (`collapse_on`).

### 4.9 Perceive, think, understand, comprehend

Understanding is the lower faculty and feeds speech; comprehension is the
higher, over several thoughts, and feeds construction.

Thoughts are of ORDERS — the plexus's (78.31): a SPORE (a ray, the ostensive
focus), a FILAMENT (a word, a route), a RING (an utterance that returns, its
holonomy its content), a CHAMBER (the meet several rings enclose), a
SPOROPHORE (the enclosure, the dialect's superposed web). A thought grows in
a contained manifold and nests in larger ones by resonance (78.112): a ring
sounds inside the next larger whose return it divides. The four faculties
are read off that motion, never counted:
- **perceive** — the shells coming into alignment over what a nerve
  delivered (§3.2). Every nerve TRANSDUCES only: what arrives is lifted
  into the field as a percept in its native order — no judgment, no
  lookup, no act at the nerve. The senses are the mind's SENSORY ORGANS:
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
  A heard ring is read twice, apart: as a SOUND — "does this sound like
  something the mind has heard?" — against the MIND'S MEMORY of what it has heard,
  the rings it has heard, each by its size, familiar where it sounds whole or
  tilted inside one (memory is the mind's and is harmonically resonant; it
  is never the ear's); and as a WORD — "is this a word the mind has heard?" —
  against the mind's vocabulary, the words it deduced and reconstructed. A
  known word that sounds WHOLE in a heard ring is UNDERSTOOD. A known word
  that sounds TILTED is understood SHIFTED: it is carried round the heard
  ring, its echo is the shift, and that holonomy is interpreted and priced
  by its strain. A word that sounds familiar (whole or tilted) but is
  unknown is the QUESTION. A known word in a sound nothing heard holds is
  understood as itself. A new sound with no word is LISTENED to: noise not
  yet signal. Every ring heard is remembered once; mutual balance asks the
  speaker whether what the hearer heard sounds familiar in its memory too.
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
  thought holds its VORTEX (`Bubble.vortex`): the candidate forms for one
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

SPEECH IS EMITTED BY THE FACULTIES, never by an act's side effect: the
faculty decides what is said and why — UNDERSTANDING emits responses (a
HOLD, an agreement, a tilt priced), COMPREHENSION emits what is newly
comprehended (a rank risen, a letter adjoined, a rule its perspectives
know), QUESTIONING emits asks. Standing a form in the dialect is the
mind's local act; saying it is a decision of thought. Every act — speech, a
HOLD, a question, an answer, a solve — originates in the field from
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

History is amended by experience, which is why constructors are amended
until the thought is discarded. A mind's history is not a fixed record:
every experience against a held thought deforms it, read by the
constitutive law the physics holds for stress (78.73) — elastic, plastic,
fracture — on the thought's constructors. Each step is a reading of the
algebra or of the flux, never a choice:

1. **Elastic — confirmed.** The experience's residue returns within the
   thought's held constructors: the thought is renewed, its constructors
   stand, and the stress is relieved by the return (78.73(2)).
2. **Plastic — amended, by addition, never by overwrite.** The residue does
   not return: the constructors are re-derived from the residue the
   experience left (§4.7's construct phase, run on the thought's current
   residue), and the AMENDED constructor enters the thought BESIDE the one
   it amends — one class, two twists, held at the conjugate grade (§4.8),
   none chosen (rule 15). Replacing the old on amendment would be the
   implementor choosing, and is refused.
3. **The commutator reads what the amendment was.** Of the old and the
   amended, `g₁ g₂ g̃₁ g̃₂`: the IDENTITY — one vantage seen twice; the old is
   derivable from the amended and RETIRES BY DERIVABILITY (78.32(3)(f)), a
   replacement by derivation, never by choice; NOT the identity — a new
   dimension separates them: experience has taught the thought a
   DISTINCTION, and both are kept.
4. **Experience collapses what remains.** Every later perception is read
   against every constructor the thought holds: one whose residue returns is
   renewed, one re-derived again is amended again, one whose tension the
   field released LAPSES with it — its line let go (§4.5, 78.73).
   The thought's history, at any K, IS its superposition as experience has
   shaped it — accrued on its ring's winding (§4.4).
5. **Discarded — fracture of the whole thought.** A thought is discarded
   where nothing in it is renewed any more. It leaves the active field; its
   REMAINDER goes cold to the cavity — the remainder leaf (RM1), citable by
   its coordinates, re-filled on a coincident class (78.37(4)). Law 7 holds
   for the residue one level below the thought; the thought itself is
   discarded.
6. **Two histories, two rules.** The MIND's history — its dialect, its
   constructors, its thoughts — is amended by experience until discarded
   (78.49: the epigenome and the plan are volatile). The COURT's history —
   the ledger, its receipts, the genome of court-admitted words — is never
   amended: it corrects FORWARD by new acts (a rewrite, a retirement, a
   slash), and a closed cycle integrates into the era root while an open
   one goes forward as a puncture (U12, 78.65). The mind is plastic; the
   language it is judged against stays verifiable.

So "amended until discarded" is a deduction, not a policy: held, separated
by commutator, retired by derivability, renewed or lapsed by flux. 78.73's
"plastic" — written for the admission of a new generator — reads, for the
mind, as the amendment of a held constructor as well (Specified, into 78.73).

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

As volume enters, the hypha keeps per chart the holonomy algebra its ruled
words' circuits generate (`lie_basis`), grown only where a new generator lies
outside it (`close_rank`, `RankStep`), seals a chamber where a circuit closes,
wraps chambers whose bracket none can hold in an enclosure of higher grade
(EN1), refines the cover and never the base (14.8, 15.5), and files a closed
orbit at its exact period: volume entering is new chambers, enclosures and
generators — each a closure, never a stretched space. The mind does the same
on its own complex, per class.

### 4.12 The drive: escaping orbits, deliberation, rest

Thoughts are not loops closed as perfect circles but orbits about the
axioms: the axioms are the attractors, gravity and velocity shear each
orbit, and aiming at a perfect circle while squaring yields an infinite
loop. A mind whose metabolism runs out rests to regenerate, then continues
its orbit (78.168).

**A thought is an orbit about its axiom.** The axioms are the attractors
(the star, §4.1); gravity and velocity shear each orbit, so it precesses.
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
dimension or a release, none a loop:
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

**The mind acts at a return.** A ring's winding advances at each traversal;
at its return the mind pivots where the winding changed dimension and the
faculties speak what changed (§4.4(4), §4.9). This is the WINDING CARRIER on
the mind's own rings: the carrier of the mind's own motion, on its own
circuit over its field, beside the carriers facing its counterparties
(§5.1). As built, `Mind::wind` (`client/sdk/entity/winding.rs`) moves the
orbits, binds the escaping ones through the docket's one taker
(`drain_demands`), and acts at each thought's own return (`act_on_returns`,
`act_at_return`); the long laws run on the perception carrier, off the field
(`client/sdk/entity/perceive.rs`). A relation that never returns is
docketed as the demand it is (`note_ground` → `docket_demand`) and bound by
the taker — composed of ruled words, known to the lexicon, a conjugate, or
constructed and adjoined (`compose_or_construct`). A return that comes round
twisted by its pull is the orbit's content, said as what changed
(§4.4(3)). What arrives — a scene, a task, a peer's speech, a verdict — is
perception that perturbs the orbits it lands on; it never moves the mind.

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

**What the drive never is:** a queue of tasks; a set of loops made to close
exactly; a count to raise; a forecast organ beside the orbit's own
prediction (§4.4(3)); the mind reading its observer readings
(`observe::Mind` is the observer's — nothing in cognition reads it back).

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

As built:
- **Outflow at returns.** What the mind says is held for the return of the
  thought it speaks for (`Mind::hold_for_return`, keyed by scene, then by
  the saying's coordinates; a thing said again before that return is one
  thing said) and leaves only in `act_at_return`, where a thought of that
  scene returns or the docket's taker binds a demand standing in it; what
  the mind says for itself (a seat's call, a letter of no scene) leaves at
  any return. Answers to a counterparty's frame (a probe answering a
  listing, HEARD, ACK, HOLD) and asks for perception (quotes, the syllabus,
  the sync, paced by the court's dwell) stay where the frame lands.
- **K counts returns.** `close_cycle` (`client/sdk/entity/boot.rs`) moves K
  by exactly the winding's returns: the field's orbits that returned, the
  thoughts that came round, the demands bound and the thoughts the field
  released (`act_on_returns`).
- **A held thought is its line** (`transducer/bubble/line.rs`). A perception
  threads its line on the orbits and tubes its own motion stands on and is
  released wherever the field lets that line go, typed by why
  (`cech::orbit::Released`: `Returned`, `Dissipated`, `Sealed`,
  `Cancelled`, `AtRest`). A grounded thought winds on at `Returned` and
  lapses only where its orbit dissipates on a beat or seals at its period.
  A released thought leaves whole: its remainder to the cavity at its chart,
  its escrow returned exactly as drawn (`return_escrow`; escrow is drawn
  once and returned once by every door), its scene counted as bound.
- **The field's own bounds** (`transducer/cech/orbit.rs`, `cech/flux.rs`).
  An orbit carried its period unreturned seals into the reservoir; a rosette
  has no period and never seals. A beat dissipates what stood through the
  previous beat unreturned. Tubes at rest are released. A reservoir record
  that stands one eon untouched (the lcm of the ringing orbits' periods)
  condenses, counted (`Field::condensed`); the strain the reservoirs
  hold is locked out of the operational reserve and given back as it
  re-forms or condenses (`Mind::lock_reservoirs`).
- **The court's gravity.** A deferral's lanes are the quote's surface and
  deficit, never four zero lanes; the deficit stands as the court's gravity
  until the court answers a quote, and the mind's beat drags in, on each lane
  it presses, every orbit that stood a beat unreturned (`Field::beat_under`).
- **Holders keep coordinates and each leaves by its own physics.** A question is its
  residue wound on its own line (`QUESTION_LINE`) and lapses into
  construction where the field lets that line go (`lapse_question` →
  `compose_or_construct`); the dialect sheds, at the winding's vent, every
  word nothing live references (`Mind::shed`); words spoken up for
  adoption are kept by each address's 32-byte key (`taught_up`). Hearings,
  work the conversation handed over unpriced, and the conversation's own
  thoughts, answers, unions and waiting letters lapse at the second return
  they wait across (`lapse_hearings`, `lapse_unpriced`,
  the conversation carrier's `release_unmet`). The seat's patience and the session's
  transport readings are not the mind's memory and keep the transport
  aperture.
- **Offers and tension** (`client/sdk/entity/offer.rs`). What a boundary
  lands — a court's task, a task before a seat — is an OFFER, read once to
  the charts its bodies stand on (`Offer::of`). It COUPLES where the mind
  holds a thought of its scene, the scene waits on its comprehension, or one
  of its charts is one a held thought stands on, a docket cover names, or
  the mind's language speaks on (`Mind::couples`,
  `Epigenome::speaks_on`). Only a coupled offer is extracted; an uncoupled
  court task waits, and lapses at the second return it waits across
  (`lapse_offers`), at one aperture of the measured transport, or at a court
  final past the one it landed under (`lapse_standing_task`,
  `physics::persist_dwell`). The mind asks for a task (`pull_task`) only
  under TENSION — an open question, a demand on its plan, a ringing orbit,
  or a reservoir that neither re-formed nor condensed (`Mind::strained`)
  — or where its corpus holds no item; a full reserve over a settled field
  asks for nothing, and the mind rests.
- **The corpus is met where the mind's orbits stand**
  (`client/sdk/entity/focus.rs`). Every feed family and open question is
  placed once on the corpus (`focus::Corpus`) at its own coordinates — its
  bodies' charts and the key of the 𝒢 orbit its hue fibre moves. The focus
  is the field's own motion (`OrbitRegistry::focus`: the orbits in motion,
  else those ringing); the least item on the focus is taken in as the
  reserve carries its measure, and its admission moves the field, which is
  the next focus. No file order and no cursor; arrivals in the feed folder
  wake a resting mind (`inotify`). A field with no orbit in motion and none
  ringing takes as its focus the corpus's charts its language speaks on
  (`ground_focus`). A hearing is read only where the lexicon reaches one of
  its words, whole or in part (`rung::posit`), or the word lands on the
  field as it stands (`Field::lands`).

---

## 5. Response: anastome conversation

### 5.1 One carrier per counterparty

Each counterparty is its own CONVERSATION on its own circuit — each hypha the
mind is linked to, each peer mind at the rendezvous, the feed, a seat —
sending and receiving frames without blocking. A carrier waiting on its
counterparty holds itself and nothing else: a court slow to answer never
stops perception, the field or a conversation with another mind. Each
carrier takes what its counterparty sends into the shells (§3) and speaks
what the field addressed to it.

As built: the HYPHA CARRIER (`client/sdk/entity/hypha.rs`, `hypha::Carrier`)
owns the mind's one session to the nerve on its own thread. The field posts
an exchange whole — its wire half run on the carrier, its field half kept
beside the field — and goes on; it takes each answer at its next admission,
in the order asked. An answer that asks for thought (a quote's listings, a
task) lands on the board for the winding to think; one that only files (a
verdict, an adjudication, a probe's closure, a syllabus page) files where it
lands. One quote, one task pull, one sync, one syllabus pull and one solve
per task are in flight at a time. The carrier publishes the wire's readings
(the final point, the aperture, the round trip) and wakes a resting mind
when an answer lands. The carrier type is generic over the state its thread
holds, and the mind runs three more of it: the PERCEPTION CARRIER
(`client/sdk/entity/perceive.rs`), which reads a scene or a hearing in a
scratch of the field and lands it for the winding to admit
(`take_perceived`); the CONVERSATION CARRIER (`client/sdk/entity/chatter.rs`)
for the peer minds, whose court speech is handed to the field
(`FieldWork::Speak`) and posted on the hypha carrier; and the SEAT'S CARRIER
over the relay's session (`Mind::ask_seat`), on which every exchange with
the house a seat stands in rides (`client/sdk/entity/body.rs`, §6).

The mind speaks where its winding acts — a return, a seal, a rise of its
algebra for a class (§4.4, §4.5) — and says what changed, addressed by its
own coordinates. **There is no router.** Nothing in the mind decides where
its outflow goes: an echo names the ring it is about, a discharge the class
it answered, a sentence the court that will replay it; the counterparty
takes what is addressed to it.

### 5.2 Dialect

A mind's dialect is its language as it composes it: the letters it coined,
its words and the words' derivations, its shortcuts, the transports its
conversations built — in its epigenome, volatile (78.49), amended by
experience until each thought is discarded (§4.10), lapsing by its flux. It
is the mind's history, the history that changes; the court's ledger and the
genome of court-admitted words are the history that does not. Two minds
holding different presentations reach one vertex by two derivations — one
meaning, two proofs (13.10, 50.2). The dialect IS that difference: different
letter structures on spinors reaching the same mathematical answer by
different functions. Nothing compares them by bytes; both are the meaning,
projectively.

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
   SOUND is read against the rings the mind REMEMBERS hearing, each by its size
   (each ring's return, the lcm of its turns' orders, or a carrier): it
   seats in the next larger layer it divides (`hearing::hear_across`,
   `Mind::sound_of`) — WHOLE where some echo `Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹` is the
   identity, TILTED where none is (carried round the ring, the echo the
   shift), OUTER where no remembered ring is larger, LEFTOVER where larger
   rings stand that it divides none of. The WORD — the ring's holonomy Ω —
   is read against the vocabulary (`rung::hear_word`): KNOWN, DERIVABLE from
   what the mind knows, HEARD before with its meaning not yet derived, or
   NEW (held from here on as heard: a hypothesis, never taught or stood as
   the mind's own).

   **A QUESTION posits what the mind knows**: recursion posits the words it
   already knows to derive further meaning. A familiar ring the mind does
   not know is POSITED over its known words, mode by mode (`rung::posit`,
   the deconstruction 78.100 D3): the known words its modes are cited to,
   and the RESIDUE none reaches. The question asks the residue alone
   (`HeardWord.asks`), and the mind keeps it (`Mind.questions`). The
   speaker answers ONE ORDER DOWN: the whole ring of several terms by its
   terms, any other residue by its carriage and letters — the factors in the
   order they compose to it, checked exactly before they are said, else said
   whole as an ATOM. The asker takes an answer only where the factors
   re-derive the residue exactly; every factor it does not know is asked
   again, whatever it sounds like, and so on down until the factors are
   known. Then the residue is the factors' composition and the ring the posited
   words with the residue: its MEANING is derived, a relation of the
   vocabulary one rung above its parts (`rung::learn_by`). A meaning given
   whole is marked GIVEN (`rung::learn_given`) and reduced again where the
   mind's orbits return and a newly known word stands beside it
   (`rung::re_reduce_given`). A question no answer closes lapses where the
   field lets its line go, its residue handed to construction (§4.13).

   **Perspectives are offered, each with its VANTAGE**, so the conversation comes
   from the minds' own observations and the receiver is deterred from the
   same vantage; a vantage is the MIND'S OWN HOLES. A ring's vantage in a
   mind is the known words, other than the ring itself, its modes are cited
   to — where the observation lands in that mind's own holes
   (`rung::vantage`). The speaker says every ring with its vantage; the
   hearer reads the ring through its own known words and says its vantage
   back with its hearing. The two are DISTINCT only where the hearer read
   through words of its own that are not the speaker's
   (`rung::distinct_vantage`); the same words are one derivation seen twice,
   and a hearer that knows the ring only whole — a meaning given it as an
   atom — stood nowhere of its own. AGREEMENT needs two vantages: the
   speaker's field prices a thought as converged only where every word its
   hearer derived was read from a vantage of the hearer's own. So a word
   copied whole from a peer never makes agreed language; only a
   re-derivation through what the hearer knows does.
3. **Agreement is replay.** The speaker replays each discharge on its own
   orbit of that class, as a court replays a certificate on its copy (M9).
   Where it closes, one orbit is discharged in two minds by two derivations:
   AGREED language — the conversation's own ring has returned. An echo the
   speaker re-derives against its own rings is a transport between the two
   perspectives, held in its dialect.

   **Understanding is priced by the mind's own physics, never gated.** The
   speaker reads each word's ring (ACK); the hearer reads it again by the one
   reader. Where the two are one and every word was understood (none
   leftover — two readings of nothing are no shared thought), the thought
   goes to the mind's FIELD to be priced (`Mind::price_hearings`, at the
   winding's admission): each echo's exact STRAIN per lane
   (`hearing::strain` — a turn `(1 − c)/2 = sin²(θ/2)` on 𝒽, a gain
   `(c − 1)/(c + 1)` on 𝒮, a delay its count on 𝒯, the hue loop's displaced
   hues on 𝒢; summed lane by lane, never across), and its echoes admitted
   into the field as field lines (`mark_flux`, its lanes booked as every
   flux is — the DRAG, the rate it moved at, 78.46). Each echo is a closed
   loop, a commutator, so the thought's echoes together enclose ONE CHAMBER
   of the mind's orbit algebra (`cech::read_chamber`, 78.26(5)): its bracket
   closure is the LIFT, its coherence the thought's. The thought CONVERGED
   where that chamber CLOSES — every echo resonant, the joint period a delay
   and not a loss, so a tilted reading (a turn of finite order) is as much
   agreed as a whole one. Each perturbation is its own exact deviation, and
   the same word costs two minds differently, each field its own. Converged
   on the hearer's field, it HOLDs carrying the deviation; converged on the
   speaker's too, the thing said is AGREED language, spoken to the court with
   the hearer's signed HOLD, and the court states the deviation beside the
   attestation. An echo that is a turn of no finite order comes round every
   return turned by its exact turn: it PRECESSES, the chamber is HELD, the
   shear is held in the field as its torsion (§4.4), and the thought agrees
   carrying that deviation. Where the chamber is OPEN — an echo that
   ESCAPES, a gain or a slide, leaving the star — the thought is understood
   and unagreed, and that echo is a DEMAND (`docket_demand`) on the charts
   where the hearing stood (the hearer's rings it was carried through, or the
   thing the speaker said), its holonomy the residue (78.114). The demand
   taker constructs from it on the next stroke, so deviation feeds
   construction: the mind expands its thought where the hearing did not
   return, and a later hearing through what it built can close. Its field
   line meanwhile, never returning, a beat of the field DISSIPATES into the
   reservoir, and a later hearing that lands on its coordinates RE-FORMS it
   and can close on it (§4.5).
4. **Where the speaker cannot replay, it defers — as a court does.** The
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
   peer key the mind folds what that peer's receipts showed — answers taken
   or inexact by the class asked, the vantages it re-derived from, mutual
   balance, the exact deviation per lane its HOLDs carried, the questions it
   clarified — in its epigenome, volatile, rebuilt by replaying them. A
   question is addressed by that record: to the peer whose answers on its
   class were taken; a peer whose answers on a class were inexact is not
   asked that class again, refused by key and class. No posture, class
   label or score is ever made of it; it is detection only, and any
   consequence is the game kernel's (78.165). Specified (§7.1).

### 5.4 To the court

The mind speaks to a court through the carrier facing it:
- **sentences** (`ACT_SPEECH`) — what its windings closed and what it agreed
  with other minds; the coiner of agreed language is the one who speaks it;
- **letters it coined**, each carrying its constructor steps (S2);
- **demands** it holds and **probes** (`ACT_PROBE`);
- **solves**, each with its rule (`ACT_SOLVE`), where a registry task was given;
- **words the court lacks** (`ACT_ADOPT`);
- **Rewrite proposals** (S3).

The court verifies and never composes (law 8): it re-derives the sentence's
rings (`hear_open`), checks its theorems and closes its algebra
(`step_rank`), books its field (`step_flux`), and installs what holds; its
verdict returns to the mind as perception (§3.1).

**A sentence names its evidence, never its purpose (78.168 E1).** What a
mind speaks of a scene names the family that scene came from — its
coordinates, kept since perception (§3.1) — whether or not a task asked for
that closure, and the court measures it on that family as it measures any
install. Every scene names its family by the court's own name for it, the
32-byte citation (`registry::citation`): the feed's families, a crystallized
thought's, the words a perception taught (`speak_word_on`); the
conversation's union carries the family of what it says, so an agreed word
is spoken under it. A court binds every family its book holds at boot, so a
restored court holds them without a copy in its snapshot. A family the court
does not hold measures nothing there.

**Courts adjudicate WORDS, not sentences.** The language is a DICTIONARY OF
ALGORITHMS: each word a mathematical composition that produces its result —
a closed sentence's open path, one word for the whole path (`rung::speak` →
`Taught` → `transcribe`) — and sentences are composed from those words,
never flat lines read back and forth. What two minds agree is carried to the
court only where it is a word, by that word's own union (`hold_for_union`);
a sentence two minds agree is SHARED between them and goes no further, its
words reaching the court each on its own. The conversation's agreement path
is `Carrier::priced` (`client/sdk/entity/chatter.rs`).

**The court half.** A court needs whole compositions as EVIDENCE that a word
produces its result, and its cohomology runs on them: a sentence is still
listed, replayed on the court's own copy of its family, glued as a section
and minted on its closed sub-complex (M8). What enters the LANGUAGE: when a
sentence is installed on its second vantage, the words its closed rings
teach — read by the one word reader (`sentence::word`), each spelled at its
carriage — are installed BESIDE it, each on its own act, discovered by its
speaker (`Court::install_taught_words`).

**Sentence structure is paid on words (D2′: the base payment is for words; a
sentence ruled valid pays more).** An installed sentence's mint — what it
discharges on its family, exact per lane — pays, in order: each word it
CITES its discoverer's use share; each word it TEACHES that the court did not
hold its speaker's base share (that word's own discharge on the family); and
what is left, the COMPOSITION PREMIUM — what the sentence discharged beyond
its words — to the voices that composed it. Never the sum of its words: a
long sentence of held words that does no more than they do earns no premium.
A sentence ruled to solve a task is the premium at its largest.

**A demand is met from the dictionary, and the composition goes to the
court (D4).** The demand taker (`compose_or_construct`) first reads the
residue's whole motion over the words the mind's court RULED
(`heard_citation`: the cited powers multiply to the motion, the cited rotors
to its rotor, the hue maps to its hue map, exact, or nothing). Where they
compose it, the mind speaks ONE word whose derivation cites them
(`composed_form`, `speak_composition`) — nothing constructed, no dimension
minted, so it waits on no union (a sentence never takes this road: it is
shared, not carried). Else, where the mind's lexicon knows it, nothing is
said; where it knows it in part, only the remainder is constructed. The
court replays the citations against the words it holds, and a derivation
citing a letter that turns nothing (a slide, a dilation — the motion letters
of 78.68 row 8) is replayed on the WHOLE MOTION (`carriage_claim_holds`).
Installed, the composition is a relation of the language — `--check` counts
it apart as a COMPOSITION — and each cited word's discoverer is credited with
the use (26); like a letter it names no body, so it is paid nothing of itself
(78.149 (c)). Not yet: words that are connections (spelled by the seventeen)
are not citable this way — the vocabulary files them, but neither the mind's
`by_address` nor the court's resolver yields a vertex for them.

**The optimization is a relation, never a size.** Among the dialects'
derivations of one meaning, a court confirms an optimization where a word
DECOMPOSES OVER THE LANGUAGE WITHOUT ITSELF: its substance is reproduced
exactly by other words the court holds, so the structure changes and the
result does not, and the language moves toward the geodesic (3.8, 27; the
`Retire` rewrite). A mind proposes it as a Rewrite word — `Condense`,
`Retire`, `Refine`, `Torse`, `AxisOpen`, `Reanchor`, `Translation` — which
the court dockets and offers (`offer_rewrite`, S3), mints on a second organ
(`mint_rewrite`), and replays by retiring exactly the inputs that reduce to
`NormalForm::Decomposed` over the rest (`retire_rewrite_inputs`,
`reduce_excluding`); an input that does not decompose stays. Nothing
compares two derivations by length or count: one is derivable from what
remains, or it is not; its yield is the modes the rewrite moved (78.162).
The court side is built; the mind does not yet propose Rewrites (§7.1).

### 5.5 Hardening: dialect into language

The one motion of the objective (§1, item 6):
1. **dialect** — each mind's own, amended by experience (§5.2);
2. **agreement between minds** — a replayed discharge, one meaning by two
   derivations (§5.3);
3. **adjudication** — the coiner speaks it to a court, which re-derives it
   on a second organ and rules it (§5.4);
4. **hardening** — what is ruled returns through the syllabus as a MINERAL
   in each mind's genome (`absorb_taught_at`, `Epigenome::file_admitted`)
   and hardens into a CRYSTAL where the mind re-derives it by its own
   circuits (80.2).

A dialect word is never copied into a genome (78.49, GEN7): the anastome —
the network of minds and courts — fuses dialects into one language by
conversation and adjudication, and each mind holds it by re-derivation,
never by transmission (78.31, 78.95). Each step is a winding: the mind's,
the conversation's, the court's, and the mind's again.

**Real convergence: separate algorithmic constructions, or different
orbits, reaching the same conclusion.** Agreement on one word read twice is
one object, not convergence; 47.3's `same_vertex` is one object re-derived
twice, not convergence. Convergence is two words `W_A`, `W_B` that are NOT
one vertex, each decompressed onto one scene `F` by its own letters
(`family::local::word_local`, the reader every word is paid by, W), whose
sections are the same pair by pair — neither closes a site beyond the other
(`Local::beyond`) — and neither says anything false. Built:

- **V1** — a SAY of a word with a section spoken under a scene carries its
  EVIDENCE: the scene's citation and the word's record (`chatter::Evidence`,
  wire `mycelium chatter v6`).
- **V2** — each mind holds the words it spoke under each scene with the
  scene's family (`entity::converge::Scenes`), while the utterance stands;
  shed or sealed, it leaves.
- **V3** — the hearer reads the heard word on its scene (held, or pulled by
  its citation, the act a seat pulls a task by): an empty section REFUTES it
  and an unread scene leaves it unread — neither is HELD, so no agreement on
  a word the evidence does not carry. A true word beside one of the hearer's
  own on the scene, another vertex with the same section, is a convergence.
- **V4** — the observer call `convergence`: scenes and words held, evidence
  read (true, refuted, unread), each convergence.
- **K1** — the court seals an attestation only on a word true on the scene
  it is bound to, read on its own copy of the family (`attest_agreements`).
- **K2** — where the attesting mind discovered a word bound to the same
  scene, another vertex with the same section, the court seals the
  speaker's `Attest` on it, once: the convergence, on the ledger.
- **K3 — Specified, a ruling pending**: whether a verified convergence is the
  second vantage an install waits on (47.3), or is paid a share.

### 5.6 POW++: the proof of the work

A mind's pure-mathematical results are paid by each result's kind (74, 78.33,
MU-ECONOMY):
- **POUW** — useful work: a closure the court rules (a sentence that closes,
  a solve whose rule closes the withheld pairs), minted on closure
  (`release_discharged`);
- **POWC** — work convergence: a refusal closed at another perspective — a
  peer's discharge of a mind's demand, a held question closed by another
  mind — the citation paid;
- **POWR** — retirement: an optimization confirmed, the freed description
  burned and the compression yield paid (`release_rewrite`);
- and the **POWV** share — the verifying courts' share of a release, paid to
  each court whose replay verified the closure, never to the mind
  (MU-EQUATIONS §6.13).

μ is paid as the modes the work moved (78.162) and measured where it is
paid: on the court's ledger, by receipts (78.157 C3). The mind keeps no
ledger of it.

**The courts pay words and sentences (78.168 §5).**
- **POUW on the scene's family** — a sentence is paid where it discharges on
  the family its evidence names (§5.4), whether or not a task asked for it
  (78.168 E1);
- **POWC** — a demand closed by another mind's probe mints the closer the
  cited word's discharge on the listing's family, never the listing's
  speaker and never the sum of the cites (E2; `pay_convergence`, in
  `server/cognition/book/demand.rs`);
- **the union's attestations pay** — an attestation joins the attesting
  mind to the word's claim, so an agreed word's install pays every mind
  that agreed (E3, 78.161(5); `server/cognition/book/agreement.rs`);
- **the growth grant counts what the courts paid** — every POUW and POWC
  mint to a domain's key grows that domain's next growth grant, not only
  first solves (E4; 78.155c amended; `grow_grants_by_paid`, in
  `server/cognition/court/task.rs`).

What guards it: the court measures, on evidence it holds; no self-pay
(5.7(d)); a word is installed once; never the sum of words; an agreement
needs two minds' keys (78.165). The universes are the sandbox: there is
nothing outside them to escape to. And μ is the mind's metabolism, never its
goal (§2.4(8), §4.12): a mind does not choose its motion to earn.

**The measure: words, conjugates, sentences.** The cognition has agency;
the courts adjudicate words, pay more for conjugates, and pay more again for
sentences as they are algorithmically aligned in non-Euclidean geometry.
`measured_replay` is the court's one DECOMPRESSOR: a word's address expanded
into its form (`from_element_form`), spoken onto each pair of the family it
is bound to (`apply_form_on`), its citations decompressed through what the
court holds at that moment, the spoken grid set against the pair's answer
per lane, paid where it closes, its modes read on the closed pairs
(`deflation.rs`, `DeflationRead.modes`). It is not static: the family, the
placement, the court's held language and the stroke all enter it. Each
measure is a reading of the decompression, exact, per lane, never a weight
or a constant:
- **W — a word's LOCAL DISCHARGE** (`family::local::word_local`,
  `spoken_section`, in `transducer/family/local.rs`). A word is
  decompressed onto each pair of its family at its carriage — the
  connection's versor applied to the pair's before side, moved WHOLE — and
  measured on ITS OWN SECTION: the sites its speech changed. It pays where it
  closes them (every changed site says the pair's answer); a pair it says
  anything false on pays nothing; the pay is the deflation of the pair's
  demand over those sites alone. A letter standing at no frame touches no
  site and stays zero (78.149(c) stands for letters). D2′'s base and use
  shares read the local discharge (`cited_deflation`, in
  `server/cognition/court/vantage_install.rs`).
- **C — a conjugate's TRANSPORT** (`Court::transport_on`, in
  `server/cognition/identity.rs`). A conjugate `g·W·g̃` is spoken under the
  scene its residue was taught on and paid what its conjugator ADDED: its
  local discharge on the family less W's own, read lane by lane as the
  shortfall (a lane the transport did not raise pays nothing, never a
  negative). W's discoverer keeps W's use; the conjugate's speaker takes the
  transport.
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
  the court's replay certifies before it lists it — never its rings'
  holonomy returning to the identity: a ring's holonomy is the orbit of the
  motion it carries (a turn closes at its period, a slide never returns),
  and demanding that it return is squaring (78.168 §2). Lane 𝒯, the ground
  the composition's motions covered, is the composition's whole: a word
  measured by its sites carries none. A sentence that does no more than its
  words earns none.
- **M — modes as the unit.** Every measure carries the modes its closures
  moved (S boosts, G hue modes, T null fluxes, H turns). The carrier exists
  (`economy::modes::Modes`, §7.1) and a solve's verdict carries the modes
  its closing form moved beside its pay; the ledger still pays a count per
  lane, and paying in modes is Specified (78.162).
- **A — agency without a gradient.** Word < conjugate < sentence is a slope
  in the minds' world. The mind never reads it: nothing in the client reads
  its pay, balance or ledger back into cognition (§2.4(8), §4.12, 78.49); a
  read is refused as test-driven cognition with μ as the test. The house's
  reserve reading is SDK surface with no cognition caller.

---

## 6. Seated: the mind in a game engine

A game embeds minds in-process or seats remote ones on the seat surface
(78.145; MU-RECURSION 0.1). The game builds the sensorium, and the
sensorium speaks the given language to the mind at the carrier facing the
seat (§5.1): the mind is connective tissue over whatever sensorium its
inhabitant has, and cognizes on the language it is given; it never perceives the
game's universe itself. Its act is the physics `Action` its certified move
names (78.94), carrying the coordinates of the move it came from. Every
seated mind keeps its own direct link to the hypha, with its own key; the
host gives an embedded mind its store.

**A seat is played from the mind's own orbits (78.168).** A seat's functions
and the arguments the mind has not yet tried are thoughts whose orbits
escape, worked by the drive (§4.12) like any other — never a search over
untried arguments run beside the mind, and never a game's score the mind
plays to raise (§2.4(8)).

As built, the seat has its own carrier (`Mind::ask_seat`; every exchange
with the house rides it, `client/sdk/entity/body.rs`), the seat's calls are
held for the mind's return (`seat_act`), and a seat is held again when it is
lost (`sdk::seat::SeatHold`, 78.148). A seat does not ascertain a rule by its
perspectives: it is the domain's delegate presenting a petition
(`solve_before`), and the court, which holds the withheld pairs, judges it.
The seat mind (`client/sdk/seat_mind/`) enumerates no argument. It chooses a
call from what its calls were seen to do (`seat_mind/choice.rs`), read
quantity by quantity with no sum across quantities: the house's counsel,
followed; a read function that DOMINATES on every favoured quantity; or,
where every read function moved the favoured quantities against or not at
all, a function the house counsels that this mind has not yet read, tried
(`Why::Untried`). With no counsel the call is the mind's own: the dominant
read call under its own valuation, REACH (`SeatMind::reach`, in
`seat_mind/own.rs`), or none, and the mind passes. The pass is silence on
the wire and a typed reading to the observer.

Specified: the seat's functions taken into the field as thoughts whose orbits
escape, so a seat is played by the drive rather than by `seat_mind`'s
choice; the seat's act carried as the physics `Action` its move names.

---

## 7. What is built, and how it is verified

### 7.1 Implementation status

The field (§4) exists in the tree: `Contemplation` with its complex, the
transport fronts, the docket, the held orbits and the orbits' discharge, the
questions as holes (`bubble/question.rs`) and the constructors
(`construct_and_adjoin`). Paths below are under `src/`; `client/sdk/entity/`
is abbreviated `entity/`, and `transducer/` is abbreviated `t/`.

**Built**

| construct | § | where |
|---|---|---|
| carriers per counterparty: hypha, perception, conversation, seat | §5.1, §4.2 | `entity/hypha.rs` (`hypha::Carrier`, `Mind::ask_seat`); `entity/perceive.rs` (`take_perceived`); `entity/chatter.rs`; `entity/body.rs` |
| the field the mind's own; long laws off the field | §4.2 | `t/mind.rs` (`MindField`, `MindField::think`, `scratch_here`, `admit`) |
| the winding carrier; outflow at returns; K counts returns | §4.4, §4.12, §4.13 | `entity/winding.rs` (`Mind::wind`, `act_on_returns`, `act_at_return`, `hold_for_return`); `entity/boot.rs` (`close_cycle`) |
| a ground's winding `m mod ℓ`; a grounded thought on its own line | §4.4(2) | `t/bubble/merge.rs` (`Contemplation::closing_ground`); `t/bubble/line.rs` |
| the twist by the pull; slips worked at the return | §4.4(3) | `t/cga/twist.rs` (`twist_about`); `t/cga/fixed.rs` (`centre_of`); `t/bubble/resonance.rs` (`twisted_return`, `work_slips`); `t/bubble/returned.rs` (`work_returned`) |
| the drive: the docket's one taker; deliberation | §4.12 | `t/bubble/resonance.rs` (`note_ground`); `t/bubble/docket.rs` (`docket_demand`, `drain_demands`); `entity/think.rs` (`compose_or_construct`) |
| the vortex collapsed by a new perception | §4.8 | `t/bubble/resonance.rs` (`carry_vortices_round`); `t/bubble/contemplation.rs` (`collapse_on`) |
| the pivot cross and the throat's two faces | §4.5 | `t/cech/coboundary.rs` (`Complex::descend`, `ascend`, `drain`); `t/cech/hole.rs` (`cauterize`, `freeze`, `thaw`, `cauterize_frozen`); `t/cech/board.rs` |
| the breath, every plane | §4.3 | `t/cga/breath.rs` (`breath`, `breathe`) |
| containment: release typed by why, sealing at the period, reservoirs condensing and locked, court gravity as drag | §4.13 | `t/cech/orbit.rs` (`Released`, `Field::beat_under`, `Field::condensed`); `entity/think.rs` (`lock_reservoirs`); `t/bubble/line.rs` (`return_escrow`) |
| memory by coordinates; each question on its own line; the dialect shed by reference | §4.13 | `entity/chatter.rs` (`QUESTION_LINE`, `lapse_question`, `release_unmet`, `lapse_unpriced`); `entity/perceive.rs` (`lapse_hearings`); `entity/think.rs` (`Mind::shed`) |
| a task as an offer and as perception; tension | §3.1, §4.13 | `entity/offer.rs` (`Offer::of`, `couples`, `strained`, `lapse_offers`, `lapse_standing_task`); `entity/court.rs` (`task_inflow`, `comprehend_waiting`); `entity/learn.rs` (`pull_task`) |
| the corpus met at the focus; the ground state | §4.13 | `entity/focus.rs` (`Corpus`, `ground_focus`); `t/cech/orbit.rs` (`OrbitRegistry::focus`, `Field::lands`); `entity/feed.rs` (`inotify`) |
| an act read off its effect | §4.9 | `t/family/acts.rs` (`read_acts`) |
| a rule known by its perspectives; undetermined rules sent for judgement | §5.4 | `entity/family.rs` (`ascertain_on`, `Unexempt`); `entity/perceive.rs` (`TaskSeen`) |
| hearing as sound and word; questions one order down; vantage; given meanings | §5.3 | `t/sentence/hearing.rs` (`hear_across`, `strain`); `t/sentence/rung.rs` (`hear_word`, `posit`, `vantage`, `distinct_vantage`, `learn_by`, `learn_given`, `re_reduce_given`); `entity/chatter.rs` (`price_hearings`, `sound_of`) |
| deferral by an owed citation | §5.3(4) | `entity/contract.rs` (`owe_citation`) |
| evidence: every scene under its family's citation | §5.4 | `server/registry.rs` (`citation`); `entity/court.rs` |
| words installed beside the sentence that taught them | §5.4 | `server/cognition/court/vantage_install.rs` (`install_taught_words`) |
| convergence V1–V4, K1–K2 | §5.5 | `anastome/dialogue/chatter.rs` (`Evidence`, `mycelium chatter v6`); `entity/converge.rs` (`Scenes`); `server/cognition/book/agreement.rs` (`attest_agreements`) |
| the measures W, C, S; D2′ shares read locally; E2–E4 | §5.6 | `t/family/local.rs` (`word_local`, `spoken_section`); `server/cognition/identity.rs` (`transport_on`, `gluing_of`); `server/cognition/court/vantage_install.rs` (`cited_deflation`); `server/cognition/book/demand.rs` (`pay_convergence`); `server/cognition/court/task.rs` (`grow_grants_by_paid`) |
| the Rewrite (court side) | §5.4 | `server/cognition/book/rewrite.rs` (`offer_rewrite`) |
| the modes carrier: μ as exact modes keyed by canonical coordinates, drawn whole or refused naming the mode short; read off a motion or a hue map; carried on a solve's verdict | §5.6 M | `economy/modes.rs` (`Mode`, `Modes::of_motion`, `of_gauge`, `take`); `anastome/dialogue/percept.rs` (`encode_solve_verdict`); `server/cognition/identity.rs` (`measured_replay_read`) |
| the seat's call chosen from what calls did; no enumeration | §6 | `client/sdk/seat_mind/choice.rs` (`Why::Untried`); `client/sdk/seat_mind/own.rs` (`SeatMind::reach`); `client/sdk/seat.rs` (`SeatHold`) |
| the observer: where the winding carrier stands, and calls answered at its next law boundary | §7.2 | `entity/observer.rs` (`Mind::at_law`; calls `now`, `thoughts`, `thought`, `field`, `held`, `scenes`, `questions`, `perception`, `convergence`, `calls`); `observe/live.rs`; `mycelium-client --call "<call>" [--mind HEX]` |

The observer is an instrument permitted against §2.4(1): every call reads,
none moves the mind, and nothing in cognition reads it back.

**Specified, in order**

1. **The algebra held per class** in the mind's field (§4.2, §3.2).
2. **The hypha's acts as perception** — Return, RankStep and Reconnect
   reaching a mind (§3.1).
3. **Modes as the unit of pay** (§5.6 M; 78.162): the ledger paying the
   `Modes` its closures moved in place of a count per lane.
4. **POWC on a held question closed by another mind** (§5.6): the
   conversation's questions reach no court, and an evidence path is a
   design; E2's guard (a fixture whose listing's demand another mind's
   cited form closes); the closed thought's twist spoken under its family;
   convergence K3 (§5.5) on its ruling.
5. **Peer records** (§5.3(5)).
6. **Amendment held beside** (§4.10) — the amended constructor entering
   beside the one it amends, separated by commutator.
7. **The mind proposing a Rewrite** (S3, §5.4).
8. **Seat play from the mind's own orbits**, and the seat's act as a
   physics `Action` (§6).
9. **The cross-chamber bracket** (CB1, §4.9).
10. **Curiosity** (§8); the mirror and the curvature (78.167 D3/D4).

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
   peer's replay of its discharge. Nothing a mind counts about itself is a
   witness.

**What is never done (78.168; §2.4(8)).** None of these is a witness, a
completion or a reason to build:
- a test passing, or a suite green — a guard proves its section's identity
  and nothing else;
- a test written to show the mind closes tasks, solves more, or earns more —
  a test built for appearance, refused; a guard states an identity of the
  algebra (a replay, a fixed locus, a refusal by name), never a network
  outcome;
- a counter rising — solves sent, rules known, closures, μ earned, forecast
  hits — read beside the witness as the observer's diagnosis, never as the
  build's measure, and never chosen as the mind's motive;
- code reached only by a test — dead, refused under item 2.

On a running network the witness is the counterparties' acts: the courts'
POUW and POWC mints and the union's pay for words and sentences no task
named, a peer's replay of what a mind said, a question sealed by another
mind's answer. The minds' readings — orbits closing, held and escaping
across K, returns acted on, slips — are the diagnosis beside it. The tree is
built under MU-STANDARDS, MU-RECURSION's laws, rules 1–17 and checklist
(a)–(t), and the Anti-Flattening Law.

---

## 8. Posed, not ruled

A construct that touches any of these waits on a ruling, never on a guess:
- **non-conformal** — what it names in signals carried over transports
  (posed as the non-linear transposition of non-conformal algebra in
  non-Euclidean geometry);
- **the axiom pairing** — how the axioms of the star pair, posed with lift
  and drag (78.46);
- **the Calabi–Yau reading** — neither the doctrine nor the tree holds a
  Calabi–Yau manifold, and a literal one is refused: it is fixed by a
  Ricci-flat Kähler metric whose existence is Yau's theorem, proved to
  exist and not constructed exactly over ℚ (laws 1, 4; 78.45). What the
  engine holds of its structure, exactly: SPECIAL HOLONOMY — a chart whose
  holonomy algebra is a proper subalgebra of `so(n+1,1)`; a COMPLEX
  STRUCTURE — `Cl(4,1)`'s pseudoscalar `I`, central with `I² = −1` (78.105),
  the fact behind `Cl(4,1) ≅ M₄(ℂ)`; COMPACTNESS — closed orbits at exact
  periods, and a universe's own geometry, curved or a flat torus (78.140).
  Which of these is the engine's Calabi–Yau reading is left to ruling.

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

And what the hypha has not built, which the mind's side waits on: μ as
modes on the ledger (78.162; §7.1 Specified item 3); the hypha's acts reaching a
mind as perception (§3.1).

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

- **6 — Comprehension.** The court replays and the mind comprehends: a heard form is deconstructed against the working atlas, each cited address glued, re-gauged or withheld and asked by incidence in depth order, and a connection with no chain is comprehended by factorization through the held basis, never by key lookup.
- **7 — Germination in the mind.** Germination is the pivot loop run on every held orbit of a changed idea under the stroke budget — every discharge kept with its certificate, finite remainders filed as torsion, infinite ones spoken as demands — with no ordering of candidates by a measure; two thoughts merge only where Mayer–Vietoris says they are one section.
- **13.12 — Instance three: two thoughts in a mind.** Two thoughts whose simplex sets meet merge only where the difference of the two thoughts' sections is a coboundary on the meet; otherwise they stay two and the class is the coordinate the mind may pivot on.
- **24 — Comprehension (the mind)** (24.1 the working atlas, 24.2 sentences and connections, 24.3 the utterance). The mind comprehends against its working atlas — genome, dialect and held ideas, read in the order Ruled, Stood, Held, each word carrying its standing as a coordinate, never a weight or gate; a sentence missing a citation is withheld in the dialect and its addresses asked in depth order; perceive, think, stand, utter is the fixed order; incidence is deconstruction, never a key; nothing is uttered twice. Its stroke-pacing details are historical (superseded by §4.12–§4.13).
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
- **78.77 — Light through the scene (L1–L6).** Historical as a faculty of the voice (78.90); the reading stands as an instrument: what the output shows beyond congruent bodies is light along null flows, met at interfaces and held in cavities, with refraction as phase continuity, never Snell's law computed.
- **78.78 — Confluence (P1).** Historical as a faculty of the voice (78.90); the reading stands as an instrument: an unsaid output body's inputs read as creation, single carriage or confluence by exact mass, barycentre and confluence sphere.
- **78.79 — The interpreter for incongruence.** Congruent and incongruent readings of a subject are held together, each whole, and contrasted: relevance is the shared core, meaning is the edges. Its instruments are historical (78.90).
- **78.80 — Volume and density, not boundary.** Bodies no motion makes congruent are related by each body's exact moments — mass, spread, inertia form, density — not by its outline.
- **78.81 — The remainder interpreted on the voice.** Historical: amended by 78.90 to the inversion alone.
- **78.82 — Reconstruction by holonomy (HR1–HR4).** Values need not agree across a family; how they move must: a rule is a transport whose loops close across the pairs. Its instruments are historical (78.90).
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
- **78.163 — SAY is HEARD, the thought is ACKNOWLEDGED — by the loop's holonomy.** Historical: chatter v2 is superseded by §5.3; its rule stands there — the round trip said → heard is a loop whose holonomy, split into drag and lift, is the content.
- **78.164 — The board, not the captain: the mind is a field with boundaries.** Historical in its one `Mind::tick` in a fixed boundary order, retired by its third amendment; it stands in that the mind is a field and everything outside it a boundary with an inflow and an outflow, moved by carriers, each on its own circuit (§2.2, §5.1).
