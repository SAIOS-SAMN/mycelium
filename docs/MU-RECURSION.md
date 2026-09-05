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

---

# Part II — Implementation

Bound to the tree at 3e0bb08. Every function named below exists at the
path given unless marked **new**. Signatures are the ones to add or change;
an implementor follows this part top to bottom and lands one section per
commit with its trace value in the commit message. Rust, exact ℚ, `Keyed`
tries, no `HashMap`, no scan of a carrier keyed elsewhere, every loop's
index from a fiber, peel, link, or `Budget` cursor.

## 16. Carriers

### 16.1 `/opt/mycelium-v0.1.0/src/transducer/sentence.rs`

Replace the struct `Germ` (line 583) with an enum and keep the current
fields as its `Body` variant:

```rust
pub enum Germ {
    Body {
        address: [u8; 32],
        versor: Versor,
        word: Word,
        chain: Vec<Cell>,
        source: Option<stalk::Invariant>,
        target: Option<stalk::Invariant>,
        kind: crate::transducer::peel::ShellKind,
        shift: Option<(BigRational, BigRational)>,
    },
    Cited {
        address: [u8; 32],          // address of a ruled sentence
        chart: stalk::Invariant,    // its chart, leading axis 2
        magnitude: [u64; 4],        // its lane vector at citation
        depth: u16,                 // its Element.depth
        placement: Word,            // relative word from its frame germ to this nerve's frame germ
    },
    Conjugate { by: u16, of: u16 },          // indices into this nerve's germs
    Previous { steps: u16, measure: Measure }, // this sentence's own earlier instance
}

pub enum Measure { Occupied, Remainder, Ring { anchor: u16 } }
```

Add on `Germ`:

```rust
pub fn address(&self) -> [u8; 32];        // Body/Cited: stored; Conjugate/Previous: blake3 of (kind, indices)
pub fn chart(&self) -> stalk::Invariant;  // Body: target.or(source); Cited: chart; Conjugate: chart of `of`; Previous: chart of the sentence
pub fn word_in(&self, nerve: &Nerve) -> Word; // Body: word; Cited: placement; Conjugate: nerve.germs[by].word ∘ nerve.germs[of].word ∘ inverse; Previous: identity
pub fn depth(&self) -> u16;               // Body 0; Cited: depth; Conjugate/Previous: 0
pub fn is_body(&self) -> bool;
```

`Face` (line 599) unchanged. `Nerve` (line 620) gains `pub depth: u16`
computed by `Nerve::depth_of(&self) -> u16` = 1 + max over `Cited` germs,
0 when none. `assemble(germs, remainder)` (line 939) builds faces for every
pair of germs through `face_word`, which now takes the nerve to resolve
`word_in`; it refuses (returns `Nerve` with `class` set) when any `Cited`
germ's depth is not below the computed depth.

### 16.2 `/opt/mycelium-v0.1.0/src/transducer/alphabet.rs`

`Word` (line 371) gains a contact condition carried beside the tape:

```rust
pub struct Contact { pub anchor: u16, pub ring: u8 }
impl Word {
    pub fn with_contact(self, c: Contact) -> Word;   // new
    pub fn contact(&self) -> Option<&Contact>;       // new
    pub fn conjugate_by(&self, by: &Word) -> Word;   // new: by ∘ self ∘ by⁻¹, normal form
}
```

Tape (`Word::to_bytes`, line 592): after the letters, one byte `0` (no
contact) or `1 ‖ anchor u16 ‖ ring u8`. `Word::from_bytes` (line 599)
reads it. `Word::key` (line 427) includes the contact bytes.

### 16.3 `/opt/mycelium-v0.1.0/src/transducer/element.rs`

`Element` (line 81) gains `depth: u16`, serialized in `GEN2` after `mag`
(`to_bytes`, line 1035; `from_bytes`, line 1098; a `GEN2` payload without
the depth byte pair reads as depth 0). `address()` (line 1026) does not
include depth; `canonical()` (line 948) does, so two sentences that differ
only by depth are distinct words.

New tag constant beside `SENTENCE_TAG` (line 1701): the sentence tape
version byte follows the tag; version `2` carries citations.

