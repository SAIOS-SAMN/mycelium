# Enumeration audit

A loop is legitimate only when its index is a keyed read, a peel, a link,
or a spend_unit budget from a keyed cursor. Fixed physical sizes (4 lanes,
17 letters, 16 blade masks, 6 planes) are allowed. Everything else was
an enumeration.

Work counters (test builds): `Keyed::get`, `Keyed::insert`, `Rotor::geo`,
`cech::extend`. Snapshot is their sum.

## Counter results

Operation | N=32 | N=128 | Δ
---|---|---|---
one fetched vertex (`coboundary_on_link` + fill of its link) | 55 | 55 | 0
one held thought (`Contemplation::hold`) | 73 | 73 | 0
one expansion stroke (budget 4) | 13 | 13 | 0
one quote (cover peel of the held bubble) | 14 | 14 | 0

A second run at 4N differs by zero. The counter does not grow with N.
Link-bounded work at multiplicity 3 (a triangle) is the 55-count fetch.

## Dual carriers deleted

- `Complex` degree scan (`degrees.iter().position(|l| l.contains(key))`).
  The fact is `by_key: Keyed<u8>` written in `remember`. Fill, class, and
  drop read it.
- `each_combination` over a chart fiber. A (k+1)-cell is the intersection
  of the link of an existing k-cell with the link of one neighbour.
- `Cover::all_charts` / `field.entries()` as the source of
  `own_cover_charts`. The hypha's cover is admitted in `write_field`;
  the digest reads `own_cover`.
- `provisional.entries()` as the source of peer confirmation. The index
  is the peer digest / have-list; membership is `provisional.contains`.
- `field.entries()` as the source of `drain_unreachable`. The docket is
  `open`, budgeted from a keyed cursor.
- `open.entries().take(cap)` in `court_stroke`. Replaced by
  `for_each_from` on `open`.
- `atlas.strata()` as the quote fallback. Quote is the peel of each
  held bubble's cover; an empty contemplation quotes nothing.
- Genome scan to pre-build rotors for expand. Rotors are keyed reads of
  the atlas fiber at the peeled chart.
- `Word::rotor()` folding the tape on every comparison. The rotor is
  memoised on the word; equality is the tape.
- Germ-pair adjacency after the cell trie exists: the second walk of
  every germ support is gone. Pairs and 4-neighbour peels run on the
  cell trie fiber.

`degrees: Vec<Keyed<Cell>>` remains the persistence partition of one
complex; `by_key` is the lookup. That is not two facts: the Vec is
order for the wire, the trie is the read.

## Priority sites

### Complex::degrees walked from zero to top

**Was:** `star`, `fill`, `class`, `drop_vertex`, `glue_fetched`,
`pivot_on`, `close`, `component_closed` walked `0..degrees.len()`.

**Now:** `link_of(vertex)` is the keyed fiber of `star(vertex)`. Fill
and class read `by_key`. Climbing is `coboundary_on_link(vertex)` and
stops when the next degree produces no volume.

Carrier was the degree Vec (grows with the tallest simplex on the
chart). Index is now the link of one vertex.

### Cells formed by enumerating vertex subsets

**Was:** `each_combination(&neighbors, take)` for `take in 2..=m`.
Size C(m,k), multiplicity licensing combinations.

**Now:** from each k-cell in `link(vertex, k)`, one neighbour from
`link(u, 1)`, candidate = vertices ∪ {w}, formed when every edge of
the simplex already exists. No subset of the fiber.

### Cover::all_charts / walk / collect

**Was:** `own_cover_charts` walked every field listing.

**Now:** `own_cover` is written in `write_field`. `all_charts` is the
signed digest payload of this hypha's cover, not the book.

`CoverNode::walk` remains for that digest and for `charts(lane)` used
to encode it. Roots stay maintained on admit/retract.

### Bubble::expand visiting every stratum / sibling

**Was:** BFS of all prefixes, `child_charts` collecting every sibling,
`fiber_under` collecting descendants.

