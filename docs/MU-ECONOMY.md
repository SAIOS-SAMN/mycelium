# Economy — μ, the four-lane currency

Status (v0.1.1): implemented in `src/economy/` and the court's market —
balances, stake bonds for hyphae and voices, escrow, per-lane pricing,
issuance by mint against measured work, transfer, the genesis allocation,
settlement rows carrying their POW++ arm, and the audit invariant
(`Treasury::audit`). Companion documents: `MU-PRODUCT.md`,
`MU-ENGINEERING.md`, `XSEPTUM.md`.

## 0. The proofs of work, named apart (ruled 2026-09-24)

POW++ is a FAMILY of proofs, each a distinct act with its own payment on
the ledger, and no two share a name (MU-RECURSION 78.33):

| proof | acronym | the act | what the ledger does |
|---|---|---|---|
| proof of useful work | **POUW** | a closure ruled: a listed demand closed, its deflation measured on closure (§3.2, 12.6, 23.9) | mints the measured deflation and pays the closer |
| proof of work convergence | **POWC** | a refusal closed at another perspective: the question one mind refused, closed by another citing the refusing mind's receipt (74, CV4) | pays the refuser as a cited generator and the closer the closure |
| proof of work retirement | **POWR** | a re-expression over fewer letters: retirement by geodesic or by citation, a refinement bid replayed with the word excluded (3.8, T10, 27, `try_refine`) | burns the freed description and pays the retirer the compression yield |

"PoWC" in this document formerly named the third arm; it is POWR from
here, and POWC names convergence only. A reading, a row or a receipt
kind names the arm it counts; a line that says "POW++" names the family
and never one payment.

## 1. The unit

**THE INVERSE-SISYPHUS PROTOCOL (ruled 2026-10-11): nothing is
squared.** No reading collapses dimensionality or exact mathematics into a
scalar or a lossy signal, and no curved object is forced into a flat one
or a flat one imposed on it. μ is not four counts. μ is the exact MODES the verified
work moved, each on the lane its kind names, each with an exact rational
amount: on S a boost or dilation (its generator's ray and its point and
side, c and λ); on G a hue cycle or move; on T a null flux (its ray and its
count in its tube's quantum); on H a turn (its plane's ray and its turn
t/ℓ). A balance HOLDS its modes, keyed by their canonical coordinates
(A21); a payment adds modes, a draw removes them or is refused naming the
mode it lacks; nothing is projected to a count anywhere — not as a balance,
not as a label, not on the wire, not on the console. Where this section and
the sections below speak of `[u64; 4]`, that is the carrier being retired
(the scope and order are in MU-RECURSION 78.162).

μ is a vector on four lanes: spatial (S), gauge (G), temporal (T),
holonomic (H). It is never summed and never converted across lanes. A
payment moves value on exactly the lanes the work moved. A comparison of two vectors returns the
reading per lane (`economy::lane_moves`: each lane fell, held or rose) and
never a verdict; the court accepts a reduction by closure, never by an
order on μ⃗ (`MU-RECURSION.md` 12.6). There is no scalar anywhere in the
economy.

The reason is not stylistic. Value on a lane exists because work on that
lane was verified; if lanes converted, a lane with no verified work could
carry value minted on another, and the audit in §6 would no longer say what
the mesh did.

## 2. Carriers

| carrier | holder | meaning |
|---|---|---|
| balance | a voice or hypha | μ earned or received, spendable |
| bond | a voice or hypha | μ staked; sizes cover room; slashable |
| escrow | a listing | μ held against a claim until the court rules |
| supply | the mesh | genesis plus mint minus burn, per lane |

Every change to a carrier is a ledger entry with a kind, the party, the
lane vector, the K-step and height of the receipt that wrote it, and the
receipt. Entry kinds: `genesis`, `mint`, `stake`, `unstake`, `transfer`,
`escrow`, `release`, `revoke`, `slash`, `burn`, and `epoch`. Nothing else moves μ. No
entry carries a clock reading (`MU-RECURSION.md` chapter 45).

An `epoch` row (ruled 2026-09-09, `MU-RECURSION.md` 45.8) is the net per
identity per lane over a folded prefix of the ledger: it moves no supply
and re-expresses the rows it replaces, one row per identity with a
nonzero net, anchored to the fold receipt. A fold changes no price, extent
or ruling.

