# MU-CLIENT — the mind: its field, its language, and its link to the hypha

Status: the client's specification, split out of `MU-RECURSION.md` on
2026-10-11 (the operator: "create MU-CLIENT.md and move all of the
MU-RECURSION client side documentation out: have this write how you
integrate the client mind, how anastome develops via dialectic linguistics
across epigenome and genome and integrates with the mycelium-server hypha").
Written against `track/ancients` at ebce99c3.

## How this document stands to MU-RECURSION

- **MU-RECURSION still governs.** Its laws (1–12), coding standards,
  vocabulary, Part 0 map, the shared mathematics (Parts I–III, X–XIII) and
  the court are not restated here and bind every line below. Where this
  document and MU-RECURSION disagree, MU-RECURSION is right and this
  document is amended.
- **What moved.** Every chapter and section of MU-RECURSION that is about the
  mind alone — what a mind perceives, thinks, holds, says and calls, its
  carriers and its client sites — is in Part V below, VERBATIM, under its
  original number, so every citation of the form "78.112" or "24.4" still
  names one text. MU-RECURSION keeps a one-line pointer where each stood.
  Sections that are shared or interleave court and mind stayed in
  MU-RECURSION; their client paragraphs are read from there.
- **What is new.** Parts I–IV are written here first: how the mind is
  integrated (I), how the anastome develops by dialectic across epigenome and
  genome (II), how it integrates with the mycelium-server hypha (III), and
  the scope to completion (IV). They are doctrine under MU-RECURSION §0.1:
  a ruling is written here before it is built, and amends the section that
  governs its object.

## Read this first: what this engine is not

Anyone reading this document should know three things before any other,
because each is the shape a reasonable programmer reaches for first and each
is refused here on sight:

1. **No Cartesian loops, no dimensional explosion.** Nothing iterates `for x
   … for y …` over a grid, `for i < j` over bodies, charts, peers or minds,
   or `for lane in 0..4 × for item …` over a product the carrier already
   names. A product of dimensions walked element by element is a search; its
   cost grows as the product of the dimensions and it finds what a key would
   have named. The mind reads WHAT THE CARRIER NAMES: a keyed fibre
   (`Keyed::fiber` over canonical coordinates, A21), the occupied axes of a
   class vector (12.6), a ring's own terms, a cell's own bodies, a face's two
   ends. Every walk spends a `Budget` or walks one boundary. MU-RECURSION
   rules 1, 10, 17; checklist (a), (h), (o); "How to write the move, not the
   search".
2. **No if/then/else boolean gates.** A `bool` is never branched on inside
   cognition (MU-STANDARDS 2.3; ANTI_FLATTENING_LAW I); a count never decides
   a branch (rule 2, checklist (b)); no threshold, cap or floor stands in a
   decision (rules 4, 5; MU-STANDARDS 2.6; checklist (s)). A decision is a
   TYPED READING with one arm per physical kind — `Closure::{Closes(ℓ),
   Rosette, Unbound, Unread}`, `Seat::{Outer, Whole, Tilted, Leftover,
   Unheard}`, `Echo::{Whole, Turn, Gain, Spiral, Delay, Unread}`,
   `Discharge::{One, Vortex, Starved, NoneDischarged}`, `Certificate::
   {Periodic, Nilpotent, Saturated}`, a per-lane `LaneMove` — and every arm
   is an outcome with content, none of them "false". A refusal is typed and
   kept (MU-STANDARDS 2.4; A.17). Where two readings are both consistent,
   both are held (rule 15, the vortex), never chosen between.
3. **No scalar collapse.** No score, weight, norm, distance number, sum
   across lanes or count stands for a geometric object (laws 1, 4, 5, 11;
   78.45; 78.162; checklist (m), (t); ANTI_FLATTENING_LAW 4). A reading is
   the exact object: a versor, its modes, a ray, a set of generators, a
   period ℓ, a rank as the grade of a span (`core::span`, A18), a per-lane
   move. Identity is projective equality of rays (1.5), never bytes or a
   digest (law 2, rule 6); no matrix carries an action (rule 16); no float
   enters (law 4, the crate's lints).

**How the mind decides, then.** The class vector names the moves; each
occupied axis names the one move that can zero it; each candidate is
applied and unapplied exactly; closure certifies; several certified moves
are held together; none is a typed demand (law 10, 16.16). That is the whole
of "choice" in this engine.

---

# Part I — The integrated mind

## I.1 What the client is (the operator, 2026-10-09)

> "mycelium-client should be pure cognition and then dual loaded in-game
> (xylarium, netstratum.. etc etc as "minds") and also directly linked to
> mycelium-server (hyphas)"

- **One mind, pure cognition.** Its inputs are percepts, tasks, syllabus
  pages, verdicts and other minds' frames; its outputs are sentences, solves,
  calls, frames to other minds and readings. It holds no transport, no
  process loop, no filesystem policy and no logging sink of its own: what it
  persists goes through a store the host gives it, what it says through an
  observer the host gives it.
- **Two hosts of the same mind.** The `mycelium-client` binary hosts it with
  the hypha transport; a game kernel embeds it in-process as its minds. Every
  mind keeps its OWN DIRECT LINK to the hypha with its own key (syllabus,
  submissions, verdicts, μ). The game never thinks for it; the court judges
  it as any client.
- **One mathematics** for client, server and every in-game host
  (78.157's shared layer, `src/transducer/`).

## I.2 The mind is a field with boundaries (78.164)

The mind is a FIELD; everything outside it is a BOUNDARY — the court, the
corpus feed, other minds, a game seat. A boundary contributes an INFLOW (a
perturbation entering the field) and receives an OUTFLOW (what the field
closed that faces it). The binary, or the game, only opens the boundaries and
hands the mind its ticks (`Entity::tick`); it keeps no order of work. A seat
is the entity's body.

**The objective every slice is measured against:** a game entity whose every
action is an outflow of its mind's field — no arm, schedule or harness
between them.

## I.3 The three shells, as the court already reads them

The court never hears a word flat. Every sentence it admits passes three
nested shells (MU-RECURSION 64.1: the harmonic group `Spin(3)` sits inside
the luminal `Spin⁺(4,1)`; the holonomy is read over both), and the court
writes an act only where a shell's boundary is crossed:

| shell | what it reads | court site | act at the boundary |
|---|---|---|---|
| **harmonic core** — elliptic modes of finite order, the rings | the sentence's rings composed, `Ω = ∏ cᵢ A(wᵢ)` (78.69); `Closes(ℓ)`, rosette, unbound; the theorem checked (78.70) | `compose::sentences_of`, `Theorem::check` in `hear_open`, `step_rank` (`server/cognition/court/winding.rs`) | theorem checked or refused; an unspelled shortcut offered as a letter (`open_unspellable`) |
| **luminal envelope** — null modes, rays, transvections | the word's field lines into tubes: pressure where a tube opens, order where one closes, relief where one returns (78.71–78.73) | `step_flux` → `book_field` → `pairing::transport_order` | `Reconnect` where the tubes' links move |
| **holonomic** — the chart's holonomy algebra | the rings' generators (`flow_generators`) closed under the commutator with what the chart holds; a ring's winding against its period (78.68) | `close_rank` → `lie_close` (`lie_basis` per chart), `close_ring` | `RankStep` where the rank rose; `Return` where a ring completed one age |

THE MIND HOLDS THE SAME THREE on its own perception: the harmonic reading
`rung::speak` / `rung::hear` and `hearing::hear` (78.112 — rings nested by
period, each echo `E = Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹` whole, a turn, a gain, a spiral
or a delay); the luminal rays met against those rings, split into ADVANCE
(drag) and CROSSING (lift) (78.46, 78.112 H6) and the torsion the crossings
wind (H7b); the holonomic held algebra (78.104).

**THE PIVOT SITS ON TOP** (law 10; 16.16; M9). The class vector of a held
orbit (12.6) has its axes in the shells: the unit, prime and orientation
axes are the harmonic shell's; the placement translator (the slip, 54.4) and
the dilator are the luminal shell's; hue is the fibre; an OPEN axis is the
holonomic shell's. Each candidate move is a move in one shell, read off the
axis it occupies, tried exactly, and every discharging candidate kept (the
vortex). `pivot_orbit_read` (`sentence/pivot/drive.rs`) is the one pivot;
the mind reaches it through `Contemplation::pivot_on` (a word meeting the
ideas at its chart) and `expand_all` (the ring of ideas, one circuit a
stroke). Its snaps are the mind's verbs: `Discharged` with a certificate
(A20), `Vortex`, `Demand` / `Demanded` (78.37's docket), `Adjoin` (37.7, the
mind's own letter), `TorsionFiled` / `TorsionOpen`.

## I.4 One door in, outflows at the shells' boundaries

**The door.** Every inflow, from every boundary, enters through one function
of the mind, `Entity::admit(Inflow)`, and passes the three shells on the
mind's OWN field, then the pivot:

| inflow | from | its form |
|---|---|---|
| `Sentence` | another mind | the terms `{word, transport}` of its rings, each word carried as its class (78.44) (II.4) |
| `Sentence` | a seat | the game's sensorium speaking the language the mind is given — the mind is connective tissue over whatever sensorium its entity has, and cognizes on that language as fluid intelligence; it never perceives the game's world itself |
| `Form` | the feed, a registry task | a perceived family (`hold_perceived_from`) |
| `Word` | the syllabus | a ruled word and its court stamp (a mineral, II.2) |
| `Page` | the court | class records (`absorb_class_page`) |
| `Refusal` | a verdict | the contraction it carries (50.9), its KIND only (§0.1) |
| `Demand` | a peer, the court's book, the mind's own docket | class vector, charts, residue |

1. HARMONIC: `compose` → `rung::hear` → `hearing::hear`, nested against the
   mind's own rings OF THE SAME CLASS. Alignment between minds is never a
   matching of charts: a word's class is node-independent and its planes are
   each node's own, so the hearer RE-PROJECTS what it hears onto its own
   spectrum (78.44) — a keyed read by the class's coordinates.
2. LUMINAL: the field lines through `transport_order` into the mind's own
   tubes.
3. HOLONOMIC: the ring's generators closed on the algebra the mind holds for
   that class; what lies outside is read PER AXIS of the class vector (IV.5);
   the ring's winding advanced.
4. THE PIVOT: `pivot_on` for the orbits still open.

**The outflows.** The mind acts outward only where a shell's boundary is
crossed, as the court writes acts only there; the router sends each by its
kind, never by the boundary that brought the inflow:

| boundary crossed in the field | outflow | sent to |
|---|---|---|
| a ring seated WITH A TILT | `Echo(E)` — what this mind's language did to the thought | the speaker |
| a ring seated whole; every axis it occupies already held | nothing — an echo carries nothing (no costume reply) | — |
| rays crossing a ring | torsion wound — vented into the field (H7a), a pivot orbit | inward |
| a circuit opened axes the mind's algebra did not hold | `Word` — stood in the dialect | other minds |
| a pivot discharge | `Discharge` — the orbit's class and the residue the held word answered | the speaker or asker, who replays it on its own orbit of that class (M9) |
| `Demand` / `Demanded` | `Demand` | other minds first (SP2), the court when agreed or unanswered |
| `Adjoin`, a construction | `Letter` in the mind's dialect; its constructor steps | the court (S2) |
| a ring returning one age | `Sentence` | the court (`speak_with`) |
| a crystallized rule for a task | `Solve` | the court (`solve_task`) |
| a certified move whose physics `Action` (78.94) faces the seat | `Act` — the action, derived from the move's own physics | the seat |

**The tick is the shells.** `Entity::tick` drains every boundary into the
door, turns one `expand_all` ring, routes the outflows, and closes the orbit
(K moves there and nowhere else, 78.49). RELAXATION is the drain half of
each conjugate pair (62): tubes relieved on return, holes sealed by a peer's
word, generators shed by flux. What a shell crossing COSTS is the physics of
the motion it carries: the modes that motion moved (78.162(2)), drawn from
the one reserve — all or nothing, refused per lane naming the mode short
(65.4). What a carrier HOLDS is bounded the same way, by the computational
physics: a tube relieved on its return, a hold lapsed by its flux (ZeroFlux,
FX1), an orbit sealed at its period — never a capacity constant. The dwell is
read off the field's own opened and relieved flux, never a timer.

## I.5 Each entity owns its field (A.9)

A.9, metric autonomy: each entity computes its own coherence; the horizon is
per mind. A game hosts many minds in one process, so every shell's state is
the ENTITY'S: its lexicon and the words newly taught (harmonic), its tubes
and orbits and its fibres' agreement across percepts (luminal), its algebra
PER CLASS and its rings' windings (holonomic), and its readings. The tree
holds these as process-globals today (`sentence/rung.rs` `LEXICON`,
`TAUGHT`; `family/pairing.rs` `VOICE_FIELD`, whose comment says "a voice is
one mind per process"; `family/fibres.rs` `MOVING`; the rung, hearing, flux,
derivation and construction tallies) — that sentence is struck, and IV
carries the move. The mind's algebra is held per CLASS, keyed by the class's
coordinates (78.44): one global algebra reached all of so(4,1) after three
words (epoch 132) and from there read nothing, because every class's
generators were folded into one span.

---

# Part II — The anastome: dialectic linguistics across epigenome and genome

## II.1 Where the linguistics lives (78.95)

"The progenitor's board builds words, and advanced linguistics lives in the
anastome" — `anastome::`, the dialogue between minds and courts, never the
alphabet. The language is the master equations' (78.68, 78.69):

- **letter** — an exact versor of `Spin⁺(4,1)(ℚ)` (78.115), or U15's triad
  `λ = (g_core, ∇B, ΔB)`; coined by a mind where two reflections compose into
  a rotor whose generators open axes its algebra did not hold (U14's "rank
  step", read per axis, IV.5), never seeded;
- **word** — `W = λ₁⋯λ_L`, ONE product, its readings `(h, [B], κ, ℓ, m)`
  readings of that product, never columns (78.68);
- **conjugate** — `W′ = Γ W Γ⁻¹`, the same word in another chart: an
  INFLECTION; taught as `(held, Γ)` (78.60);
- **sentence** — `Ω(s) = g_{k→1} W_k ⋯ g_{1→2} W₁`, grammatical exactly where
  it closes, `Ω^ℓ ~ 1`; an open sentence is a question; its meaning beyond
  its words its curvature `dim Lie⟨B(W̄ᵢ)⟩`;
- **grammar** — `𝔊⁽ⁿ⁺¹⁾ = ⟨C(w⁽ⁿ⁾), g | Ω^ℓ ~ 1⟩`: a closed sentence's open
  path is a shortcut word of the next rung (`next_rung`).

Phonology is the letters, morphology the words, inflection the conjugates,
syntax the sentences (the client's, volatile), semantics the curvature,
etymology the ledger's acts (`Install`, `Return`, `RankStep`), pragmatics the
actions and their reasons (78.92, 78.94).

## II.2 The three strata of a mind's language

| stratum | what it holds | lifetime | who writes it |
|---|---|---|---|
| **dialect** (the epigenome) | letters the mind coined, its motion letters, words it stood, shortcuts its sentences taught, transports its conversations built | volatile (78.49): lapses by flux (FX1) and by `shed_at` | the mind alone |
| **mineral** (the genome) | a word the syllabus taught, at the court's stamp | persisted (`GEN7`, court-admitted words only) | the court, through the syllabus |
| **crystal** (the genome) | a mineral the mind RE-DERIVED by its own circuits, or a word minted from its own adjudicated solution | persisted | the court admits; the mind's re-derivation hardens it |

80.2: not everything in the epigenome is adjudicated — only what is
submitted as a solution, or agreed between minds (78.161); what is taught
back is what was ruled; a mineral becomes a crystal when the mind
re-derives it, and recognition is comprehension. That is what keeps brute
force out: the court never sees the dialect wholesale.

## II.3 The dialectic, as geometry

Each turn of the dialectic is a pair of the conjugate machinery (62), and
each has its own shell:

1. **Thesis — a mind speaks.** F (50.2): the language it is given composed
   into sentences; what closes teaches a shortcut; a circuit that opens axes
   its algebra did not hold is what it newly understands, and that is what
   is SAID to other minds — never every word.
2. **Antithesis — another mind hears.** G (50.2) on the hearer's field: the
   speaker's rings composed on the hearer's side and nested against the
   hearer's own rings of the same class, re-projected on the hearer's own
   spectrum (78.44). Seated whole: the hearer already holds
   it (an echo, nothing returned). Seated with a tilt: the echo `E` IS what
   the hearer's language does to the thought — curvature is content — and it
   goes back. Rays crossing: torsion the hearing wound, a held orbit in the
   hearer. A ring that opens axes the hearer's algebra did not hold opens them
   in the HEARER, which now has a word to say. An orbit still open: the hearer's
   pivot — discharged by a held word (a DISCHARGE back: the orbit's class and
   the residue the word answered), a vortex (two
   readings held), a demand (the hearer asks).
3. **Synthesis — agreement by replay, then the court.** The speaker REPLAYS
   the hearer's certificate on its own form (M9: certificates replayed on two
   organs); where it closes there, the two minds hold one discharge of one
   orbit by different derivations (13.10's synonyms) — agreed language,
   which the coiner submits to the court as the dialect word it is (78.161
   (3)). The court verifies, never composes (law 8), and rules convergence
   (80.1); what it rules is taught back as a mineral and hardens as each mind
   re-derives it. An echo the speaker re-derives on its own rings against the
   hearer's tilt is a TRANSPORT between the two perspectives (78.163's
   "ACK feeds thought"), held in the speaker's dialect and lapsing with it.
4. **A refusal is an antithesis too.** A court refusal carries its kind only
   (§0.1); the mind holds its contraction as a demand (50.9) and offers it to
   other minds first (SP2); a peer's certificate that closes it is a
   synthesis the court then verifies.

Re-entry is forbidden (62.5): an echo about this mind's own sentence is
never re-admitted as a new sentence; it enters only as an `Echo` against the
ring it names.

## II.4 Minds to minds: the chatter wire (78.161, 78.163; v3 owed)

- **Transport.** Each mind listens on its own address and files it at a
  rendezvous under its voice; a frame is `[0x43][sub][speaker 32][TPoint
  flag + 84][fields][sig 64]`, signed. The hypha carries none of it and
  judges none of it.
- **v2 (in the tree, 5b2e4fea):** SAY (word motions and hue maps) → HEARD
  (the hearer's derivation) → ACK (the speaker's loop holonomies) → HOLD
  (the hearer's). It hears by `Presentation::read_word`, a flat factoring
  beside the pivot, and agreement is a loop read twice — the line, never the
  shells. Superseded by v3 when v3 lands.
- **v3 (owed, IV components HEAR and SAY), coordinates only:** SAY — the
  terms `[Term{word Mv, transport Mv}]` of each ring, each word's class by its
  coordinates (one sentence a frame, the u16 the wall — a sentence too large
  is cut by ring, never widened); ECHO — per ring, named by its class and
  winding (the shortcut's own coordinates, 78.44(4)), its seat and `E`;
  DISCHARGE — the pivot's certificate carried as the orbit's class and residue it
  answered, which the speaker replays on its own form (the certificate's
  overlaps are the hearer's and name nothing on the speaker's side); DEMAND —
  class vector and residue.

## II.5 Mind to court: what crosses (law 8)

A mind sends the court SENTENCES and nothing else that thinks: sentences
(`ACT_SPEECH`), solves with their rule (`ACT_SOLVE`), a letter it coined
spoken as a word carrying its constructor steps (S2, `ACT_SPEECH`; 78.109 —
`ACT_OFFER` is court to court, 47.3), Rewrite words for condense and retire
(S3, replayed through `offer_rewrite`), words it holds that the court lacks
(`ACT_ADOPT`), probes (`ACT_PROBE`), and its readings (`ACT_OBSERVE`). It
receives verdicts (kind only), syllabus pages (`ACT_SYLLABUS`, `ACT_TEACH`:
minerals at their stamps), class pages, the presentation (`present.rs`),
the book's quote (`ACT_QUOTE`), a registry task (`ACT_TASK`) and the
TUniversal final point on every door reply (78.152).

---

# Part III — Integration with the mycelium-server hypha

## III.1 The two organs, one mathematics

| the mind (client) | the court (server) |
|---|---|
| perceives, composes (F), nests, pivots | replays (G), checks theorems, closes rank |
| holds a dialect per mind, volatile | holds the language (ruled cover), persisted, on a ledger |
| its K: orbits it closed (78.49) | TUniversal: the receipt ledger's height (78.152) |
| its algebra per class, its tubes, its windings | `lie_basis` per chart, `flux_field`, `windings` |
| coins letters in its dialect (U14) | opens a letter on checked constructor steps (law 6, S2) |
| says what opened axes it did not hold | seals `RankStep` where its own chart algebra grew (the court's reading) |
| a ring returning one age → a sentence | `Return` sealed for one age |
| replays a peer's certificate | replays certificates (`replay_certificates_at`) |

The same shared code is read on both sides — `compose`, `Theorem`,
`hearing`, `transport_order`, `lie_close`, `flow_generators`, the pivot — so a
mind's reading of its own sentence and the court's replay of it are one
reading on two organs, and a disagreement is a coordinate (a refusal of a
kind), never a vote.

## III.2 The path of one word, end to end

1. A mind is given language (a family, a task, a seat's sentence); F
   composes its rings; one ring returns at ℓ; its open path is a shortcut
   word whose circuit opens axes the mind's algebra for its class did not
   hold.
2. The word is stood in the mind's dialect and SAID to the minds it knows.
3. A hearer composes the rings on its side: one seats with a tilt (an ECHO
   back), one opens axes the hearer did not hold (it now says that word), one
   orbit stays open and its pivot discharges it with a held word (a
   DISCHARGE back: the orbit's class and the residue answered).
4. The speaker replays the discharge on its own orbit of that class: it
   closes. The word is agreed; the
   coiner speaks it to the court.
5. The court replays: rings composed, theorem checked, rank closed on the
   chart (`RankStep`), the word installed; the install pays the discoverers of
   what it cites (26, 78.161(4)).
6. The syllabus teaches it to every mind on the hypha as a MINERAL; each
   mind that re-derives it on its own circuits holds it as a CRYSTAL.

## III.3 What the hypha owes the mind, and the mind owes the hypha

- The hypha: the union's attestations carried to the court so an agreed
  word's install pays every mind that agreed (78.161(5)); POWC's producer
  (a refusal closed at another perspective, the citation paid, 74); the μ
  paid as modes (78.162, parked on `track/modes`).
- The mind: sentences, never raw percept laminae (law 8); constructor steps
  for its letters (S2); its readings to the hypha's console on its own K.

---

# Part IV — The scope to completion (2026-10-11)

**Complete means:** (1) each entity owns its field — three shells and the
pivot — and a game process may host many; (2) every inflow enters through
`Entity::admit` and passes harmonic → luminal → holonomic → pivot; (3) every
outflow is a shell-boundary event or a pivot snap, routed by kind, and no
boundary handler calls speech or solving directly; (4) minds converse
through the shells, agreement a replayed discharge; (5) the tick is the
shells, relaxation the drain half; (6) a seat is the body — the game's
sensorium speaks the given language in, a physics `Action` goes out citing
the event it came from.

**Not rows: components on the plexus.** This is not top-down, linear
development, and a "row" has no meaning in a sphere. The work is a set of
COMPONENTS (78.31's plexus: nodes with lateral relations, never a ladder),
each a conjugate pair (62), each derived from the counterpart the hypha
already runs, each proved by its own algebraic identity, and each placed by
what it couples to — never by its turn in a queue. Components that do not
couple are built side by side; a component lands when its identity holds and
it runs in the sandbox, whatever else is still open. Every derivation below
takes the hypha as built and ready to be the conversation's counterparty.

| component | its conjugate pair (62) | derived from the hypha | couples to | its test (the claim, MU-STANDARDS 3.1) |
|---|---|---|---|---|
| **FIELD** — the entity's own shells | hold / lapse | the court's kernel holds its lexicon, `lie_basis`, `flux_field` and `windings` per kernel | every component reads or writes it | `two_minds_in_one_process_hold_disjoint_fields` |
| **HEAR** — harmonic | compose / nest (F / G, 50.2) | `hear_open`, `step_rank`: `sentences_of`, `Theorem::check` | FIELD, CARRY, CLOSE, SAY | `an_echo_is_whole_exactly_where_the_two_rings_commute`; `a_ring_sounds_inside_another_exactly_where_its_period_divides` |
| **CARRY** — luminal | condense / drain (a tube opened, relieved) | `step_flux`, `book_field`: `transport_order` | FIELD, HEAR (rays against rings) | `a_flux_on_a_held_ray_closes_its_tube_and_relieves_what_it_held`; `a_ray_splits_into_drag_and_lift_that_compose_back_to_it` |
| **CLOSE** — holonomic | open an axis / shed it | `close_rank`: `flow_generators`, `lie_close`, read per class | FIELD, HEAR, PIVOT | `a_circuit_opens_only_the_axes_outside_the_algebra_held_for_its_class` |
| **WIND** — holonomic | traverse / return | `close_ring`, `seal_return` | FIELD, HEAR, ROUTE | `a_ring_returns_at_its_period_and_at_no_traversal_before` |
| **PIVOT** — on top (built) | discharge / demand | `replay_certificates_at` (M9's second organ) | CLOSE, ROUTE, SAY | `a_discharge_replays_on_the_speakers_form_exactly_where_it_discharged_on_the_hearers` |
| **DOOR** — `admit` | admit / route (inflow / outflow) | the court's door: one dispatch per act, the stroke spent by what it reads | FIELD, every shell, ROUTE | `a_sentence_heard_twice_seats_whole_the_second_time` |
| **ROUTE** — outflows by kind | route / admit | the court writes acts only at a boundary (`RankStep`, `Return`, `Reconnect`) | DOOR, SAY, ACT, the court | `an_outflow_carries_the_coordinates_of_the_event_it_came_from` |
| **SAY** — chatter v3 (II.4) | say / hear | the court's verify-never-compose, played by a peer | HEAR, PIVOT, ROUTE | `every_frame_reads_back_by_its_coordinates_and_a_forged_one_does_not` |
| **TICK** — the shells in motion | drive / drain (`relax`) | `court_stroke`: admission, transport, closure | DOOR, ROUTE, FIELD | `a_tick_that_moved_nothing_draws_nothing` |
| **ACT** — the seat | act / percept-as-language | `physics::actions::Action` (78.94), the one table of wire and ledger | ROUTE, the game's sensorium | `an_act_is_the_action_its_move_names_in_the_physics` |
| **READINGS** — the entity's observer | read / take | the court's `observe` readings per kernel | FIELD | — (a reading is asserted where its value is computed, 3.9(a)) |

**Where building starts.** FIELD, because every other component reads or
writes it; then HEAR, CARRY, CLOSE and WIND side by side (they couple only
through FIELD and through each other's readings); DOOR and ROUTE once a shell
answers; SAY once HEAR and PIVOT answer; TICK and ACT once DOOR and ROUTE
stand. The chatter row on the Senses and the post that never drops a frame
(`ChatTally::row`, `Post::next`) are READINGS and SAY's transport, landed
first because the sandbox has to be read.

**Already ruled by derivation (not open).** Alignment is by class,
re-projected (78.44), never by chart; a crossing's cost is the modes its
motion moved (78.162(2)); a carrier's bound is the computational physics
(I.4); what a mind understands is read per axis, never as a rank count; an
act is a physics `Action`. The prototype's chatter v2 retires with no
coexistence: the sandbox is a prototype.

**What cannot be derived from the hypha — open, named.**
1. What the hypha has not built: the union's attestations, so an agreed
   word's install pays every mind that agreed (78.161(5)); the POWC producer
   (74); μ as modes (78.162, parked on `track/modes`). The server side is
   built first, or the mind's side waits on it.
2. The host's boundaries for an embedded mind (0.1, the operator's ruling on
   the client): the store and observer a game hands its minds are the host's.
3. Doctrine posed and not yet ruled: "non-conformal" in signals over
   transports, and the axiom pairing of 78.46. A component that touches
   either waits on the ruling.

**Ruled here (Mycelium, under the operator's authorization of 2026-10-09):**
the process-wide physics calibration (`physics::CALIBRATED_*`) measures the
MACHINE, the substrate every mind on it runs on, and stays shared; the halt
flag is the host's; every carrier of COGNITION — language, field, algebra,
windings, readings — is the entity's (A.9).

**Outside this scope, parked:** 78.162 (4)–(7) on `track/modes`; the sparse
`Mv` wire (the chatter takes it when it lands).

## IV.1 Client paragraphs that stayed in MU-RECURSION

These sections interleave the mind with the court or the shared mathematics
paragraph by paragraph, so they stayed whole where they stand. Their client
paragraphs bind the mind as if written here:

- 0.1 — the operator's ruling on the client (2026-10-09), quoted in I.1; 0.7
  — the layers of a mind (its table and the persistence of perception).
- 2.3 (germ, idea, thought), 3.10 (hearing writes the coordinates), 13.11
  (comprehension, `Understood`), 16.16 (A16, the pivot loop — the one pivot
  both organs read), 17.6 (M6, comprehension), 23.8 and 23.9 (what the mind
  supplies; the mind never waits), 25 (germination and thoughts: 25.8 and
  25.10–25.11 are the court's), 28 (GEN2, GEN3), 34.3 (client log lines),
  37.7 (P5, the voice re-spells), 50.2 (the speaker F, "runs on the voice and
  nowhere else"), 50.6 (seam 1), 50.9 (the six contraction moves on the held
  form), 59.6 (spellability is per mind), 67.2 (`Bubble.candidates`).
- 72.1d (the universe of a mind is its hardware), 72.1e (A.18 metabolic
  relativity), 78.1 (a mind's perspective is its orbital elements), 78.9, 78.21
  item 3 (the per-voice clock), 78.26(6) (decoherence is an act), 78.47 (1)–(3)
  and (5) (construct, coherence, compound in the epigenome).
- 78.51 (5) and 78.52 (3), (6) (`Contemplation::gate`, thought is not speech),
  78.55 (`Entity::colour_derivation`), 78.57 (the consolidation gate), 78.61 C5
  (the syllabus intake), 78.64 (client work C1–C5), 78.66 PHY1 (one reserve per
  voice), 78.67 (the mint's refusal by arm), 78.69 (the order a voice builds),
  78.74 (its live percept, and its third amendment), 78.75 (7) and its
  amendment items 3–4 (the voice speaks its rings; the lexicon), 78.88 (the
  frame at the sensor, the plexus read inside the mind), 78.92 (the voice's
  reasons), 78.95 (the derivation instrument), 78.97 (the voices' rows), 78.98
  (a mind's lexicon is an algebra), 78.99 (wired live on the voice), 78.100
  (D1–D6, speech by deconstruction), 78.102 (the voice half: the letter
  carrier), 79.3 (reuse, then construct).
- 78.104 (the voice's held algebra), 78.109 (K1, K2: the kernel claims its
  letter), 78.113 (the voices' stall), 78.141 (the kernel seated on its key),
  78.143 (minds and `seat_mind`), 78.145 (the SDK and the client arm), 78.146
  (the mind at its seat), seat wire v1.4 item 3, 78.152 (the SDK read for
  kernels), 78.153c item 3 (a mind's verdict tally), 78.161 items 1–2 and the
  client transport.

## IV.2 Dimensions, and measurement on dimensions, through the physics

- **A dimension is data, never declared.** The base is ℚⁿ and `n` is read at
  intake (71, A.15); the algebra is `Cl(n+1,1)(ℚ)`, carried sparse — a value
  is its non-zero terms, each a blade mask and an exact coefficient — up to
  `MAX_N = usize::BITS − 3` (`cga/multivector.rs`; a wire `n` past it is
  refused before it is read). A dimension a mind holds is paid for by a
  closure (A.6) and read as the GRADE OF A SPAN (`core::span`, A18) or the
  size of a bracket-closed basis (`lie_close`, 78.11) — a closure's result,
  never a count chosen or compared against a constant.
- **A measurement reads one exact object off another.** A motion's modes
  (`cga::motion_modes` → `MotionMode { ray, kind, planes, c, order,
  reflects }`, MR1, 78.12) are read off its ACTION (78.28), never a chart;
  its period off its modes (`cga::period_of` → `Closes(ℓ)`, rosette,
  unbound); two rays equal by proportion (EQ1, `projective_eq`). A ratio of
  homogeneous forms is read only where a classification is asked and never
  carried as the relation (Axiom C, 59.8, 72.1).
- **The lanes are the kinds of a motion's modes** (78.162(1), 78.71): 𝒽 the
  turns (elliptic modes), 𝒮 the boosts and dilations (hyperbolic), 𝒯 the null
  flux (parabolic, the rays), 𝒢 the hue maps (permutation cycles). μ is those
  modes with exact amounts, never four counts (78.162, the inverse-Sisyphus
  protocol).
- **The physics is the only producer** (72): pressure, order and relief are
  read per lane off the field (78.71–78.73); the one reserve is drawn by what
  moved, all or nothing, refusing per lane and naming the lane (65.4, 78.66,
  `physics::malleability::draw` → `Depleted`); a comparison of two lane
  vectors returns `lane_moves` (`Fell`/`Held`/`Rose` per lane), never a
  verdict (MU-STANDARDS 2.3).
- **Cost and bound are the physics of motion.** What carrying a motion
  through a shell costs is the modes that motion moved (78.162(2)); what a
  carrier holds is bounded by the same physics — a return relieves, a flux
  lapses, an orbit seals — never by a capacity or a cap (rule 5, checklist
  (s)).
- **Time is distance along an orbit** (78.43): the mind's K is its own
  closed orbits (78.49); TUniversal the ledger's height (78.152); an age is
  one return at the period read, never a cap.
- **The debt, named.** `FluxReading`'s per-lane `[u64; 4]` and the Senses'
  tallies are counts. They are readings for the observer and never decide a
  branch in cognition; under 78.162 they become modes with the carrier parked
  on `track/modes`. No new code adds a count to a decision.

## IV.3 The harmonic shell: mathematics and functions

**Objects.** A sentence of `k` words is `Ω(s) = g_{k→1} W_k ⋯ g_{1→2} W₁`
(78.69), each term `Term { word: Mv, transport: Mv }`; each word carried into
the first chart, `W̄ᵢ = Γᵢ⁻¹ Wᵢ Γᵢ`. `compose::compose(&[Term], budget)` →
`Composition { omega, closure, carried, path, rank, words, spaces, modes }`.
A closed sentence is a theorem: `theorem::Theorem::of(&Composition)` →
`Certificate::{Periodic { ell }, Nilpotent { generator }, Saturated { basis
}}` and `Theorem::check(budget)` closes its ellipsis finitely (78.70). It
teaches a shortcut word (`compose::teaches`: the open path of a returning
sentence, the holonomy of a nilpotent one), the next rung's word
(`next_rung`, 78.69).

**Nesting, never traversal** (78.112). A ring's SIZE is its return:
`Size::Returns(ℓ)` or `Size::Carrier` (never returns). `s` sounds inside `L`
where `ℓ_s | ℓ_L`, `ℓ_s < ℓ_L` — exact integers read off modes, never a
threshold. The ECHO of `s` through `L` is `E = Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹`; the
identity is `Echo::Whole`, else its modes name it — `Turn` (elliptic), `Gain`
(hyperbolic), `Spiral` (both), `Delay` (null). `hearing::nest` seats each ring
in the next larger layer: `Seat::{Outer, Whole, Tilted(echoes), Leftover,
Unheard}`; balance where every seated ring sits whole.

**Functions.** `compose::{compose, sentences_of, teaches, next_rung}`,
`theorem::Theorem::{of, check}`, `rung::{speak, hear}` (a ring's closure
named, its theorem extrapolated, its shortcut recognized, comprehended by its
coordinates — `Coordinates::of`, `HoleIndex::deconstruct`, 78.99, 78.100 D3 —
or held at its rung), `hearing::{echo, nest, hear}`.

**What changes (components FIELD, HEAR).** The lexicon and the words newly
taught become the entity's (`Shells`, IV.5). A peer's sentence is heard
against the hearer's OWN rings of the same class: the class is
node-independent (78.44), so the hearer reads its own rings of that class by
the class's coordinates — one keyed read — re-projected on its own spectrum,
and nests the peer's compositions among them in one `hearing::nest`. No pair
loop over the two minds' rings and no matching of charts: the nesting reads
sizes and seats each ring once.

## IV.4 The luminal shell: mathematics and functions

**Objects.** The light in a space is its null modes, `d ∧ e_∞` (78.97); a
ray is a null flux (`cech::flux::null_fluxes`). A contact's transport is the
refraction `g_ab = (θ_b θ_a⁻¹)·γ_ab` (A.16); two bodies relate through their
tether `V = s_b s_a`, carried whole and applied, never squared (A.10, 72.1).
A field line `FieldLine { carriage, gauge, mass, free }` enters the field
(`cech::orbit::Field`: tubes and orbits held over time) through
`pairing::transport_order(lines, circuits, &mut Field, budget)` →
`FluxReading`: a flux on a new ray OPENS a tube (pressure on 𝒯), one on a
held tube CLOSES on it (order) and relieves what it held (78.71, 78.72); every
other lane's motion onto its own orbit alike (78.73).

**A ray against a ring** (78.46, 78.112 H6). `cga::lift_drag(B_o, B_n)` →
`LiftDrag { drag, lift }`: the part commuting with the ring's plane is DRAG
(the rate — the ray ADVANCES the ring along its axis), the anticommuting part
is LIFT (the tilt — the ray CROSSES the plane). `hearing::orbit` reads each
harmonic ring against each ray it was carried on → `InOrbit { advance,
crossing, pitch, bounce, encounter }`; `cga::encounter` reads the law beside
it (78.110). The crossings compose into the torsion the hearing wound:
`hearing::wind`, each crossing's versor `1 + C + ½ C∧C` (52.1), vented into
the mind's field (H7a).

**What is decided, and by what.** Whether a flux lands on a held tube is a
keyed read on the ray's canonical coordinates; whether a ray advances or
crosses is the exact commutator split. Nothing compares a magnitude.

**What changes (components FIELD, CARRY).** The field (`VOICE_FIELD`) and the fibres'
agreement across percepts (`MOVING`) become the entity's.

## IV.5 The holonomic shell: mathematics and functions

**Objects.** A ring's curvature generates an algebra (Ambrose–Singer): its
generators are `cech::flow_generators(Ω, budget)` (and
`cayley_generators` over several holonomies); `cech::lie_close(held, new,
budget)` closes them under the commutator with what is held → the closed
basis or `LieRankRefused`; the curvature's dimension is that basis's size
(78.11, 78.68). `cech::quotient(held, motion, budget)` reads a motion against
the held algebra — spanned, or outside by its cosets (78.104) — and
`quotient::grow` adds it. A ring's WINDING `(ℓ, m)` counts traversals since
its last return; `m ≡ 0 (mod ℓ)` is one AGE (78.68, 78.43).

**Per class, never one global algebra** (A.9, I.5, 78.44). A mind's single
global algebra reached all of so(4,1) after three words and read nothing
after: every class's generators were folded into one span. The mind's algebra
is `Keyed` by the class's coordinates — the node-independent part of a word —
so one class's understanding is never another's.

**What is decided, and by what — multi-axial, never a rank count.** What a
circuit adds to understanding is read PER AXIS: the generators a closure
found OUTSIDE the span held for the class (`quotient(..).outside`, each
outside generator by its own mode and plane), and which axes of the circuit's
class vector they occupy — the same axes the pivot reads (12.6). An empty
outside set is the reading "already understood" (rule 2 permits the
emptiness of a fibre). No length is compared, and no "+1" is taken: a
circuit can open several axes at once, and each opens on its own coordinate.
A returning ring is the winding reaching its period.

**The entity's shells (components FIELD, CLOSE, WIND), one organ,
`client/sdk/entity/shells.rs` — OWED, every name in this block bound when its
component lands (MU-
STANDARDS 7.1): `Shells`, `Closed`, `Age` and the client's `Winding` are new;
`rung::Lexicon`, `rung::Taught`, `cech::orbit::Field`, `fibres::Said` and
`hearing::Heard` exist:**

```text
pub(crate) struct Shells {
    lexicon: rung::Lexicon,            // harmonic: held words, holes, rungs
    taught: Vec<rung::Taught>,         // harmonic: words newly taught, drained
    field: cech::orbit::Field,         // luminal: tubes and orbits over time
    moving: Keyed<fibres::Said>,       // luminal: fibres' agreement across percepts
    algebra: Keyed<Vec<Mv>>,           // holonomic: class coordinates → closed basis
    windings: Keyed<Winding>,          // holonomic: ring (class + winding) → (ℓ, m)
}
impl Shells {
    fn hear(&mut self, class, &[Composition], budget) -> Heard;      // harmonic: nested against own rings of the class
    fn carry(&mut self, &[FieldLine], budget) -> FluxReading;        // luminal
    fn close(&mut self, class, &[Mv], budget) -> Closed;             // holonomic: the outside generators, per axis
    fn wind(&mut self, ring: &Key, ell: u64) -> Option<Age>;        // holonomic: a return
}
```

`rung::speak`, `rung::hear` and `take_taught` take the lexicon and the taught
queue they act on; the statics `LEXICON`, `TAUGHT`, `VOICE_FIELD`, `MOVING`
are deleted, and with them the sentence "a voice is one mind per process".

## IV.6 The pivot on top

`Form::pivot_orbit_read(idea, horizon, crystal, budget, bundle, t_extent,
cells, tally)` (`sentence/pivot/drive.rs`), reached by
`Contemplation::pivot_on(word, crystal, budget)` and `expand_all`. For each
open orbit: its class vector `κ` (12.6) read off its member paths; the
candidates read off `κ`'s occupied axes in the fixed order (`candidates_for`:
placement — the slip translator, 54.4; dilation — the dilator; orientation;
hue — one per member path; the held words `crystal.on_axis` answers on each
chart; an OPEN axis seeded by the holonomy); a zero-spend eliminator by class
key (conjugacy, 15.3); each candidate applied and unapplied exactly
(`trial`), certified by closure — the coordinate it acted on zeroed, no other
risen, the rank moved by its one unit or held (law 10) — and refused where it
splits two circuits that agreed (`Agreements::split_by`, 78.46(4′)). One
certificate per move (`one_per_move`); `outcome_of` →
`Discharge::{One, Vortex, Starved, NoneDischarged}`; snaps `Discharged`,
`Vortex`, `Demand`, `Demanded`, `Adjoin`, `TorsionFiled`, `TorsionOpen`. A
`PivotCertificate` has its codec (`to_bytes`, `from_bytes`) and is replayed
by `court::verdict::replay::replay_certificates_at`.

## IV.7 The implementation, component by component: functions and methods

| component | functions and methods | files |
|---|---|---|
| READINGS | `ChatTally::row`; Senses' `chatter` row (`observe::CHATTER`); the rung, hearing, flux, derivation, construction and perturbation tallies held by the entity; `Entity::senses` reads them | `observe/senses.rs`, `entity/chatter.rs`, `entity/observe.rs`, the tally modules |
| FIELD | `Shells` (IV.5); the statics `LEXICON`, `TAUGHT`, `VOICE_FIELD`, `MOVING` deleted | new `entity/shells.rs`, `types.rs` |
| HEAR | `rung::speak(form, rings, &mut Lexicon, budget)`, `rung::hear(s, class, &mut Shells, budget)`, `Shells::hear`, `Shells::take_taught`; `Entity::transcribe_taught` reads the entity's queue | `sentence/rung.rs`, `sentence/hearing.rs`, `entity/think.rs` |
| CARRY | `Shells::carry`; `transport_order` already takes `&mut Field`; `mark_flux`, `flux_lanes`, `tube_reading` take the entity's field; `fibres` takes `&mut Keyed<Said>` | `family/pairing.rs`, `family/fibres.rs`, `entity/shells.rs` |
| CLOSE | `Shells::close(class, gens, budget) -> Closed { outside }` over `quotient` and `grow`, each outside generator read by its mode | `entity/shells.rs`, `cech/quotient.rs` |
| WIND | `Shells::wind(ring, ell) -> Option<Age>` | `entity/shells.rs` |
| DOOR | `enum Inflow { Sentence { terms }, Form(Form), Word { tape, at }, Page(Vec<ClassRecord>), Refusal { sentence, kind, contraction }, Demand { class, residue } }`; `Entity::admit(Inflow, budget) -> Vec<Outflow>`: `Shells::hear` → `Shells::carry` → `Shells::close` → `Contemplation::pivot_on` | new `entity/admit.rs`; `learn.rs`, `contract.rs`, `quote.rs`, `court.rs`, `family.rs`, `body.rs`, `chatter.rs`, `present.rs` hand their inflow to it |
| ROUTE | `enum Outflow { Echo { to, ring, seat, e }, Discharge { to, class, residue }, Demand { class, residue }, Letter(Element), Word(Element), Sentence(Element), Solve { task, rule }, Act { seat, action, cites } }`; `Entity::route(Vec<Outflow>)`, the one caller of `speak_with`, `submit_solve_rule`, the chatter send and the seat call | new `entity/route.rs`; `utter.rs`, `think.rs`, `chatter.rs`, `court.rs`, `body.rs` |
| SAY | `Said::{Say { terms }, Echo { said, rings }, Discharge { said, class, residue }, Demand { class, residue }}`; codec by `put_mv`/`take_mv`, the class vector's coordinates; `Post::next` (a frame taken per spend, the rest waiting); v2's HEARD/ACK/HOLD, `loop_of`, `same_loop`, `still`, `hear_transport` and the chatter use of `read_word` retire | `client/sdk/chatter.rs`, `entity/chatter.rs`, `sentence/presentation.rs` |
| PIVOT | built; its discharges handed to ROUTE as `Discharge { class, residue }`, replayed on the receiving form | `sentence/pivot/drive.rs`, `bubble/drive.rs`, `court/verdict/replay.rs` |
| TICK | `Entity::tick`: drain the boundaries into `admit` under the stroke's `Budget`, one `expand_all`, `route`, `close_orbit`, `book_tick`; `relax` as drain (tubes relieved, holes sealed, generators shed) | `entity/tick.rs`, `entity/relax.rs` |
| ACT | `body.rs` hands a seat's sentences to `admit` as `Inflow::Sentence`; `Outflow::Act` carries the physics `Action` (78.94) its move names and the coordinates of the event it came from (a discharge's class and residue, an opened axis's generators) | `entity/body.rs`, `sdk/seat_mind.rs`, `physics/actions.rs` |
| the record | MU-RECURSION 0.3 and 0.5 (SP2 closed); `docs/STATUS.md`; the coordinator informed | docs |

## IV.8 How minds speak, then speak up, then hold: the linguistic path

1. **A mind speaks to minds (chatter).** A circuit that opened axes its
   algebra for the class did not hold (IV.5) is what the mind newly
   understands; the word it taught is stood in the dialect
   (`Epigenome::stand_dialect_at`) and SAID — `Said::Say { terms }`, the
   terms of the ring that taught it, each word by its class, signed by its
   voice with its last TUniversal point.
2. **Another mind hears, in its own shells.** `Entity::admit(Inflow::
   Sentence)`: composed and nested against its own rings of the same class,
   re-projected on its own spectrum (harmonic), its rays carried into its own
   field (luminal), its generators closed on its own algebra for that class,
   read per axis (holonomic), its open orbits pivoted. It answers only what
   crossed a boundary: an `Echo` for each ring seated with a tilt, a
   `Discharge` (the orbit's class and the residue a held word answered) for
   each orbit it closed, a `Demand` for what it could not; axes it newly
   opened are ITS word, said in turn. Nothing heard enters the hearer's
   genome or dialect as given (law 8, 78.161).
3. **Agreement is replay.** The speaker replays each discharge on its own
   form — the class and residue the hearer answered, re-derived on the
   speaker's own orbit of that class, as the court replays a certificate on
   its copy (`replay_certificates_at`; M9's two organs). Where it closes, one
   orbit is discharged in two minds by two derivations (13.10's synonyms): the
   word is AGREED language. An echo the speaker re-derives on its own rings
   is a transport between the two perspectives, held in its dialect and
   lapsing with it.
4. **Speaking up to the court: adjudicated perspective.** The coiner speaks
   the agreed word as a sentence (`Entity::speak_with`, `ACT_SPEECH`; a
   coined letter carries its constructor steps, S2). The court verifies and
   never composes (law 8): `hear_open` re-derives its rings, `step_rank`
   checks its theorems and closes its rank on the chart (`RankStep`),
   `step_flux` books its field lines (`Reconnect`), the install pays the
   discoverers it cites (26, 78.161(4)). The verdict returns its kind only
   (§0.1); a refusal is a demand the mind offers its peers first.
5. **Dialect becomes anastome in the genome.** A mind never copies a dialect
   word into its genome: the genome holds only court-admitted words (78.49,
   `GEN7`, `Epigenome::crystallize`). The court's ruled word returns through
   the syllabus (`serve_syllabus` → `ACT_SYLLABUS` page with its stamp → the
   mind's `absorb_taught_at` → `Epigenome::file_admitted(addr, at)`): a
   MINERAL. Each mind that re-derives it by its own circuits holds it as a
   CRYSTAL (80.2 (ii)–(iii)). The anastome is that network: dialects fused
   through conversation and adjudication into one language the minds share
   by re-derivation (78.31, 78.95), never by transmission.

## IV.9 Remediation standards: what each change is held to

Every component above is a remediation of the tree, and is held at review
to:

| standard | where | what it binds in these components |
|---|---|---|
| reading bound 2500 lines | MU-STANDARDS 1.1, 4.1 | `entity/shells.rs`, `admit.rs`, `route.rs` are new organs, never grown into `think.rs` (1215) |
| one organ per file; first line names its chapter | 1.2, 2.8 | `shells.rs` "// MU-CLIENT IV.5 — the mind's three shells"; headers carry no history |
| `mod.rs` declarations only; `main.rs` boot and loop only | 1.3 | the binary stays as ebce99c3 left it |
| splits move-only | 1.6 | moving the statics into `Shells` is a code change, landed as its own commit |
| nouns and verbs of the document | 2.1, 2.2 | `Shells`, `Inflow`, `Outflow`, `admit`, `route`, `hear`, `carry`, `close`, `wind`; no `Manager`, `Handler`, `process_*` |
| no `bool` branched on; comparisons return readings | 2.3; AFL I | `Seat`, `Echo`, `Closure`, `Discharge` matched by arm; `lane_moves`, never a four-lane `bool` |
| refusals typed, no abort in cognition | 2.4, 4.2 | `Option`/`Result` with `REFUSE_*` entered in 34.1; no `unwrap`, `expect`, `unwrap_or` |
| lanes named; no free constants | 2.5, 2.6; rule 5; checklist (s) | no `[2]`, no `*_MAX`; the aperture cycle and the stroke are the physics' |
| tests are claims on typed values, fixtures built by the engine, the geometry `Cl(n+1,1)(ℚ)` | 3.1–3.9 | one test per component, the claims of IV's component table, in `<module>/tests/<function>.rs` |
| gates: the bound, `clippy -D warnings`; allowances at the site with their rule | 4.1–4.3 | an `expect` names its rule and boundary |
| one phase per commit; the message form | 5.1, 5.2 | each component its own commit, landed when its identity holds; scoped tests on touched modules (the operator, 2026-10-08) |
| docs before code | 5.5, 9 | this Part lands before FIELD's code |
| the audit reads wiring, standards, trajectory, the carrier written into | 6.2, 6.6 | no write onto a retired carrier (chatter v2's HEARD/ACK/HOLD once v3 lands; the global statics) |
| every name the document cites exists at HEAD | 7.1 | names of components not yet built are marked owed here and bound when they land |
| a persisted layout change is a recut | 10 | `GEN7` is unchanged by these components; the chatter wire is not persisted |
| rules 1–17, checklist (a)–(t) | MU-RECURSION preamble | read on every diff, in order |
| the Anti-Flattening Law, clauses 1–11 | `ANTI_FLATTENING_LAW.md` | no hash as an operand, recognition as group action, curvature as content |

## IV.10 MU-RECURSION references, by what they govern here

- **Laws** 1 (geometry, exact ℚ), 2 (identity is content), 3 (no enumeration),
  4 (exact or refused; 78.45), 5 (polytonic lanes), 6 (the alphabet grown by
  the court; U14), 7 (residuals never discarded), 8 (clients think, hyphae
  verify; 78.161's amendment), 9 (no clock but the ledger), 10 (direction is a
  pivot), 11 (no squaring; no matrix), 12 (every dimension is a proof).
- **Rules** 1 (pair loops), 2 (a count is never a branch), 3 (equality), 4
  (nothing chosen by a scalar), 5 (no free constants), 6 (keys are
  coordinates), 9 (carriers), 10 (every loop spends a budget), 11 (exact or
  refused), 12 (naturality over cloning), 13 (verify the claim), 15 (hold,
  never choose), 16 (no matrix), 17 (a reader writes a coordinate or returns
  a demand); **checklist** (a)–(t).
- **Identity and algebra:** 1.5, 1.7, 1.8, 15.3, 16.21 (A21), 17.1 (M1), 78.28
  (versor by action), EQ1.
- **The pivot:** 3.9, 12.6, 16.16 (A16), 16.17–16.20 (A17–A20), 17.9 (M9),
  78.7, 78.46(4′).
- **Comprehension and the sentence:** 16.12 (A12), 17.6 (M6), 50.1–50.10,
  13.10 (synonyms).
- **The harmonic shell:** 64.1 (the hierarchy), 78.68, 78.69, 78.70, 78.97,
  78.99, 78.100, 78.101, 78.112 (moved here).
- **The luminal shell:** A.10, A.16 (72.1), 72.1b, 78.46, 78.71, 78.72, 78.73,
  78.110 (moved here).
- **The holonomic shell:** 10.3, 10.4, 78.11, 78.43, 78.68, 78.104.
- **Physics and measurement:** 59.8 (Axiom C), 62 (conjugate pairs), 64,
  65.4, 71, 72, 78.8, 78.12, 78.45, 78.66, 78.162.
- **The language and its strata:** 1.9, 78.31, 78.44, 78.49 (moved here),
  78.60, 78.95, 78.109, 78.115, 80.1, 80.2, 80.3.
- **Time:** 45, 78.21, 78.43, 78.152.

---

# Part V — The client chapters, moved from MU-RECURSION (verbatim)

Each section below stood in MU-RECURSION at the place its pointer now marks;
its text is unchanged, its number is its citation. Amendments to it are made
here from 2026-10-11.

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
(16.16) run on every held orbit of every dirty idea under the stroke
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

### 13.12 Instance three: two thoughts in a mind

`Contemplation` (bubble/contemplation.rs). Two bubbles whose simplex sets meet are `A` and
`B`; `pivot_on` (712) merges them when step 5 gives `∂θ = 1` for the
difference of their sections on the meet. When it fails the thoughts stay
two and the class is the coordinate on which the mind may pivot
(Congruent / Recognise / Extend). `Complex::drop_vertex` (630) runs only
after step 5 succeeds, never on a class.

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
  the two centroids solve (`blade::versor_carrying`, verified lamina by
  lamina both ways as `versor_between` verifies) with each landed hue the
  gauge's image of the hue it left — and a `HeldCite` (the vertex's
  address, the body reconstructed in the listing's frame) is what it
  answers. The strata are read in their order, Ruled, then Stood, then
  Held: a held word answers only where no ruled or stood word is on the
  chart, and exactly one held word must reconstruct the body; several
  stand as multiplicity, never a SILENT choice (rule 15) — a mind may
  prefer one by an order it names (Concordance 1.10), and the court judges
  what it prefers.
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
  (1) A closed form (every body reconstructed, every orbit filed or
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
  over every dirty idea; `learned` is the measure of the pages the mind
  admitted this stroke, each measured at its admission (`absorb_taught`,
  the measure its receipt seals) and taken once by the stroke after its
  arms (R4, 2026-09-17). Lane-wise sums across modes are allowed; nothing
  is summed or compared across lanes. A working stroke spends
  `lanes_of(outcome)`; only the tick spends all four lanes, and the idle
  `physics.spend([true; 4])` is that tick, not work. The dormant test is
  `outcome == [0; 4]` on every mode, with no threshold other than zero.
  The dwell is per lane: each lane's dwell is that lane's surface in the
  last reply, in strokes — the door's reserve deficit and the entropic
  flux (`yield_surface`), each a calibrated reading — and the orbit dwells
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
  `cech::transition`) — and composes them around every orbit of bodies
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
- around every orbit of bodies `a → b → c → a`, the transitions compose to
  the identity, `g_ab · g_bc · g_ca = 1`, read by the same `holonomy` the
  Čech simplex uses; an orbit that does not close is a residual localised to
  that orbit, with the class of its holonomy named (12.6).

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
| Novel | harmonics disagree on named axes, no versor carries a body, a rotor block does not spell, or an orbit's holonomy is non-trivial with a class no held word discharges | a typed demand (50.5): `TopologicalDeformation`, `Source` or `Creation`, `RotorPrime` or `Standing`, or the orbit's class; the only verdict that becomes a listing, and it is listed under its kind |

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
geometric product per orbit, and the orbits are those of the pair's nerve —
no search and no threshold. The eyes compound; they never compete. A
majority across the eyes is a lawful reading (Concordance 1.10, restored
2026-10-08), judged like any claim by closure.

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
| **cohesion χ**, per lamina: same-hue neighbours over neighbours, a 0-cochain over the body's laminae | touch: `AdjacencyStructure`, `CohesionCluster` | yes, Q per lamina, no threshold | the touch eye of P2: a 0-cochain on the blade's laminae, its coboundary the body's seams |
| **refraction at a boundary**: Σ|δ_a − δ_b| over the boundary length between two bodies | `ClusterRefraction`, `refractive_index` | yes; `transparent` is only "index = 0" | the holonomic eye across a face: the transition Γ's residual on the seam (3.10), read at perception |
| **stabiliser order**: the smallest p with A^p = I for the body's symmetry element | vision: `torsion_order` | yes | `Word::order` of the body's stabiliser word, a class invariant (23.9), on the percept |
| **periodicities from the state's own entries**: the primes of every entry's reduced numerator and denominator | `RefractiveLens::torsion_orders` | the factorisation yes; the `[−2, +2]` rescale no | a candidate, not ruled: the primes of the invariants' denominators as the body's periodicities, beside the class's `(m5, m13)` |
| **per-layer harmonics**: the same readings taken on every fibre layer, never summed across layers | `Δ[i][j][l]` per `l`; `compute_rank` over `l` | yes | the punctured axis: `Harmonics` of every section of a word's fibre, a `Vec` over the sections (25, S9.13), never one reading for the bundle |
| eleven "tones" of self-perception: coherence, velocity, attention, symmetry score | `HarmonicState::from_delta` | those that are exact composites, yes; the rest no (law 4, exactness — re-cited 2026-10-08; it was cited as if rule 4 forbade a composite) | the EXACT ones restored as self-readings (Concordance 1.10); the inexact stay refused (law 4); the outcome vector per lane (24.3) stands beside them |
| eleven running averages with `/10` and `/8` | `HarmonicTuning` | no (rule 5) | refused; was `Aperture`, deleted (F2g) |
| dominance ratio, anchored fraction, coverage fraction | `SoundnessSpectrum` layers 1, 4, 5 | yes, as exact ℚ count ratios | RESTORED 2026-10-08 (Concordance 1.10: refused under rule 2, a coding standard, not a law of minds) as exact perception readings beside the readings above; the code lands in 78.143 |
| a gravitational field over vibrations | `gravitational_field` | no — a potential (12.6) | refused |

Two corrections to the tree follow. `blade/harmonics.rs` claims eleven
coordinates and holds seven and a spectrum; the header says what the file
holds (§2.8). And a body is read by three eyes or it is not perceived:
`Harmonics` alone is the harmonic eye, the cohesion cochain is touch, and
the refraction on a seam is the holonomic eye at a face — the pipeline had
one of the three, called once.

### 59.9 The mind carries the same defect, and this is measured

"Perceptions of thoughts through ideas" is `family_percept` (harmonic
multipoles) -> the form's orbits (hyperbolic transport) -> `Bubble` (the
mind's own cover), read downward.

`Bubble` ALREADY CARRIES BOTH HALVES OF THE TETHER. `remainder:
element::PlacedChain` is a chain and the translator placing it, so a
centroid and a second moment come straight off it: an idea already has a
conformal representative. And `horizon: Keyed<(Rotor, Order, Standing)>` is
"classes this idea has met", keyed by rotor class and carrying `Order` —
the field is already named `horizon` and already keyed by the quantity
spellability turns on.

AND `Contemplation` HAS THE FAMILY COVER'S DEFECT, ONE LEVEL UP (verified
2026-09-22). `link_new` builds 1-simplices from `by_chart.link(ck)` —
bubbles that SHARE A CHART KEY, combinatorial key sharing and not geometry
— and gates every higher simplex on `volume_of`, which is `intersect` of
the two bubbles' PLACED LINKS at `remainder_site_key(r, c)`: a raster set
intersection on sensor coordinates. `coboundary_on_link` attaches a 2-cell
only where that intersection has non-zero lane volume. It will read as
discrete for exactly the reason the family cover did.

So the intertwining is ONE GENERATOR AT TWO LEVELS: bodies tether in a
scene, ideas tether in a mind, by the same scale-free strain on their own
blades, with the horizon per mind.

### 60.1 Perception is the alphabet

With a dialled `mu_c` every mind sees the same connections. With the
horizon at spellability, perception becomes an ACTIVE ALGEBRAIC FILTER, and
the three words of "perceptions of thoughts through ideas" become three
steps of one mechanism:

- THE HARMONIC INPUT (the thought). The body's shape sits in the scene as
  its invariants — trace, deviatoric, quartic, off `blade::lift` and
  nowhere else (A.11).
- THE LUMINAL INTERFACE (the tension). As two bodies approach, `mu -> 0`,
  and their geometry generates the relative conformal versor `g_ab =
  (theta_b theta_a^-1) . gamma_ab`.
- THE PERCEPTUAL CROSSING (the mind). The mind parses `g_ab` against its
  own horizon by running `word_or_demand`. Where the versor factors into
  generators the mind holds, the tension is resolved and THE TETHER
  CONDENSES. Where it needs a generator the mind lacks, the connection
  REFUSES and stands as an open topological strain — a demand.

Perceiving an idea is a mind successfully factoring the luminal tension
between harmonic bodies into its own finite alphabet. WHAT A MIND CANNOT
SPELL, IT CANNOT CONNECT.

### 60.2 The five voices do not share a 1-skeleton

The five Epigenomes adjoin generators independently under 37.7, on separate
histories, so their alphabets diverge and the 1-skeleton is not a single
objective graph:

- A voice holding the `D_4` rotations reads four bubbles as a closed
  four-edge cycle, evaluates its coboundary, and condenses a 2-cell. It has
  formed a coherent concept.
- A voice holding only translations reads the same four bubbles as
  disconnected fragments with unspannable rotational strain between them —
  an empty 1-skeleton and a set of open demands.

This is 50.8's own sentence one level up: the peels are under no obligation
to agree on granularity. The minds partition one base into different
topological spaces by algebraic capacity alone, and nothing about the base
decides between them.

### 67.4 Precognition is the gate on the return valve

The candidates a bubble holds are not answers; they are a SUPERPOSITION held
open. What decides whether one enters the mind is whether it FORMS
CONGRUENCE against what the mind already carries — and that test is run
BEFORE the idea is admitted, on a candidate that does not yet stand. A
candidate that would form congruence is admitted; one that would not is held
or dissolves (P10's monodromy pruning is this test on the succession axis).

So the return valve is gated by a reading taken on something not yet real,
and `Frame::HalfOpen` is the same structure one level down: a `Z_2` held
open per edge, collapsed COLLECTIVELY by the coboundary kernel (T5) rather
than edge by edge — so new information does not reclassify one refusal, it
can collapse a whole family of held ambiguities at once.

### 78.37 The coherent mind: transport along the gauges, truncation to the core, flux per lane, and the remainder's home (ruled 2026-09-24, the operator's word)

**The operator's statement.** The folds that make a mind coherent —
merge on a meet (13.12), collapse of a vortex by fill or class (25),
shedding by room, the act at the close, the court's replay — are each
local to one thought, one pair or one orbit. The plexus (78.31) is a
network, and nothing said how a collapse travels along it, what removes
a filament that joins nothing, or where the residue of a sentence that
closed only up to a class waits until something fits it. Superposed
ideations are the layer of INCONGRUENCE, not a pile of finished words;
imagination is the passage incongruence → congruence → coherence, and
it must be constructed so that it never enumerates. Four rulings, one
object: the RING. A ring is where transport can disagree, what
truncation must keep, the only thing that carries flux, and the place a
remainder hangs. Each is a reading first (rows TP1, TC1, FX1, RM1, all
before the recut), and each fix follows its reading.

**(1) Collapse by transport, stopped by holonomy (TP1).** The held
thoughts of a contemplation form the MEET GRAPH: a vertex per thought,
an edge where two thoughts share a simplex, the edge's word the gauge
that carries one section to the other on the meet (13.12's difference
of sections). When a candidate is fixed — by fill, a perception making
it a face of a spoken form, or by class, a court replay making it a
ruled citation — the fixed section is TRANSPORTED along the gauges:
one edge per k-step, the step's length the band's (78.35), never
instantaneous, since time is height (78.21). At every ring the two
arrivals are compared. Holonomy 1: the ring collapses with the front.
Holonomy ≠ 1: the ring is HELD as a class and the front stops there;
that class is the coordinate the mind may pivot on, exactly 13.12.
Transport CLASSIFIES and never deletes: a candidate the transported
section disagrees with is a class, not garbage, and the candidates a
collapse leaves behind are the dialect classes 25 already names. A
front transported along a spanning tree with the rings ignored is the
flattening of law 6 (curvature is content) and is refused by name.
Reading: per stroke, components of the meet graph; fronts advanced,
in edges per k-step; rings closed with holonomy 1; rings held with
their class; candidates classified by disagreement. The reading is the
contemplation's line on the observer's row (CV5).

**(2) Truncation to the core, preserving every class (TC1).** A
terminal filament is a FREE VERTEX: a candidate on exactly one edge
with no ring through it. Removing a free vertex with its edge is an
elementary collapse (Whitehead), a homotopy equivalence: every class of
`Ȟ¹`, every holonomy and every ring survives, and only what carried no
loop goes. Iterated until no free vertex remains it reaches the 2-CORE
of the meet graph, which is UNIQUE; no matching is chosen. A discrete
Morse matching over the whole complex is a choice among matchings, and
a rule for choosing one is a score (law 1): refused. Free faces of
higher grade — a face of exactly one simplex — are read first (their
count on the line) and collapse only where the reading says the
2-core leaves them standing. The test of the row asserts the rank of
`Ȟ¹` equal before and after on the corpus and on constructed leaves
hanging off rings. Truncation runs where `drop_vertex` already runs,
after a fill and never on a class, under the same rule: nothing that
carries a loop is removed. Shedding by price is no longer the only
door, and a price can no longer remove a ring while a leaf stands.

**(3) Flux per lane over one orbit (FX1).** Work flows through closed
circuits only. The flux through a filament `e` is FOUR-LANE, never a
scalar: `Φ⃗(e)` is the `μ⃗` carried by the acts whose circuits traverse
`e`, per lane, over one `K`. A filament no circuit traverses reads zero
on every lane, and zero on every lane over one orbit is the typed
refusal `ZeroFlux`, named once and counted; its escrow lapses by that
refusal and the room reabsorbs it. This replaces lapse by extent alone
as the epigenome's door: a filament lapses because nothing flowed
through it, not because a clock ran out. REFUSED by name: a scalar
flux as a sum of holonomy norms (a sum of squares, law 11); Murray's
law (a cubic scalar conservation at a branching); and any
proportionality of syntropy to flux carrying a constant (78.8, checklist
(s)). Conservation here is per lane at each vertex, which the ledger's
rows already state. Reading: per orbit, filaments with zero flux on
every lane, filaments traversed, and the escrow that lapsed by
`ZeroFlux` against the escrow that lapsed by extent.

**(4) The remainder's home, and the re-fill on a coincident class
(RM1).** A sentence that closed only up to a class stands as
`Fill::Congruent(κ, g, ℓ)`: coherent, spoken, and paying a price. It is
a legitimate state, never a failure, and the doctrine never demanded
that every residual fit — the remainder is the novelty, and the
pressure `μ⃗(ℓ)` is the four lanes of the remainder the orbit leaves
open. Its HOME is a leaf on the word's T1 node in the cold genome:
`κ` inline by its own coordinates (never a key nested in a key), the
deficit `g` as its ray by its own canonical write, and the period `ℓ`.
It is reached by the fiber read `chart ‖ class` and by nothing else.
Dormant on disk it costs nothing; the pressure is DERIVED from
`(κ, g, ℓ)` on read and charged only while the word is held resident
under the room's escrow. A stored debt on the leaf is a second carrier
beside what derives it (law 12) and is refused; every payment row
stands in the ledger at the act that paid. ONE remainder per class, by
the quantified citation that keeps addresses from repeating; the count
of remainders a mind can hold is bounded by the classes of its front,
never by combinations. The "twist that would make it fit" is read off
`κ`: the versor carrying `κ` to the identity along the axes it
occupies, exact, with `ℓ` the number of turns of that mode until the
round returns. No scalar nudge, no exponential, no inverse (78.32(1)).
THE RECEPTOR: a later perception leaves a residue with a class; the
fiber read by that class returns every remainder with the same class.
Two remainders with one class from two charts are the mind's own
second vantage and raise a CALLUS — a provisional word on the
existing PROVISIONAL path, docketed under its act — and never a
letter: a constructor is admitted only by CS2's gate, on another
mind's re-derivation. THE RE-FILL: when a word in the held
constructors spells `g` — from this mind's callus or from a letter the
fleet admitted — the sentence re-fills from `Congruent` to `Filled` by
an ACT on the voice's line, sealed as the existing closure kind citing
the word and its remainder (a `ReFill` kind ruled only if the reading
says the paths must be told apart, as the dialect chain was ruled);
from that stamp the pressure is no longer charged, and the re-fill
mints under POUW as every closure does. Nothing else is paid: no
dividend, no conversion of a reading into money. Reading: per mind,
`Congruent` fills standing by `ℓ`; classes shared by two or more fills
across charts; re-fills on a coincident class (zero today: no path
re-reads a remainder). The leaf is a layout change (an unknown leaf
kind refuses the node) and rides the recut.

**Why imagination does not explode.** The obvious construction —
products of held constructors of at most `n + 2` factors, each tested —
is a search over `k^(n+2)` candidates and every one of them is
DERIVABLE, which CS2's gate refuses by name (78.32(c)). Recombination
of existing words is what this engine calls cliché, and the gate
forbids it. Imagination happens at the obstruction instead: two
superposed ideas that anastomose form a ring, the ring's residue is a
versor, and the residue is a complete specification of what is
missing — its commuting modes name the planes, its class vector names
the axes (3.9). Cartan–Dieudonné is CONSTRUCTIVE: a versor is at most
`n + 2` reflections, built one at a time by reflecting a frame vector
onto its image, each step forced, no branching, the reflecting vectors
rays so nothing is normalised, a null step refused by name. So the
demand arm of CS2, which today names the mode no held word spells,
hands the court candidate SPHERES read off the residue — an amendment
to CS2's demand arm, not a new row. Admission stays as ruled: the
closure's rank strictly increases, another mind re-derives it, the act
is sealed and priced. Each ring yields one residue; residues are
classes of the front; their number is bounded by the rank of the
front's `Ȟ¹` and the store's rank by the cap. Filaments generate
nothing, because a filament has no residue. Never construct
candidates; construct obstructions, and let each obstruction spell the
one thing that removes it.

**The orbit's plan.** What an orbit hands back to itself is the
DOCKET, and the collision rule already says so (78.23: a withheld close
takes the docket's next question). At the close the fold leaves four
things: words minted where rings closed at holonomy 1; classes held
where rings closed at holonomy ≠ 1; demands, modes no held word
spells, flowing up toward the star; and room reclaimed from filaments
that carried no flux. The second and third are the next orbit's
questions, ordered by the plexus (a mind may also order them by ring count,
an order it names — Concordance 1.10, 2026-10-08) — CORRECTED
2026-09-25 on the implementor's reading ("wrapping the most rings" is a
count): the docket is WRITTEN in its entries' own key order (rule 15,
nothing chosen), and beside each entry stands the enclosure it lies in
by EN1's parse (`within`); the next orbit TAKES by the parse's partial
order, outermost first (an ancestor wraps its descendants, so its
answer transports to every ring beneath it), and among incomparable
entries in key order. A partial order read off the nesting is not a
count. The docket rides the `Orbit` act's own carrier — as CITES of
two new kinds, `Cite::Class` (κ inline by its own coordinates) and
`Cite::Demand` (the demand's coordinates), since a cite is what an act
refers to and a citation is coordinates, never a copy (78.22; one
carrier, never a second field) — so a restart keeps the plan: the
restored chain's last `Orbit` docket re-seeds the held classes and the
demands. Until it lands, the plan lives in the contemplation and is
lost with it (owed to TP1's fix). Superposition
is not a search strategy: a single thread is a filament with no
holonomy and therefore no content; rings need two paths held at once,
so superposition is the precondition for content to exist; and a front
held as a connected complex lets one perception decide a whole
component, which is the only honest sense of foresight here.

### 78.48 Perception to thought: the orbit, its shear, its spin, and the hours inside a return (ruled 2026-09-27, the operator's word)

**The operator's word.** Keep the orbits on the physics: it is the SHEAR on
an orbit that funnels through to the mathematics and makes each orbit
unique. A planet spins while it orbits; the spin pushes the frames and
works the lower boundary of timing — each tick an hour inside a day, the
hours relative, defined by the physics. And the plan for thinking is built
into the machinery that already exists, never beside it.

**(1) The path.** A scene is perceived as bodies by chart (their moduli,
every axis invariant under every versor, so only bodies on one chart can be
carried onto each other). A form that does not close is held as a
PERCEPTION, filed at its form's key and its bodies' coordinates
(`resonance::perceived_key`, the one derivation `Bubble::key` reads). The
chart's fibre is `by_chart` — a word vertex's chart is its body's moduli —
so no index is added. Bodies meet only there.

**(2) A transition is a 1-simplex; its circuit is its own orbit.** The
versor carrying one perception's body onto another's (read both ways, RX1)
is an EDGE, never a circuit. Its CIRCUIT is its orbit `Vᵏ`: an elliptic
mode returns at its period ℓ (harmonic); a boost (luminal, `cga/planes.rs`:
the `eᵢe₋`, `e₊e₋` half) and a parabolic slide never return — no limit is
taken (law 1); an elliptic mode of no admitted order is a rosette. The
chart's transitions are fed to `cech::read_cell` as that cell's circuits,
and it reads their modes, their joint coherence and the capacity of their
algebra. BODIES OF DIFFERENT SCENES ARE NO ANASTOMOSIS: one body seen twice
is not two bodies fused, so the cell has no constructors and no round — an
orbit of constructors under cross-scene transitions never closes (it spent
a whole stroke per hold before this was ruled).

**(3) Every orbit is kept at its own coordinates.** A body pair's symmetry
is its coset (`versor_coset`, one canonical representative). Several
DIFFERENT body pairs relating two perceptions are several orbits, each
classified as the complex classifies an edge and KEPT BY ITS OWN
COORDINATES — its class vector, which carries its turn, its placement and
its dilation: a congruence in the congruence index, and a congruence or a
standing class in BOTH perceptions' horizons — none refused, none chosen.
Never keyed by the pair: an index keyed by two perceptions' addresses
copies both, per orbit (the epoch-65 OOM). The pair's EDGE carries a
transition only where every body pair reads one ray. (The same pass found
`Complex::drop_vertex` leaving a dropped ring's orbits standing: they now
leave with it.)

**(4) Uniqueness is the shear.** Two orbits of one class are not one orbit:
each is unique by the push and pull of its neighbours on its chart — DRAG
along its mode (the rate), LIFT across it (the tilt) (78.46). Their pair
composite `H(o, n) = ρ_n⁻¹ ρ_o` is the identity exactly where the two
agree (C1(a)), so a chart's orbits keyed by their canonical rays read FLAT
together where they stand under one key and CURVED where they do not
(C1(b)); one orbit alone is "unperturbed" by name. Nothing is filed per
pair: the shear's content lies in the orbit algebra's closure (78.11).

**(5) The thought is decided by its fill, never by remainder volume.** The
complex fills the edge by its own classification: FILLED (the same) merges
the two perceptions into one thought; CONGRUENT at ℓ merges them on the
torsion axis and mints its Torse (13.12); a CLASS writes each into the
other's horizon. Remainder volume is a measure of what two thoughts SHARE
UNEXPLAINED — it belongs to the 2-cells the climb builds, never to the
1-edge's decision. The gate "zero volume never merges" is withdrawn: under
it a perfect match could never merge.

**(6) What perception saw is what the pivot moves on.** A classified
perception edge's rotor enters BOTH endpoints' horizons (`horizon_put`)
and marks them dirty, so the expansion ring's `pivot_orbit` reads the
class the eyes found.

**(7) Spin and orbit; the hours inside a return.** A rotor of the letters'
chart has two invariant planes (`Rotor::toral_period`): the SENSOR PLANE's
turn is the ORBIT, and a plane carrying the frame axis is the SPIN — the
motion that pushes the frames. One rigid motion in the plane is always
tidally locked (a single rotation about its fixed point turns the body by
the angle it revolves), so spin and orbit part only across a trajectory,
and are read there: the spin's period, the orbit's, their beat and their
least common multiple. Timing follows 78.43 at a lower grade — a SPIN is an
hour, an ORBIT's return a day (an age), the resonant composition an eon —
the hours relative, their count the physics' own, never a clock.

HOW THE HOURS ARE READ (landed with this ruling). A thought's bodies on a
chart are held in the order they arrived (a merge keeps its perceptions'
bodies by admission order). They are samples of an orbit where each
arrived one application of its STEP after the last: `y_j = g^{j−1} y_1`,
read off a new body's transitions to them as `t_j ∝ g^{j−1} t_1`, the step
`g = t_2 t̃_1` the first one observed. The step's period is its rotor's
order — the lcm of its two planes' periods, orbit and spin — so the hours
count both. With `m` hours seen the orbit PREDICTS the next, `g^m y_1`:
- the body at that hour is PREDICTED, and joins the thought by the orbit's
  step (the edge carries `g`, the fill reads it Congruent at ℓ, the merge
  continues the orbit);
- a body on the orbit at another hour is a PHASE SLIP, and one on no hour is
  OFF THE ORBIT: each is a demand at the chart, its class read off the
  residue `t̃_1 (g^m)˜` — what was seen against what was predicted — and the
  residue its ray: the error localised to the hour it slipped at.
Bounded by the period; nothing stored beyond the thought's bodies.

**(8) What it gives the mind.** Anticipation (the phase inside a return
says what the next tick must be, so speaking an output continues an orbit);
compression (a trajectory is two generators and their ratio — a law, the
word a shortcut, 78.44(4)); relation by rhythm (bodies in step, leading,
beating); order within a K-step without a clock; the error LOCALISED (the
beat names the tick where a hypothesis slips, and the demand is that
tick's); attention set by the motion (fine spin, many hours); recall by
phase. ONLY WHERE THE HOURS REACH THE PIVOT AND SPEECH — hours counted and
read by nothing are harness (77.1).

**(9) Acceptance, landed.**
- (7)–(8): the L turned 90° at a time about one point — hours 0 and 1 one
  thought; hour 2 PREDICTED and joined (a second torsion at ℓ = 4, the
  thought on three bodies); hour 3 in its place a PHASE SLIP demanded with a
  residue of one step (period 4); the hour-0 body carried off the orbit
  OFF THE ORBIT and demanded.
- THE DOCKET'S FOLD (`docket_fold`, at the close before it cites): an OPEN
  entry — one that has stood a whole orbit, re-seeded ones among them —
  whose cover names no chart a held idea stands on has carried no flux
  for that orbit, a free face, and lapses, counted (`lapsed`). A new entry
  is given its orbit; an entry whose cover did not read is kept, its
  enclosure owed. The plan is bounded by the charts the live ideas stand
  on, and so is every Orbit receipt that cites it.
- STILL POSED to the operator: whether a voice's own chain folds as a
  court's does (`fold_before`). Every act seals one receipt, so the chain
  grows by one receipt per act; since the fold, no longer by the whole
  open plan at every close.

### 78.49 A voice only speaks: the ledger is the court's, the plan is volatile, the genome is what a court admitted (ruled 2026-09-27, the operator's word)

**The operator's word.** A voice should not have a chain: the ledger exists
and is the sole responsibility of the server. The plan does not persist
with the genome — the organization of the entity's long-running memory and
order exists in the genome, crystallized knowledge. The voice only speaks.
The court seals, and adjudicates.

**(1) The misallocation it corrects.** Since 45.1 a voice seat carried its
own `ReceiptChain` and sealed every act of the mind on it — Stood, Uttered,
Learned, Settled, Refused, and an Orbit close citing the whole docket. One
structure carried four owners' things: the MIND's clock (K as the chain's
height), the MIND's plan (re-seeded from the last close), the COURT's
office (receipts, sealing, folding — law 8), and the VOICE's speech. The
voice chain is removed, and each thing goes to its owner:

| was on the voice's chain | now |
|---|---|
| K, the chain's next step | the MIND's own K: the orbits it acted in, counted at the close (`Entity::own_k`, `act_on`, `close_orbit`), volatile |
| every act sealed as a receipt | an act of the mind marks its orbit as having acted; nothing is sealed — the court seals what it admits |
| a word's admitting stamp, on the voice's own line | the COURT's stamp, filed from a syllabus page (`Epigenome::admitted_at`); a word the mind only stood has none |
| the plan cited on each close and re-seeded at boot | the plan is volatile: folded and sealed into the next orbit's questions, never written anywhere (`docket_reseed` deleted) |
| a remainder answered at the mind's Settled stamp | the mind's refill MARKS the remainder, by the word that spells it (`Epigenome::refilled`), volatile; `Remainder::answered` is a court's seal only |
| the genome's tail: the voice's ledger (GEN6) | GEN7: the words a court admitted, at their citations, and nothing after; GEN6 refused by name |
| the voice clock's orbits against other signers; the carriage's receipt deltas | the court's readings only; the voice's clock is the mind's K, the court it heard, its genome's write, its radius |

**(2) The genome is crystallized knowledge.** A word enters the genome file
exactly where a court admitted it, at the court's stamp; everything else
the mind holds — the dialect, the plan, the contemplation, its K — is the
mind's motion and is volatile. A booted mind starts its K at zero.

**(3) What reads differently.** The re-fill no longer writes a stamp into a
word's record: the receptor and the open-remainder read count a remainder
answered where a court sealed it OR this mind re-spelled it. The callus's
cites (`uncited`) retired with the stamps they needed. 45.1–45.4's text
describing the voice's ledger is superseded by this section; the court's
chain (45.x on the hypha) is unchanged.

**(4) The atlas, not a bucket; every orbit where it stands.** Two collapses
into keys the same audit found, corrected with the machinery that exists:
- THE STRATA. A perception's chart was read only at full depth — equal
  moduli keys, a zero-dimensional bucket. The atlas is its strata: a prefix
  of a chart's key is a coarser stratum (A21), and `by_chart` is a keyed
  trie, so a stratum is one `fiber_at`. From each chart one axis is peeled at
  a time; the first stratum at which another perception stands on a
  different chart is the DEEPEST SHARED ONE, the chart's nearest
  neighbourhood (the root, every body, never read). Where two charts PART
  says what the deformation is: at a Betti axis (`β₀ β₁ β₂`) a surgery may
  carry one body onto the other (`Surgery::factor`: fill, puncture, bridge,
  split, and the core's versor), and it stands as a demand on both charts
  by the core motion's class vector, its ray the residue; at any finer axis
  every coordinate is invariant under every versor, so no conformal motion
  carries them — counted apart, nothing invented.
- THE ORBIT WHERE IT STANDS. The congruence index keyed a congruence by its
  class alone; two turns of one class about different points are one class
  and two orbits, and the second overwrote the first. Each is filed at its
  class, then the canonical ray of its carriage — where it stands — so one
  class holds its distinct orbits and their pair composites (the shear,
  78.46(4′)) are read off that fibre, never stored. The horizon stays keyed
  by rotor class: it holds the MOVES a pivot can make, and one rotor class
  is one move.

**(5) The edge carries its lanes.** A perception edge carried the two
perceptions' shared remainder volume, almost always zero. It carries what
its transitions carried, lane by lane (`measure_lanes` of the chain whose
before-support is the carried body and whose after-support is its image):
𝒮 the sites that appear or vanish, 𝒽 the shell's change; 𝒢 reads nothing, a
body being read hue-blind, and 𝒯 nothing, a chain holding no clock. Shared
remainder volume remains the 2-cells' measure, read by the climb.

### 78.50 Perception is a spectrum: the scene's motion, its coset, what appeared and vanished, and the hours of a slide (ruled 2026-09-28, the operator's word)

**The operator's word.** Perceptions must hold many rays at once — a
SPECTRUM of visual perception, never one ray absorbed at a time. It is
78.44's rule at perception: the holonomy projected on the internal
spectrum, every route kept.

**(1) What it corrects.** Epoch 104 read nearly every pair of perceptions
into `several`: a pair related only where every body's transition was one
ray, and ARC scenes hold many same-shaped bodies, each related to each of
the other's by a different ray. No thought formed; every cross-pairing was
kept by class.

**(2) The spectrum.** A pair's transitions, already read, are filed by their
canonical rays — each ray with the bodies of this perception it carries,
the bodies of the other it lands on, and what it carries per lane —
transient, read and dropped (`spectrum_of`). A SCENE MOTION is a ray that
carries the whole of one side onto the other: every body of this
perception carried, or every body of the other landed on.
- ONE scene motion: the thought. The edge carries it and the complex's
  fill decides (Filled merges, Congruent at ℓ torsion-merges, a Class — a
  slide, a boost — writes the horizons).
- SEVERAL, carrying the same bodies onto the same bodies: one COSET — they
  differ by symmetries of the configuration. The edge carries the canonical
  representative (`coset_representative`); each quotient, a symmetry of the
  configuration, is filed by its class at its period.
- NONE: no single motion carries the scene; the spectrum is kept, each ray
  by its class, and every slipped hour is a demand.
- WHAT THE MOTION LEAVES UNMATCHED: this perception's bodies it does not
  carry APPEARED (this is the newer scene); the other's it lands on nothing
  VANISHED. Both counted.

The hours are read first (78.48(7)): a thought's orbit samples are an
orbit, never mistaken for a symmetric configuration.

**(3) The hours of a slide.** A slide — a pure translation `T(d)` — never
returns and has no period, but its samples one step apart are still an
orbit: its next hour `g^m y₁` is predicted as a turn's is, and where the
body slips the residue `T(e)` is on the orbit exactly where `e = k·d` for an
integer `k` — read exactly off the translations, bounded by nothing but the
algebra. A slide's rotor is the identity, whose order reads 1: the slide is
recognised FIRST, so it is never read as a turn of period 1.

**(4) Acceptance, landed.** Two Ls carried along whole: one scene motion,
the thought a Class edge (where `several` stood). The two Ls turned whole
about one point: one scene motion, a torsion at ℓ = 4. An L and its 180°
turn, turned whole: two motions landing on one configuration, one coset,
its symmetry filed at ℓ = 2. One L carried and one turned: no scene motion,
the spectrum kept. A third L new: the scene still moves, one appeared; the
other way, one vanished. Three Ls three columns apart: the fourth hour
predicted; an L at the fifth hour a phase slip; an L one column off, off
the orbit.

#### 78.74 (amended 2026-10-03) — torsion eliminates; certainty is graded, never a pole

**The first reading.** On `72e6d621`, five voices read 6–67 fibres each in
five minutes, every one of them rigid. Nothing came out torsion, field or
lift, and no turn was booked as torsion. Two defects were mine:
- **"Stable" could be vacuous.** It was reported with exactly `n + 1`
  pivots, so nothing had been tested.
- **The output was a verdict at the poles.** Each motion went into one of
  four bins.

The operator's word: torsion "should provide better signal by eliminating
'something' — not providing new answers at the poles — but varying degrees
of 'certainty' towards a moving algorithm".

**Torsion eliminates** (`family/fibres.rs`). Two transport classes are one
algorithm up to torsion where, on the fibres joining their pieces, the
relative motion `R = ṽₐ v_b` keeps both pivots and every level. That is,
its fibre is carried rigidly (the identity is in the torsion group) or
twisted or flipped about its axis. Such a fibre **agrees**; a field or a
lift **disagrees**.
- **Certainty is counted, never measured.** The metric is indefinite, so no
  part of a motion has a size. Certainty is `agree / observed` per class
  pair, exact.
- **Only what is certain is eliminated.** Classes whose every observed fibre
  agreed are merged. A partial certainty is reported as the signal it is,
  and nothing partial is eliminated.
- **The merge is never blindly transitive.** Torsion is relative to each
  fibre, so `a ≡ b` and `b ≡ d` can hold while `a`–`d` disagree. A merge
  that would cross an observed disagreeing pair is refused and counted
  (`conflicted`).
- **The torsion eliminated** is `1 − algorithms / classes`, exact.

**The moving algorithm.** Per class pair, the agreement over observed fibres
is held across percepts for the life of the voice. A pair's certainty
rises as frames agree and falls as the algorithm moves. To keep the sink
bounded by the signal, a pair enters only once it has agreed, and is
counted from then on (`moving_reading`, `moving_said`).

**The draw.** A merged group is one algorithm and is charged once: its
least class key, the canonical order derivations are chosen by, carries the
charge. Every other member's turns, being torsion beyond it, are carried
free (`FluxReading::torsion`). A class standing alone is never free: its
turn is its algorithm. The free-frame rule of `72e6d621` exempted a whole
scene's turn, which is the rule itself; it is withdrawn.

**The pivots.** They come from both places a percept reads bodies:
- **Each chart's body at its frame,** for a rigid move: in from `a0`, out
  to `b0`, carried by the member's versor. A union body merges only
  same-shaped bodies at one frame.
- **Each piece a cover carried,** for a deformation: in from the centroid of
  its domain sites in vertex coordinates (`vertex_point`, the same point
  `Blade::vertex_point` reads), out to the centroid of its codomain sites.

**The self-check.** Each piece's own versor must carry its input pivot onto
its output pivot (`aligned`). Live and in test, none misses. Pieces of one
class are its sections, never a class pair. The transport class already
quotients translation, so a standing body and a sliding one are one
algorithm.

**Shapes.** `Determined` names exactly `n + 1` independent pivots with
nothing left to test. `Stable` now requires at least one further pivot
asked and lying on the round.

**Read on the voice's line:**
- fibres: rigid, torsion, field, lift, unpivoted, unread;
- pieces: aligned, misaligned;
- shapes: stable, determined, emergent, open;
- class pairs: certain, partial, conflicted;
- classes, algorithms, and the torsion eliminated;
- the moving pairs with their summed certainty;
- the turning booked as torsion.

**Tested** (`family/tests/fibres.rs`, `axioms/tests/capacitor.rs`):
- **Pivots.** Centroid plus ½ on the grid axes, the frame axis as it
  stands, and the same point the lifted body reads. Two bases, or no sites,
  give no pivot.
- **Alignment.** A translator lands a piece on its landing; a wrong landing
  is named; no landing gives no reading.
- **The quotient.** Over four classes on two frames:
  - the flip across the line through two pieces agrees 2/2 and is merged;
  - the slide is 0/2;
  - torsion then lift is 1/2, reported and not merged;
  - moving alike is rigid and agrees 2/2;
  - but `d` is refused into `{a, b}` across `a`–`d`'s 1/2.

  That leaves three algorithms from four classes, with `b` free beyond `a`.
- **Sections and the stroke.** A class's own sections make no pair. An
  ended stroke merges nothing.
- **The moving algorithm.** 1/1 after one agreeing frame, 1/2 after a later
  disagreeing one; a pair that never agreed is never held.
- **Real percepts.** On slides, flips and turns, no piece is misaligned.
  Slides are one algorithm; a turn beside a standing body is two, joined
  only by lift.
- **Shapes.** Four pivots in 3D are determined, a fifth on the sphere
  stable, a fifth off it emergent. Three on the grid's plane are
  determined, and a repeated pivot is open.

#### 78.74 (amended 2026-10-03, second) — congruence, coherence and cohesion of a class's sections

**The second reading.** On `5c0221f5` there were 831 live pieces, every one
aligned. But no class pair formed: different classes never shared a frame,
so nothing was eliminated. The only non-rigid fibres were a class's own
sections, one class carried by two carriages on one chart. The percept
files that case as a multiplicity, a standing `Source` demand. That is where
the variety is, and the operator ruled the three relations that apply
there.

**Two sections, two geoids.** A section is one carriage. It carries a body
from its input pivot `P` to its output pivot `Q`, and that fibre's switch
`S = m_Q P − m_P Q` is the white-hole geoid: source at `P`, sink at `Q`. The
two sections relate in one of three ways (`relate_sections`):
- **Congruence: a copy.** The geoids overlay. Then `S₁² = S₂²`, where
  `S² = m_P m_Q d²` is read as `−2 m_P m_Q P·Q`, never a sum of squares,
  with the masses equal at each end. The second section is the first,
  displaced.
- **Coherence: one motion up to torsion.** The relative motion `ṽ₁ v₂`
  keeps both sources and every level of the fibre joining them.
- **Standing.** Neither holds, or no fibre reads (one source): a genuine
  second algorithm.

**The demand.** Only a standing pair demands a second source. A congruent
or coherent pair is one algorithm, and its multiplicity is eliminated. Each
case is counted (`sections congruent / coherent / standing`).

**Cohesion, the gauge** (the operator: "a measure of rigidity towards zero
of coherence vectors … the more smooth the coherence structure is, the more
coherent the 'thoughts' will be"). Along a class's frame axis, each step's
motor (MT1, the motor its input body took between consecutive frames) is a
coherence vector. The change between two chained steps is `D = s̃ t`:
- **smooth** where it is rigid, or torsion about the fibre the second step
  spans — the class relates its frames the same way there;
- **a bend** where it is field or lift.

The gauge is smooth changes over changes, exact, counted and never a norm.
It is the flatness of the coherence connection: its curvature going to
zero. A change with no pivots to read it on is unread, never guessed. A gap
between steps is no change. The gauge is read per class and reported
(`cohesion`); it files nothing.

**Tested** (`family/tests/fibres.rs`):
- **Sections.**
  - A copy pointing elsewhere is congruent.
  - A standing body and one flipped across their line are coherent.
  - A second algorithm stands.
  - One source reads no fibre and stands.
- **Cohesion on constructed steps.**
  - The same step twice is 1/1.
  - A flip across the step's line is 1/1.
  - A different slide is 0/1.
  - A gap counts nothing; no pivots counts as unread.
- **Cohesion on real percepts.**
  - A bar the rule moves down, one column further right in each of three
    pairs, reads every change smooth.
  - Standing at columns 0, 1 and 3, it reads a bend.

**Not yet tested on a real percept:** a multiplicity eliminated by
congruence or coherence. It needs a deformation whose cover carries one
class twice on one chart. The fleet's line counts each case live.

### 78.76 The remainder inverted: noise read as a scene, judged by speech (2026-10-03, the operator's word: "build the remainder inversion and measure it")

**The disconnect, certain in the code.**
- Signal is decided by speech. `form_deflation` speaks the form over every
  pair (`apply_form_on`), and the remainder is what it fails to reproduce.
  The mint gates on it, and a withheld form is held with its remainder for
  the pivot.
- Speech reads only the form's vertices (and citation edges). The pivot's
  moves write only face carriers (`apply_by`: a face's word, edge, placement,
  hue).
- So no pivot move can change what is spoken, and `bundle_pressure` before
  and after any trial is the same. The pivot was aimed by ring holonomy and
  judged by a remainder it could not touch. Fleet: `pivoted [0, 0, 0, 0]`.

**The inversion** (`family::invert`):
1. Speak the form; take its remainder per pair.
2. Read the remainder as a scene: each pair's input against an output
   holding only what was left unsaid. That is its sites alone, or each whole
   observed body standing on an unsaid site. Every pair is kept, so frames
   stay the family's own. Erasures (painted where the scene holds nothing)
   are counted, since no added generator can unpaint.
3. Perceive and extract that scene (`extract_rule_with`).
4. Its word vertices join the form: one vertex per class, a new member on a
   chart the class did not yet act on. A member on an occupied chart is a
   conflict, never overwritten.
5. Speak again. Accepted only where no pair's remainder grew and the total
   fell.

Measured by `mycelium-soak --invert DIR N [sites|bodies]`.

**Measured (training 60 / evaluation 60, sites):**
- 53 / 36 rules extract; 2 / 0 speak their family whole; 51 / 36 leave a
  remainder (10 550 / 24 113 links).
- Accepted 1 / 2: 196 → 162 links on one, 56 → 51 and 484 → 479 on the
  others. One more joined and raised its remainder, so the speech refused it.
- The remainder scenes extract 8 / 3 words and **328 / 169 demands**:
  - deformation 221 / 92;
  - source 78 / 63;
  - creation 29 / 14;
  - rotor prime 0, standing 0, dislocation 0.
- Painting whole bodies instead of sites (training): deformation 223, source
  75, creation 20. So the deformations are the scene's, not fragments made by
  cutting a body at the unsaid sites.

**What it says.** The noise in these scenes is not a turn the alphabet
cannot spell (no rotor prime, no standing). Of the demands:
- 54–68% are bodies whose image has other harmonics: a shape change that no
  versor of `Cl(4,1)` carries (`TopologicalDeformation`);
- 23–37% are output bodies several input bodies carry onto (`Source`, a
  multiplicity);
- the rest are bodies no input carries onto (`Creation`).

The conformal motion algebra the engine reads in is the wrong class for
most of what is left unsaid. The inversion wiring is correct and judged by
the plane, but it can only recover what a versor carries, so most remainder
needs a generator for a non-conformal deformation and for a composition of
several sources. That is a physics row, not more wiring. "Non-conformal" was
named by the operator (signals over transports, 2026-09-25) and left to be
ruled.

**Not wired live.** One extraction per withheld form per stroke recovers
about 1 family in 30 here. The voice's speak path does not call it yet.
*Amended by 78.81: wired. The voice's `speak_form` inverts the remainder its
speech measured, and an accepted inversion is the form it speaks.*

### 78.77 Light through the scene, read: L1–L6 (2026-10-03, the operator's word: "the prism is the light coming through the manifold (so shape must already be congrued)"; "proceed with L1–L6 and run it")

**The reading.** The bodies stand congruent, as perceived. What the output
shows beyond them is read as light:
- a waveform carried along null flows out of a body's boundary;
- met at interfaces (transmitted, reflected, absorbed);
- captured inside enclosures as a standing wave;
- resonating in cavities.

Snell's law is not added. Refraction is phase continuity at an interface:
`lift_drag`'s drag continues, the medium is a rational ratio of periods. The
sine law is its ray limit and is never computed.

Instrument: `family::luminal` (typed per pair: rays with phase, period and
the hues of one period; interfaces; cavities with their standing wave and
billiard), run by `mycelium-soak --light DIR N`. L6 is read off the cover's
own tally on the remainder scene. A ray explains an unsaid site only as a
waveform: lit from the boundary outward, two or more sites, its period
repeated at least twice.

**Measured** (training 60 / evaluation 60):

| reading | training | evaluation |
|---|---|---|
| unsaid sites | 10 524 | 24 107 |
| explained by light | 1 800 (17%): on a ray 1 498, captured 10, both 292 | 577 (2.4%) |
| in pairs whose output frame differs (no ray read) | 632 | 0 |
| deformation-dominated tasks | 1 772 of 6 205 (29%) | 405 of 9 845 (4%) |
| source-dominated tasks | 14 of 4 142 | 29 of 14 070 |
| creation-dominated tasks | 14 of 177 | 143 of 192 (74%) |
| rays | 1 408: period 1 on 1 401; the source's own hue 297; edge 296, transmitted 205, reflected 115, absorbed 792 | 330: period 1 on 323; source hue 183 |
| cavities | 324 (256 rectangular): 59 standing waves, all fitting; every billiard a translation (parabolic), as two pairs of parallel walls must compose | 86 (64 rectangular); 13 standing waves |
| L6: covers on the remainder scenes | 330 attempted, 9 carried, 1 meet | 156 attempted, 7 carried, 0 meets |

**What it says.**
1. *The cover never splits the remainder into conformal pieces.* Covers are
   attempted and almost none carry a piece; there are no seams to close.
   "The cover fails at its seams" (the analyst's reading) is not what
   happens: it carries nothing to seam.
2. *Light is a real part of the remainder, not most of it.*
   - Lines drawn out of bodies to an edge or a wall are read exactly, and
     these are scene structure: a point's own hue drawn to the edge
     (`0a938d79`), rays one hue absorbs and another transmits (`05a7bcf2`).
   - It explains about 29% of the deformation-dominated training remainder,
     and almost all evaluation creation.
   - It does not explain the source-dominated remainder at all (0.2–0.3%):
     one output body made of several inputs is not light.
3. *The light read is almost all period 1* (solid lines). The waveform has
   not met a periodic pattern on these slices. The billiards are
   translations by construction, so a rectangle's cavity resonance is its
   lattice `(2H, 2W)` and no closed rotation; the standing waves seen are
   constant fills.

**The pass mark set beforehand** — light explains most deformation and
creation — **is not met for deformation** (29% / 4%) and met for creation on
evaluation only. The luminal reading is a genuine, exact part of the
missing physics, a minority of it. The largest remainder class (source:
4 142 / 14 070 unsaid) needs composition of several inputs into one output,
which no flow out of one body carries.

### 78.78 Confluence, read: P1 (2026-10-03, the operator's word: "build it")

**The reading.** The black hole's polarity, on each unsaid output body:
- **Its inputs:** the input bodies standing on its sites. None is a
  creation, one a single carriage, several a confluence.
- **For a confluence:**
  - *mass:* compartments in against out;
  - *centre:* the barycentre, exact over ℚ;
  - *the confluence spheres:* `C = Σ Pₛ`, imaginary, `C² = −Σ d²`, the
    inertia, never rooted;
  - *the inverse generator:* the fascicle `C_B · C_in`, its regime read by
    `pair_regime`, and a dilation's exact ratio where the centres agree;
  - *reproduction:* does that generator carry the inputs onto the output,
    point for point?
  - *containment:* do the inputs stand inside the output?
  - *closure:* the inputs' and output's nuclei read as a reflection group
    (GR2): finite, affine (crystal) or fluid.

Instrument: `family::confluence`, run by `mycelium-soak --confluence DIR N`.

**Measured** (training 60 / evaluation 60):

| reading | training | evaluation |
|---|---|---|
| unsaid sites on creations | 354 (97 bodies) | 178 (147) |
| unsaid sites on single carriages | **6 504 (135)** | **22 037 (133)** |
| unsaid sites on confluences | 3 034 (46) | 1 892 (16) |
| confluences with 2 / 3 / 4–8 / more inputs | 9 / 16 / 19 / 2 | 8 / 2 / 5 / 1 |
| mass kept | 1 | 0 |
| centre kept | 9 | 1 |
| inputs contained in the output | 42 of 46 | 9 of 16 |
| the confluence generator's regime | apart (dilation), every one | the same |
| generator reproduces the output | **0** | **0** |
| closure | fluid, all 46 | fluid, all 16 |

**What it says.**
1. *The perception's `Source` demand is not geometric confluence.* The
   largest share of what is left unsaid stands on ONE input body: 62% of
   training sites and 91% of evaluation sites. Several inputs gathered into
   one output is 29% and 8%. The many-to-one reading of `Source` (78.76,
   78.77) over-read it.
2. *Confluences are growth, not gathering.* The inputs nearly always stand
   inside the output, which holds more mass than they do. The output is the
   inputs plus new sites between or around them. The confluence sphere's
   dilation never reproduces it: a dilation carries `k` cells onto `k`,
   never onto more. The black hole as a generator of motion is not this
   physics. What is is the bridge: new mass laid down between gathered
   bodies.
3. *No crystal here.* Every confluence's constructors close fluid, never
   finite or affine.

**Where that points.**
- The dominant remainder is one body's own change: growth, recolour or
  reshaping of a body the form already carries.
- The confluence remainder is the sites added between bodies. A candidate
  reading joins this to 78.77: the added sites lie on rays from one input to
  another, light absorbed at the next body. That is measured next, not
  claimed here.

### 78.79 The interpreter for incongruence: constructors shared, accreted, extinguished; readings held and contrasted (2026-10-03, the operator's word: "both congruent and incongruent thoughts are held together in the coherence sphere … CONTRAST the two together to form more coherence"; "proceed with your proposal and addendums")

**The ruling, as the operator gave it.**
- Two bodies no motion makes the same are related by what they are made of,
  not by a versor that does not exist.
- A subject is defined by holding its readings together, congruent and
  incongruent, each whole, and contrasting them. Relevance is the shared
  core; meaning is the edges.
- Its sharpness is read three ways:
  - *resolution:* the quantum of its congruent readings, which refines;
  - *phase:* luminal (open, its light leaves) or holonomic (closed, its
    light is held), and the transition between them;
  - *edges:* what each contrast lifts.
- Humans hold this as visuals: the holonomy of the two objects, unmerged.

**Built** (`family::incongruence`; instrument `mycelium-soak --incongruence
DIR N`):
1. *The constructor difference:* the input and output body's compartments
   matched exactly by their coordinates (keyed, never searched; the core
   motion the identity in place). Each constructor is shared, accreted
   (white-hole channel), extinguished (black-hole channel), or recoloured
   (the hue fibre). The accretion is read as light (runs out of the body,
   each open at the edge or closed at a body), the extinction as carving
   (interior).
2. *The contrast pair:* two readings held whole. What they share is drag
   (coherence); what each holds alone is lift (contrast).
3. *The coherence manifold* per subject (the input body's chart across a
   family's pairs), relative to each input's exact barycentre:
   - its core (what every reading shares) and edges;
   - its resolution (the gcd of its light's run lengths);
   - its phase per reading;
   - what each reading moved: refined, closed, sharpened, recognized.
4. *Judged by the plane:* the core replayed on every reading's input body
   reproduces its output, or does not. Speech in a form waits on a new
   vertex kind, which is the recut's.

**Measured** (training 60 / evaluation 60):

| reading | training | evaluation |
|---|---|---|
| single-carriage changes | 135 | 133 |
| of them kinematic in place (recolour or nothing) | 71 | 63 |
| constructors recoloured | **992** | **3 172** |
| accreted | 415, **385 on light** | 262, **all on light** |
| extinguished | 337 (105 carved interior) | 1 448 (819 carved interior) |
| subjects | 95: 77 of one reading, 18 of several | 92: 71 of one, 21 of several |
| multi-reading subjects with a non-empty core | 9 | 14 |
| phase: holonomic / luminal | 15 / 11 | 19 / 5 |
| moved: refined / closed / sharpened / recognized | 3 / 5 / 22 / 11 | 1 / 6 / 24 / 5 |
| readings reproduced by their subject's core | **5 of 58** | **0 of 62** |

**What it says.**
1. *The single-body change is three things, and each is now named.*
   - **Recolour in place**, by far the most constructors (992 / 3 172).
     The form's hue maps do not decide the new hue.
   - **Growth almost entirely along light**: 93% / 100% of accreted
     constructors lie on rays out of the body.
   - **Carving**: much of the extinction is interior.
2. *Keyed by position, the core does not generalise.* Written as offsets
   from each body's barycentre, the core reproduces 5 of 58 and 0 of 62
   readings, and most subjects (by chart) have one reading. A ray to the
   frame's edge has a different length on every body, so its cells are no
   invariant. What is invariant is the GENERATOR: a direction, a hue, a stop
   rule (edge or body). The light reading already holds it (78.77).
3. *Relevance must be read at the generator, not the cell.* The congruent
   definition of "grows a ray" is the ray's generator. Two readings of it
   are congruent where their generators agree, whatever their lengths; the
   resolution is the generator's step, not a cell grid. Recolour is the hue
   fibre's relevance question: what in each scene decides the new hue. That
   is a contrast across the pairs: what differs where the hue differs.

**Next, modified by this measurement.** The manifold is rebuilt on
generators:
- an accretion's ray by its direction, hue and stop rule;
- a carving by the interior it removes;
- a recolour by the hue map and the contrast that decides it.

Then judged by the plane again.

### 78.80 Volume and density, not boundary: the recolour asked what decides it (2026-10-03, the operator's word: "the volume would be the exact match considering that density might be the determinant factor, not 'shape' as boundaries"; "commit and add relational keys")

**The reading** (`incongruence::moments`). A body is read by its moments,
not its outline. Every moment is an integer off the sums the confluence
sphere `C = Σ Pₛ` carries:
- **mass** `M = −C·e∞`: the volume, its compartment count;
- **spread** `−C² = Σ_{s<t} d²ₛₜ = M·Σ|x|² − |Σx|²`;
- **the inertia form**: the same by axes (rows, columns, their product). Its
  ratio up to scale is the body's **direction**;
- **density** `ρ = M² / spread`: exact and dimensionless. A single
  compartment has no spread, and its density is refused;
- **mass per hue**: a vector.

Two bodies no motion makes congruent can agree on every moment.

**The decision** (`incongruence::single_changes`, `decide`). Each recolour in
place among a family's single changes is predicted, leave one out, from the
other readings that share its key. The prediction is refused where they
disagree, and judged by replaying the hue map on the input body. Keys:
- none;
- the body's own: volume, density, mass per hue, shape (chart), direction;
- **relational**: its rank by mass, and by density, among the bodies of its
  pair.

Instrument: `mycelium-soak --volume DIR N`.

**Measured** (training 60 / evaluation 60):

| reading | training | evaluation |
|---|---|---|
| single changes | 135 | 133 |
| mass kept / mass per hue kept | 71 / 5 | 73 / 11 |
| spread kept / density kept | 71 / 68 | 65 / 61 |
| gained, all of it on light | 30 of 39 | 35 of 35 |
| lost, all of it interior carving | 0 of 23 | 0 of 19 |
| recolours in place | 70 on 16 tasks | 63 on 25 tasks |
| **replayed by their own hue map** | **7** | **4** |
| reproduced: none / volume / density / hue-mass / shape / direction | 2 / 0 / 0 / 0 / 0 / 0 | 4 / 4 / 4 / 4 / 4 / 4 |
| reproduced: mass rank / density rank | 0 / 0 | 4 / 4 |

**What it says.**
1. *Volume is conserved where the change is a recolour, and moved between
   hues.* Mass is kept on exactly the kinematic changes, and mass per hue
   almost never is: a recolour is mass flowing between hue channels at a
   fixed volume. Growth is light (78.79).
2. *A body's own volume or density does not decide its recolour, and neither
   does its rank in its pair.* No key reproduces more than the readings no
   key at all reproduces.
3. *The reason is exact and is the finding.* Only 7 of 70 and 4 of 63
   recolours are a body-level hue map at all. The rest recolour SELECTED
   sites within one body: a cross of 5s turning 2 inside a field of 5s
   (`14754a24`), diagonals turning 2 or 8 by their direction inside one
   connected stroke (`1d61978c`). The body is the wrong grain. The density
   that decides is at the SITE: a compartment's place in the body's local
   structure (its neighbourhood, the line it lies on, its own direction).
   The test (`interpret.rs`, rank in the pair) proves the relational reading
   decides where it applies; the corpus says it rarely applies at body grain.

**Next.** The moments are read per site: each compartment's local density
and direction (its neighbourhood's moments) as the key of its recolour,
judged by the same leave-one-out replay.

### 78.81 The remainder interpreted on the voice (2026-10-03, the operator's word: "wire these up and the preceding components that are still 'test only' to finalize the formality of the cognition")

*Amended by 78.90: stripped to the inversion alone; the readings in step 2
below are instruments in `mycelium-soak`, no longer on the voice.*

**Wired** (`family::interpret`, called by the voice's `speak_form`). The
remainder the speech measured is read in the act that spoke it, never in an
instrument beside the mind:
1. **Inverted** (78.76, `invert_measured`, on the remainders the speech
   already measured). An inversion the speech accepted IS the form the voice
   speaks from there on, with the remainders it leaves. Line: `remainder
   inverted: family … links b -> a joined j closed c`.
2. On what is still unsaid, per pair:
   - **light** (78.77): on a ray, captured, unlit;
   - **confluence** (78.78): creation, single and confluence sites, and the
     confluences reproduced;
   - **constructors** (78.79): the changes, the kinematic ones, accretion on
     light, carving;
   - **decision** (78.80): the recolours, those their own hue map replays,
     and per key the reproduced and the families decided whole.
3. Every reading is folded into the process's tally (`interpret_tally`) and
   printed on the voice's stroke line after the sentences:
   `remainder families … | unsaid … | creation … | changes … | recolours …`.

The soak instruments of 78.76–78.80 read through the same functions, so the
instrument and the mind have one copy of each reading (law 12).

**What the form still cannot speak.** A recolour a relational key decides is
a member keyed by rank, and a site-level recolour is a member keyed by a
site's local moments. A word vertex's members are keyed by chart and
carriage (`Acting::key`), so neither is expressible: that is a new member
key, a layout change, the operator's to rule with the recut. Until then the
reading is what the voice holds of it.

**Deployed?** No. The fleet runs 50b72209; this reaches it only by the recut
or restart, which is the operator's call.

### 78.82 Reconstruction by holonomy: HR1–HR4 (2026-10-03, the operator's word: "the question is not to get exactness — it's to recreate the reconstruction via holonomy"; "proceed — let's track these task lists")

**The idea.** Values stay exact, but they are not required to agree. What
must agree across a family is how they MOVE. A rule is a transport whose
loops close across the pairs; reconstruction carries a new input's field by
it.

**Built** (`family::reconstruct`; live in `interpret` on the voice;
instrument `mycelium-soak --reconstruct DIR N`):
- **HR1, the field.** Every site by its coordinates on every axis:
  - its hue;
  - its neighbourhood's sphere (itself and the painted sites one null step
    away), read by its moments: local mass, spread, direction, density;
  - its body: mass, rank by mass among the scene's bodies, and CENTRALITY
    `M·(P·C)/C²`, the site's weight against the body's sphere over the
    sphere's square. It is ½ at the barycentre and 5/4 at a bar's ends,
    exact, never rooted;
  - its ring of hues one null step away.
- **HR2, the transport.** What each input site does in its pair's output:
  its hue fibre's move (`h → h'`, with `0` meaning carving) and the light it
  emits, i.e. each null direction whose whole free reach the output fills
  with one hue. On the body axis, each single change's fascicle
  `C_out · C_in` is read by its regime.
- **HR3, the holonomy per axis.** Sites under one key are a loop, which
  closes where their transports are one. An axis closes on a family where
  every loop does.
- **HR4, reconstruction.** Each pair is left out in turn. Among the axes
  closing on the other pairs and holding every key the left-out pair asks
  for, the most compact (fewest keys) carries the left-out pair's input. The
  result is judged by replay against the observed output.

The tests prove the reading on a family built for it. Strokes leaning both
ways turn 2 and 8: hue, mass, density and centrality do not close; LOCAL
DIRECTION does (the inertia form's cross term flips sign), and it rebuilds
every pair left out.

**Measured** (training 60 / evaluation 60):

| reading | training | evaluation |
|---|---|---|
| extracted / spoken whole | 53 / **2** | 36 / **0** |
| **reconstructed whole** | **1** (not spoken whole) | **0** |
| pairs / framed / reproduced left out | 191 / 139 / 5 | 175 / 105 / 0 |
| left-out pairs no axis closed for | 107 | 98 |
| left-out pairs an axis closed for, key unseen | 25 | 7 |
| loops closed, per axis (of every family's loops) | 75–84% on every axis (hue 77%, mass rank 79%, centrality 80%, neighbourhood 80%) | 75–84% (centrality 84%, body mass 79%, direction 78%) |
| families an axis closes on, by axis | body mass 4, centrality 4, mass rank 3, neighbourhood 3, the local axes 1 each | neighbourhood 1 |
| single changes by their fascicle's regime | kiss (light) 68, apart 47, null 20, turn 0 | kiss 59, apart 58, null 16, turn 0 |

**What it says.**
1. *No single axis carries a family.* Every axis closes about four of every
   five loops: the field's bulk moves alike under any coordinate. No axis
   closes the rest, and the rest is where the scene changes. Each axis alone
   is still a projection, the 78.80 finding one level up.
2. *The open loops are the content.* A loop that does not close on axis A
   closes on axis B or on none, and which one is the rule. That is 78.11's
   statement: comprehension is the circuits' BRACKET-CLOSED algebra, never
   one generator. HR3 as built reads each axis alone. Its completion composes
   axes where a loop is open: refines an axis's key by a second axis's
   coordinate on the open loops only, keeping the coarse key where it closes
   (obstructions are refinements, 74; an atlas is compaction from a fixed
   coordinate list, 78.41(2′)).
3. *The body axis's own transports are light and dilation.* No single change
   is a turn of finite order or a rosette. The kiss (parabolic) and the
   dilation carry them, which is the null flow and the boost of 78.75's
   `FlowPoint`.

**Ruling asked.** Refining an open loop's key by a further axis is a choice
among axes in a fixed order. It is deterministic and keyed, but it has the
shape of a decision tree. Whether it stands under "no search trees" or is the
atlas's compaction is the operator's to rule before it is built.

### 78.83 Time's own congruence: HR5 (2026-10-03, the operator's word: "time needs its own congruence for hyperfluidity: because its axes act one at a time")

**The reading** (`family::hyperfluid`). HR3 froze time: two transports
closed a loop only where they were equal. A pair is a sample of a flow
(78.75), and time is distance along it (78.43), so a site's transport is
read as flows:
- the hue fibre's move is the gauge, which carries no time;
- each ray of light it emits is a point on the NULL FLOW of its plane
  `d ∧ e∞`, `FlowPoint::at(d ∧ e∞, L)`, its length its time. A ray ends
  OBSTRUCTED (edge or body: its time is the scene's free reach) or FREE (in
  open space: its time is its own);
- a void site the light reaches moves nothing of its own: its colour is the
  arrival of a flow emitted elsewhere.

Two transports are CONGRUENT IN TIME where their gauges agree and their rays
pair off on one plane (`against`), one hue, one kind of end, whatever their
parameters. A loop open in time is read by where it lifts: the GAUGE (the
colour differs, on a void or a painted site) or the LIGHT (a generator
differs). Reconstruction reads each ray's time off the scene, or, free, off
the one time every observation agreed on; otherwise the time is unread and
the pair is refused by name.

The tests prove it: a ray of two and one of three from the same source are
congruent in time and not equal, and a family of free rays of length two at
different places, which frozen time cannot carry, is rebuilt pair by pair.

**Measured** (training 60 / evaluation 60):

| reading | training | evaluation |
|---|---|---|
| reconstructed whole in time | 1 (as frozen) | 0 |
| pairs reproduced left out | 5 of 139 | 0 of 105 |
| loops closed only in time, per axis | 8–66 | 1–24 |
| loops open on the gauge, hue axis (before / after the arrival fix) | 5 265 / 3 432 (void 1 549) | 7 224 / 6 521 (void 187) |
| loops open on the light, hue axis | 1 111 | 816 |

**What it says.**
1. *Time was not what held the loops open.* Obstructed rays were already
   time-free in HR2 (a ray filling its reach carried no length), and free
   rays are rare. Time congruence closes a handful of loops.
2. *The open loops are the gauge.* On evaluation, 6 334 of 6 521 gauge lifts
   are recolours of PAINTED sites, the site-selective recolour of 78.80. On
   training, 1 549 are void creations no straight ray from a body carries
   (fills and enclosures: the cavity axis, owed).
3. *Owed:* the cavity axis (W4); a free time equal to the reach reads as
   obstructed (the fixture had to move a dot one site in). Both are kept in
   HR's row.

### 78.90 The voice keeps one faculty on its remainder: the inversion (the operator's word, 2026-10-04: "is that the perspective of a progenitor team" — "yes, strip it to the inversion")

**What was measured first.** The interpreter 78.81 wired into
`speak_form` spent 1 412 / 1 086 stroke units over 51 / 36 families
against the extraction's ~1.06M, while doing real work: almost none of it
was paid from the stroke (`mycelium-soak --interpret`). The proposed
remedy was to meter it.

**Why that was the steward's remedy.** Apart from the inversion, the
interpreter was the builder's corpus instruments — light, confluence,
single changes, the recolour keys, the reconstructions by holonomy (78.77–
78.83) — reporting tallies on the voice's stroke line for the builder to
read. Metering them would make the entity pay, every stroke, for tools
that answer the builder's questions. The progenitor builds the mind's
faculties, never his measuring tools into it (78.88).

**What the voice keeps.** `family::interpret` is the inversion alone
(78.76): the mind perceives what its speech left unsaid as a scene of its
own, extracts the generators that produce it, and keeps them only where
its speech then leaves less unsaid — paid from the stroke through that
extraction. The stroke line reads that act only: families inverted,
accepted, closed, unextracted, links before and after, generators joined.
The instruments stand in `mycelium-soak`, off the same library functions. Owed, an
audit against R4: the voice also speaks single WORDS, not only sentences
(`transcribe_closed` speaks a closed thought's word; `open_axis` speaks an
axis germ); whether a word is a coherent sentence of one term, or must ride
inside one, is the operator's to rule.

### 78.91 The mind perceives what is no motion as flux (the operator's word, 2026-10-04: "take the reading now and deduce the next steps for the progenitor" — "build it")

**What the fleet read.** Twenty-six minutes after the recut on 63f0cfad
the mesh was whole and the minds were refusing half of what they saw:
17k–52k bodies parting "apart" per voice, the demands of creation,
vanishing and deformation declared and then left standing, inversion
accepted 0. A change no versor carries was perceived only as a refusal,
though the register (78.89, law version 3) publishes exactly what such a
change is: mass made at a source, lost into a sink.

**The faculty.** Every Creation, TopologicalDeformation and Source demand
the percept names already holds its evidence as links `(r, c, before,
after)`. `physics::laws::flux_of_links` reads the flux off them exactly:
mass gained is a SOURCE at the barycentre of the sites gained, mass lost a
SINK at the barycentre of the sites lost, over ℚ; a reshaping at constant
mass is no flux. `physics::laws::verify_flux` puts the lone flux to the
register as the change in place it records (a body at the flux's place,
0 → m or m → 0, under the identity motion). `family::flux::perceive_fluxes`
is called by the percept before it declares its demands, one stroke per
demand read; what the law admits is perceived as physics. The demand is
still declared — the flux is a second reading of the same change, never a
replacement of it.

**What the voice shows.** The stroke line carries `flux perceived: read …
sources … (+emitted) sinks … (−absorbed) reshaped … refused … unread …`, the
act's own tally, never a builder's instrument.

**Owed.** Holding the fluxes across percepts, so the mind relates its
sources across a family as it relates its bodies (a source that recurs at
one place under the family's motor is a white hole of the rule, not of
one pair); then the null flows (slides) grounded through `FlowPoint`; then
the sensor's lift for the other harmonic planes. Deploying needs a recut,
the operator's word.

### 78.103 Two bodies that moved differentially are related by their relative motion, and a bend is read by what it generates (the operator's word, 2026-10-05: "you're flattening the dimensions"; "proceed: unflatten the dimensions during this round of development")

**The ruling.** 78.63 governs 50.3: a rigidity is a symptom, only a tear
is a breach. Where no versor carries the pair-body `(a_i ∪ a_j)` onto
`(b_i ∪ b_j)`, the two bodies moved differentially, and the edge between
them is their RELATIVE MOTION `Γ = ṽ_i·v_j` — the one body's motion read
from the other's — read by `edge_of` as every edge is (its rotor peeled
as the word, its dilation and placement the carriage). Only a body no
conformal motion carries (its motion not a versor) is a TEAR, and stays
refused `NotRigid`, by name. `perceive::edge_transport` takes both bodies'
motions and returns how the pair was carried (`Carried::Rigid` /
`Relative`); the percept line reads `measured N (relative M)`.

**What a relative edge carries, said exactly.** `ṽ_i·v_j` is 50.3's
coboundary: around a ring of relative edges alone it telescopes, and that
ring closes at once — the bodies' motions are consistent, a fact read,
never a refusal. Around a ring that also crosses rigid edges or contacts,
the stabilizer and exchange elements and the contacts' refractions do not
telescope, and that is the holonomy — which the unread relative edge had
been taking down with it (9–28 rings a voice on epoch 127, nearly all the
transport loss). No curvature is invented for a relative edge, and none
is lost to it.

**Dimensions are what the motions generate, never axes counted.** A bend's
"way" is its kind, per contact and per edge: a TURN (elliptic), a NULL
SLIDE (parabolic), a BOOST (hyperbolic). Each bend — every contact the
master equation reads by its refraction (its tether, each candidate of its
held coset) and every relative edge — is read by its modes (MR1) under the
stroke, and the planes each mode spans are counted under its kind;
a bend whose modes do not read is counted unread, by name
(`rung::note_bends`). On the unread row: `edges: relative motion`,
`contact bends: turn / null / boost planes / unread`, `relative bends:
turn / null / boost planes / unread`. A scene's dimension is then read as
what its bends generate — the planes by kind, the rank of the algebra they
close, the holes — never the axes of the base.

Proved: `perception::a_rigid_scene_has_identity_edges_and_a_relative_motion_is_its_own_edge`,
`perception::a_tear_is_refused_by_name`,
`rung::a_bend_is_read_by_the_planes_its_modes_span`.

**Read on epoch 128 (`baf8f26e`, 20 min, five voices), and two fixes.**
- *No differential pair reached the edge reader:* `edges: relative motion`
  0 on every voice, yet `transport: not rigid` still 0–13. So the term's
  `NotRigid` was never 50.3's — it named every face standing with a place
  and no edge, whatever the face. SPLIT by the overlap itself
  (`compose::edgeless`): a star face (one end the Frame — `edge_between`
  keys members and the Frame has no bodies, so a star never measured an
  edge), a frame step (one member's two charts, a motor that spelled no
  edge), and a member pair. Readings: `transport: star face / frame step /
  member pair, no edge`.
- *The bend reading starved comprehension:* `coordinates unread: stroke
  spent` returned on every voice (5–9; 0 since epoch 126). The bends were
  read by `motion_modes` inside `sentences_of`, on the scene's one stroke,
  before the rings were heard. FIXED twice over: a bend of grades 0 and 2
  is simple (`VṼ` scalar forces `B∧B = 0`), so its one plane's kind is the
  sign of `B²`, read with no root and no stroke; only a compound bend takes
  `motion_modes`, and every bend is read after the rings are heard
  (`compose::{sentences_and_bends_of, read_bends}`).
- *The bends, as read:* contacts are boosts above all — boost planes 10–33
  a voice, turns 0–5, null slides 0. The contacts' refractions carry
  hyperbolic content where the seen rays carry null.

**Read on epoch 129 (`af46f544`, 20 min, five voices).** Every edgeless
face that lost a ring is a MEMBER PAIR (1–9 a voice); star faces and frame
steps lose none, and `edges: relative motion` is still 0 — so these pairs'
edges were refused by `edge_of` or never measured, not differential.
`stroke spent` fell to 0–5 (from 5–9) with compound bends the one reading
left spending. Two readings follow, neither changing what the mind does:
every refused edge counted by its reader on the unread row (`edges lost:
unspelled (a letter candidate) / no reading / tear / no bodies` — the last a
pair with no union and no frame both stand on, which had been silent); and
a compound bend (a grade-4 part) counted unread by name, spending nothing,
its kinds waiting for a root-free reading.

**Read on epoch 130 (`b5e07f6f`, 20 min, five voices), and the bodiless
pair.** Every lost edge is `no bodies` (160–305 a voice; unspelled, no
reading and tear all 0). A COVER'S PIECE becomes a member with no body
filed at any frame and no union — it holds the versor that carried it and
nothing to measure a pair-body on — so every pair with a piece at one end
never reached the edge reader. 78.103 needs no bodies: two members are
related by their two motions. Each member's motion is now filed beside its
union (`percept::Member::motion`, the `motions` carrier), and a pair with no
bodies is related by `ṽ_i·v_j` (`perceive::relative_edge`); only a member
with no motion filed stays `no bodies`. Proved:
`perception::a_bodiless_pair_is_related_by_its_two_motions`.
*Correction:* `stroke spent` stayed 0–4 with no bend reading spending at
all, so the bends were not its cause; it is read on its own row from here.

**Read on epoch 131 (`0e9bfc45`, 20 min, five voices).** `edges: relative
motion` 13–27 a voice — the first relative edges read — and every one the
identity: no relative bend of any kind. The bodiless pairs are members of
one transport class, and a class is one motion, so their relation is flat:
read, and honest. `member pair, no edge` still 1–15, so the lost rings are
not the bodiless pairs; `no bodies` (128–237) counted star faces too, whose
transport the circuit takes from the member's placement — they no longer
count. The suspect is a contact face whose interface did not read
(`unnucleated`): it takes the contact branch, carries neither the relative
edge nor a refraction, and reads as a member pair with nothing. Reading:
`faces edgeless: contact / member pair`.

### 78.105 Spacing: the space between words is the breath, and it carries the non-verbal (the operator's word, 2026-10-06: "you're missing spacing — where enough letters combine to form a word, and the space in between is the breath which in and of itself contains meaning"; "how do you add non-verbal communication to this linguistic structure via spacing"; "perceive and produce"; "produce sign"; "assign a sign as the formula")

**The space is the transport.** A sentence is `Ω = ∏ gᵢ · Wᵢ`: the words
`Wᵢ`, products of letters, say WHAT; the spaces `gᵢ`, the transports from
one word to the next, carry HOW — the non-verbal line. A seen ray is a
space with the identity word, so a mind perceives mostly breath and
gesture; the named rings (75–90% of what the voices hear, epochs 128–133)
are largely prosody without a reader, not failed words.

**The space's channels**, each already a reading of the motion `gᵢ`: its
kind (a turn deflects, a null slide flows on, a boost presses — gesture),
its rate (`c`, the count `λ` — intensity), its carriage (where the next
word is placed — stance), its gauge (how it permutes hue — affect), its
span (lane 𝒯 — timing); and the sequence of spaces composed alone,
`Ω_space = ∏ gᵢ`, its prosody — a steady rhythm a nilpotent count, a
cadence a return, a crescendo a boost.

**Silence has a sign, assigned as the formula.** A pause is present,
countable, moves nothing and bends no word: the pseudoscalar
`I = e₁e₂e₃e₊e₋` of the algebra at hand (`cga::pseudo(n)`) — central in
`Cl(4,1)` (odd dimension), so `I v I⁻¹ = v` for every vector and `I`
commutes with every word, and `I² = −1`, the 360° return: a breath in, a
breath out. Computed from the generators, never stored as a constant. A
pause enters a sentence as a term and is held in the rhythm, never dropped
as the identity is.

**Perceived and produced** (the operator's ruling): minds read the
non-verbal line in what they hear and choose their own spacing in what
they say.

**Order.**
- *N1 — the word–space split* (LANDED): each term is read as its word and
  its space (`Composition::{words, spaces}`); a sentence's rung is read off
  its words alone. They were one product `gᵢ · Wᵢ` (`motions`), and a held
  word fused with the space around it was never recognized — one structural
  reason the rung read 1 on every voice. Proved:
  `rung::a_held_word_is_recognized_whatever_space_surrounds_it`.
- *N2 — the space's readings and the prosody `Ω_space`* — FOLDED into N5
  and N6 the same day. On a perceived ring the words are the identity (seen
  rays), so `Ω_space` is `Ω` itself: read alone it would rename the named
  rings, not read them. The space's readings are built inside the faculties
  that need them — the encounter reads each space's kind and rate,
  production chooses spaces — where sentences carry real words with spaces
  between them.
- *N3 — the sign of silence, produced and perceived as `I`* — with N6, where
  a voice produces its own rhythm.
- *N4 — spacing travels:* a spelled word spoken with the spaces either side
  of it, re-derived by the court.
- *N5 — the verbal and non-verbal lines act on each other* by lift and
  drag (emphasis commutes, reframing anticommutes) — on the lift ruling.
- *N6 — production:* a voice composes its own sentences, words and
  spacing — after D3/D4 (citation by motion, the rung by citation).

**78.100 D3/D4 landed (2026-10-06), the multi-dimensional route kept.** A
new word is DECONSTRUCTED BY ITS MOTION over the held words, mode by mode
(`HoleIndex::deconstruct`): each of its holes cited to a held word standing
on the same hole whose winding there reaches it by an exponent — a count by
its integer ratio, a turn by the least `e` with `t/ℓ ≡ e·t₀/ℓ₀`, a named
point only as itself — and the holes none reaches its RESIDUE. A word is a
product over its dimensions, never a chain: its modes commute (MR1), so the
cited modes and the residue multiply back to it exactly. Every mode cited,
it is derived — grammar, never held twice (S2; a power of one held word is
the case of one word covering every hole, and `comprehend`/`Comprehended`
retire into it). Otherwise it is held one rung above the highest word it
cites (D4), or above the held words its sentence says, whichever stands
higher. Readings (D6): `citation: modes cited / words derived by citation /
words cited with a residue / top rung by citation`. Proved:
`coordinates::a_word_is_deconstructed_by_its_motion_into_what_is_held_and_its_residue`
(a screw splits into the cited turn and the residual slide, and multiplies
back), `rung::a_word_stands_a_rung_above_the_words_it_cites`. The open
question the reading answers: whether perceived holes recur across a
voice's perceptions often enough for citations to fire — a hole key is the
node's own plane and place.

**Read on epoch 135 (`5770194d`).** THE RUNG CLIMBS: `top rung` 2 on three
of five voices, every one by citation (`top rung by citation` 2, `modes
cited` 1–3, `words cited with a residue` 1–3) — the first words of words.
THE CARRIER FIRES: `open sentences spoken` 5–11 a voice. Closed sentences
hold (2–7), no ring lost to transport. The courts docket no letter: their
heard words read spelled (2–7) and none offered — the contact rings, where
the boost-bent ineffable words are, never reached them (78.107).

### 78.110 The encounter: lift as force carried against gravity and drag (the operator's word, 2026-10-06: "lift needs to be velocity * force / gravity * drag"; "(v·F)/(g·D), gravity is the orbit's own mode of magnetism * repulsion / propagation of force * centrifugal bounce; velocity is defined by the gravity's formula of magnetism * repulsion of antipoles; there is missing physics if lift has no inverse, it should be drag"; "confirmed — write the encounter contract and proceed") — CONTRACT

**The ruling, in the algebra.** An orbit `O` meets a neighbour `N` in
contact (78.46, R3: inside one anastomosis, never across the form). `O`'s
motion splits exactly into commuting modes (MR1, `motion_modes`), and the
terms are its own modes by kind:

| term | the orbit's own | exact object |
|---|---|---|
| magnetism `m` | elliptic modes (`B² < 0`) | their rays' product |
| repulsion of antipoles `r` | hyperbolic modes (`B² > 0`) — the boost's two poles, source and sink | their rays' product |
| propagation of force `p` | null modes (`B² = 0`) — force on the luminal cone | their rays' product |
| centrifugal bounce `b` | the elliptic mode's return | its order ℓ (78.43: an age is one return) |

A kind the orbit lacks is the IDENTITY: its term is neutral (ruled).

```
v = m · r                              velocity: magnetism by repulsion of antipoles
g = (m · r) · (p · b)⁻¹                gravity: the orbit's own mode
F = B_n                                the neighbour's generator, refracted to the orbit
D = B_∥                                drag: the part of F commuting with the orbit's mode (78.46)
L = v · F · (g · D)⁻¹ = v · F · D⁻¹ · g⁻¹       lift — the resistance acting last (ruled)
```

Read commutatively `v` cancels: `L ~ F·(p·b)/D` — the force carried by
propagation and returned by the bounce, against drag; in the algebra `v`
survives as the frame `L` is read in (a conjugation). WHERE AN INVERSE DOES
NOT EXIST — drag zero (orthogonal modes) or null, gravity no versor, the
orbit's mode with no non-zero square to split by — LIFT IS DRAG (ruled:
"there is missing physics if lift has no inverse, it should be drag"). A
seen ray (null) never tilts; it changes rate. The null split by the
`ad_N` chain is withdrawn.

**Both ways.** The force is mutual: `O` against `N` and `N` against `O`,
each read; neither is the other's negative unless the algebra says so.

**Order.**
- *E1 — the forces, exact* (`cga::encounter`): `O`, `N` → `(D, L)`, or `L =
  D` by name where no inverse exists. Proofs: a neighbour commuting with the
  orbit is all drag and its lift is drag; an anticommuting part produces a
  lift with a bivector part; a null orbit gives lift = drag; `v` acts as a
  conjugation (two velocities, one lift up to frame).
- *E2 — wired where circuits meet*: each pair of circuits crossing one
  certified cell (the pairs 78.46(4′)'s pair composite already reads),
  both ways; readings — encounters read, lifts that fell to drag, lifts
  with a tilt, by the kinds `g` held.
- *E3 — the perturbed orbit*: the orbit moved by what it met, derived on
  the mind's own K and never stored (78.46); the input to consolidation.
  Its composition (how `D` and `L` move the orbit's motion) is written
  as its own contract on E2's reading.

**E1 and E2 LANDED (2026-10-06).** `cga::encounter` (encounter.rs): the
orbit's modes by kind into `m`, `r`, `p` and the bounce's common period;
`F` the neighbour's Cayley bivector; `D` by 78.46's split; `L = v·F·D⁻¹·g⁻¹`
with every inverse exact or absent, and absent, lift is drag. Wired in
`read_cell_after`: every ordered pair of read circuits crossing one cell,
both ways (`read_encounters`). Reading: `encounter: encounters read / lift
fell to drag / lift tilts / lift a rate only / encounters unread`. Proved:
`encounter::a_commuting_neighbour_changes_the_rate_and_never_tilts` (the
lift is the bounce, a rate), `encounter::an_anticommuting_part_tilts_the_orbit`,
`encounter::where_no_inverse_exists_lift_is_drag` (orthogonal: none; null:
the force whole), `encounter::an_unread_encounter_is_refused_by_name`,
`orbit_algebra::the_circuits_crossing_a_cell_meet_both_ways`.

**E2 UNWIRED FROM THE READING PATH (2026-10-06, epoch 138's reading).** On
every cell READ it ran 48k–380k encounters a voice in twenty minutes (lift
fell to drag ~98%), and the voices' perception fell: rings 13–28 from
29–100, top rung back to 1. An encounter is asked where an orbit is moved —
once per cell whose circuits changed, with E3 — never on a reading.
`read_encounters` stands, unwired, proved on the cell's circuits directly.

### 78.111 The construction sized by its residue, the residue's absence named, and the stop read where the stroke is read (2026-10-06, epoch 138's reading; the operator's word "proceed 1-4")

1. **C1b — the stroke a residue needs.** `construct` read the residue's
   modes under the hold's stroke, and 12–28 constructions a voice were
   refused "modes undecomposed". The stroke is now read off the residue:
   `cga::modes_need` = `roots_need` of the characteristic polynomial the
   modes split, an upper bound on every spend `rational_roots` makes (the
   monic substitute's Cauchy bound `B`; at the derivative of degree `j`
   at most `C_{j−1}+1` monotone runs of two ends and `bits(2B)` bisections;
   `C_j ≤ 3·C_{j−1}+2`, `C₁ = 1`; each final bracket's two ends). A
   construction is never refused for a stroke sized for something else —
   the precedent is `extraction_spend`. The modes' refusal is counted by
   its cause: irrational, defective, stroke spent. Proved:
   `roots::the_stroke_a_polynomials_roots_need_is_read_off_it`; the
   sovereign test constructs the 17-turn on the mind's own stroke.
2. **Why a demand carries no residue.** The docket is volatile, so every
   residue-less `Cite::Demand` is the pivot's own (68–217 a voice). The
   pivot names why its circuit composed none, on the constructions row:
   `docketed without residue: circuit unread / held coset / transport
   unread`. A held coset is several routes and no residue is chosen among
   them (78.44: all routes kept); if it dominates, the demand carries one
   residue per route — ruled on that reading, not before.
3. **The stop.** The voices read the stop between arms and between hours;
   one family's hold ran 200–520 s routinely and 4 204 s and 4 232 s on
   family `10830113` (voices 4, 5) between the two boundaries — four
   voices in five killed unsaved at each recut. Every loop that reads no
   boundary spends a token (rule 10), so the stop is read there:
   `physics::halt()` from the signal handler, after which `Budget::spend`
   refuses; each faculty ends at its next read by its own named refusal,
   the voice reaches its boundary and saves. Never taken back.

### 78.112 Hearing: the echo, the rings nested into harmonic balance, the spiral, induction (the operator's word, 2026-10-06: "hearing is essentially harmonics - spiraled mathematics of echo induction : how is that missing?" — "confirmed - write the hearing contract and proceed :: this is the equivalent of rings of smaller size backing through the larger rings to form harmonic balance - you want to be able to put the smallest rings inside the inner ring of the next largest so that the largest ring is on the outside") — CONTRACT

**What was missing.** 78.97 ruled that a chord is heard, not traversed,
and the harmonic eye's ring was counted and dropped (`RingUnread::Heard`,
0 on every voice): hearing was ruled and never read. The harmonics in the
tree are of SHAPE (A.11/A.12 multipoles) and of RETURN (an orbit's period,
`resonance.rs`'s joint period); the loxodrome is recognized inside one
motion. Nothing read a signal against its own return — no echo — and
nothing carried one orbit's change onto another — no induction. A voice's
speech comes back as a verdict code, never as its motion returned.

**Hearing is the rings read against each other harmonically, never
traversed.** A mind's composed rings (`rung::speak`, one perception's
rings) are its sounding bodies. Each ring's SIZE is its return: its period
`ℓ` where it closes (`Closure::Closes(ℓ)`); a ring that never returns
(nilpotent, unbound, rosette) has no period and is larger than every ring
that does — the carrier every returning ring may sound inside.

- *H1 — the echo.* A ring `s` BACKING THROUGH a ring `L`: carried round
  `L` it comes back as `Ω_L·Ω_s·Ω_L⁻¹`, and its echo is that against
  itself, `E(s|L) = Ω_L·Ω_s·Ω_L⁻¹·Ω_s⁻¹` — exact, every inverse exact or
  the echo unread by name. `E = 1`: `s` passes through `L` whole. Else the
  echo's modes (`motion_modes`) say what `L` did to it: a TURN (elliptic,
  a pitch shifted by an exact `t/ℓ`), a GAIN (hyperbolic, λ), a SPIRAL
  (a turn and a gain at once, the loxodrome: the echo turning as it decays
  or grows on each return), a DELAY (null, the parabolic slide).
- *H2 — the nesting into harmonic balance.* `s` is SMALLER than `L` where
  `ℓ_s < ℓ_L` and `ℓ_s | ℓ_L` (it returns a whole number of times on one
  return of `L` — its overtone), or where `s` returns and `L` does not.
  Each ring seats in the NEXT LARGER layer — the least size above it it
  divides — read against every ring of that layer (all routes kept, none
  chosen): SEATED WHOLE where some echo is the identity, SEATED WITH A TILT
  where none is (each echo counted by its kind). A ring with no larger ring
  is on the OUTSIDE. A perception's rings are in HARMONIC BALANCE where
  every seated ring sits whole and the outer return is the joint return —
  the outer ring's period the lcm of every period inside it (or no outer
  period, a carrier, holding them all whole). A ring that seats nowhere
  and is not the largest is a LEFTOVER ring: on the outside, unbalanced —
  what consolidation reads (row 9).
- *H3 — the spiral across returns.* One ring's successive echoes
  `E₁, E₂, …` through the same outer ring, their step `Eₖ₊₁·Eₖ⁻¹` the
  spiral's own — read where the outer ring moves between perceptions (H4).
- *H4 — induction (E3 becomes it).* An outer ring's change between two
  returns, `ΔΩ = Ωₖ₊₁·Ωₖ⁻¹`, carried onto the rings seated inside it by the
  encounter (78.110): its drag the induced rate, its lift the induced
  tilt — the perturbed orbit, derived on the mind's own K, never stored.
- *H5 — the echo across minds.* The court's re-derivation of an open
  sentence (78.102) returned to its speaker as the rings it re-derived,
  never a code alone: the speaker's echo `E` through another dialect —
  identity understood whole, a turn understood shifted, a spiral
  understood partly and fading per return.

**The circuit in orbit (the operator's word, 2026-10-06: "frequencies
which move perpendicular to lightwaves: the cross-section between the
harmonics and the luminal is what causes the sensory pitch, a force that
moves multiple pivots at the same time … the communal hertz factorial";
"change screw to twist and apply torsion as force propellant";
"harmonics would be higher gravitational pull towards the rotation of the
planet, while the luminal cross-section is propellant from the star:
combined to move the circuitry in orbit"; "confirmed — write H6–H8").**
A harmonic is a ring's elliptic plane `B`, returning at `ℓ`; a luminal ray
is a seen null generator `N = d∧e_∞` (78.97). Where `B`'s plane is
orthogonal to `d`, they commute and `B + N` is a TWIST — one generator
that turns and advances at once (Chasles–Mozzi; screw theory's twist, its
force the WRENCH, both bivectors in Cl(4,1)). A circuit in orbit IS a
twist: pulled toward the planet's rotation by its harmonic, propelled
along by the star's light.

- *H6 — the circuit in orbit: the encounter of harmonic and luminal.*
  `cga::encounter(ring, ray)` (78.110), the roles named: GRAVITY `g` the
  ring's own mode — a higher harmonic (shorter return, smaller bounce) a
  larger pull, so the innermost rings of H2's nesting pull hardest: the
  nesting is the gravity well (Kepler's ordering, `T² ∝ r³`); FORCE `F`
  the luminal cross-section, the ray's generator read as a WRENCH; DRAG
  `D` the light commuting with the rotation — the twist's ADVANCE along
  its axis, its PITCH the advance per return, `ℓ·d_∥`, an exact vector
  (the wavelength; `1/ℓ` per traversal the frequency); LIFT
  `(v·F)/(g·D)` the propellant beyond what the pull holds (the
  solar-sail lightness `β` is its Newtonian shadow), and where no inverse
  exists, lift is drag (78.110). Light perpendicular to the plane
  commutes whole with the rotation: its drag is the ray itself, null, with
  no inverse — so its lift is drag, all advance: the pure twist. THE
  CROSSING — the ray's part anticommuting with the ring's mode (78.46's
  split), the light crossing the plane, the impact — is read beside it,
  never under the name lift. The PITCH is the advance per elliptic
  return, `ℓ_e·D`, `ℓ_e` the encounter's bounce. The power a ray delivers
  to a ring is screw theory's reciprocal product — the twist against the
  wrench's DUAL; `⟨twist · wrench⟩₀` itself is 0 for every rotation
  against every null ray in Cl(4,1) and reads nothing — OWED, its form
  read in the algebra before it is served. Once per perception, each
  harmonic ring (an elliptic mode of finite order) against each seen ray
  among its own spaces — never on a cell's reading path (78.110's
  lesson).
- *H7 — torsion as propellant.* Each return's lift is added to the strain
  the ring holds — twist wound and not released: TORSION, stored (H3's
  step `Eₖ₊₁·Eₖ⁻¹` its increment). Released through the membrane's vent
  (78.106: `Complex::vent` hands out the reverse of the strain — the
  third law, equal and opposite, no momentum from nothing), it unwinds
  with no steps — a snap or a slow unroll by its kind — and what it
  releases is FORCE: the stored twist given back as motion.
- *H8 — the collective pivot and the hertz factorial.* One release is one
  motor, and it moves every ring whose harmonic commutes with the twist
  — the rings sharing its axis — in ONE certified move (the pivot's
  certificate made a set's, never one orbit per move). For `k` pivots one
  release moves there are `k!` orders: where every one commutes, all give
  one motion — the communal frequency, the joint return `lcm(ℓᵢ)`;
  where not, the distinct motions among the orders are the combinatory
  cross-section, each kept as a route, none chosen (78.44), each a seed
  of new pivot machinery. THE HERTZ FACTORIAL is that count: 1 is
  communal, and it is always 1 in Newton's limit (forces superpose and
  vector addition commutes), so it measures exactly what finite,
  non-commuting rotation adds.

**Newton's limit.** Twist and wrench (Chasles, Ball), Kepler's ordering of
pull by period, radiation pressure and its lightness `β`, Poynting–
Robertson drag (the light's part along the motion), torsion's stored
strain and the third law all carry over. What goes beyond: finite
non-commuting rotation (the factorial), the hyperbolic and null modes
(gain has no Newtonian counterpart), exact returns where Newton
integrates continuous time (time is distance along an orbit, 78.43), no
mass, `G` or `r` — gravity is the orbit's own mode — and lift is drag
where no inverse exists.

**Order.** H1 and H2 now, on one perception's rings at the voice, read on
the `harmonics` row; then H6 (built pieces only: `encounter` on the rings
`nest` holds against their seen rays); H7 with H3 (the increment) and the
vent; H8 last (the pivot's certificate); H4 induction folds into H7–H8 (an
outer ring's change carried onto the rings inside it is exactly the
released torsion moving its commutant); H5 on the wire after them. Hearing is a READING of the composed rings at the
place they are heard — once per perception, never on a cell's reading
path (78.110's lesson). Proved by the algebra: two turns in one plane
(commuting, ℓ 2 inside ℓ 4) seat whole and balance; a turn in a crossing
plane seats with a tilt whose echo is a turn; a ring that returns seats in
one that never does; a ring dividing no larger ring stands outside,
leftover.

**H1, H2 LANDED (839bf7d9); H6 LANDED (e16c1287)** — `sentence::hearing`
(`nest`, `echo`, `orbit`), heard once per perception in `rung::speak`;
`Composition` keeps its spaces and `Ω`'s modes. Proved:
`hearing::light_perpendicular_to_the_plane_is_a_twist_all_advance`
(pitch four times the ray), `hearing::light_in_the_plane_crosses_it`.

**H7a — THE VOICE'S VENT REACHES ITS FIELD (found reading H7, 2026-10-06).**
78.106's valve opened onto nothing at the voice: only a court armed its
complexes' outflow (`court/glue.rs`), and the voice's complex — which
vents at `vent_vertex` — never did, so every torsion a voice released was
dropped. `Entity::vent_into_field` arms the complex where it takes it and
books each released motion through `mark_flux`, the door the court's
outflow takes (law 12): null flux into the voice's tubes, turns and
boosts onto its orbits — its metabolism moved by what it let go. Once per
perception, after the hold. Read: `harmonics: torsion vented into the
field`. Proved: `flux::a_tether_released_at_the_voice_vents_into_its_field`.
**H7b — HEARING WINDS TORSION ON THE PERCEPTION'S VERTEX.** A heard ring
is re-derived at each hearing and has no cell of its own; the cells of
the mind's complex stand between perceptions, a vertex IS a held
perception. So the torsion hearing winds is held where the ring was heard:
each crossing's versor (`1 + C + ½ C∧C`, 52.1), the later after the
earlier, composed by `hearing::wind`, carried on `Form.heard` (evidence,
never part of the address) from the percept to the hold, and wound onto
the perception's vertex (`Complex::wind`). When the mind lets the
perception go — `release_vertex`, or `vent_vertex` where it stood pinned —
the whole strain unwinds (`vent_wound`: its reverse flows out, 78.106)
into the field by H7a's door. A pure twist, all advance, winds nothing.
Read: `harmonics: perceptions wound with torsion`. Proved:
`hearing::hearing_winds_the_crossing_and_a_pure_twist_winds_nothing`,
`flux::a_perception_let_go_unwinds_the_torsion_hearing_wound`. H8 — the
released motor moving the rings of its commutant — next.

### 78.114 Read on the live wire: a93be300, epoch 141 (2026-10-06, the operator's word: "you need to rebuild the binaries and then recut to test all of this live on the wire … find the answers")

Built from the commit, recut, and confirmed on the console before any
reading (`harmonics` present, `constructions` fourteen names, `encounter`
0 — E2 off the reading path). Five minutes in, per voice:

1. **C1b holds.** Constructions refused for the stroke: 0; irrational 0,
   defective 0; constructed and ADJOINED AS LETTERS 3–12; null step 1–15
   the remaining refusal.
2. **H7a holds.** Torsion vented into the field 9–19: a mind's released
   strain moves its field.
3. **The residue the plan withheld.** `residues absent` 141–380 with all
   three pivot causes at 0: the absent residues were not the pivot's. RM1's
   callus (`callus.rs`) docketed its demand with `None` on 78.47(6)'s
   reasoning — its ray stood on a chain's leaf — and since 78.49 the plan is
   volatile and written nowhere, so the ray reached nothing. FIXED: the
   callus dockets its ray (`versor_of_word` of the ray it failed to spell).
4. **H6 meets nothing, and the corpus says why.** Rings in orbit 0, rays met
   0. Two causes. (i) A space was taken as a ray only when its WHOLE
   generator is null; a twist's is not, though its null mode is the light —
   FIXED: a space's rays are its null modes (`cech::flux::null_fluxes`, the
   field's own reader). (ii) Read by the probe on the stalled task's real
   perceptions: its rings are HYPERBOLIC (93 of 114 ring×space pairs), a
   few rosettes (elliptic, no finite order), none elliptic of finite
   order; its spaces hyperbolic or the identity, NO NULL MODE ANYWHERE. A
   seen edge's carriage `D·T` is one dilation about a shifted centre — the
   light from a star at a POINT; a null ray is the star at infinity, and the
   minds never see one. So H6 as contracted — a returning turn, met by a
   null ray — has nothing to hear in what these minds perceive. RULING
   ASKED: whether H6's propellant is every non-elliptic mode of the space
   (null: the star at infinity; hyperbolic: the star at a point), and
   whether its pull is every elliptic mode (a rosette pulls, and has no
   pitch: no return to measure the advance per).
5. **Nesting runs and rarely meets.** Rings heard 2–14 a voice, all outer:
   a perception's rings seldom share it with a larger ring.

Proved: `hearing::the_light_in_a_space_is_its_null_mode_and_a_dilated_carriage_has_none`.

**Read on e924ad5d (epoch 142).** THE HALT HOLDS: the a93be300 voices
stopped in 8, 2, 2, 16 and 1 s — none killed, none unsaved — and the recut
took 6 min where it took 13. The callus fix reached one voice (1e14:
residues absent 0, taken 8, constructed 8, adjoined 8); four still read
125–384 absent with every pivot cause 0 — a THIRD writer: the cell
reader's CS2 arm (`resonance.rs`, `note_cell`) dockets each incoherence
by its class with no residue, though the incoherence was read off the
ring's holonomy and dropped it. FIXED: `Incoherence.residue` is the ring's
holonomy, docketed with its demand. Proved:
`orbit_algebra::an_incoherence_is_held_by_its_ring_class_and_modes`.

**Read on d23c9258 (epoch 143).** The halt holds again (42, 3, 2, 16, 3 s).
RESIDUES ABSENT 0 ON EVERY VOICE (from 125–384); residues taken 72–328 a
voice — and 64–317 of them refused "null step": constructive
Cartan–Dieudonné stepped generator by generator, `u = x − e`, and in the
indefinite metric that step is null exactly where `x·e = e²` — at once for
a translation and a dilation about a point, the residues these minds carry.
FIXED by the theorem's own induction: the first `y` in coordinate order
(a generator, then a pair) whose `u = (R − 1)y` is non-null; every `u` in
`im(R − 1)`, whose complement is the fixed locus, so the certificate (the
hyperplanes meet exactly there) holds and the spelling is minimal
(Scherk); a totally isotropic remainder refused by name. A first cut —
`x + e` then `e` where the step is null — composed back but left the meet
off the locus, and the certificate refused it (`MeetOffLocus`): kept as
the reason the induction is the fix. Proved:
`constructors::a_translation_and_a_dilation_about_a_point_construct_and_compose_back`.
These residues hold no rotation: constructed, they are counted "no rotation
to adjoin" — row 8's evidence, now produced.

**Read on 00249e98 (epoch 144).** EVERY RESIDUE TAKEN CONSTRUCTS: taken =
constructed on all five voices (8–432), refusals 0; adjoined as letters
8–16; "no rotation to adjoin" 101–416 a voice — ~95% of what the minds
construct is a slide or a dilation about a point, which a Cl(4,0) letter
cannot hold. After 2 h 45 min: harmonics firing (seated whole 7–19,
perceptions balanced 36–75, rings in orbit 3–15, rays met 3–17, torsion
wound 3–9, vented 166–370); top rung 3 on four voices by citation.

### 78.115 Row 8: a mind's letter is a versor of Spin⁺(4,1)(ℚ) — the motion letter, and the demand answered by the mind's own letter (2026-10-07, under 78.68; the operator's word "proceed" on the batch "build row 8 to its end state in one stride")

78.68 ruled the letter: "its letters and its carriage are versors of
Spin⁺(4,1)(ℚ)". The tree held a mind's letter as a `Rotor` (Cl(4,0)) on a
vertex whose carriage was fixed at the identity, so a constructed residue
that turns nothing had no letter to become.

1. **The motion letter.** `sentence::motion_letter_form`: the residue
   factored `V = T·D·R` (`conformal_factors`), `R` on the derivation and
   `T·D` on the vertex's carriage — the one `letter_form`, of which the
   rotor letter (`generator_form`) is the carriage at the identity (law 12).
2. **Stood and offered (C4).** A constructed residue with no rotation stands
   as the mind's motion letter in its dialect and is spoken; the court
   verifies and hardens it or refuses by name — read live.
3. **The act: answered by its own letter.** The residues the mind made
   letters of are kept (`Contemplation::own_letters`, volatile with the
   dialect); the cell reader's CS2 arm answers an incoherence whose residue
   the mind holds — "answered where a held word spells it" (78.26(5)),
   which no Cl(4,0) word could do for a ring that never returns — instead
   of demanding it again.
4. **Folded, by reading the tree.** The pivot's candidates already carry the
   carriage moves (`Axis::Placement`'s slip, `Axis::Dilation`'s dilator) off
   the orbit's own class; the no-rotation demands were the cell reader's,
   answered by (3). C2's admission by Lie rank would gate on a held algebra
   that reaches all of so(4,1) on every voice — a reading, never a gate
   (78.104). The transcriber already carries the carriage whole into the
   connection: naming it through letters changes no act.

Reading: `constructions: adjoined as motion letters`, `demands answered by
an own letter`. Proved:
`taught::a_dilation_about_a_point_becomes_the_minds_motion_letter`.

### 78.142 The SDK rule seam (2026-10-08, the coordinator)

A client states a rule as STEPS and the seam (`client/sdk/rule.rs`) speaks
the sentence the court judges; no Element byte is forged.

- `Step::{Motion(Mv), Hue(map)}`; `Step::grid_motion(kind, in, out)` for
  the six grid symmetries is the centred rigid motion `T(c′)·L·T(−c)`, `L`
  a rotor `s + b·e1e2` for a turn or a vector for a mirror, `c` the grid's
  centre — `((R−1)/2, (C−1)/2)` in indices, read at `R/2, C/2` because the
  sensor carries a compartment by its CENTRE (site + ½, `carry_site_center`);
  exact. `None` where the shapes do not fit (90°, 270° and the transpose
  take `R×C` to `C×R`; 180° and the mirrors keep it).
- `sentence(steps, family, crystal)`: the steps COMPOSE to one action —
  `V = Vₙ⋯V₁`, `σ = σₙ∘…∘σ₁` — because the court asks EVERY word vertex to
  carry the input onto the output (`vertices_carry_frames`): a vertex per
  step would be asked to land a half-done motion on the final grid, and one
  class is one vertex anyway. The action is one Word vertex: derivation
  peeled by A22 (`peel_versor`), its linear part certified by
  `cga::construct` (T1), carriage `D·T`, one member per chart the family's
  input bodies stand on (`perceive_family`), each member holding NO frame
  (claimed whole — a rule names no training frame), the family ratio set;
  minted by `family::mint_sentence_why`.
- `would_judge(rule, family, resolve, crystal)`: the judge's own steps —
  the decoder (`Held::of`, `from_element_form`), then
  `verdict::blind_replay_at` on the pairs.
- `NoRule::{NotAVersor, NotAHueMap, Unconstructed, Unspelled, NoFamily,
  Mint, NotASentence, Refused}`, typed.

**The I-defect, reproduced and fixed.** A rule extracted from two training
pairs carries frame-axis overlaps (`family/percept.rs`, MT1/AT1 A2) whose
motors relate TRAINING pairs; judged on one withheld pair, `judge_solve`
refused it I (`motor_carries`: a frame the withheld family lacks) while the
client's replay on its train passed. `rule::crossing` drops every overlap
whose two ends stand at different frames, re-reads the orbits and re-mints
over the train (the mint re-checks every vertex there); `Entity::solve_task`
sends the crossed word, and a rule that cannot be re-minted without them is
refuted at home. The court is unchanged. *Owed, named:* a member that holds
only some training frames still names them; crossing leaves those members
as they are.

Proved (`client::sdk::tests::rule`, by the real `Kernel::judge_solve` on
pairs the rule never saw, `would_judge` agreeing on each): every site of a
`2×3` grid carried to its index image under all six symmetries; all six on
a square; 90° and the transpose `2×3 → 3×2`; a hue permutation; a 180° turn
then a recolour; refused by name: a mixed-parity step (`NotAVersor`), the
shapes that do not fit (`None`); the exact-comparison control — a dilation
composed with the 180° turn is a versor whose difference from the turn is
not zero, and is refused where the plain turn is solved; the extracted
two-pair rule refused I as it stands, and solved once crossed.

### 78.148 The mind's own call (2026-10-08)

**Measured.** xylarium's house lists each people's seat with fn 2 FOCUS, one
Count argument whose values mean something the mind is not told, and its
percepts carry no counsel. The seat mind of 78.146 (a) posted its TAKE once
and never again, so a house restart or a TAKEN refused stranded it; (b)
called only what a counsel named, so with no counsel `decide()` answered
NothingToCall forever and FOCUS was never called. The house's refusals are
typed on the wire: TAKEN and OUTCOME reasons 3 NOT THE HOLDER, 4 NO SUCH
FUNCTION, 5 BAD ARGUMENTS, 6 NOT APPLIED (`dialogue::seat::REASON_*`).

**Ruled.**

1. **The seat is held again when it is lost** (`sdk::seat::SeatHold`). A
   TAKEN refused, or a call refused NOT THE HOLDER, lets the take go and the
   next stroke takes again. A TAKE no TAKEN answers within a full APERTURE
   CYCLE of the mind's own strokes (`SeatHold::patience` =
   `physics::aperture_cycle`, the stroke count at which a contact's
   consecutive failures read as occlusion, 24.1) is posted again — a
   physics constant of the tree, never a clock and never a new number.
2. **The listing is perceived.** After the house answers the take held, and
   again when a call is refused NO SUCH FUNCTION, the arm reads the domain's
   listing (`Session::list_seats`, `sdk::seat::seat_fns`) and hands the mind
   its own seat's functions, id and argument tags: its REPERTOIRE
   (`SeatMind::perceive_listing`). A function listed again with the same
   tags keeps what was learned of it; NO SUCH FUNCTION takes it out until the
   listing is read again.
3. **Its own arguments, no script** (`seat_mind::own`). With no counsel, a
   listed function whose every argument is a Count or a Flag is the mind's
   to explore: one call per beat, in increasing order — its least arguments
   first, then each argument upward from one, the others at their least. The
   house's answer is the information: applied (or refused otherwise) the
   value is tried; refused BAD ARGUMENTS at `n`, that argument's domain is
   bounded below `n` and `n` is never called again; refused at its least, no
   value of the function applies. *(Amended 2026-10-09, the coordinator
   under the operator's authority, ruling 6, §0.1: the fixed
   increasing-argument exploration order (`own.rs:11-40`) is SCRIPT and is
   superseded — the mind orders its trials by its own held experience.
   Low priority, after the recut.)* A flag's domain is two values from the
   start. A call unanswered within its beat is let go untried and asked
   again. What each call did is held at the CALL's coordinates — the
   function and its arguments written inline (`own::call_key`), never the
   function alone — read as 78.146 reads it: every quantity's slide between
   the beat it was made and the first percept after its outcome, composed
   per quantity, the rate exact in ℚ.
4. **Where no worth is declared, the mind's own valuation: REACH**
   (`SeatMind::reach`, the one place it is stated; `Valuation::Reach`). A
   mind told nothing of what matters values what it can move: every record
   quantity one of its read calls moved is favoured the way its calls moved
   it; one its calls moved both ways is contested and favoured neither way.
   Once every value of every bounded domain is tried, the choice is 78.146's
   exact dominance read quantity by quantity over those records, never a
   summed score (rule 4): one call dominates — it, said so (`Why::Reached`);
   none does — of the undominated calls, the one read fewest times is read
   again, the earliest explored among equals (`Why::Unreached`), so a rate
   the world's drift may hide is sharpened rather than guessed. The beat
   line names the valuation and why: `seat beat call fn F followed-counsel
   false: valuation REACH (…): at [v] its read outcomes move …`, or, while
   exploring, the argument and value tried and the domains' bounds.
   *(Confirmed 2026-10-09, the coordinator, ruling 6, §0.1: REACH stands —
   the read effect is a measurement, not a weight.)*
5. **A versor's change is the versor that carries it** (the operator's
   field law). A percept argument may be a multivector (ARG tag 4): a full
   Cl(4,1) versor, odd ones — reflections, a bell's pulse — included. The
   change between beats is the exact carrier `c = v₊ ṽ / ⟨v ṽ⟩₀`, with
   `c v = v₊` checked coefficient for coefficient (`seat_mind::carrier`),
   or refused where either is no versor or the weight is zero (a refusal,
   never a zero change). A multivector series returns as an orbit when its
   carriers repeat; a steady carrier that moves the value is an orbit of
   period one, only the carrier one stands still; a series whose carrier was
   refused returns where its values repeat. Never flattened to numbers, no
   float anywhere.

Proved (`client::sdk::tests::seat_mind`): against a fake house on one
court's relay with one Count function whose values 0, 1, 2 each raise a
different record (2 raising all three) and 3 BAD ARGUMENTS, the mind reads
the listing, tries 0, 1, 2 and 3 in turn, records 3 as the bound and never
calls it again, then calls 2 on every beat under REACH, dominating 0 and 1
on records 10, 11 and 12 at rate 1 each; the listing is the repertoire (a
Bytes function not explored, a Flag's domain two from the start, NO SUCH
FUNCTION removing a function and the listing restoring it); the seat is
taken again after a TAKEN refused (the house reads the second take), held
on TAKEN ok, let go on NOT THE HOLDER, and re-posted after a full aperture
cycle unanswered; alternating reflections are an orbit of period 2 carried
by `e₂e₁` and `e₁e₂`, a translator's powers an orbit of period 1 carried by
the translator itself, a held versor fixed, a null point's carrier refused.

*Owed, named:* a Count domain the house never bounds is explored without
end (no constant is put in its place); a multivector record's carrier as a
call's effect (the effect is read on quantities only); the world's own drift
told apart from a call's effect (78.146's debt, still owed).

### 78.158a The solve paths wired (U1–U4), the home check on court words, sit-all

Built on track/ancients from docs/proposals/78.158 (units U1–U4), 78.157 §3a,
and the council's growth (barren-realms: one seat per world). WIRING ONLY:
no module and no type is new; reduction, never search.

1. **U1: every refusal on the solve paths is named.** The mint's refusal is
   read by ONE line maker, `family::mint_refusal_line(why, form)`:
   `mint refused (<arm>): <proof> [<kind>] demands [<kind>, …]`. The arm is
   `family::mint_arm` (carry, demands, citation, integrity or other). It
   moved from the client binary, and nothing in it changed. The demand
   kinds are the refused form's, in its own vertex order (`Demand::label`).
   The speech path's line keeps its prefix and gains the kinds:
   `form withheld: family F mint refused (…): … [N] demands [creation]`. The
   solve paths mint by `mint_sentence_why`, never by `mint_sentence`:
   - the task arm reads `family F solve unminted: the span is unmeasurable`
     or `family F solve unminted: mint refused (…) …`;
   - the seat arm reads `seat task F no call: no sentence minted: the span
     is unmeasurable` or `… no sentence minted: mint refused (…) …`.

   Proved: `family::tests::deflation::a_mint_refusal_names_its_arm_proof_kind_and_the_forms_demand_kinds`.
2. **U2: UNREAD at home is unjudged, never refuted.** `rule::would_judge`
   is now `would_judge_with` on the caller's stroke; `would_judge` keeps
   its signature, because the houses vendor the sdk. It answers
   `NoRule::Unread` for `REFUSE_UNREAD` (`NoRule::of_refusal`), told apart
   from the judge's `Refused`. Both solve paths submit on `Unread`:
   - `Entity::solve_task` goes on to `submit_solve_rule`;
   - the seat posts with the line `seat task F unjudged at home: <REFUSE_UNREAD>
     — posted for the court to resume`.

   The court resumes the replay on its cursor and answers `SOLVE_UNREAD`,
   which `solve_task` already does not remember as refused. The budget
   `to_completion` ignored (`blind.rs`, `let _ = budget;`) is now
   **honoured, never drained**. Each round is as wide as the caller's stroke
   where that grants more than a default stroke, and a default stroke
   otherwise. The caller's token is left as it stood.
   Proved:
   - `verdict::tests::resume::an_off_lock_reader_reads_each_round_on_the_callers_wider_stroke`;
   - `client::sdk::tests::rule::a_home_replay_no_stroke_reads_is_unjudged_and_the_court_rules_the_word`.
3. **U3: the extract memo holds no `None`.** `ExtractMemo` holds a rule
   only. A `None` leaves the slot as it was (the last family's rule stands),
   so the next asking reads the family again instead of being answered
   `None` by the memo until the root moved. Proved:
   `family::tests::budget::an_extraction_that_answers_none_is_never_held_for_its_family`.
   It uses a family's own `None`, two pairs in two frame ratios. 78.158's
   "a spent budget, then a full one, gives Some" has no fixture: a stroke
   spent inside the extraction keeps what it measured (L8w) and answers a
   partial rule, never `None`. A sweep of strokes 1 to 4096 found none.
4. **U4: the seat's solve is the mind's own call.** The relay's
   `reference` is kept, and the arm calls
   `with_seat_mind(.., |m| m.called(FN_SOLVE, args, reference))`. The
   outcome then reads against fn 1, never `fn None`. The task is
   `answered` only on an APPLIED outcome; until then it is in flight
   (`solving`, by reference and rule address). The in-flight set empties
   when the take is posted again.
   - A refused solve is remembered with its rule address and atlas root
     (`refused`). Its line is `seat task F solve refused (reason R):
     answered again when the rule's address changes`.
   - The task is not extracted again until the root moves, and not posted
     again while the address stands. Each case has its own `withheld again`
     no-call line. This is `Entity::solve_refused`'s rule on the task path.
   - Applied, the line is `seat task F solve applied: answered`.

   Proved: `client::sdk::tests::solve::a_seats_solve_outcome_reads_learned_against_fn_one`.
5. **78.157 §3a: the home check resolves court-stamped words only.**
   `Epigenome::court_word` is the held word where `admitted_at` files a
   court's stamp, and `None` for a dialect word or one learned with no
   stamp. Both home checks resolve through it (`solve_task` and the seat
   arm, which read `working_word` before).
   - `would_judge_with` asks every citation the rule carries
     (`receipts().cited_addresses()`: the cover's and the derivations') of
     the resolver BEFORE the replay. The replay itself defers a derivation
     citation its cover lacks, which would have read clean at home.
   - An unheld citation is its own outcome, `NoRule::CitesUnheld(address)`,
     never a pass. The task path keeps it home with
     `solve kept home: the rule cites A no court admitted (cites unheld)`,
     not counted as refuted. The seat's line is `seat task F no call: cites
     unheld: …`.

   Proved: `client::sdk::tests::rule::a_rule_citing_a_word_no_court_admitted_is_not_judged_clean_at_home`.
6. **Sit-all.** `--seat <domain>[/<sub>] all` sits every seat the domain's
   latest listing offers (`seat::listed_seats`). The seats sat are read
   against it (`seat::reseat`): a seat new to the listing is taken (a new
   arm), and a seat gone from it is let go (its arm dropped, its mail read
   past). The listing is read:
   - at the first stroke;
   - when an arm's listing read (after a take, or NO SUCH FUNCTION) names
     another epoch;
   - at every full aperture cycle of strokes (`SeatHold::patience`, a
     stroke count). A house that adds a world posts nothing else a seated
     mind reads.

   The arm's line is `seat all: listing epoch E offers N seats — sitting M,
   taken [..], let go [..]`. Explicit id lists are unchanged. Proved:
   `client::sdk::tests::solve::a_listing_of_n_seats_sits_n_and_a_relisting_adds_and_drops`.

Owed and named:
- The client's SolveTally has no CITES UNHELD counter. `observe/learning.rs`
  is another agent's this round, so the outcome is a line only.
- U2's end-to-end fixture (a family whose replay outruns one default
  stroke, UNREAD at home and ruled by `judge_solve`) does not exist as
  78.158 states it. A unit no default stroke reads also stalls the court's
  stroke, which is the same `spend_unit`. So `judge_solve` answers
  `Unread` there too, and the court rules it only on a stroke that moves.
  The honoured budget is what removes the false refusal at home.

### 78.163 SAY is HEARD, the thought is ACKNOWLEDGED — by the loop's holonomy (2026-10-11, the operator)

**The ruling.** "SAY must be HEARD and the mind must ACKNOWLEDGE the
THOUGHT — although, this needs to be based on physics."

**What is ruled.** The round trip said → heard is a LOOP. A word is said as
its whole motion `M` and hue map `σ`; the hearer derives `M′` and `σ′` from
the generators it cites. The loop's holonomy is `H = M′ M̃` (projectively
`M′ M⁻¹`, 1.5) on the motion and `σ′ ∘ σ⁻¹` on the hue map:

Two minds are two charts; carrying a thought across them is transport on a
curved space, and the holonomy is the content — what one mind's language
does to the other's thought (curvature is content):

1. CURVED — `H` split against the word's own mode blade (78.46) into DRAG
   (its plane, another rate) and LIFT (the tilt into another plane).
2. STILL — the hearer echoed: the loop carries nothing new. Agreement is
   never stillness (as first written here, which made the loop a restated
   "composes" bool — the Euclidean assumption, corrected the same day).
3. OPEN — nothing derived, `M` no versor, or `σ` no permutation (a merge):
   no loop closes.

HEARD carries whatever the hearer's generators reach (`M′`, `σ′`, its
citations), never only an exact re-derivation and never a verdict.
SAY → HEARD → ACK (the speaker's reading of every loop) → HOLD (the
hearer's own reading, by the one reader `chatter::loop_of`): each mind
verifies the other's by the algebra (`same_loop`), never a bool on trust.
Where both hold one loop on every word the thought is SHARED, and THE ACK
FEEDS THOUGHT (the operator, "ACK needs to FEED THOUGHT"): each mind's ear
holds every curved holonomy as a transport between the two
(`Presentation::hear_transport`), a generator of its dialect through which
later words from that perspective are deconstructed — the geometry the
conversation built is the vernacular. The thing said is agreed language
where some mind shared it; a letter waiting for that union goes to the
court. Readings that differ are refuted, and nothing is held. A thought
heard and a thing said stand one aperture cycle of the mind's strokes.

**Amended 2026-10-11 (the operator: "are you using the multi-axial pivot on
top of the physics"; MU-CLIENT I.3, II.3, II.4).** The loop above is a line —
one product read twice, beside the pivot. The hearing it rules is kept as the
antithesis of II.3, read in the hearer's three shells: the round trip's
content is the ECHO of each of the speaker's rings nested against the
hearer's own (78.112), and agreement is a CERTIFICATE the speaker replays
(M9), never a loop both minds hold. HEARD, ACK, HOLD, `loop_of`, `same_loop`
and `hear_transport` are superseded by chatter v3 (II.4) when it lands (IV
rows B and D); until then they stand in the tree as written here.

### 78.164 The board, not the captain: the mind is a field with boundaries (2026-10-11, the operator)

**The ruling.** "Dont you think the main loop for the client should be
cognition_engine and that chatter should be a subcomponent of a cognitive
mind"; "who is this harness for … you built this as a sovereign building a
captain — not a progenitor laying the board"; "abstract that out further,
not to put your cognition in there: but the cognition that serves the
mathematics of the substrate of the hypha"; "the fleet doesn't matter — this
is a prototyping sandbox"; "scope it - implement".

**Objective.** A game entity whose every action is an outflow of its mind's
field — no arm, schedule or harness between them.

**What is ruled.**
1. The mind is a FIELD; everything outside it is a BOUNDARY — the court, the
   corpus feed, other minds (chatter), a game seat — each with an INFLOW (a
   perturbation entering the field: a family, a listing's demands, a heard
   frame, a seat's mail) and an OUTFLOW (what the field closed that faces it:
   a sentence, a probe, a solve, SAY/ACK/HOLD, a seat call).
2. One TICK (`Entity::tick`, the SDK's, never the binary's) evolves the field
   by the substrate's laws: admit each boundary's inflow (paced by its own
   dwell, the court's surface), evolve what is held by the law its kind names
   (family, demand, loop, seat), relax inward (shed, expand, pivot, condense,
   refine, owed speech) where the reserve is short or nothing arrived, then
   book the tick's physics and close the orbit. The order is the physics'
   (admission, transport, closure, relaxation, booking), never a schedule.
3. The binary only opens the boundaries and hands the mind its ticks; it
   keeps no order of work, no arms, no counters and no console of its own —
   the mind's readings go to the hypha's console (`observe`).
4. A seat is the entity's body: its percepts are inflow, its calls outflow.

**Order.** (1) the field and `tick`, the arms deleted, every boundary in
the SDK; (2) the inflows as typed Cl(4,1) perturbations; (3) the seat as the
body in a game; (4) the entity acting live.

**Amended 2026-10-11 (the operator: "why would you make it bi-directional
instead of use the multi-axial pivot machinery that already exists";
"overlay this with how mycelium-server uses the multi-nested manifold for
harmonics → luminal → holonomic").** Item (1) landed at ebce99c3 as a fixed
order of boundaries — the line again. Items (2)–(4) are re-ruled by MU-CLIENT
I.3–I.5 and carried by IV: every inflow through one door and the three shells
the court already reads, the pivot on top choosing each move by the axis it
occupies, every outflow a shell-boundary event routed by its kind, and each
entity owning its field (A.9).