**Now:** one peel, `Stalks::each_child` budgeted from a keyed cursor
on the branch node, `fiber` at that sibling, resume from `Bubble.cursor`.

### pivot_on iterating every bubble / every degree

**Was:** already by_chart at prefixes (peel), then `for d in 1..degrees`.

**Now:** `link_of` of each bubble from the by_chart fiber.

### Contemplation merge rebuilding the complex

**Was:** `drop_vertex` walked every degree layer per cell key.

**Now:** `by_key` names the layer; only cells in `star(vertex)` drop.
`link_new` recomputes the link of the merged vertex.

### own_cover_charts, court_stroke, drain_unreachable, confirm_peer_holds

See dual carriers. `chart_listings` is written beside the field so
`rank_over`'s moved set is a fiber, not a scan of pending.

### rank_over recomputing every listing

**Was:** `pending.iter().enumerate()` locating every listing.

**Now:** when the basis grew, the moved charts are the index;
`chart_listings.get(chart)` is the fiber. First measure of a book is
still the snapshot of that book (the quote page itself).

### Sentence assembly germ pairs / independent cycles

**Was:** after building the cell trie, a second walk of every germ
support for 4-neighbours; `first_class` walked degree layers.

**Now:** pairs and peels run on the cell-trie fiber; `first_class`
reads `unfilled` / `classes`. `close` fills `link_of` each germ.
`project_cycles` still walks degree-2 cells of this nerve (the chords
that exist), not a search per chord of a cartesian graph.

### `for lane in 0..4` picking one

Occupied-lane reads stay (all four, then the occupied subset). No
max/min/first lane pick was introduced. `write_field` stagnation is
all four lanes. `home` uses Dershowitz–Manna on occupied lanes, never
lexicographic lane order.

### Word::rotor folding the tape

Memoised on `OnceLock`. Comparisons are the tape. `Rotor::geo` is
sixteen by sixteen, annotated, left in place.

### Vec::contains / any / find / position on keyed carriers

Reintroduced scans on Bubble/Cell/Complex were replaced by `cover`,
`horizon`, `by_key`, `star`, `dirty`. Remaining `iter().any` on a
cell's vertices is the simplex (bounded by degree). Remaining
`sections.contains` on an Element is that word's fibre, not N.

## Grep hits (src/, excluding `#[cfg(test)]`)

For each: carrier, how it grows, where the index should come from,
which of the four sources.

### Keyed / peel / link / budget — kept and annotated

File | Line | Carrier | Grows with | Should have been | Source
---|---|---|---|---|---
network/nerve.rs | PATRICIA children | two children of a branch | fixed arity 2 | — | keyed (branch)
transducer/cech.rs | `link_of` | star(vertex) keys | link size at multiplicity m | — | link
transducer/cech.rs | `coboundary_on_link` | link of vertex, then link of a neighbour | m, not N | not C(m,k) of the fiber | link
transducer/cech.rs | `faces_ready` | edges of one simplex | degree | — | link
transducer/cech.rs | `remember` vertices | simplex vertices | degree | — | link
transducer/cech.rs | `by_key.for_each` classes/unfilled/byte_len | cells of this complex | one chart's nerve | not 0..top of empty degrees | keyed
transducer/bubble.rs | `expand_with` | `each_child` of the peel | spend_unit siblings | not all strata | peel + budget
transducer/bubble.rs | `pivot_on` | by_chart fiber, then `link_of` | bubbles on that chart | not every bubble, not every degree | keyed + link
transducer/bubble.rs | `expand_all` | `dirty.for_each_from` | spend_unit dirty bubbles | not all bubbles | budget
transducer/bubble.rs | `component_closed` | `link_of` each vertex of the component | the component | not 0..degrees | link
transducer/sentence.rs | assemble `at.for_each` | cell-trie fiber | cells of this residual | not germ×germ | keyed + peel (4-neighbour)
transducer/sentence.rs | `close` | germs of this nerve, `link_of` each | germs of this sentence | not all degrees | link
server/cognition.rs | `court_stroke` | `open.for_each_from` | spend_unit | not all open | budget
server/cognition.rs | `drain_unreachable` | `open.for_each_from` | spend_unit | not field.entries | budget
server/cognition.rs | `confirm_peer_holds` | remote digest charts, `ruled.fiber` | peer digest | not all provisional | keyed
server/cognition.rs | `confirm_peer_holds_addrs` | peer have-list | wire page | not all provisional | keyed
server/cognition.rs | `own_cover_charts` | `own_cover.all_charts` | this hypha's charts | not field.entries | keyed (digest)
server/cognition.rs | `glue_fetched` fill | `link_of(&peer)` | link of the fetched vertex | not 1..degrees | link
server/cognition.rs | `write_field` lanes | four lanes | fixed 4 | — | fixed
server/cognition.rs | `rank_over` moved | moved charts, `chart_listings` fiber | new words' charts | not every listing | keyed
client/sdk.rs | quote charts | bubble cover peels | thoughts this mind holds | not atlas.strata | peel
client/sdk.rs | expand rotors | `atlas.fiber(&inv.peel())` | fiber at the peeled chart | not the genome | keyed
alphabet.rs | `Rotor::geo` | 16×16 masks | fixed 16 | — | fixed
alphabet.rs | `LetterId` plane tables | 6 / 4 planes | fixed | — | fixed
stalk.rs | `each_child` | branch keys of one node | spend_unit | not all siblings | budget + peel