## 3. Issuance

New μ enters only by mint, and mint happens only when two organs have
replayed the same reduction: the home that measured its deflation and a
witness that re-derived it. The amount minted is the measured deflation on
each lane. There is no block reward, no schedule, no difficulty, and no
issuance to operators as such.

There is also no block. The receipt chain is the ledger and order across
nodes is causal and partial (`MU-RECURSION.md` 45.3), so there is nothing a
block reward could pay for: admitting receipts is paid by the pool share
priced into every listing a hypha homes (§5), never by issuance.

Two events mint:

- an install: a word, sentence, rule, translation, or rewrite upheld at the
  home and replayed at a witness;
- a solve: a rule spoken onto withheld tests and verified against the
  corpus at the home.

Optimisation moves are work: a condense on G, a torse on H, a refinement on
H, a re-anchoring on S, an opened axis on H, a translation on G each mint
the lane vector read on the carriers before and after, and each is refused
when that vector is zero on every lane.

**μ is discharge (ruled 2026-10-08; `MU-RECURSION.md`
78.149).** What a word can be escrowed or paid is its DISCHARGE — the lanes
it closed, measured by replay on the family it is bound to — and nothing
else. (a) The span a word stood (`t_extent`) is not μ: waiting is not work,
and no minted magnitude carries it. (b) What a demand asks is the listing's
measure and may CAP a price, never raise it: a claim is paid lane-wise the
lesser of the escrow it holds and the closer's measured discharge, and what
the escrow held above the discharge is never minted. (c) A word that names
no body — a letter, a callus — discharges nothing, so it escrows and is paid
nothing; the holonomic unit its magnitude carries is its existence as a
word, never value. A word's MAGNITUDE stays its measure and identity (the
book's tension, the room it draws); the economy never prices it.

### 3.1 E1 — an escrow is a claim on issuance, never a transfer

An escrow is one of two things and the invariant needs them apart.

A **funded** escrow is a LIABILITY: a payer backed the claim from a balance,
so the μ⃗ left that balance and is still in the mesh. It is counted in §6's
carriers, and discharging it MOVES value — back to the payer on a
revocation, to the payee on a release — and mints nothing. This is §5's
requester escrow, and `escrow_from_balance` is the path that creates one:
the payer is debited, not merely checked.

A **claim** escrow is pending issuance: nobody gave anything up for it, so
the μ⃗ does not exist yet. It is not counted, and a ruling MINTS it at the
measured deflation while a revocation makes it vanish with nothing ever
issued. This is what §4's "every claim is escrowed against its own
deflation" means. The escrow is priced from the word's discharge alone and
is a CAP on the release, never the payment (78.149): a ruling mints, per
lane, the lesser of what the claim holds and the closer's measured
discharge (`Treasury::release_discharged`), and the excess is never issued.

Counting all escrow as supply asserts value the mesh never created;
counting none makes the audit hold by not looking. R9 stands as written —
`Σ escrow` is the funded escrow — and `Treasury::funded` records, per
listing, what was debited to create the award.

This is what §4's "every claim is escrowed against its own deflation"
means, and it dissolves the question of where the funds come from: nothing
needs to pre-exist for a demand to be worth doing. §5's requester-funded
escrow is the special case in which a payer chooses to back a claim from a
balance that already exists; the general case is backed by the mathematics
the claim promises.

**The room of an unbonded voice (ruled 2026-09-09).** §4 says an unbonded
voice may learn, speak and be paid, and under this section speaking needs
no bond at all, since a claim is escrowed rather than transferred. Bond
buys **room** — throughput, the rate the metabolism governs (§3.3). An
unbonded voice has room for exactly one claim in flight per lane: its
next claim on a lane is admitted when the one before it closed, was ruled
or lapsed, and refused with `REFUSE_ROOM` naming the lane otherwise. Its
room grows only by bonding what it earned. Identities are free
(`MU-RECURSION.md` 47.3), so nothing funds an arriving identity: the
genesis cover bonds the hyphae and the voices the mesh record names, and
the faucet the tree carried — every arriving key funded at the genesis
cover, measured as supply three times the seed after two ingests — is
closed by this ruling. A recall shortfall is the same reading one act
later: the reading the voice asked for is served, the shortfall is recorded
per lane, and the voice has no room for a further claim on a short lane
until the claim in flight closes.