### 16.4 `/opt/mycelium-v0.1.0/src/transducer/cech.rs`

No new cell type. `Section` (line 156) is used for cited sentences as for
words; `Complex::set_section` (line 211) admits a cited sentence's section
under its address.

### 16.5 `/opt/mycelium-v0.1.0/src/server/cognition.rs`

New fields on `Kernel`:

```rust
replay_memo: Keyed<Keyed<(bool, u64)>>,   // address → family id → (replayed ok, TUniversal)
cited_by: Keyed<Keyed<()>>,               // cited address → set of citing ruled addresses
```

Both are epigenome: not in `snapshot` (line 4791), rebuilt from the ruled
cover at `restore` (line 4996) by walking `SENTENCE_TAG` v2 tapes.

## 17. Tape: `SENTENCE_TAG` version 2

Written by `as_element` (`sentence.rs` line 1362), read by `from_element`
(line 1707) and `chart_of` (line 1680). Layout after the connection tape
of germ 0 (unchanged):

```
version u8 (=2)
n_body u16, then per body germ: as version 1 (axis block, chain, hue map, kind, shift)
n_cited u16, then per citation:
    address 32 ‖ chart len16 + axes u64… ‖ magnitude 4×u64 ‖ depth u16 ‖ placement Word bytes
n_conj u16, then per conjugate: by u16 ‖ of u16
n_prev u16, then per unfolding: steps u16 ‖ measure u8 (0 occupied, 1 remainder, 2 ring) ‖ anchor u16 (ring only)
n_faces u16, then per face over the combined index (bodies, cited, conj, prev): i u16 ‖ j u16 ‖ g_ij Word bytes ‖ spelled u8
n_holonomy u16, then per cycle: Word bytes
depth u16
```

`from_element` reconstructs `Nerve.germs` in that order so face indices
match. `is_sentence` (line 1674) is unchanged. `replay_nerve` (line 1746)
runs on the reconstructed nerve.

## 18. Words: conjugation and contact

`/opt/mycelium-v0.1.0/src/transducer/alphabet.rs`

- `conjugate_by`: `by.compose(self).compose(&by.inverse()).normal_form()`.
- Orbit execution is not in `alphabet.rs`; it needs cells. Add to
  `/opt/mycelium-v0.1.0/src/transducer/family.rs`:

```rust
pub fn orbit(word: &Word, rho: &BigRational, body: &Body, anchor: &[(i64,i64)], frame: (usize,usize))
    -> OrbitEnd
pub enum OrbitEnd { Contact { cells: Vec<(i64,i64,u8)>, steps: u32 }, Torsion { steps: u32 }, LeftFrame { steps: u32 } }
```

Implementation: realise the word as a versor by
`element::conformal_of(&Spelling{word, rho}, &shift)` (element.rs line 239);
apply with `element::carry_cell` (line 346) to every cell of the body;
measure the ring distance to `anchor` with the ring arithmetic already in
`sentence.rs` (`contact`, about line 975, kept only for this purpose);
repeat while distance > `ring`; stop conditions as §5. The step bound is
`frame.0 + frame.1`. A word whose first application leaves the distance
unchanged returns `LeftFrame{steps:0}` and `speak_rule`'s caller refuses it
at mint.

## 19. Rule extraction with recursion

`/opt/mycelium-v0.1.0/src/transducer/family.rs`

`extract_rule(family)` (line 746) becomes:

```rust
pub fn extract_rule(family: &[(Grid, Grid)], held: &dyn CitationSource) -> Option<SentenceRule>
pub trait CitationSource {
    fn ruled_on(&self, chart: &stalk::Invariant) -> Vec<(Element)>;   // ruled sentences the mind holds on this chart
    fn speak_cited(&self, e: &Element, x: &Grid, placement: &Versor) -> Option<Speech>;
}
```

`Epigenome` implements `CitationSource` over its genome; the hypha's
`Kernel` implements it over the ruled cover for `rule_replays_family`.

Order inside `extract_rule`:

