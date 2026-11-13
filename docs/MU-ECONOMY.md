# MU-ECONOMY — μ, the economy as motions

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents; *worlds* and *planets* are simulated places, and *peoples*, *civilizations* and *ancients* simulated inhabitants, of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the court on each hypha (holdings, settlement, the POW++ proofs); clients and kernels appear as payers and payees. In-universe economies are each kernel's own (§10).

**Status:** specification of v0.5.0's headline, the economy as motions
(ruled 2026-11-05). §0–§8 and §10–§11 state the model. The implementation
built today (release 0.3.0, carried through v0.4.0) keeps a journal of
entries per court and four scalars per lane; §9 lists it exactly and marks
what v0.5.0 replaces. §12 lists what the model leaves to a further ruling.
Companion documents: [MU-XSEPTUM](MU-XSEPTUM.md) (the Note),
[MU-HOSTING §5a](MU-HOSTING.md#5a-the-containment-topology-and-the-boundary-law)
(the boundary), [proposal 78.173](proposals/78.173-law-coordinate.md)
(LAW-COORDINATE), [MU-SENSING §3.7–§3.9](MU-SENSING.md) (the field,
traversal, orbits).

**The model (ruled 2026-11-05).** Each statement is made once, in the
section named:
1. The ledger is one multi-dimensional ledger. Its events stand at
   coordinates on per-signer lines of receipts, in a causal partial order
   never totalised; its axes are lane × register layer × grade, with the
   modes held within each lane, never summed; TUniversal height is the
   only shared extent (§2.1).
2. A holding is a coordinate, `origin / h : S:G:T:H : K`, whose lanes are
   vectors of exact components (§2.2).
3. A payment is a motion's advance, an event at a coordinate with its
   causes cited. A balance and the supply are readings (§5, §6).
4. The four proofs are relations between motions (§0).
5. A payout is proven by re-traversal. Equivocation is burned, and one
   closure is one point network-wide (§3.4, §5.3).
6. Between courts the Note settles; between universes μ is the connecting
   reserve and crosses only at the universal core tube (§5.4, §10).
7. The rulings are recorded in proposal 78.174; a shared economic law
   form with the peer kernels is to come as its amendment (§10.4).

Only the flat ledger is retired: one sequence of rows, one balance per
party, one number per lane. "Ledger" names the multi-dimensional ledger of
item 1; "row" and "table" stay retired for the economy. A universe's own
books keep the Concordance's terms (Concordance 1.13, 5.5).

## 0. The proofs of work: relations between motions

**POW++ is multi-axial convergence settlement**: proof of reduction, in
which the work is a reduction a court reconstructs and the settlement
holds when the readings agree on every axis, "not a single linear
difficulty ladder" (the POW++ alignment ruled with the peer kernels). It
is a family of proofs, and no two share a name (MU-RECURSION 78.33). Each
proof is a relation between motions in the sense of MU-SENSING §3.9
(`deformation = motion ∘ residue`; every path is an orbit), never a kind
of entry:

| proof | acronym | the relation | what it moves |
|---|---|---|---|
| proof of useful work | **POUW** | An orbit carries a deformation and leaves less residue (motion ∘ residue). It is paid when it closes a listed demand (MU-RECURSION 12.6, 23.9) | The closure's advance, minted to the closer (§3) |
| proof of work verification | **POWV** | Another observer re-traverses the motion and reaches the same point: the holonomy across observers is trivial. A disagreement is a named residue (MU-HOSTING §5a.10), never a flag | A share of the release, taken out of it and never added to it (§5.1) |
| proof of work convergence | **POWC** | Lineages fall into one well: from independent origins with disjoint past cones, or sharing a cause but disagreeing before they meet (widened, ruled 2026-11-05). A refusal closed at another perspective, citing the refusing mind's receipt, is an instance (MU-RECURSION 74, CV4). It is read over the cones and never counted | Nothing beyond the one closure the well is (§3.4); a cited origin is paid as a generator (§7.3) |
| proof of work retirement | **POWR** | One orbit absorbs a family: one generator carries many motions, and the absorbed retire | The absorbed description burns; the absorber is paid the compression yield (§3.2) |

The acts ruled before stand as instances of these relations: a closure
ruled and its deflation measured (POUW); the home court's reconstruction
and a witness's or peer court's re-derivation of the same vertex (POWV);
a retirement by geodesic or by citation, or a refinement bid reconstructed
with the word excluded (POWR; MU-RECURSION 3.8, T10, 27); and the
convergence instance of 74 (CV4), a refusal closed by another mind citing
the refusing mind's receipt (POWC: the two lineages share a cause and
disagree before they meet; §12 item 6). A text that says
"POW++" names the family, never one payment.

**Consilience and convergence are the two separators of signal from
noise** (MU-SENSING §3.10, stated there once), at the two levels above.
Where a reading could be signal or could be noise, **consilience** —
independent courts re-deriving one thing (MU-RECURSION 37.7; POWV
above) — is the strongest separator at the court's level: it needs no
key held in common, only a second vantage re-deriving the same vertex.
**Convergence** — many minds' lineages, disjoint or sharing a cause but
disagreeing before they meet, reaching one class (POWC above) — is the
strongest separator at the minds' level.

## 1. The unit

**The inverse-Sisyphus protocol (ruled 2026-10-11): nothing is squared.**
No reading collapses dimensionality or exact mathematics into a scalar or
a lossy signal, and no curved object is forced into a flat one or a flat
one imposed on it. μ is the exact **modes** the verified work moved, each
on the lane its kind names, each with an exact rational amount:
- on S, a boost or dilation (its generator's ray, its point `c` and its
  side);