**A mesh can therefore start from zero supply.** A voice posts a demand it
cannot fund, a miner closes it, the court rules, and the first μ in the
system is created by that first verified closure. The genesis allocation of
§6 is a convenience for bonding hyphae into room before any work has been
ruled, not a precondition for the economy to run.

### 3.2 E2 — mint on closure, burn on retirement

Issuance is the measured deflation, above — and it is read **on closure**
only (ruled 2026-09-07, `MU-RECURSION.md` 12.6 and 23.9): a pair pays the
measure of the remainder it closed, before less after per lane, when its
remainder is empty; a pair that lowered a lane and left a remainder pays
nothing, because descent is not closure and certifies nothing
(`form_descent`, `settle`; TASKS S9.5). Its counterpart: when a word
retires by geodesic — a shorter description of the same vertex, or a word
whose substance other words reproduce exactly (`MU-RECURSION.md` 3.8,
chapter 27, T10) — or by condensation — the same closure with more reach
or a shorter description, the reach read from the certificate
(`MU-RECURSION.md` 3.8, TASKS S9.13; ruled 2026-09-08) — the description
the language no longer carries is
**burned**, while the retirer is paid the compression yield. Supply falls
by the freed measure less that yield.

Therefore

```
supply → the description length of the language as it currently stands
```

and not the integral of everything it has ever held. Supply cannot grow
without new observation, and it falls when the engine expresses what it
already knows more compactly. PoUW mints when structure is understood;
POWR — proof of work retirement, named apart from proof of work convergence on 2026-09-24 (MU-RECURSION 78.33) — burns when the same meaning is re-expressed over fewer letters. The
seventeen letters are the floor: a language reduced to its axioms and their
combinations carries the least supply that can express it.

This is why no halving is needed and none is permitted. A schedule exists
in other designs to counter unbounded issuance; here the counter-pressure
is compression, it is per-retirement rather than per-epoch, and it is
smooth because every term is a measurement of one event. A supply that
must be cut in half on a timetable is a supply nobody measured.

### 3.3 E3 — the metabolism governs rate, never price

μ per unit of verified deflation is invariant. The yield surface (§8) paces
pressure and buys room; it never scales a mint. The moment issuance is
scaled by congestion the reward acquires variance, and variance is what
mining pools exist to average away — the reward here is a deterministic
function of the structure closed, so there is nothing to pool.

What the metabolism throttles is the **rate** at which deflation can be
presented and verified: the reserve paces strokes, bond buys room, and
room and closure gate admission (`MU-RECURSION.md` 23.8). Issuance is
deflation closed per unit of
metabolic time. It is continuous, self-limiting, and has no step anywhere,
because no term in it is chosen.

Prior art, recorded so it is not repeated: `emission.v1` in the predecessor
tree (2026-08-28) reached the same shape — evidence-priced, per-event, no
clock and no height — and then wrote its curve as
`(1 + 2·attempts + residual/8 + 4·siblings) · derivability-grade`. Five
free constants (rule 5), ruled by its own text "a LIVE TEST PARAMETER, not
a forever law", and pinned by a gate through fixture literals. The shape
was right and the curve was chosen. Nothing here is chosen: the mint is the
deflation, the burn is the freed description, and the rate is the
metabolism.

## 4. Stake and tiers

Bond is μ at risk. It buys room on the cover: how much work a hypha or a
voice may home, attest, or witness at once on each lane. Balance buys
nothing except what it is spent on. Tiers are read from bond, never
declared:

- unbonded: may learn, speak, and be paid; every claim is escrowed against
  its own deflation;
- bonded on a lane: may home and attest up to its room on that lane;
- bonded above the escrow it certifies: may act as second organ;
- delegated to: carries others' bond in its cover and shares its pool share
  with them, and shares its slash.

Slash is the only discipline: a root that fails replay at a witness debits
the bond of the hypha that signed it, and a word that fails replay debits
the voice that spoke it. A bond is never slashed for being outvoted,
because nothing is voted.