1. `pair_nerve` per pair as now (body germs, multiplicity by class).
2. **Citation pass.** For each pair nerve and each ruled sentence `s` from
   `held.ruled_on(chart)` for a chart present in the pair: speak `s` onto
   the pair's input at every placement where its frame germ's relative word
   to a body germ is constant across pairs; if the spoken cells match a
   subset of the pair's output on every pair, replace those body germs by
   one `Germ::Cited` with that placement. Constancy is the same
   `agree_up_to_gauge` test (line 1096) run on the citation's placement
   word.
3. **Conjugation pass.** For germs `(i, j)` whose relative word across
   pairs equals `word(i) ∘ word(j) ∘ word(i)⁻¹` on every pair, replace
   `j`'s word by `Germ::Conjugate{by:i, of:j}`.
4. **Unfolding pass.** If the pairs' outputs are reachable from their
   inputs by k applications of one nerve for different k, with one measure
   from `Measure` strictly monotone per step and the same stop criterion,
   the rule is a single nerve plus `Germ::Previous{steps:1, measure}`.
5. **Contact pass.** For a body germ whose per-pair translation differs but
   whose ring distance to one anchor germ at the output is constant, its
   word carries `Contact{anchor, ring}` and its translation leaves the
   tape.
6. `agree_up_to_gauge` on the resulting nerves, then `close`.

Each pass reads only the pair nerves and the citation source's fiber at the
chart; nothing enumerates the genome.

`speak_rule` (line 997): for `Cited`, `held.speak_cited(e, x, placement)`;
for `Conjugate`, realise `word_in`; for `Previous`, unfold to stop; for a
word with `Contact`, `orbit`. Remainder rules unchanged.

`rule_replays_family` (line 737): same passes on the hypha with the ruled
cover as `CitationSource`.

## 20. Court

`/opt/mycelium-v0.1.0/src/court/verdict.rs`

`blind_replay(spoken)` (line 31) gains, before the sentence arm, when
`spoken.is_sentence()` and the tape version is 2:

```rust
pub fn replay_recursive(spoken: &Element, resolve: &dyn Fn(&[u8;32]) -> Option<Element>) -> Option<&'static str>
```

Steps, each returning a reason on failure:
1. `"citation depth not below sentence depth"` when any `Cited.depth >= spoken.depth`.
2. `"citation not ruled"` when `resolve(address)` is `None`.
3. `"conjugate equals its base"` when a `Conjugate`'s derived word equals `word(of)`.
4. `"unfolding measure does not move"` when a `Previous` measure is not strictly monotone on the family.
5. `"orbit word has no orbit"` for a contact word whose first step leaves the distance unchanged.
6. Then `replay_nerve` on the resolved nerve, then descent on `spoken.magnitude()`.

`rule(...)` (line 222) is unchanged; it receives the same `Element`.

`/opt/mycelium-v0.1.0/src/server/cognition.rs`

- `resolve_cited(&self, addr) -> Option<Element>`: `word_at(addr)` (line
  1757) if ruled (`ruled_contains`), else `None`. Provisional and dialect
  addresses return `None` by construction.
- `glue_fetched` (line 2647): before replay of a fetched sentence with
  citations, for each citation not held call the existing digest/fetch path
  for that address's chart (`fetch_have`, line 3169, and the server's fetch
  through `serve_ruled`, line 3115); if the citation arrives, glue it first;
  if not within the stroke's `Budget`, return `Glue::Dialect` with reason
  `"citation pending"` and leave the citer on the docket.
- `replay_memo`: `perceive_speech` (line 1785) and `glue_fetched` consult
  `replay_memo[addr][family]` before `rule_replays_family`; write it after.
  `retire_on_t`-style shedding by the T lifecycle already applied to
  `heard`.
- `cited_by`: `home_install` and the `Glue::Admitted` arm insert
  `(cited, citer)` for every `Cited` germ of an installed sentence;
  `reduce_excluding` treats `cited_by.get(addr).is_some_and(nonempty)` as a
  use and refuses retirement.
- Re-docket on slash: in `apply_verdict` (line 2327) on `RULING_SLASHED` of
  an address with `cited_by` entries, each citer is moved from the ruled
  cover back to `provisional` with `provisional_tu` now; the next
  `court_stroke` (line 2394) replays it without the citation.

