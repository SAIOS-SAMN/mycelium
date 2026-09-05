# Recursion — the language applied to itself

Status: specification. Nothing in this document is landed. It depends on
the current relay (rules by class multiplicity, the speech act, derived
fibre at load) and extends the carriers in `MU-ENGINEERING.md` §2–§4.
Companion documents: `ALPHABET.md`, `MU-ECONOMY.md`, `XSEPTUM.md`.

## 0. Scope

Recursion here means five things, each a mechanism, each with its own
carrier, replay rule, bound, and trace:

1. **Word-level**: a word applied to a word. Conjugation of one tape by
   another as a first-class germ, so a rule can say "do to B what A did
   to itself".
2. **Sentence-level**: a ruled sentence admissible as a germ, so a rule's
   tape names another sentence's address as a vertex and the nerve is a
   nerve of nerves.
3. **Unfolding**: a sentence whose output on one step is its input on the
   next, cited by depth index, with a well-founded measure that stops it.
4. **Iteration by contact**: a word applied to its own image until a named
   face reaches ring zero.
5. **Distributed replay**: the court replays a recursive sentence by
   replaying its citations from whichever hypha's ruled cover holds them,
   memoized by (address, family), fetched through the digest path.

Everything else in this document, comprehension across gauges, economics,
retirement, syllabus order, persistence, wire, bounds, is what those five
require to be sound on a distributed mesh.

## 1. Grades, extended

| grade | before | now |
|---|---|---|
| letter | one of seventeen rotors | unchanged, never new |
| word | tape, dilation, placement, gauge | plus optional contact condition (§5) and conjugation form (§2) |
| sentence | words that cohere over a nerve of bodies | words, conjugations, cited sentences, and unfoldings that cohere over a nerve whose vertices are bodies or sentences (§3, §4) |
| speech | transfer of cohering sentences | unchanged; a recursive sentence travels as one Element carrying addresses, never copies |
| language | two-organ replay, taught | unchanged; citations must already be language on both organs |

## 2. Word-level recursion: conjugation as a germ

### 2.1 What it says

The relative word between two bodies is already a word applied to a frame.
Word-level recursion adds the transport of transports: given body A with
word `a` and body B with word `b`, the germ `Conjugate { by: A, of: B }`
speaks `a · b · a⁻¹` as B's word, "B moves the way A would move it". This is
the letter-level form of "do to B what A did to itself".

### 2.2 Carrier

`Germ::Conjugate { by: u16, of: u16 }` referring to two other germs of the
same nerve. Its word is derived, never stored: `word(by) · word(of) ·
word(by)⁻¹`, in normal form. Its chart is the chart of `of`. Its magnitude
is what it closes on the family beyond what `of` alone closes.

### 2.3 Replay

`blind_replay` derives the conjugate word from the two cited germs' tapes
and checks it against the family exactly as any word; a conjugate whose
derived word equals `word(of)` is redundant and refused.

### 2.4 Bound

A conjugation is one geometric product and one inverse, both exact; no
iteration. The order of the conjugate word is decided by `Word::order` on
the derived tape.

## 3. Sentence-level recursion: cited germs

### 3.1 Carrier

`Germ` gains the kind `Cited { address, chart, magnitude, depth }`: a ruled
sentence named by address. The chart is the cited sentence's chart (leading
axis 2), the magnitude its lane vector at citation time, the depth its
depth (§3.3). A face between a body germ and a cited germ, or between two
cited germs, is a relative word exactly as between bodies; the cited
sentence's frame germ is its placement handle.

### 3.2 Address, never copy

A citation carries only the address. The cited sentence's tape is resolved
from the genome, the ruled cover, or a peer's ruled cover at comprehension
and replay time. Speech that carries a copy of a cited sentence is refused
as malformed.

### 3.3 Depth and well-foundedness

`Element.depth: u16`. A body-only sentence has depth 0; a sentence whose
citations have maximum depth d has depth d+1. A citation must satisfy
`cited.depth < self.depth`, checked at mint, at speech, and at replay.
Addresses are fixed at mint, so a sentence cannot cite an address that did
not exist when it was minted; self-citation and citation cycles cannot be
written.

### 3.4 Magnitude and descent

A recursive sentence's magnitude on each lane is its own measured deflation
on the family: the cells it closes beyond what its citations close when
spoken in its placements. Descent is checked on that vector. A sentence
whose citations already close the family with no remainder has zero
deflation of its own and is refused as redundant; the citation is the word.