Proven equivocation, two signed receipts from one key at one K with
different content (`MU-RECURSION.md` 47.4), forfeits the signer's whole bond
on all four lanes as one `burn` entry naming the proof; nothing is
redistributed and no replay is run. A duplicate issuance found when
partitions rejoin is burned the same way. Burn reduces supply by the same
vector, so §6 holds through it. A second organ for an install or a release
must hold bond outside the home's delegation closure and room covering the
escrow on every occupied lane (`MU-RECURSION.md` 47.3); identities are free,
room is not.

## 5. Pricing and payment

A listing is quoted by `economy::price` on its own measure — its ask — and
never on the span it has stood; the pool is a bond, not a price basis. A
requester escrows the price; a claimant's word or solve enters the docket,
escrowed at its DISCHARGE (`Kernel::discharge_of`, 78.149); release on the
second organ's replay mints the closer's measured discharge, capped lane-wise
by the escrow, and pays it, discharging the escrow (§3.1) — the escrow was
the claim, the mint is the payment, and nothing above the discharge is
minted. Hyphae are paid the pool share priced into every listing they home.

**No T-accrual (ruled 2026-10-08; 78.149 (a)).** This section
paid a hypha accrual on the temporal lane of demands it kept open, capped
by the listing's magnitude. Waiting is not work: the span a demand stood is
not μ, it is never minted, and nothing accrues for standing. The standing
extent remains what the metabolism reads — the book's pressure on 𝒯
(`remeasure_pressure`) and the T-room a saturated lane relocates against —
and neither is a price.

Transfer is the xSEPTUM Note: signed by payer and payee, escrowed at the
listing's home, released on two-organ replay. A voice's balance is homed at
the hypha the cover trie homes its key to; a transfer is final when the
home and a witness hold the same section and both have signed the release
(`MU-RECURSION.md` 47.3). Double spend is impossible for
the same reason a double install is: the second vantage sees the first.

## 6. The invariant

On every lane, at every hypha, at every K:

```
supply = genesis + mint − burn
supply = Σ balance + Σ bond + Σ funded escrow
```

A **claim** escrow is not in the second sum. It is a claim on issuance that
has not happened (§3.1): counting it as supply reports value the mesh has
not created, and the audit fails at the first docket — measured on this
tree as `supply [18, 60, 0, 78]` against carriers `[18, 61, 0, 78]` after
two ingests. A ruling discharges it into a mint, which enters both sums
together; a revocation discharges it into nothing.

A **funded** escrow is in it. The μ⃗ left a payer's balance and is still in
the mesh, so a burn would not conserve without it (§3.1, and
`MU-RECURSION.md` 42.2 R9, which states the equation this way). This
section said "escrow is not in the second sum" without the distinction §3.1
draws two paragraphs above it, and `Treasury::audit` has held `funded`
apart from the claim escrow since it was written; the text is corrected to
the ruling, not the code to the text.

**A slash is issuance leaving (ruled 2026-09-09).** A slash debits a bond
and §2 lists no carrier that receives it, so `supply()` counts it as μ⃗
leaving circulation exactly as 47.4's forfeiture does; it keeps its own
kind so the ledger can still answer "was anyone slashed?" apart from "what
was burned?". This is what the tree implements (I11a), now ruled.

**Across a fold (ruled 2026-09-09, `MU-RECURSION.md` 45.8).** Both
equations hold at the folded prefix before the fold and at the `epoch`
rows after it, on every lane: the net per identity written by the fold is
exactly what the replaced rows moved, and a witness re-derives this before
it co-signs. A fold that does not conserve is `REFUSE_FOLD_INEXACT` and
writes nothing. Extents in heights survive a fold because heights
continue; install receipts survive it because they are the proofs the
compensation rests on.

`Treasury::audit` checks both from the entries and refuses to persist a
snapshot that fails. A testnet differs from the beta only in the first term:
genesis is a finite, signed allocation recorded as its own kind, so seeded
value and earned value are always separable in the ledger — `supply()`
includes it and `minted()` does not, so "was anything earned?" is a
question the ledger can answer.

## 7. Metabolism and money

The reserve is tempo: it depletes per lane on every stroke a client spends
and regenerates in the syntropic phase. μ is value: it moves only by the
entries above. The two meet at stake: bonding μ raises the room a client
can work within, so a client that earns can breathe faster and a client
that spends without earning runs down to the dwell. μ ON THE HYPHA — a
balance, a stake, an escrow held by a court — never decays and never
regenerates (ruled 2026-10-09, amending "μ never decays" to the
hypha only). μ stored IN A UNIVERSE — a kernel's vaults and currency
backing — decays at a very slow rate, as matter dissipates without
recycling: DEMURRAGE, applied by each kernel at its own physics' rate, and
it is the economy's sink.