### Wire / persistence — tape length of this object

`for _ in 0..n` in `from_bytes` / `to_bytes` (element, cech, bubble,
sentence, dialogue, economy, ledger, cognition snapshot, alphabet
Word). Carrier is the byte tape of one object. Grows with that
object, not with the mesh. Index is the length prefix on the wire.
Source: budget (the payload). Not deleted.

`dialogue.rs` `for x in items.iter().take(255)`: wire page cap.
Source: budget.

### Fixed physical size — kept

`for i in 0..16` (Rotor), `for l in 0..4` (lanes), `SLOT_PLANES_SIX`,
matrix `0..n` where n is the matrix dim of one operator (≤ GERM_DIM),
`core/rcf.rs` companion n, `core/engine.rs` field dim. These do not
scale with the state.

### Peel of a residual / invariant

`Invariant::prefixes` / `peel` loops until the axis list is empty.
Axis count is the moduli of one support, not N. Source: peel
(governor shrinking).

`family.rs` / `blade.rs` / `arc.rs` grid cells: the scene being
perceived, one percept. Source: peel of that residual.

`element/carrier.rs` `sub` / `sections` / `torsion` / `transitions`: the
fibre of this word. Source: keyed-on-the-word (the word is the
cursor). Not a mesh scan.

### Remaining Vec walks that are this-object persistence

`Kernel` snapshot of `basis` / `pending` / `court_federation`:
persistence of this hypha's ledger. Dual `court_fed_index` already
exists for lookup. The Vec is write-order for MYC1. Not used as the
index of glue, fill, or quote.

`home` fallback `rooms.entries()` when the cover fiber is empty:
the novel case, identity after room. Size is hyphas on the mesh, not
listings. A remaining smell; identity is the last tiebreak and the
fiber is the first read.

`component_count` walks bubbles of this contemplation (this mind's
thoughts). Measurement, not glue.

### Hits that stay as comments on the object they already are

`chronometry.rs` receipts of this chain. `economy.rs` ledger entries
of this treasury (restore). `registry.rs` tasks of this hypha.
`perception/ear.rs` gauge of this ear. `anchor.rs` grown terms of
this Sel. `spectral.rs` witnesses of this beam.

None of these are "all listings" or "all words on the mesh".

## Surviving loops — annotated index source

Every loop that remains on the hot path has a one-line `// index:`
comment naming keyed / peel / link / budget / the four lanes / sixteen
masks / the simplex / the wire length. A loop that could not be
annotated was deleted (`each_combination`, degree-position scan,
field.entries own_cover, provisional.entries confirmation, genome
rotor prebuild, atlas.strata quote, expand BFS).