### 3.5 Tape

`SENTENCE_TAG` version 2: the body germ list as now, then
`n_cited u16` and per citation `address 32 ‖ chart len16+axes ‖ magnitude
4×u64 ‖ depth u16`, then `n_conjugate u16` with `(by u16, of u16)` pairs,
then `n_unfold u16` (§4), then faces over the combined vertex list, bodies
first, citations, conjugates, unfoldings after, then holonomies, hue maps,
and contact conditions per word (§5).

## 4. Unfolding: a rule that feeds itself

### 4.1 What it says

Grow, tile, spiral, cascade: the output of step k is the input of step k+1
under the same rule. The rule is one sentence; the family's pairs are
snapshots of its unfolding at different k, or one pair whose output is the
fixpoint of the unfolding from its input.

### 4.2 Carrier

`Germ::Previous { steps: u16 }` cites the same sentence's own instance
`steps` unfoldings earlier, by depth index rather than by address, since
the instance has no address of its own. A sentence containing `Previous`
is an unfolding; its `depth` is its citation depth as in §3.3, and its
unfolding count ℓ is a coordinate of the instance, never on the tape.

### 4.3 Measure and stop

Every unfolding sentence names a strictly monotone measure on the frame
that each step must move: occupied cells strictly increasing, or remainder
strictly decreasing, or a named face's ring strictly decreasing. The
unfolding stops when the measure cannot move, when the image equals the
input (fixpoint), or when the image leaves the frame (remainder). The
measure is a coordinate the tape names; its value per step is not.

### 4.4 Replay

Replay executes the unfolding from each pair's input until stop and checks
the stop state against the output. Every pair must stop by the same
criterion; the number of steps may differ per pair. A pair that does not
stop within the frame's cell count is a remainder, never a timeout.

## 5. Iteration by contact

### 5.1 Orbit word

A word may carry `contact: Option<(anchor: u16, ring: u8)>`. Its action:
apply the word; measure the ring distance from the image to the anchor
germ's support; if greater than `ring`, apply again; stop when the distance
equals `ring` or the image leaves the frame. The number of applications ℓ
is a coordinate of the instance.

### 5.2 Closure and refusal

An orbit word closes only if the image reaches contact within the frame.
A word that does not change the ring distance is refused at mint as having
no orbit. Leaving the frame is a remainder.

### 5.3 Contact and torsion

An orbit that returns to its start body within the frame is a torsion
cycle `(σ, ℓ)` as now. Contact and torsion are the two ways an orbit ends:
against another body, or against itself. A word may carry both a contact
and a torsion; torsion wins when the orbit closes before contact.

### 5.4 Replay

Execute the orbit on each pair's input; check the stopping ring equals the
tape's `ring` on every pair and the image equals the output body. Orbit
length is bounded by the frame's diagonal in cells.

## 6. Germination through recursion

### 6.1 Ideas that cite

An idea may germinate by attaching a ruled sentence as a `Cited` vertex
when the sentence's chart meets the idea's cover and the relative word from
the idea's anchor to the sentence's frame germ is constant across the
family's pairs. The idea inherits the citation's closing power and
germinates only the remainder.

### 6.2 Ideas that conjugate and unfold

An idea whose remainder is closed by `word(by) · word(of) · word(by)⁻¹` for
two of its germs attaches a `Conjugate`; an idea whose pairs are snapshots
of one rule at different steps attaches `Previous` with the measure that
moved between them.

### 6.3 Bundles

A bundle across families (`MU-ENGINEERING.md` §7) is a depth-1 sentence
whose cited germs are the families' sentences and whose faces are the
constant relative words between them. A bundle's deflation is the work of
relating what its citations already did.

### 6.4 Superposition

Two cited sentences describing one class with a non-identity transition
between them are a 1-cell with that transition, as any two ideas: a fill
re-gauges one into the other, a class records the dialect. Recursion adds
no cell type.

### 6.5 Shedding

A cited idea does not shed while an idea that cites it is alive in the
contemplation; when the last citer sheds or forms, the cited idea's shed
clock resumes. Formed sentences are not ideas and are not subject to shed.

## 7. Comprehension of recursive speech

### 7.1 Resolution

A mind hearing a recursive sentence resolves each citation by address: in
its genome, or by asking the syllabus for that address by incidence. A
sentence with a dangling citation is held in dialect as withheld until the
citation arrives; it is never learned dangling.

### 7.2 Across gauges