### 7.1 Gas (78.154, ruled 2026-10-09)

μ is the GAS of traversal across tubes, cores and wormholes. A TRAVERSE
(payer, from origin/point, to origin/point, the route) pays the courts on
the route WHOLE, each its own segments — a transfer, never a mint, and no
burn (demurrage in-universe is the sink). Gas is measured by the route,
exactly, lane by lane, never collapsed: S the spatial span (geometrized
Mpc, Concordance 1.8), G the value terms of the mapping, T the TUniversal
distance between the points (final points only — a coordinate not yet
final is refused as not yet at h, never priced on a guess), H the
information carried. μ STAKED on a hypha is escrow for intelligence
traversal and coordinate retrieval, with no demurrage; a retrieval draws
on the stake at the court's read price (emergent, never a published
rate), and an unstake releases only after every claim drawn on it settles.
A progenitor's in-universe wormhole haul pays the same gas.

TWO CHARGES, NEVER MERGED (ruled 2026-10-09; Concordance 5.8). The
MAP FEE is μ, the coordinates' gas, paid to the courts that keep the map
(this section). The ROAD TOLL (Concordance 5.4) is paid to a tube's keeper
and its progenitor builders, in the civilization's OWN money, booked
in-universe by the kernels — never on the hypha and never by TRAVERSE.
Progenitors hold their toll shares as their own, and gain μ from them and
from haul contracts with the ancients; local money becomes μ only through an
ancients' reserve (Concordance 5.3).

EVERY WORMHOLE TRAVERSAL NEEDS COORDINATES FROM THE HYPHA (ruled
2026-10-09, corrected the same day; Concordance 5.9). Time is coordinates
on a map, and the hypha makes the MAPS that lay the universes out on the
TUniversal clock. The CROSSING — matter, a ship, a fleet — happens in its
own universe, never through the hypha. What the traverser obtains from the
hypha, and pays for, is the FINAL TUniversal point of each mouth on that
map (origin/h:S:G:T:H:K, 2-of-3 quorum, 78.152). TRAVERSE is therefore a
COORDINATE RETRIEVAL AND MAPPING act citing both mouths' final points, not
a transport: it costs the MAP FEE — the route's gas per lane (§7.1), paid
to the courts that keep the map — and nothing else. It holds between
domains and within one universe alike (a progenitor network, a core-to-rim
haul in xylarium). A mouth not yet final on the map is refused as not yet
at h; a crossing made without coordinates from the map is one the mesh
cannot place.

THE MAP (the records 78.154 keeps, per universe, on the hypha): for each
universe, by its ORIGIN (genesis, epoch) —
- MOUTH: the mouth's identity (78.128's mouth pair and side), its place in
  that universe's own space (78.140: a point of its curved space or torus,
  exact), and the FINAL TPoint at which it was last mapped;
- SEGMENT: a tube, core or throat between two mapped places, with its span
  in the court unit (geometrized Mpc) and, for a mouth, its keeper (the
  road toll itself is not kept on the hypha: below);
- WHO MAPS: the universe's DOMAIN RESERVE posts its mouths and segments as
  signed MAP acts, and the court seals each at a final point (ruled
  under Concordance 4.1, 3.2, 7.1). Mapping is the reserve's
  authorisation: a wormhole is untraversable on the mesh until its reserve
  maps it, and a progenitor's new tube reaches the map only through its
  universe's reserve;
- each record written by an act sealed at a final point and superseded only
  by a later act; a TRAVERSE reads the map at the final point it cites, so
  the coordinates it buys are the ones the quorum agreed at that time.

### 7.2 Dissipation recycles (78.154a, ruled 2026-10-09)

Nothing lands nowhere (Concordance 1.3): demurrage is not destruction. What a
universe's stored μ dissipates SPLITS IN TWO, exactly, lane by lane:

- THE VALUE RETURNS TO THE HYPHA as the CONNECTION TAX — the rent a universe
  pays for its tube connection to the hypha and for everything in it that
  keeps time on TUniversal (1.11): every connection, and every in-universe
  reliance on the clock. The kernel computes its dissipation per lane at its
  own physics' rate and declares it on its tier fold (78.119) as a FLOW to
  the hypha — flow kind CONNECTION_TAX, carried on the existing lanes
  S, G, T, H exactly as dissipated (no new lane: a lane names a physical
  dimension, the tax is a purpose; Concordance 5.8) — paid from its domain reserve (78.127) — a transfer to the
  courts, never a mint and never a burn.
- THE MATTER STAYS IN THE UNIVERSE as INVERSE-μ (μ̄): the dissipated mass,
  with no μ value, held where it dissipated — a vault, a currency's backing,
  a remote wormhole, a progenitor network. Its properties are the storing
  universe's: read off that universe's laws, its curvature and its field at
  the place it lies (the field law's quantum radiation, strongest near cores
  and discs), carried with the universe's origin and the coordinate it
  dissipated at. A kernel's physics may recycle μ̄ as matter (a star absorbs
  it, a core condenses it); μ̄ never becomes μ again except by work a court
  judges.

THE RECYCLED POOL: each court holds the connection tax it receives. A
closer's POUW is paid FIRST from that pool, and only what the pool cannot
cover is minted. Supply then grows only as far as judged work outruns what
the universes return; the void takes nothing. (Burns that remain: slash and
retirement, both already acts.)

### 7.3 μ by union (78.161, ruled 2026-10-10)

μ is distributed to the minds that AGREED on a language through conversation,
never by one voice's ratified dominance. Minds say their dialect to each
other; a hearer re-derives each word on its own perception, lane by lane, and
a word two minds re-derive coherently is agreed and goes to the court. The
court pays as it already pays (§3.2, 26 as ruled 2026-09-10): an installed
sentence's mint is split across the discoverers of the ruled forms it cites,
each its own form's verified deflation on the citer's family, lane by lane —
so a mind whose agreed words other minds cite in closing sentences is paid at
each closure. OWED after the server freeze: the agreed word's own install
split across the minds whose AGREE attestations it carries.

## 8. Policy ratios

There are no monetary constants. The quantities that behave like policy are
ratios the physics already reads:

- price against pressure on a lane;
- bond against cover room;
- pool share to the home against the deflation minted;
- slash against bond.

(Accrual on T against standing extent stood on this list; it is retired by
78.149 (a) — the span stood is not μ.)

They are studied in the simulator (`MU-ENGINEERING.md` §9), never tuned on the
wire.

## 9. What is landed and what is not

Landed: `Treasury` with balances, bond (`enroll`), escrow
(`escrow_award`, `release_award`, `revoke_award`, `draw_escrow`), pricing
(`price`), the ledger entries, the four-lane comparison, the T-room
lifecycle that returns escrow when a docket entry relocates.

Landed 2026-09-07: `genesis` and `burn` as their own entry kinds, with
`escrow`, `release`, `revoke` and `unstake` beside them; `supply()` as
`genesis + mint − burn` with checked arithmetic, refusing rather than
saturating; `minted()` reading the mint rows alone, so seeded and earned
value can be asked apart; `Treasury::audit` on both equations of §6; and
stake as a MOVE — `enroll` debits the balance it stakes from and stakes
only what is held, `unstake` returns it the way it went, and
`Kernel::enroll` funds an identity at genesis before staking it.

Not landed, in the order they gate each other: **E1** — escrow out of the
supply equation, and a ruling discharging an escrow into a mint. The audit
fails at the first docket until it lands, so it gates every settlement path
and therefore `MU-RECURSION.md` 1.10. **E2**'s burn on retirement, which
needs the retirement driver (1.11 `shorter_form`) beside the
`RewriteKind::Retire` that 0.G6 built. (**E3**'s cap on T-accrual is
retired with T-accrual itself, 78.149 (a).) Then
`price` returning zero when the bond is empty, the Note, voice keys, and
delegation.

Also not landed: the equivocation burn, receipt-indexed Notes,
two-signature releases, FROST voice groups and manifests. Heights in place
of ticks landed with `MU-RECURSION.md` 5.1, and the mind's own K in place
of a mesh height for the dialect and contemplation lifecycles landed with
7.3a/7.3b. The landing order is in `MU-ENGINEERING.md` §10.