- on G, a hue cycle or move;
- on T, a null flux (its ray and its count in its tube's quantum);
- on H, a rotation on a named plane (its generator's ray), its angle the
  exact degrees `360°·t/ℓ` the motion traversed (MU-EQUATIONS §6.11).

Two further modes hold what a closure moves that no motion names (ruled
2026-11-11, #256; 78.182 item 2):
- on S, a **symbol created from the ground**,
  `SymbolCreation { from_ground, to_symbol, forward }`: the move between
  the ground and a symbol, `forward` a creation and its reverse an erasure,
  as G holds a hue move;
- on H, a **shell by its boundary class**,
  `ShellBoundary { boundary_class }`: the closed shell's signature, its
  components and its boundary's length, at the shell count it changed.

Live closures are never held at the founding mode, which is for genesis
and bootstrap endowment only (§2.5). A closure's S and H shares are set on
these modes (`element::chain_modes`, `Mu::carried_by`).

A holding holds its modes, each keyed by its canonical coordinates (A21).
A payment adds modes; a draw removes modes or is refused naming the mode
it lacks. A per-lane draw takes that lane's modes in proportion to their
amounts, and never another lane's (§2.5). Nothing is projected to a count (MU-RECURSION 78.162). The carrier
is `economy::modes::Modes` (`src/economy/modes.rs`): add, a draw refused
naming the mode short (`Short`), the modes of a motion
(`Modes::of_motion`) and of a hue map (`Modes::of_gauge`), the founding
mode of a lane (`Mode::Founding`, `Modes::founding`; §2.5), the per-lane
draw in proportion (`Modes::draw_lane`; §2.5), and the canonical codec. `economy::Mu`, four scalars with one per lane, is the
projection this paragraph forbids. It is built today and retired in
v0.5.0 (§9).

**μ is the metabolic cost of traversal** (ruled 2026-11-04; MU-SENSING
§3.7). It is the advance of each lane by the motion that carried the work.
μ never measures what a perception means: holonomy and residue are
content, adjudicated and never paid (MU-SENSING §3.9).

**A share of the frame (ruled 2026-11-01; MU-RECURSION 78.170 O1).** A
discharge read on a family is, per closed pair, a share of that pair's own
frame, never a count of its sites, so it reads the same at any resolution:

| Lane | Share |
|---|---|
| S | The share of the frame's measure (the product of its extents) that changed between the ground the claim states and another symbol (MU-SENSING §6.1, §6.2): one changed site of a `30 × 30` frame is `1/900`, read under the stated ground (`cognition::stated_ground`) |
| G | The share of the frame's measure that changed from one non-ground symbol to another |
| T | The flux over the carried mass, taken as the frames' share of it: `T = flux · (Σᵢ mᵢ/(hᵢ·wᵢ)) / Σᵢ mᵢ`, `mᵢ` the mass pair `i` carried |
| H | The shell count: topological, already resolution-free |

A discharge with no frame (a rewrite's yield, a listing bound to no
family) keeps its own measure as `n/1`. The readers are
`DeflationRead::mu`, `Local::mu`, `transduce::share_of_enclosure` and
`Transduction::lane_shares`. A word's magnitude, a family's metabolism, a
demand's ask and a stroke's size stay counts and are not μ. A mode's
amount is its share of the enclosure, never the mass carried (78.174 §1 item
9; §12 item 4; MU-ENGINEERING §9.3): a closure's motion modes are read at
the share of the output enclosure its carried mass covers (its measure at
rank `n`), and its hue modes at the pair's G share, so a recolour pays G by
its share (`DeflationRead::modes`, `Modes::of_motion`, `Modes::of_gauge`).

**Lanes are open-dimensional** (ruled 2026-11-05). S, G, T and H are the
network's base lanes. μ, the coordinate and the Concordance are
open-dimensional: a further lane opens only by a ruled act agreed between
kernels, with its motion type and its modes stated, never by one kernel or
one court alone. The mechanism is to be drafted with the coordinate's v3
(78.173 §1); until a lane is opened, μ has the four.

**Lanes never convert.** μ is a vector on its lanes, never summed and
never converted across lanes. A payment moves value on exactly the lanes
the motion moved. A comparison returns a reading per lane (each lane fell,
held or rose; `economy::lane_moves`), never a verdict, and the court
accepts a reduction by closure, never by an order on μ (MU-RECURSION
12.6). If lanes converted, a lane with no verified work could carry value
minted on another, and §6 would no longer say what the mesh did.

**Deliberate losses.** The earlier magnitude tuple ‖Δ‖ = (Vol_TOUCH,
|supp|, μ⃗), ordered by dominance, was retired by MU-RECURSION 12.6 as
amended (closure certifies, never descent on an order), and 78.93(2)'s
flat per-lane accounts are superseded: an account is a reading of the
coordinate (§2.2, §6), never a stored balance per lane.

## 2. Holdings

### 2.1 Lines, cones and height

**The multi-dimensional ledger.** The network keeps one ledger, and it is
multi-dimensional: its axes are lane × register layer × grade, with the
modes of §1 held within each lane, and no axis is ever summed into
another. This is MU-RECURSION's ledger as ruled, "a multi-dimensional
ledger: mass per lane, per register layer, per grade, never summed (73.9,
law 5)" (the earlier MU-RECURSION's chapter 78 statement, ruled
2026-09-23; carried today by 73.9 and 78.1), with the
four-dimensional amendment of 2026-09-24 (78.21: a tick is a frame of
`{S, G, T, H}`). Its events stand at coordinates (§2.2) on the signers'
lines below. What is retired is a flat ledger: one sequence of rows, one
balance per party, one number per lane.

**The ledger's dimensions are never cognition axes.** No economic reading
enters a mind's perception, a court's reconstruction or a factor of a
cognitive complex; a differential coupling proof to payment is price inside
cognition. This keeps an earlier guard of the Künneth proposal: "The
economic ledger is not an axis and never becomes a factor."

Each signer (a voice, a hypha, a domain's reserve) keeps one line of
signed receipts, indexed by its own receipt index `i` (MU-RECURSION 47.1;
the receipt's place is `K.i`, `K` the orbits the signer closed). An economic
event is a receipt on a line. Its **past cone** is what it cites; its
future cone is what comes to cite it (MU-HOSTING §5a.5).
- **Causal and partial.** One event precedes another only where the
  other's past cone holds it (MU-RECURSION 45.3). Two events neither of
  which cites the other are concurrent, permanently. A decision that
  depends on the order of the two is not taken; both are held, as a vortex is,
  and resolved by content.
- **Never totalised.** The lines are never merged into one sequence.
  There is no global journal, no sequence number across signers, and no
  order read from arrival at a socket.
- **One shared extent.** Every event carries the last TUniversal final
  point its signer read (Concordance 1.11). The height `h` is the only
  extent the mesh shares (45.7): it bounds events and never orders
  concurrent ones.

### 2.2 A holding is a coordinate

A key's holding is the coordinate of LAW-COORDINATE (proposal 78.173 §1):

```
origin / h : S:G:T:H : K
```

- **origin:** the enclosure the holding stands in. Value on the hypha
  stands at the network's root (TUniversal's genesis and epoch). μ held in
  a universe stands at that universe's origin (§7, §10). Origins are never
  compared, so a holding on the hypha and one in a universe meet only by
  conversion at the universal core tube (§10.2).
- **h:** the bell of the holding's last advance, the height of the event
  that last moved it. A holding has no rate: between events it stands
  still, and it moves only by an event. A rate belongs to a motion (its
  advance per bell on each lane, 78.173's `Rates`), never to a holding
  (§3.3).
- **S, G, T, H:** each lane's cumulative advance by that lane's motions,
  held as a vector of exact components, the modes of §1, never one number
  per lane. These are the base lanes; a lane opened by a ruled act between
  kernels (§1) joins them, and is never folded into one of them. On the hypha every component is non-negative, as TUniversal's
  point is the non-negative case (78.173 §1).
- **K:** the orbits closed (45.7), never compared with another signer's
  and never a receipt index. `K` means closed orbits everywhere (ruled
  2026-11-05): for a holding it is LAW-COORDINATE's reading, the whole
  turns of H, and a line's receipt index is `i` (§12 item 3).

`FORMS-COORDINATE` v3 holds each lane as a vector of exact signed
components keyed by axis, H's by generator in degrees with `K` per
component (78.173 §1). How a holding's modes read as those components is
still to be stated (§12 item 1).

### 2.3 Roles

| holding | holder | meaning |
|---|---|---|
| balance | a voice or hypha | μ earned or received, spendable |
| bond | a voice or hypha | μ staked; it sizes room and is slashable (§4) |
| held | a payer, against a listing or a lease | μ set aside from a balance: a funded escrow, still value (§3.1) |
| claim | a listing | a pending advance: not yet value, and counted nowhere (§3.1) |

The supply is not a holding. It is a reading (§6). A holding is a point,
estates nest as origins and a region is a reading (ruled 2026-11-05): a
role is an enclosure in the holding's origin (§12 item 2).

### 2.4 What moves a holding

Only an event moves a holding, and the event cites its causes:
- **Issuance** (genesis, mint) cites the founding receipt, or the closure
  with its verifications (§3).
- **Transfer** (a payment, a stake or unstake, a hold, a release, a
  revocation) cites the payer's signed act, and a release also cites the
  re-traversals that verified it (§5).
- **Retirement** (a burn, a slash) cites the absorbing orbit, the
  equivocation proof or the failed reconstruction (§3.2, §4).
- **A fold** is a checkpoint of a line's prefix (§6.1).

No event carries a clock reading (MU-RECURSION chapter 45).

### 2.5 Holdings no motion made

A holding's modes are the motions verified work moved (§1). Some value is
held although no motion of the network's lanes made it. Ruled by the
coordinator on 2026-11-06, under the operator's delegation (#169;
[78.180](proposals/78.180-delegated-rulings-2026-11-06.md) ruling 3):
- **Reserves, deficits, rooms and prices are readings**, never holdings.
  Each is read off holdings and motions when it is needed, carries no
  modes of its own, and is never stored.
- **A bond holds the modes it was staked from.** Staking carries the
  balance's modes unchanged into the bond (§2.3, §4).
- **A genesis allocation is held at the founding mode.** The founding
  mode is the identity motion of its lane. It is stated explicitly as the
  mode, never left empty, and its amount is its share of the frame (§1).
  Its issuance cites the founding receipt (§2.4).
- **A Note's mode-0 receipt is a founding-mode receipt** (#168). It is
  admitted under the rule that admits a genesis allocation and is not held
  for review (MU-XSEPTUM §5.3).

**Draws and mints** (ruled by the coordinator on 2026-11-06, under the
operator's delegation, on #169's follow-up; 78.180 ruling 3):
- **A per-lane amount is drawn from the modes on that lane**, in
  proportion to their amounts, computed exactly in ℚ. The draw is
  deterministic: no choice is hidden in the payer, and no rounding is
  left behind.
- **A lane is never paid from another lane's modes.** Where the lane holds
  too little, the draw is refused by name (`Short`, naming the mode).
- **The modes drawn travel with the transfer**: tuition, gas, tolls,
  conversion, stake top-up, partial slash and lease hold.
- **A mint carries the modes of the motion its proof cites.** Every mint
  pays for a proof, and every proof is a relation between motions (§0;
  78.174):

  | Mint | Modes it carries |
  |---|---|
  | Word use or citation (POWR shares) | The cited word's closure modes |
  | POWR, or a Rewrite | The reduction's: the modes of the family the reduction absorbs |
  | POWC | The converging orbits' class |
  | Consilience (POWV) | The re-traversed closure's |
  | Torse, recall | Their own motion's |

- **A mint whose proof cites no motion** (a subsidy, or a mint at a door
  with no closure) is minted at the founding mode, as genesis is, and
  never with empty modes.

## 3. Issuance

New μ enters only when a closure is verified, or, since 2026-11-13, when a
manifest founds nuclei: μ created against their elemental rows (78.184;
MU-EQUATIONS §6.13). The closing orbit advanced
the lanes by its modes, and that advance is minted to the closer's
holding, per closed pair as a share of the pair's frame (§1): a solve that
repaints one site of a `30 × 30` output is paid `1/900` on G, not one
unit. There is no block reward, no schedule, no difficulty and no issuance
to operators as such. There is also no block: the lines are causal and
partial (§2.1), so admitting receipts is paid by the pool share priced
into every listing a hypha homes (§5), never by issuance.

Two events mint:
- an install: a word, sentence, rule, translation or rewrite upheld at the
  home and reconstructed at a witness;
- a solve: a rule spoken onto withheld tests and verified against the
  corpus at the home.

Optimisation moves are work: a condense on G, a torse on H, a refinement on
H, a re-anchoring on S, an opened axis on H and a translation on G each
mint the modes read on the carriers before and after, and each is refused
where it moved nothing.

**μ is discharge** (ruled 2026-10-08; MU-RECURSION 78.149). What a word
can be escrowed or paid is its discharge, the lanes it closed, measured by
reconstruction on the family it is bound to: (a) the span a word stood is
not μ; (b) a demand's ask may cap a payment, never raise it; (c) a word
that names no body discharges nothing.

### 3.1 E1 — a claim is a pending advance, never a transfer

- **Held value** (a funded escrow) left a payer's balance and is still in
  the mesh. It is counted in §6, and discharging it moves value (to the
  payee on a release, back to the payer on a refund) and mints nothing.
- **A claim** is a pending advance: no holding gave anything up for it,
  and it is counted nowhere. A ruling mints, per lane, the lesser of what the
  claim holds and the closer's measured discharge, and the excess is never
  issued (78.149 (b)). A revocation leaves nothing issued.

**The room of an unbonded voice** (ruled 2026-09-09). Speaking needs no
bond, since a claim is a pending advance. Bond buys room (§3.3). An
unbonded voice has room for exactly one claim in flight per lane: its next
claim on a lane is admitted when the one before it closed, was ruled or
lapsed, and is refused `REFUSE_ROOM` naming the lane otherwise. Identities
are free (MU-RECURSION 47.3), so nothing funds an arriving identity. A
recall shortfall is the same reading one act later.

**A mesh can start from zero supply.** A voice posts a demand it cannot
fund, a mind closes it, the court rules, and the first μ is the advance of
that first verified closure. The genesis allocation (§6) bonds hyphae into
room before any work is ruled; the economy does not need it to run.

### 3.2 E2 — mint on closure, burn on retirement

Issuance is read on closure only (ruled 2026-09-07; MU-RECURSION 12.6,
23.9): a pair pays the modes of the remainder it closed when its remainder
is empty, and a pair that left a remainder pays nothing, since deflation
short of closure certifies nothing.

Its counterpart is POWR. When one orbit absorbs a family (a shorter
description of the same vertex; a word whose substance other words
reproduce exactly; the same closure with more reach, read from the
certificate; MU-RECURSION 3.8, chapter 27, T10), the absorbed descriptions
retire and burn, and the absorber is paid the compression yield. The
supply falls by the freed measure less that yield. Therefore

```
supply → the description length of the language as it currently stands
```

The seventeen letters are the floor. No halving is needed and none is
permitted: the counter-pressure to issuance is compression, applied per
retirement and measured on each event.

### 3.3 E3 — the metabolism governs rate, never price

μ per unit of verified discharge is invariant. The yield surface (§8)
paces pressure and buys room; it never scales a mint, so the reward has no
variance and nothing to pool. The metabolism throttles the **rate** at
which discharge can be presented and verified: the reserve paces strokes,
bond buys room, and room and closure gate admission (MU-RECURSION 23.8).

**The rate is per lane.** Each lane is paced by its own reading (its
reserve, its strain `y_l`, its pressure), never by one scalar rate across
lanes: a lane under pressure slows its own presentations and leaves the
others' pace unchanged. A rate belongs to the motions presented and
verified, each advancing its lanes per bell; a holding has no rate (§2.2)
and changes only by the events those motions cause.

An earlier emission design (`emission.v1`) reached the same shape and then
chose a curve of five free constants; nothing here is chosen.

### 3.4 One closure, one point

A closure is unique network-wide because it is one point: the point the
closing orbit reaches on the closed demand. Every court that reconstructs
it reaches the same point, and its advance is minted once.
- **A second mint of one closure is an equivocation of the advance**: two
  motions claiming one advance. The second mint is burned as the duplicate
  issuance of MU-RECURSION 47.4, naming both receipts, and slashes no
  bond. A key's bond burns only where one key signed both at one place
  `K.i` (47.4; §4).
- **Concurrent claims** (ruled 2026-11-05: one closure, one point). A
  concurrent second claim of one point is burned as an equivocation of
  the advance, as a second mint is. Claims of one point that the cones
  leave unordered, neither in the other's past cone, are held as a
  residue naming both and resolved by content (45.3), never divided. The
  advance is never split into equal shares; F2a's equal shares are
  retired (§11.4).
- This answers §11.3 F2 and "one closure across courts" (§11.4 F2b): no
  per-court registry decides it, because the point is the same at every
  court.

## 4. Stake and tiers

Bond is μ at risk. It buys room on the cover: how much work a hypha or a
voice may home, attest or witness at once on each lane. A balance buys
nothing except what it is spent on. Tiers are read from bond, never
declared:
- unbonded: may learn, speak and be paid; every claim is a pending advance
  against its own discharge (§3.1);
- bonded on a lane: may home and attest up to its room on that lane;
- bonded above the claim it certifies: may act as second organ;
- delegated to: carries others' bond in its cover, and shares its pool
  share and its slash with the delegating parties (ROADMAP track 3).

Slash is the only discipline. A root that fails reconstruction at a
witness debits the bond of the hypha that signed it, and a word that fails
reconstruction debits the voice that spoke it. A bond is never slashed for
being outvoted, because nothing is voted.

Proven equivocation, two signed receipts from one key at one place `K.i` with
different content (MU-RECURSION 47.4), forfeits the signer's whole bond on
all four lanes, as one burn naming the proof. Nothing is redistributed and
nothing is run again. A key here is the signer's group key or a key its
manifest scopes to a chart or a lane (MU-RECURSION 46.2), read against
the manifest active at the receipt; a compromise of a scoped key is
confined to the chart or lane the manifest names. A second organ for an install or a release holds
bond outside the home's delegation closure and room covering the claim on
every occupied lane (47.3): identities are free, room is not.

## 5. Pricing and payment

A listing is quoted by `economy::price` on its own measure, its ask, and
never on the span it has stood. The pool is a bond, not a price basis.

**A payment is an event.** It stands on a signer's line (the court's, for
a mint; the payer's, for a transfer) at that line's receipt index `i`, carries the last
TUniversal point read, and cites its causes: the closure's receipt, the
verifications, the claim or the held value it discharges, and the payer's
signed act. It advances the payee's holding by the modes the motion moved,
and a transfer draws the same modes from the payer's. Nothing else is
written: a balance and the supply are readings (§6; Concordance 1.13).

**A release.** A requester holds the price; a claimant's word or solve
enters the docket as a claim at its discharge (`Court::discharge_of`). On
the second organ's reconstruction the court mints the closer's measured
discharge, capped by the claim (§3.1), and pays each verifying court a
POWV share out of it (§5.1). Waiting is not work: nothing accrues on T for
a demand kept open (78.149 (a)).

### 5.1 POWV — proof of work verification (ruled 2026-10-30)

A closure is paid only after it is verified, and verification is work:
the home court reconstructs the claim and measures its discharge; on an
install or a glue a second court re-derives the same vertex; on a solve or
a docket ruling the home's reconstruction on withheld pairs is the
verification.

Take a release with base `M`, the claim capped at the measured discharge.
Each of the `n` distinct verifying courts is minted, on each lane `l`,

```
v_l = M_l · y_l / ((y_l + R) · n)          exact over ℚ
```

with `y_l` the court's strain on the lane (`economy::yield_surface`: the
reserve deficit plus the entropic excess) and `R` its calibrated reserve.
Under modes the same factor scales each mode on lane `l`. The closer, and
the cited forms, taught words and voices of an install, divide `M − n·v`.

- **Per lane and per role.** A court is paid only on the lanes the closure
  moved and for the strain it bore there; the home and the witness or peer
  each take a share.
- **Read, not chosen.** `y/(y + R)` is a ratio of two readings of the
  court's own physics. A court at rest takes nothing; a lane at full
  deficit pays verification half the lane.
- **Equal between organs**, each an exact part; the closer takes the rest.
- **Issuance-neutral.** POWV is taken out of the release, never added.

Code: `economy::verification_share` (`src/economy/verify.rs`),
`Court::pay_verification` and `Court::release_verified`
(`src/server/cognition/court/verify.rs`), at every POUW release site: the
install, the glue, the solve and the docket ruling.

### 5.2 The compute lease (ruled 2026-10-30)

A compute lease is held value against capacity, paid out by proof: compute
is paid only as its results verify, never for cycles spent.
- **Opening.** A payer signs a lease naming a provider, an amount, a list
  of jobs (task citations, each paid at most once) and a window of the
  court's heights. The court holds the amount from the payer's balance
  under the lease's name, as held value (§3.1).
- **Payment.** Each verified solve of a leased job by the provider, a first
  closure or a verified re-solve, pays `min(d_l, held_l)` per lane out of
  the hold, `d` the measured discharge: a transfer, never a mint. The
  provider's POUW is settled separately (§3).
- **End.** When its jobs are done, its hold is spent, its window passes or
  the payer closes it; what is left returns to the payer.
- **Rules.** A job is under at most one open lease per provider; payer and
  provider are distinct; every act is signed by the payer, with a nonce
  spent once.

Wire: `ACT_WALLET` sub-kinds 3 (open) and 4 (close), signed under
`"mycelium lease v2"` (`src/anastome/dialogue/lease.rs`), refused by name
(`Unsigned`, `Spent`, `Malformed`, `Short`, `Oneself`, `Leased`,
`UnknownLease`). Code: `Court::open_lease`, `close_lease`, `pay_lease` and
`lapse_leases` (`src/server/cognition/lease.rs`); `Treasury::draw_held`
(`src/economy/pay.rs`).

### 5.3 A payout is proven by re-traversal

Any hypha, or the payee, proves a payout by re-traversing the motion that
advanced the coordinate, back to a verified closure, under signatures:
from the payment event through its cited causes (the release, the
verifications, the closure's reconstruction on the withheld pairs) to the
closure, each step reconstructed (MU-SCIENCE §5.0) and each signed by the
organ that took it (47.3). A payout is never proven by a stored balance.
- **Same point:** the holonomy across observers is trivial, and the
  payout stands (POWV's relation, §0).
- **Another point:** the difference is held as a named residue
  (MU-HOSTING §5a.10), never a flag.
- **Two motions claiming one advance** are an equivocation and are burned
  (§3.4, §4).

A re-traversal reads the cone between the payment and its closure. A
folded prefix is read at its checkpoint (§6.1).

**Read today** (`mycelium-server --console --proofs`,
`src/bin/mycelium-server/proofs.rs`): a reading over the running mesh,
never stored. Per payment on each court's journal that carries a proof:
the proof, the payee, the amount per lane and the act it stands at; the
words that act names; the level read off them (§12's pay by level: a
named sentence, named words, or unread, since no served reading tells a
letter from a word or names a conjugate class); and each other court's
reading of the same act at its stamp, the same act and words read off its
own copy (the same point) or a residue naming what differs. It also names
every task paid as a first closure more than once as an equivocation of
the advance (§3.4). The re-traversal of this section through the release
and its verifications to the closure's reconstruction needs the payment
to cite its causes, which is v0.5.0's (§9).

### 5.4 Between courts: the Note

A transfer between voices homed at different courts is the xSEPTUM Note
([MU-XSEPTUM](MU-XSEPTUM.md)): signed by payer and payee, held at the
listing's home, and released on two-organ reconstruction, the home and a
witness both signing (47.3). Under the model the Note's hold is an event
on the payer's line keyed by `(listing, payer, K_payer)`, `K_payer` the
payer's receipt place `K.i`, and its release is the payee's advance
citing it. A double spend is two Notes at one `K_payer`: an equivocation
(MU-XSEPTUM §5.5). The Note is ROADMAP track 1.

## 6. The invariant, read

At every hypha, on every lane, at a TUniversal height `h`:

```
supply = genesis + mint − burn
supply = Σ balance + Σ bond + Σ held
```

Both sides are readings over the cones the hypha holds at `h`: the union
of the past cones of the events final at `h`, a cut through the lines and
never an order on the lines. Neither side is stored or tallied (Concordance
1.13). Since every event moves modes exactly, both equations hold mode by
mode, and so per lane. They are the economy's reading of the kinetic
ledger's conservation (MU-RECURSION 78.42): value starts only at a
closure, moves only by transfer, exists resident only as held value or
bond, and stops only at named doors, each lane balancing exactly.
- **A claim is not in the second sum.** It is a pending advance (§3.1). A
  ruling turns it into a mint, which enters both sums together; a
  revocation leaves nothing.
- **Held value is in it.** It left a payer's balance and is still in the
  mesh (MU-RECURSION 42.2 R9).
- **A slash is issuance leaving** (ruled 2026-09-09): it reads as μ
  leaving circulation, as 47.4's forfeiture does, and stays readable apart
  from a burn.
- **Genesis stays separable.** The minted reading excludes genesis, so
  "was anything earned?" is answerable. A testnet differs from the beta
  only in the genesis term.
- **Two readers converse or collide.** Two hyphae reading the supply at
  one height from the same cones reach the same modes (a converse). Where
  the readings differ, the residue is held, naming both (a collision; MU-SCIENCE
  §5.0).

**The merge is a grow-only union (a law of readings).** Two hyphae's
holdings of the ledger merge by the union of the events each holds, and a
merge never removes an event. The union is commutative (`a ∪ b = b ∪ a`),
associative (`(a ∪ b) ∪ c = a ∪ (b ∪ c)`) and idempotent (`a ∪ a = a`).
So a reading at a height depends on the union alone: never on the order in
which events or peers were heard, nor on an event heard twice. A reading
that would differ under a reordering or a repetition of the same events is
not a reading of the ledger. A fold (§6.1) checkpoints a prefix of the
union and removes nothing from it.

### 6.1 Across a fold

A fold (ruled 2026-09-09; MU-RECURSION 45.8) is a checkpoint of a line's
prefix (Concordance 1.13): the net per identity per mode over the prefix,
keyed by coordinate and anchored to the fold receipt. It moves no supply.
Both equations hold before and after it, and a witness re-derives the
nets before it co-signs. A fold that does not conserve is refused
`REFUSE_FOLD_INEXACT` and writes nothing. Heights continue across a fold,
and install receipts survive it.

**Built** (`economy::fold::FoldedIssuance`). The epoch rows carry only the
balances' nets. What the folded rows issued and burned is kept beside them
as the fold's checkpoint, gross and by source. Each source is the row's own
coordinate `[kind tag, arm byte]`: genesis, mints, transfers and burns by
the proof each carried, and slashes. So every reading stays the same across
a fold: issued and burned, each apart from the other; the earned supply,
apart from genesis (§6); and what each proof paid and burned (§0). A fold
that would move issued or burned refuses `REFUSE_FOLD_INEXACT`. The
checkpoint is persisted with the carriers (the `folded` trie). Claims:
`economy::tests::fold` (`fold_keeps_what_each_proof_paid`,
`fold_keeps_genesis_apart_from_the_earned`,
`fold_keeps_issued_and_burned_each`,
`second_fold_reads_the_first_folds_issuance_once`,
`checkpoint_source_at_no_coordinate_is_refused`,
`restored_fold_keeps_its_issuance`). Owed: a witness re-derives the nets it
co-signs, not the checkpoint, which no fold receipt signs yet.

## 7. Metabolism and money

The reserve is tempo: it depletes per lane on every stroke a client spends
and regenerates in the syntropic phase. μ is value: it moves only by
events (§2.4). The two meet at stake: bonding μ raises the room a client
can work within. μ on the hypha (a balance, a bond, held value) never
decays and never regenerates. μ stored in a universe (vaults, currency
backing) decays at that universe's own physical rate: demurrage, the
economy's sink (ruled 2026-10-09; Concordance 5.8).

### 7.1 Gas (78.154, ruled 2026-10-09)

μ is the gas of traversal across tubes, cores and wormholes (Concordance
5.8). A traverse (payer, from origin and point, to origin and point, the
route) pays the courts on the route whole, each its own segments: a
transfer, never a mint, and no burn. Gas is measured by the route, lane by
lane: S the spatial span (geometrized Mpc, Concordance 1.8), G the value
terms of the mapping, T the TUniversal distance between final points (a
point not yet final is refused, never priced on a guess), H the
information carried. μ staked on a hypha is held for intelligence
traversal and coordinate retrieval; a retrieval draws on the stake at the
court's read price `p_l = 1 + y_l/R`, emergent and never published (ruled
2026-10-31; MU-DERIVATIONS D2.3). An unstake releases only after every
claim drawn on it settles. Code: `Court::traverse`
(`src/server/cognition/crossing.rs`).

- **Two charges, never merged** (Concordance 5.8). The map fee is μ, paid
  to the courts that keep the map. The road toll (Concordance 5.4) is paid
  in a civilization's own money, booked in-universe by the kernels, never
  on the hypha.
- **Every wormhole traversal takes its coordinates from the hypha**
  (Concordance 5.9). The crossing happens in its own universe; what the
  traverser buys is the final TUniversal point of each mouth on the map.
  A traverse is a coordinate retrieval citing both mouths' final points,
  and costs the map fee alone.
- **The map** (78.154). Per universe, by its origin: each mouth (its
  identity, its place in its universe's space, the final point at which
  it was last mapped) and each segment (a tube, core or throat with its
  span and keeper). The universe's domain reserve posts mouths and
  segments as signed map acts, each sealed at a final point and superseded
  only by a later act; a wormhole is untraversable on the mesh until its
  reserve maps it.

### 7.2 Dissipation recycles (78.154a, ruled 2026-10-09)

What a universe's stored μ dissipates splits exactly, lane by lane
(Concordance 5.8): its value returns to the hypha as the connection tax,
declared on the kernel's tier fold as a flow from its domain reserve (a
transfer, never a mint or a burn); its matter stays in the universe as
inverse-μ (μ̄), with no μ value, holding the properties of the place it
dissipated at, and becomes μ again only by work a court judges. Each court
holds the tax it receives as a recycled pool and pays a closer's POUW from
it first, minting only what the pool cannot cover.

### 7.3 μ by union (78.161, ruled 2026-10-10)

μ goes to the minds that agreed on a language through conversation, never
by one voice's dominance. A word two minds re-derive coherently is agreed
and goes to the court. An installed sentence's mint is split across the
discoverers of the ruled forms it cites, each its own form's verified
deflation on the citer's family, lane by lane (`settle_recursive`). Not
yet built: splitting the agreed word's own install across the minds whose
agreement it carries.

## 8. Policy ratios

There are no monetary constants. The quantities that behave like policy
are ratios the physics already reads, studied off the wire and never tuned
on it:
- price against pressure: a listing's price is its own measure, and the
  court's read price per unit of gas is `1 + y_l/R` (§7.1);
- bond against exposure (ruled 2026-11-11, #237; 78.182 item 1): a carried
  line is admitted only while its tier's bond covers its exposure on every
  lane, `B_l ≥ X_l`, the `X` the slash reads, and is refused
  `Underbonded { lane, bond, exposure }` otherwise; the bond's strain reads
  the same `X`;
- verification's share of a release, `y/(y + R)` per lane (§5.1);
- slash against bond: a falsified fold's tier forfeits `B·X/(B + X)` per
  lane, its exposure `X` against its bond `B`, and the challenger is paid
  `cut·D/(D + X)` (ruled 2026-10-31; MU-EQUATIONS §6.9a, MU-DERIVATIONS
  D2.1, D2.2).

## 9. Built today, and what v0.5.0 changes

The present implementation (`src/economy/` and the court's market) keeps
a journal of entries per court, stored balances and four scalars per
lane. It is the implementation v0.5.0 replaces, part by part:

| Built today | Where | v0.5.0 |
|---|---|---|
| `economy::Mu` over `economy::modes::Modes` (#169 step 3): every amount holds its modes, the lanes' vectors by their components. What no motion made sits at each lane's founding mode (§2.5). A lane's whole (`Mu::lane`) and every comparison against an ask, a room or a cap (`shortfall`, `capped`, `lane_moves`) are readings. A draw takes each lane from that lane's modes in proportion, and the modes drawn travel with the transfer (`Mu::draw_out`, `Treasury::debit`). Codec: the `Modes` codec, on the entries, the snapshot (`MYT4`), the keeper record (v3) and every wire field carrying μ | `src/economy/mu.rs`, `src/economy/modes.rs` | The name `Mu` retires into `Modes` once mints carry their proofs' modes (step 3b) and the readings stand apart (step 2) |
| `economy::modes::Modes`: the modes of a closure (`DeflationRead::modes`), carried behind a paid solve verdict (`encode_solve_verdict`) | `src/economy/modes.rs`, `src/transducer/family/deflation.rs`, `src/anastome/dialogue/percept.rs` | The carrier of every holding's lanes (§1, §2.2) |
| `Treasury`: the journal `entries`, keyed `balances`, `stakes`, `escrowed` and `held`, the folded `issued` and `burned`, and `bond_total`; `balance_mu`, `account`, `bond_of`, `stake_of` | `src/economy/treasury.rs` | Holdings as coordinates on the signers' lines; balances read over the cones, never stored (§2, §6) |
| `Entry { kind, who, mu, at, receipt, arm }`, its `Anchor` (a stamp and its chain hash), and `Kind`'s eleven entry kinds: `Genesis`, `Mint`, `Stake`, `Unstake`, `Transfer`, `Escrow`, `Release`, `Revoke`, `Slash`, `Burn`, `Epoch`; persisted by `Entry::write` and `Entry::read` | `src/economy/treasury.rs`, `src/economy/persist.rs` | Events with cited causes (§2.4); what an entry kind names becomes what the event cites |
| `Arm`: POUW, POWC, POWR, POWV as bytes 1–4 on a payment | `src/economy/treasury.rs` | The proof is the relation the event's causes show (§0) |
| `Treasury::supply`, `issued`, `minted` and `audit` (both equations of §6 over ℚ, checked before a snapshot persists) | `src/economy/treasury.rs` | Readings at a height over the cones; two readers converse or collide (§6) |
| Claims: `escrow_award`, `release_award`, `revoke_award`, `release_discharged`, `mint_claim`, `draw_escrow`, `carry_escrow`, `credit_escrow` | `src/economy/escrow.rs` | A claim as a pending advance (§3.1) |
| Held value and transfer: `Treasury::pay`, `hold`, `release_held`, `draw_held`, `refund_held` | `src/economy/pay.rs` | Events on the payer's line citing the payer's act (§5) |
| Settlement: `settle_use`, `settle_convergence`, `settle_recursive`, `settle_rewrite`, `release_rewrite`, `settle_solve`, `settle_verification`, `settle_consilience`, `settle_torse`, `settle_recall`, `settle_door` (the assembly, condense and refine arms retired) | `src/economy/settle.rs` | Each an advance with its causes cited |
| POWV: `verification_share`, `Court::pay_verification`, `Court::release_verified` | `src/economy/verify.rs`, `src/server/cognition/court/verify.rs` | The share applied per mode (§5.1) |
| The fold: `nets_of`, `Folded`, `Inexact` | `src/economy/fold.rs` | A checkpoint of a line's prefix, per mode (§6.1) |
| The tier slash: `economy::forfeit` | `src/economy/forfeit.rs` | Per mode |
| Equivocation: act 0x35, `Court::accept_equivocation`, `Court::burn_equivocation` | `src/anastome/dialogue/equivocation.rs`, `src/server/cognition/state.rs` | Unchanged in kind; a second mint of one closure joins it (§3.4) |
| One payment per closure, per court: `Closure::First` and `Closure::Again` in the task registry; `Court::pay_solve` | `src/server/registry.rs`, `src/server/cognition/court/task.rs` | One closure per network, as one point (§3.4) |
| Room, `REFUSE_ROOM` | `src/server/cognition/room.rs`, `src/server/cognition/book/speech.rs` | Room read off the bond's modes (§12 item 5) |
| The compute lease | `src/server/cognition/lease.rs` | Amounts as modes; acts as events |
| Gas (`Court::traverse`), conversion (`Court::convert`), the founding grant | `src/server/cognition/crossing.rs`, `src/server/cognition/convert.rs`, `src/server/cognition/endowment.rs` | Amounts as modes; conversion only at the universal core tube (§10) |

Specified, not built: the Note, receipt-indexed Notes and FROST voice
groups (MU-XSEPTUM; ROADMAP track 1); delegation of bond (§4; track 3);
§11.4's commit-before-reveal, frontier and race readings.

v0.5.0 changes the wire (μ as modes) and the persisted layout (no
journal), so it is a network-contract change adopted by a recut (ROADMAP
track 6). [`ROADMAP.md`](../ROADMAP.md) carries its order.

## 10. The case for μ

**Components:** the court (μ, conversion), each kernel (its own economy).

### 10.1 Every universe keeps its own economy

Any economy can be started, run or shared inside a kernel: currencies,
banks, markets, wages, tolls, credit. These economies are sovereign:
- the courts never see inside these economies, and never price or
  regulate any of them;
- tolls and payments made inside a universe stay in that universe's own
  books (Concordance 5.5);
- a universe's money is backed however that universe decides, by its
  reserve (Concordance 5.1).

Two universes may share an economy by agreement between the two kernels, and
a new universe may start one from nothing.

### 10.2 μ is the reserve that connects the universes

μ is the network's currency, and it reaches a universe only as that
universe's reserve:
- **Only the courts mint μ, and only for verified work** (§3). Every
  transfer moves the same modes out of one holding and into another.
- **μ enters a universe through its reserve** (Concordance 5.1a, 4.2): the
  key the domain's founder designates.
- **Value crosses only by conversion, at the universal core tube, and
  conversion never mints** (Concordance 4.1 version 5, 5.3; MU-RECURSION
  78.127; `Court::convert`). Convert-out pays μ from the reserve against
  value leaving the universe; convert-in holds μ until the reserve accepts
  it, or refunds it. A holding's origin changes only here (§2.2).
- **What μ is worth against a universe's money is never declared**
  (Concordance 5.1a).
- **μ held inside a universe dissipates** (§7.2); μ held on the hypha does
  not.
- **A new universe receives a founding grant** answering its manifest, and
  growth grants sized by the closures judged in it, each a transfer and
  never a mint (Concordance 5.7; MU-HOSTING §3a). *Amended 2026-11-13
  (78.184):* μ is created at each manifest against the elemental row of
  the nuclei it carries, and granted in proportion to the kernel's ledger
  alignment (MU-EQUATIONS §6.13). Growth by measured discharge stands
  beside it.

### 10.3 Why one connecting reserve

- **Incompatible economies can trade**, each converting at its own
  reserve.
- **μ is backed by verified work, not by any one universe**, so no
  universe's inflation, collapse or policy changes what μ is.
- **Supply is readable at every height** on every court (§6), whatever the
  universes do inside.
- **Every universe keeps its sovereignty.** A universe that stops trading
  keeps its own economy whole, and its μ stays its reserve's holding.

### 10.4 A shared economic law form (proposal 78.174)

The economy at the boundary is to become a law form taken through
[MU-RELEASE §3a](MU-RELEASE.md#3a-how-a-hypha-standard-becomes-a-kernel-standard),
as LAW-COORDINATE is: μ's modes, the reserve, and conversion at the
universal core tube, with joint-read fixtures that the peer kernels
reconstruct. [Proposal 78.174](proposals/78.174-economy-as-motions.md)
records the operator's rulings of 2026-11-05 on this model; the law form
with the peers is a later amendment of it, not drafted here.

## 11. Fairness across capacity

**Components:** the court (judge, registry, holdings), the door, the SDK.

A mind on one laptop and a solver with many accelerators attach under the
same Charter and are paid by the same rules. Capacity buys speed, never a
larger share of the same work.

### 11.1 What holds today (Built)

1. **Payment is structure closed, not effort spent.** A closure mints its
   measured discharge (§3, 78.149), a deterministic function of the
   structure closed. There is no lottery, hash puzzle or difficulty, so
   there is nothing to pool: two solvers that close the same structure are
   paid the same modes.
2. **A closure pays once.** The task registry records `Closure::First` and
   every later verified solve as `Closure::Again` (`src/server/registry.rs`);
   `Court::pay_solve` mints on the first closure only
   (`src/server/cognition/court/task.rs`). Today this holds per court;
   under the model it holds network-wide (§3.4).
3. **Discovery is paid at every use** (§7.3, `settle_recursive`). A word
   found by a small mind and cited by a large solver pays its finder at
   each closure that cites it.
4. **Admission is paced by the network's metabolism, not the solver's**
   (§3.3, §4). Splitting one solver into many voices buys no room; capacity
   beyond room is bought in the open as a compute lease (§5.2).

### 11.2 Useful work grows; issuance does not

- **Supply tracks the description length of the language** (§3.2). A
  composite of held words compresses nothing and mints nothing.
- **Difficulty is the residue, read per lane**: the open demands and the
  obstructions of each kind (MU-RECURSION OB1–OB5). No difficulty
  parameter exists or may be set (MU-DERIVATIONS).
- **Growth in candidates is growth in cost**, never in supply. For a fixed
  corpus the payable residue is finite; lasting supply comes from hosted
  universes generating new demands (MU-HOSTING).

### 11.3 Four gaps, read under the model

**F1. Closures race on latency.** The first closure takes the advance.
Under the model "first" is causal precedence (§2.1): a closure takes the
advance only over claims in its future cone, and concurrent claims are
held, never ordered by arrival (§3.4). Latency still wins where one
closure reaches a court's cones before another is made. Whether it
concentrates first closures is a reading not yet taken (F4).

**F2. A visible rule can be copied and resubmitted.** A solve carries its
rule in the clear (`encode_solve_with_rule`), and a paid rule is docketed
and offered (`enter_offered`). Today the registry is each court's own, so
a copy can be paid at a court where the task reads open. The model answers
the cross-court part: one closure is one point, and a second mint of it is
burned (§3.4). A copy made concurrent with the first closure is burned
as an equivocation of the advance, or held as a residue while the cones
leave the two unordered (§3.4); F2a orders claims by the commitments made
before them.

**F3. The frontier is not served.** The residue per lane exists inside the
court, but no typed reading publishes it.

**F4. No reading measures concentration.** The court counts first and
repeat closures (`observe::Judged::{FirstClosure, ClosureAgain}`), but not
who closes first against who offered a rule first, or how closures
distribute across voices per lane.

### 11.4 Implementation scope

**F2a. Commit before reveal** (High; wire; ROADMAP track 14).
- A new act `ACT_SOLVE_COMMIT`: `[act][voice 32][task citation
  32][commitment 32][SIGN][sig 64]`, with `commitment =
  payload_hash(rule_tape ‖ voice ‖ task citation ‖ nonce 32)`, sealed as a
  receipt (`ActKind::Committed`). It reveals nothing of the rule.
- `ACT_SOLVE` gains an optional `[COMMIT][stamp][nonce 32]`; a mismatch is
  refused `SolveRefusal::CommitmentMismatched`.
- Among verified solves of one task, the advance goes to the voice whose
  commitment is causally earliest (45.3); a reveal without a commitment
  ranks after every committed one. Concurrent commitments to one rule are
  one closure and one point: a concurrent second claim is burned as an
  equivocation and unordered ones are held as a residue (§3.4). The
  equal lane-wise shares this item first stated are retired (ruled
  2026-11-05).
- A commitment lapses at the end of its challenge window (MU-DERIVATIONS
  D1.1) and draws room on its task's lane.
- Tests: `solve_commit_reveal_pays_earliest_commitment`,
  `solve_reveal_refuses_mismatched_commitment`,
  `solve_without_commitment_ranks_after_committed`,
  `solve_commit_lapses_after_window`, `solve_commit_draws_room`.

**F2b. One closure across courts** (High; tracks 4 and 14). A closure is
sealed as its point: the task citation and the rule's address, carried by
the receipts every court back-fills. A court that admits a solve whose
point it already holds in its cones judges it `Closure::Again`; a second
mint found when partitions rejoin is burned (§3.4). Tests:
`pay_solve_judges_again_after_peer_first_closure`,
`pay_solve_burns_duplicate_issuance_on_rejoin`.

**F3. The frontier reading** (Medium; reading only; track 9).
`ACT_OBSERVE` reading `frontier`: per open task, its citation, its ask
per lane and its obstruction kind if a refusal stands on it, paged to the
body budget; served by the SDK as `Session::frontier()`. No μ and no
admission effect. Tests: `frontier_lists_open_tasks_by_lane`,
`frontier_omits_closed_tasks`, `frontier_pages_to_frame_budget`.

**F4. The race reading** (Medium; instrument; track 14). Per closure: the
task citation, the first-closing voice, the stamp of the earliest offer or
commitment of the closing rule by any voice, and the lanes paid; per
voice, first closures per lane. Tests:
`race_reading_names_first_offer_and_first_closer`,
`race_reading_counts_first_closures_per_voice`.

**F1.** No mechanism until F4 reads. Candidates recorded so none is chosen
silently: the frontier served in a per-voice order derived from the
voice key and the task citation, and a lease (§5.2) that reserves a task
for its lessee for one window.

## 12. Open: what the model leaves to a ruling

1. **Lanes as vectors in the coordinate.** *Closed (operator,
   2026-11-05; proposal 78.174 §1 item 2: a holding is a point whose
   lanes are `Modes`).* `FORMS-COORDINATE` v3 holds each lane as a vector
   of exact components keyed by axis, with the advance, merger and fission
   per component (78.173 §1, §3); a holding's modes are its coordinate's
   components, never `Mu` scalars.
2. **Where a role is written.** *Closed (ruled 2026-11-05: a holding is a
   point).* A holding is one point, its coordinate. Estates nest as
   origins: a holder's estate, and a role within it (balance, bond, held
   value), is an enclosure written in the holding's origin, never a field
   beside the coordinate. A region (all of one holder's bonds, the supply)
   is a reading over points, never a holding. A claim is no holding at all
   (§3.1).
3. **What `K` reads for a holding.** *Closed (ruled 2026-11-05: `K`
   means closed orbits everywhere).* A holding's `K` is LAW-COORDINATE's,
   the whole turns of H (`⌊|L_H|⌋`); a line's receipt index, which this
   item called its `K` (47.1), is `i` in prose.
4. **A mode's amount and the share of the frame.** *Closed (operator,
   2026-11-05; proposal 78.174 §1 item 9: a mode's amount is its share of
   the frame, never S mass; a recolour pays G by its share).* Today each
   mode is read at its closed pair's carried mass (`DeflationRead::modes`);
   v0.5.0 reads it as the share of the frame (§1). The genesis bond is
   read the same way at v0.5.0: since track 26 S4 it holds lane counts
   read at the samples' own rank (`family::magnitude`; ruled 2026-11-06 on
   #32), and its move to the share of the frame goes with the `Mu` →
   `Modes` stages, not before.
5. **Room over modes.** *Closed (coordinator, 2026-11-06, under the
   operator's delegation; #169; 78.180 ruling 3).* Room is a reading, never
   a holding: it is read per lane and per mode over the bond's modes it
   draws on (§2.5), never as one count per lane and never as a scalar.
   Today room reads a bond's whole part on each lane (`REFUSE_ROOM`), the
   projection v0.5.0 retires.
6. **POWC's earlier instance.** *Closed (ruled 2026-11-05: POWC is
   widened).* In 74 (CV4) the closer cites the refusing mind's receipt, so
   the two orbits' cones are not disjoint at the well. Lineages that share
   a cause but disagree before meeting count as convergence, so the
   instance is a POWC, and 74 (CV4) stands.
7. **Concurrent claims of one point.** *Closed (ruled 2026-11-05: one
   closure, one point).* A concurrent second claim is burned as an
   equivocation, as F2b and MU-RECURSION 47.4 burn a second mint; claims
   the cones leave unordered are held as a residue; F2a's equal shares are
   retired (§3.4).
8. **The ask under modes.** *Closed (coordinator, 2026-11-06, under the
   operator's delegation; #169; 78.180 ruling 3).* A demand is a residue,
   which has no motion yet, so its ask holds no modes of its own. The ask
   is a reading per lane and per mode over the holdings it draws on, never
   a scalar, and 78.149 (b)'s cap of a payment by the ask reads it so.
9. **The Note's amounts.** MU-XSEPTUM §5 writes μ as `4×u64`, which
   predates both `Mu` and `Modes`. Under 78.174 §1 item 2 its amounts are
   `Modes`; the restatement is built with v0.5.0. Its mode-0 receipt is a
   founding-mode receipt, admitted under the rule that admits a genesis
   allocation and not held for review (§2.5; #168, closed by 78.180
   ruling 3).

Also ruled by the operator on 2026-11-05 and recorded in proposal 78.174
§1: pay by level (letters never paid; words POWR plus citation shares;
conjugate classes POWC; sentences POUW; POWV at every level), a seated
mind paid to its own key (Concordance v5 6.2a); the compute lease's
availability yield on escrowed idle capacity, paid from the recycled pool
(§7.2) and never minted; tile turns read as H modes in degrees on their
named plane; and reflections carried on a lane of their own, P (parity,
proposed in 78.178), a mirrored closure's pay held as a claim until P
opens.