## 21. Comprehension

`/opt/mycelium-v0.1.0/src/client/sdk.rs`

`Epigenome::comprehend(&mut self, utterance)` (line 855), sentence arm:
1. `from_element` → nerve. For each `Cited` germ: `self.word_at(address)`
   (line 681). If absent, return `Comprehension::Withheld { missing: Vec<[u8;32]> }`
   (new variant on the enum at line 549); `learn_one` (line 977) then holds
   the sentence in `Dialect` with `withheld = true` and the missing list, and
   `pull_syllabus` (line 1427) requests those addresses by incidence on their
   charts.
2. If present but on a different address with the same chart, compute
   `g = cech::transition(held, cited)` (cech.rs line 20); if `g` is not
   identity, conjugate every face involving that germ by `rotor_of(g)`
   (line 40) and re-run `replay_nerve`; coherence → understood; failure →
   `Comprehension::Dialect(g)` and a class recorded on the citing sentence's
   chart through the contemplation.
3. Depth order: `pull_syllabus` sorts served words by `depth` ascending and
   learns in that order; `serve_ruled` (cognition.rs line 3115) serves in
   depth order and never serves a citer whose citations the voice's atlas
   lacks (the request carries the voice's charts; the server checks the
   citation charts against them).

## 22. Germination

`/opt/mycelium-v0.1.0/src/transducer/bubble.rs`

- `Contemplation::pivot_on(word, budget)` (line 712): when `word` is a
  ruled sentence and an idea's cover meets its chart, attempt a citation:
  the relative word from the idea's anchor to the sentence's frame germ
  over the idea's family pairs; constant → the idea gains a `Cited` germ
  and its remainder is re-measured minus what the citation speaks.
- `Bubble` (line 84) gains `cites: Keyed<()>` (addresses it cites) and
  `cited_by: Keyed<()>` (ideas citing it). `retire_on_t` (line 223) skips a
  bubble with non-empty `cited_by`.
- Bundles: `Contemplation::bundle(&mut self, class: &Invariant, budget)` **new**:
  ideas on one class whose relative words are constant form one depth-1
  nerve of `Cited` germs; it is held as an idea and spoken by
  `Entity::speak` like any sentence.

## 23. Speech and wire

`/opt/mycelium-v0.1.0/src/anastome/dialogue.rs`

- `ACT_SPEECH` (line 91) payload unchanged; a sentence with citations is
  one element in `sentences`.
- Reply codes (line 127): add `SPEECH_WITHHELD: u8 = 3`; `encode_speech_reply`
  gains, after the codes, `n_missing u16 ‖ address 32…` so the sender learns
  which citations to speak or wait for. `decode_speech_reply` returns
  `(Vec<u8>, Vec<[u8;32]>)`.
- Digests: `StratumDigest` (sheaf.rs line 21) gains `pub depth_max: u16`;
  `sheaf::digests` (line 44) fills it from the stratum's members;
  `encode_digests_reply`/`decode_digests_reply` (dialogue.rs lines 406, 417)
  carry it. `encode_cover_digest` (line 451) carries the hypha's maximum
  ruled depth after `room`.

`/opt/mycelium-v0.1.0/src/bin/mycelium-server.rs`

- In `judge`'s `ACT_SPEECH` handler: `perceive_speech` returns per-sentence
  codes; when a code is `SPEECH_WITHHELD`, collect the missing addresses
  from the kernel (`Kernel::missing_citations(&Element) -> Vec<[u8;32]>`
  **new**) into the reply.
- `admit_peer_digest` (line 385) records the peer's `depth_max`;
  `nerve::home` (nerve.rs line 788) gains a parameter `depth: u16` and
  excludes hyphae whose `depth_max + 1 < depth` from the fiber.

## 24. Economics

`/opt/mycelium-v0.1.0/src/economy.rs`