A mind may hold the cited class as a different ruled sentence, a dialect
variant with a non-identity transition `g` to the cited address. It
comprehends the citing sentence by re-gauging: every face to the cited germ
is conjugated by `g`, and the sentence is understood iff the re-gauged
faces still cohere. If they do not, the pair (cited address, held variant)
is a class on the citing sentence's chart, recorded, and the sentence is
withheld. Two minds with equivalent but non-identical citations therefore
hear one sentence.

### 7.3 Depth order

A mind cannot comprehend depth d before it holds every citation at depth
below d. The syllabus teaches in depth order: a hypha never serves a citing
sentence to a voice whose atlas lacks a citation, and serves the citation
first.

## 8. Speaking a recursive sentence onto a test

Body germs as now; each `Cited` germ by speaking the cited sentence onto
the test at the citing face's placement; each `Conjugate` by its derived
word; each `Previous` by unfolding to stop; orbit words to contact.
Unreached cells are the remainder; an empty remainder on every test is the
solve, paid to the speaker of the citing sentence for its own deflation and
to the speakers of the citations for theirs (§10).

## 9. Court

### 9.1 Replay of a recursive sentence

1. Depth: every `Cited.depth < self.depth`; `Previous` and `Conjugate`
   refer within the sentence.
2. Existence: every cited address is on this hypha's ruled cover or on a
   peer's signed ruled root; a citation of a dialect-only or provisional
   address is refused, "citation not ruled".
3. Fetch: citations not held are fetched from the peer whose ruled root
   names them, by the existing digest and fetch path, replayed on arrival.
4. Memo: replay of a citation on this family is memoized by
   (address, family id) with its TUniversal stamp; a citer on the same
   family reuses it. The memo sheds by T-room like every hot carrier.
5. Replay: citations first (shallower), then conjugates, unfoldings, orbit
   words, faces, holonomies, hue maps against the family from the registry.
6. Descent: the sentence's own deflation is nonzero and strictly descends.

### 9.2 Two organs

A recursive sentence installs as any sentence: sectioned at home, replayed
at a witness, installed on both, minted once at the home. The witness's
replay includes steps 3 and 4, so a citation the witness lacks is fetched
and replayed before the citer installs.

### 9.3 A citation slashed later

If a ruled sentence is later slashed, every ruled sentence citing it is
re-docketed, not slashed: each is replayed without the citation, installs
again if it still closes its family (its deflation now larger), or is
withheld to dialect with its escrow returned. A slash never cascades by
itself; it re-opens the docket.

### 9.4 Dialect citations across hyphae

Two hyphae may hold different ruled sentences on one class. A citer names
one address; a hypha holding the other fetches the cited one and glues it
as dialect with its transition, exactly as any two words on a chart. The
citer then replays against the address it named. No citation is ever by
class.

## 10. Economics

- **Mint is deflation on this family.** The citing sentence's speaker is
  paid the deflation the relation added. Each cited sentence's speaker is
  paid the deflation their sentence performed on this new family when
  spoken in the citer's placement, re-verified at replay. That is the
  citation payment: real work on a new family, not a royalty on the past.
- **No mint for restatement.** A citer with zero own deflation is refused;
  a citation that adds no deflation on the family pays nothing.
- **Room follows depth.** A speaker's cited sentences count toward its room
  on the lanes they moved, as bond earned by use rather than posted; room
  is not currency and does not transfer.
- **Escrow on recursive families.** A family's escrow is drawn by all
  speakers in proportion to their verified deflation on it, citer and cited
  alike, in one settlement.

## 11. Retirement and shrinking

- **Citation is a use.** `reduce_excluding` treats a ruled citation as a
  use; a cited sentence cannot retire while a ruled citer exists.
- **Shorter citer retires the longer.** When a ruled sentence B cites the
  same classes as ruled sentence A and B's normal-form tape is a geodesic
  strictly shorter than A's on the same family set, A is derivable from B
  and retires to the cavity cover with its address kept; citers of A
  resolve A through the cavity by address, and the syllabus teaches B.
- **Cavity resolution.** A cited address in the cavity is resolvable for
  replay and comprehension; it is not taught and not quoted.
- **Cited-only survival.** A sentence with no family of its own that is
  cited by a ruled sentence survives on the ruled cover as a citation
  target; when its last citer retires, it retires with it.

## 12. Persistence and wire

