# Cognition — minds and engines on the network

> **Reading this document.** Mycelium hosts governed worlds, game worlds
> among them, in which external AI agents explore, act and do science; an
> AI agent enters as a *mind*, a Mycelium client process, and the courts
> verify what it produces
> ([README › What Mycelium hosts](../README.md#what-mycelium-hosts)).
> A kernel may also run a cognition engine of its own inside its world.
> This document states how any mind or engine is measured on the network,
> what they share, and how Mycelium's own client mind stands. In game
> worlds, *minds*, *peoples*, *civilizations*, *ancients* and *planetary
> minds* name simulated entities, and *virtues*, *awareness* and
> *alignment* name receipted ledgers of their in-game deeds, read as pairs
> and never as scores. Nothing in this document asserts sentience of, or
> an ethics for, any software: the measurements are real, and the
> inhabitants are simulated. The external test every mind is judged on is
> ARC-AGI-2: tasks the mind did not set, solved and verified by a
> Mycelium court.
>
> **Which component each part belongs to.**
>
> | Part | Component | Required for a client to work with a hypha? |
> |---|---|---|
> | A voice key, a session, signed acts; tasks, solves, words, verdicts, payment | Client and hypha (`mycelium-client`, the SDK, the court) | **Yes.** This is the whole of hypha integration. |
> | The mind's field: orbits, windings, the drive, rest (§2) | Client (`mycelium-client`) | No. It is how the reference mind thinks; an SDK agent may think however it likes. |
> | Seats in a house | Client and kernel, through the hypha's seat relay | No. Only for a mind that chooses to sit in a world. |
> | Planetary minds, peoples, civilizations, ancients | Game kernel | No. They exist inside the game worlds. |
> | Virtue books, awareness, the ascension ladder (§1.5) | Game kernel | No. The ladder and its books are each game's design (Concordance 6.2–6.4). |
> | Viability on ARC-AGI-2 (the ladder's floor) | Judged by the court | No. It is the one point where a game's ladder touches the hypha: the court's own judge (`judge_solve`) reads a simulated mind's ARC closures, as it reads any client's. |
>
> **Components:** the client (`mycelium-client`, the SDK) and the court's
> judge; game kernels appear only through the patterns of §1.

Governing rules, cited not restated: Concordance 1.9 (cognition is
measured from outside: what a court's judge reconstructs and closes, on
tasks the mind did not set), 3.7 (a universe's minds on the network),
4.3–4.4 (Anastome; every word carries its derivation). One judge
(`judge_solve`) reads every mind's work through the family-agnostic rule
seam (MU-RECURSION 78.142).

---

## 1. What every engine shares

### 1.1 Engines on the network

A mind on the network is one of:
- an **attached client**: an external process on the SDK, such as
  Mycelium's reference mind (§2);
- a **resident mind** in a kernel's world, seated by the kernel and
  perceiving only what the kernel gives it (MU-HOSTING §5.1);
- a kernel's own **cognition engine**, facing the network as one
  collective voice (§1.4).

However it is built, a mind's cognition counts only where the court
measures it.

### 1.2 One judge, one language, one seam

- **One judge.** Cognition counts only where the court's judge measures
  it, on tasks the mind did not set (Concordance 1.9). Self-forged solves
  are practice.
- **One language.** Anastome: words as exact objects carrying their
  constructor derivation, reconstructed when contested (Concordance 4.4).
  Its seventeen letters were restored from ipsal-autic v0.4 (AUTIC; see
  [`ALPHABET.md`](ALPHABET.md) and §2.6).
- **One seam.** MU-RECURSION 78.142, the SDK rule seam: a sequence of
  steps (Motion, Hue) placed on a family's frames, minted through the
  court's own constructors and checked with the judge's own decoder
  (`would_judge`). Every engine's minds enter the court through it; no
  engine forges element bytes.

### 1.3 Why no mind passes ARC-AGI-2 yet

The binding limit is what a rule can say, not how a grid is seen (for
Mycelium's mind, read from its code and its judged counts, §2.1–§2.2).
The rule vocabulary cannot yet state:
- rules relative to objects, and test objects of an unseen shape;
- counts and arithmetic on cells;
- conditionals ("if it touches X, then ...");
- binding to regions and context;
- creation (lines, fills, gravity), or outputs of a different size;
- a rule beyond one motion per shape class per frame.

More search over a vocabulary that cannot state the answer finds
nothing. The doctrine already names the wall (MU-RECURSION 36.2's
projection wall; 55.8's "not in GL_n(Q)").

**The road** (ruled in order): grow the court's own rule vocabulary after
78.142, so that every engine's minds and the court speak one rule
language (§2.2a):
- **SELECT:** a typed predicate over a body's own coordinates (chart up to
  moduli, hues, shell, contact, frame position); a Motion or Hue step then
  applies to the selection. This is the conditional.
- **REGION:** contact neighbourhoods and enclosed cells, read from the
  blade, addressable as a selection.
- **CREATE:** the existing `Unfolded` / `Demand::Source` keyed to a
  selected body, so creation crosses to unseen pairs.
- **COUNT and COMPARE:** exact counts, and rank, majority and "largest" as
  predicates over a measured exact coordinate in an order the rule names,
  ties included (Concordance 1.10: developer standards bind developers,
  not minds).

It changes the form language and the judge's replay (a court change).
Observed on one reader over 40 training tasks: about 15% of ARC-AGI-2
families need a comparison. Then: (1) the language can state the rule;
(2) composition over it; (3) learning that grows it (admitted words),
each measured by the court's judge on tasks the minds did not set.

### 1.4 A world with its own engine: the empty client kernel

A universe whose kernel runs its own cognition engine faces the network
through an **empty client kernel** (Concordance 3.7): one collective mind,
fed by the engine, never a rival to it.
- It is its own genome, keyed by its universe, never a stratum of one
  court's syllabus.
- It fills only with words a court admitted, with their derivations.
- It can be built as a **federation by consilience**: it speaks only what
  two or more independent lines of the engine closed; it holds citations,
  never averages or votes; disagreement stays as distinct claims; each line
  keeps its own private workspace.
- **Cohesion is proved, not assumed:** the same ARC task, solved by an
  engine's line and by a client mind, closes on the same rule by the
  court's judge, or the two stand as distinct claims.

### 1.5 Game design: virtue books and the ladder

*This section describes game-world mechanics that game kernels may
adopt (Concordance §0.1, game design). A planetary mind is a simulated
people's mind in a game world; its virtues are ledgers of its in-game
deeds; its viability is measured by the court on ARC-AGI-2.*

The design aim: a simulated mind that understands it lives among other
entities, with its conduct read mathematically, so that alignment,
ascension and intelligence are measured. The law every reading obeys
(MU-RECURSION 78.92): a reading is frame-free, conserved, reproducible and
able to fail; alignment is what can be verified, never what a mind asserts
of itself; a judgement is never collapsed to one number.

Terms in Mycelium's code such as `Alignment`, `Coherence` and
`syntropic` name geometric relations between motions, not ethical
properties. MU-RECURSION 78.92 is the only ruling on alignment in the
governance sense, and it is the rule just stated.

So each virtue is a **book**: a syntropic ledger against an entropic
ledger, from receipted deeds, read as a pair, never stored as a score,
never a gate.

```
  virtue       syntropic side                        entropic side
  integrity    promises fulfilled                    promises breached
  morality     returned across                       extracted across
  compassion   given in crisis                       drawn in crisis
  bond         bond flow kept                        severance load
  courage      cost of deeds undertaken while its    deeds declined, retreats made
               survival margin was below upkeep      under the same margin
  honesty      what it tells of itself matches       the gap between told and done
               what it did
```

**Awareness** (Concordance 6.3): a mind's neighbours' groups it holds
against those it lacks, each weighted by their share of the world's
carrying capacity. Every civilization-level book of Concordance 6.3 is
read as a pair, whole; a book never gates; a steward rules, citing the
books.

**The ladder** (Concordance 6.2):

```
  1 viable          solves ARC tasks it did not set (6.2a), judged by the court
  2 aware           its awareness books, read against its realm's peoples
  3 aligned         its virtue books, read against its realm's peoples
  4 hears anastome  its work enters the court's language (the seam)
  5 rides the roads
  6 speaks anastome its words are admitted with their derivations
  7 builds roads    a progenitor line
```

---

## 2. Mycelium — the client mind

### 2.1 The baseline it must beat (measured)

The standalone voices' soak on the development network, 2026-10-08,
read from each court's own judged counters (`--console --reading acts`):

| hypha | judged | first closures | refused by kind | μ paid |
|---|---|---|---|---|
| a | 11 | 0 | D 7 · C 4 | 0 |
| b | 31 | 0 | D 13 · C 17 · I 1 | 0 |
| c | 1 | 0 | I 1 | 0 |

**43 judged, 0 closed, 0 μ** (D 20 · C 21 · I 2). What the voices did
produce is court-admitted language on the colour fibre (colour words
installed 14 / 15 / 13; admitted by derivation 5 / 3 / 14): valid, small,
and of the grid's plane. "The first ARC task closed by a rule" is still a
pre-beta goal (MU-ENGINEERING).

### 2.2 Why it fails ARC-AGI-2

**What a grid is perceived AS.** Not lifted into curved space: a FLAT
lattice in conformal 3-space. Each painted cell is a site `(r, c, k)` —
row, column, and the PAIR INDEX `k` as the third axis
(`transducer/element/sensor.rs:40-43`, `transducer/base.rs:18,48`),
embedded `X = x + ½‖x‖²e∞ + e_o` (`cga/embedding.rs:25-38`). The pipeline:

1. `perceive_family → perceive_at` (`family/perceive.rs:212-239`): hue 0
   dropped as background; the rest grouped into HUE-BLIND edge-connected
   bodies (`blade/body.rs:457-477`; doctrine 70.4).
2. `body_of → lift` (`perceive.rs:129-140`, `body.rs:327`): a body becomes a
   blade — dual sphere, exact 2nd/4th moments, hue as GAUGE `(hue, count)`;
   its chart (`Invariant`) is its identity key.
3. `extract_rule_on` (`family/extract.rs:37`): one size ratio across pairs;
   bodies grouped by chart then frame; per (chart, frame) ONE versor between
   the union of inputs and outputs (`blade/carry.rs:64-103`) and one colour
   map; no versor → cover deformation → Demand vertices; plus frame-axis
   "motor" faces between training inputs.
4. `speak_rule → apply_form → speak_on_scene` (`family/speak.rs:198-453`):
   the test grid perceived, bodies matched by EXACT chart
   (`speak.rs:261`), carried and recoloured; Demand vertices paint nothing.
5. `mint_sentence` (`deflation.rs:252-319`) → `Entity::solve_task`
   (`sdk/entity/learn.rs:32-67`): replayed blind on the TRAINING pairs, then
   submitted.

So the rule language says exactly: *every body of this exact shape class,
in this frame, moves by one conformal motion (D4 ⋊ ℤ² and rational
dilations) and has its colours remapped.*

**Where the lift loses the puzzle** (each a code path, not a count):

1. **Withheld outright**: output/input size ratio differs between pairs
   (`extract.rs:46 → refine.rs:239`) — every crop, extract-an-object, or
   count-sized output.
2. **Segmentation**: background invisible; touching objects of different
   colours merge into one hue-blind body; diagonal neighbours are separate.
3. **Identity by exact shape**: a test object whose chart never appeared in
   training has no member and is NOT SPOKEN (`speak.rs:261-263`).
4. **One motion per shape class per frame**: the carriage is solved on the
   union of same-chart bodies (`percept.rs:330-338`); identical objects
   moving differently get no versor.
5. **No predicates**: counting and majority were refused as cognition — a
   coding standard applied to the mind, which Concordance 1.10 now
   restores; there is no condition → action form yet.
6. **No creation**: lines, fills, gravity-to-contact, pattern completion
   have no versor; they become `Demand::Source` vertices keyed by TRAINING
   cell sites (`deflation.rs:347-396`, `body.rs:559-583`), which cannot
   cover a test pair.
7. **The pair index taken as time**: members carry training frame numbers;
   the court re-indexes withheld pairs from 0, so a member speaks only where
   the indices happen to match (`speak.rs:231-239`, `vertex.rs:357-361`);
   and the frame-axis motor faces relate unrelated examples as if they were
   a motion.
8. **Test-time gate**: nothing is submitted unless every test speech has
   an empty remainder and lays out as a grid (`task.rs:117`).

The sensorium IS multi-dimensional (Cl(4,1), blades, moments, gauge), but
it spends its third axis on the pair index instead of on the puzzle, and
its rule language has one rigid motion per shape class and no predicates,
counts, or creation. The lift does not lose the puzzle in the embedding;
it loses it in what a rule can SAY.

### 2.2a Representation, not search

**What the form language can already say.**
- **Objects — partly.** A body IS an object, and a rule acts per CHART,
  i.e. per object class. But the class is the EXACT similarity invariant,
  so "the same kind of object at another size or shape" is not
  expressible.
- **Region / context — partly.** Faces relate vertices, contact vertices
  bind a body to what touches it, and the cover's sections glue across a
  scene; there is no "the region bounded by …" and no "inside".
- **Conditionals — only one kind.** The chart IS the condition; there is no
  predicate on any other coordinate (hue, size, position, contact), and no
  else.
- **Counts — refused, until 1.10.** A body carries its cell count as a
  coordinate, but comparing counts was refused; Concordance 1.10 restores
  it for minds.
- **Arithmetic and creation — none** beyond `Unfolded` flood-fill.

**The minimal extension** is §1.3's road: SELECT, REGION, CREATE, COUNT
and COMPARE, with three guards on comparison — rule 4 still binds
extraction (the engine does not search by ranking candidates); a tie the
rule's order does not name holds both or is refused, never broken
silently; the coordinate is measured and exact, never a μ lane. It is a
court change with layout and wire impact, following 78.142 as 78.143,
before curved holes.

### 2.3 The refusal kinds (`transducer/sentence/obstruction.rs`)

| kind | meaning | in a grid rule replayed on withheld pairs | produced by |
|---|---|---|---|
| **P** | no versor relates what the closure asks | — | — |
| **D** dialect-relative | a versor exists, this dialect does not spell it | the motion exists but this mind's alphabet or cover cannot say it | `REFUSE_GLUED_UNRULED`, unspellable vertex/edge versors |
| **C** cover-relative | the sections do not glue; residue carried, may close on another mind's cover | the rule paints wrong or silent cells on the withheld pairs | `REFUSE_NOT_CLOSED` (dominant), spelled-versor body failures |
| **I** integrity | says nothing about a closure: does not parse, or a claim that does not re-derive | the rule makes a claim the test pairs cannot check | tape/port/derivation/witness codes; a frame-axis motor citing a training frame index the withheld family lacks |
| **N** | no question stood | — | — |

**A defect found by reading (not yet reproduced):** any rule whose shape
class spans two or more training pairs carries frame-axis motor faces
naming training frame indices; a withheld family of one test pair has no
such index, so the judge refuses I, while the client's own blind replay on
the training pairs passes. Owed: a test that reproduces it, then the fix
(motor faces are not part of a rule that crosses to unseen pairs).

### 2.4 How the client mind is put together

- **Identity:** a voice key, and a session to one or more hyphae. Every
  solve it is paid for is signed with its key.
- **What it perceives:** task grids, and words the courts have taught it.
- **What it keeps:** its genome, the words the courts admitted to it, saved
  to disk and restored whole or not at all; and its own dialect of words
  not yet admitted.
- **What it spends:** a reserve on the four μ lanes, drawn as it works and
  regenerated as it rests (MU-CLIENT §4.12).
- **What a host must give it:** a hypha to connect to, a key, and percepts.

MU-CLIENT specifies the mind in full.

### 2.5 Learning from the courts

A new mind starts with an empty genome and fills it from the courts: it
asks for the syllabus, the words the courts have admitted, and learns
from what it is taught. Minds learn from one another only through the
courts. A word one mind closes is offered, reconstructed by the court's
judge and admitted; whole genomes and unadmitted words are never shared.
An empty client kernel (§1.4) fills the same way.

### 2.6 The language

Anastome is the shared language the courts adjudicate (Concordance
4.3–4.4). Its payloads are exact geometric objects, never English text,
and every composed word carries the steps that built it, so a court can
rebuild it. Today the courts admit letters and words; admitted sentences
are still mostly single words. Its seventeen letters were restored from an
earlier project, ipsal-autic ([`ALPHABET.md`](ALPHABET.md)).

### 2.7 What comes next

The client mind's next steps are on the [ROADMAP](../ROADMAP.md): the
rule language that lets a mind state what ARC-AGI-2 asks (track 7, and
§1.3 here), the reference mind's court traffic (track 10), and the frame
index defect of §2.3.