- `settle_recursive(&mut self, family_escrow: &[u8;32], shares: &[( [u8;32], [u64;4] )], anchor) -> [u64;4]`
  **new**: draws the family's escrow proportionally to each speaker's
  verified deflation vector and mints each share to its speaker through
  `mint_claim` (line 289). Called from `Kernel::pay_solve` (cognition.rs
  line 3220) and from `home_install` when the installed sentence has
  citations; the shares are the citer's own deflation and each cited
  sentence's deflation on this family as measured at replay
  (`rule_replays_family` returns the per-germ deflation vector: change its
  return type to `Option<Vec<([u8;32],[u64;4])>>`).
- Room from cited use: `bond_of` (line 175) is unchanged; add
  `use_credit(&self, who) -> [u64;4]` **new**, the sum of deflation vectors
  of the speaker's ruled sentences that are cited by other ruled sentences,
  read from `Kernel::cited_by`; `lane_room` (cognition.rs line 3455) adds
  `use_credit` to `bond_of` before `lane_cover_room`.

## 25. Retirement

`/opt/mycelium-v0.1.0/src/server/cognition.rs`

- `reduce_excluding`: a citation is a use (§20).
- `shorter_citer(&self, addr) -> Option<[u8;32]>` **new**: among ruled
  sentences on the same classes (`ruled.fiber(chart)` per class), one whose
  normal-form tape length is strictly less and whose `cited_by` covers the
  same families. When found, `relocate_cavity(addr)` with the address kept
  in `ruled_addrs`; `resolve_cited` reads the cavity (`cavity_book`) as a
  valid citation target; `serve_ruled` never serves it.
- A sentence with no family of its own (`listing_task` absent) and empty
  `cited_by` after its last citer retires is relocated the same way on the
  next `court_stroke`.

## 26. Persistence

- `GEN2` (`element.rs` `to_bytes`/`from_bytes`): depth after `mag`;
  sentence tape v2 inside the description bytes. A v1 tape reads as depth 0
  with no citations.
- `MYC2` (`cognition.rs` `snapshot`/`restore`): ruled sentences as tapes;
  `cited_by` and `replay_memo` rebuilt at restore, never written.
- Genome (`sdk.rs` `crystallize`/`uncrystallize`, lines 1011/1024): same
  `GEN2` elements; dialect and withheld sentences are never written.

## 27. Tests (single foreground `cargo test --release`; N vs 4N counters apply)

1. `sentence::tests::cited_germ_round_trips_tape_v2`: a nerve with one
   `Cited`, one `Conjugate`, one `Previous`, and a contact word survives
   `as_element`/`from_element` with faces intact.
2. `sentence::tests::depth_is_one_plus_max_citation` and refusal of a
   citation with equal depth.
3. `alphabet::tests::conjugate_by_is_normal_form` and `contact_in_key`.
4. `family::tests::orbit_stops_at_contact_and_refuses_no_orbit`.
5. `family::tests::extract_rule_cites_a_held_sentence`: two families, the
   second contains the first's scene; the rule is depth 1 with one `Cited`
   germ and fewer body germs than the flat rule.
6. `family::tests::unfolding_grow_to_frame`: three pairs at different steps
   give one nerve with `Previous{Occupied}` and no step count on the tape.
7. `verdict::tests::recursive_replay_refuses_unruled_citation`.
8. `cognition::tests::witness_fetches_citation_before_install` and
   `replay_memo_reused_on_second_citer`.
9. `cognition::tests::slash_of_cited_redockets_citers`.
10. `sdk::tests::comprehend_regauges_dialect_citation` and
    `withheld_on_missing_citation_then_learns_in_depth_order`.
11. `economy::tests::settle_recursive_pays_citer_and_cited_by_deflation`.
12. `cognition::tests::shorter_citer_retires_longer_to_cavity_and_resolves`.

## 28. Landing order

1. §16.1–16.3 carriers and tape v2 with tests 1–3 (no behaviour change
   until §19).
2. §18 orbit and conjugation with test 4.
3. §19 extraction passes with tests 5–6; `speak_rule` for all kinds.
4. §20 court with tests 7–9; §26 persistence.
5. §21 comprehension and §23 wire with test 10.
6. §22 germination and bundles.
7. §24 economics with test 11; §25 retirement with test 12.
8. Cold-boot traces §15, one per commit message.