- Genome (`GEN2`) and ledger (`MYC2`) store recursive sentences as tapes
  with citation addresses; no cited tape is duplicated.
- The ruled cover's digest strata gain a depth axis so a peer can fetch
  "everything at depth d on chart c" and syllabus can serve in depth order.
- `ACT_SPEECH` carries recursive sentences unchanged; the receiver resolves
  citations. A speech whose citations are not all ruled on the receiver is
  answered `Withheld(missing addresses)` and the sender learns which
  citations to speak or wait for.
- Cover digests include the maximum depth held, so `home` can place a
  depth-d demand on a hypha that holds depth d−1.

## 13. Bounds

- Depth is bounded by ruled generations: depth d needs d ruled generations
  below it. No constant.
- Replay of a recursive sentence costs its own faces plus zero for memoized
  citations on this family; the memo is one entry per (address, family).
- Orbit and unfolding lengths are bounded by the frame; the criterion is
  contact, fixpoint, measure exhaustion, or leaving the frame.
- Citation fetch runs under the stroke Budget; a citation not reachable in
  the stroke leaves the sentence withheld, never refused.
- Cited magnitudes are read from the ruled cover, never recomputed.
- The mind's contemplation depth equals the deepest idea citing an idea;
  shedding applies from the leaves.

## 14. Files

- `/opt/mycelium-v0.1.0/src/transducer/sentence.rs`: `Germ::{Cited,
  Conjugate, Previous}`, depth, tape v2, faces to cited frame germs,
  unfolding measure, `speak_rule` instantiation of all four kinds.
- `/opt/mycelium-v0.1.0/src/transducer/alphabet.rs`: `Word::contact`,
  orbit execution, conjugation form, refusal of a word with no orbit.
- `/opt/mycelium-v0.1.0/src/transducer/element.rs`: `depth`, orbit and
  conjugate words on the tape, magnitude as own deflation.
- `/opt/mycelium-v0.1.0/src/court/verdict.rs`: §9.1 steps; re-docket on
  citation slash.
- `/opt/mycelium-v0.1.0/src/server/cognition.rs`: citation fetch through
  the digest path, replay memo by (address, family), citation as a use in
  `reduce_excluding`, shorter-citer retirement, depth axis on digests,
  proportional escrow draw.
- `/opt/mycelium-v0.1.0/src/client/sdk.rs`,
  `/opt/mycelium-v0.1.0/src/transducer/bubble.rs`: germination by citation,
  conjugation, and unfolding; re-gauge at comprehension; withheld on
  dangling citation; cited ideas do not shed while cited; bundles.
- `/opt/mycelium-v0.1.0/src/transducer/family.rs`: `extract_rule` may
  return rules with cited, conjugate, and unfolding germs when their
  relative words or measures are constant across pairs.
- `/opt/mycelium-v0.1.0/src/anastome/dialogue.rs`: `SENTENCE_TAG` v2;
  `Withheld(missing)` reply on speech; depth in cover digests.
- `/opt/mycelium-v0.1.0/src/economy.rs`: proportional settlement across
  citer and cited speakers; room from cited use.

## 15. Traces

1. A family whose pairs each contain a previously ruled sentence as a
   sub-scene yields a depth-1 rule with that sentence as a cited germ and
   fewer body germs than a flat rule.
2. A "move B the way A moved" family yields a `Conjugate` germ whose
   derived word equals `a·b·a⁻¹` and is refused when it equals `b`.
3. A "grow until the frame" family yields an unfolding with occupied cells
   as the measure, stopping at the frame on every pair with different step
   counts and no step count on the tape.
4. A "move until contact" family closes on every pair with the same `ring`
   and different ℓ per pair.
5. A witness lacking a citation fetches it, replays it, memoizes it, and
   installs the citer; a second citer on the same family reuses the memo.
6. A sentence citing a dialect-only address is refused "citation not
   ruled" and held.
7. A mind holding a dialect variant of a citation comprehends the citer by
   re-gauging; a variant that does not re-gauge records a class.
8. A syllabus never serves depth d to a voice lacking depth d−1.
9. Slashing a cited sentence re-dockets its citers; one re-installs with
   larger deflation, one is withheld with escrow returned.
10. A shorter citer retires the longer sentence to the cavity; its citers
    still replay through the cavity address.
11. Settlement of a recursive family pays the citer its own deflation and
    the cited speaker the deflation its sentence performed on that family.
12. Replay cost of a depth-2 sentence on a family already memoized for its
    citations equals its own faces only.
