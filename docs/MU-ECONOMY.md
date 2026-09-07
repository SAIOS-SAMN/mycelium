# Economy — μ, the four-lane currency

Status at f595b4e: balances, stake bond, escrow, per-lane pricing, and the
ledger entries exist in `src/economy/`. Issuance as mint, stake for
voices, transfer, genesis, and the audit invariant are specified here and
not yet landed. Companion documents: `MU-PRODUCT.md`, `MU-ENGINEERING.md`,
`XSEPTUM.md`.

## 1. The unit

μ is a vector on four lanes: spatial (S), gauge (G), temporal (T),
holonomic (H). It is never summed and never converted across lanes. A
balance is `[u64; 4]`; a price is `[u64; 4]`; a payment moves value on
exactly the lanes the work moved. Comparison is the Dershowitz–Manna order
(`economy::descends`), which is the same order the court uses to accept a
reduction. There is no scalar anywhere in the economy.

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
`escrow`, `release`, `revoke`, `slash`, `burn`. Nothing else moves μ. No
entry carries a clock reading (`MU-RECURSION.md` chapter 45).

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
deflation" means.

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

**Open, and named here so it is not mistaken for settled: what room does an
unbonded voice have?** §4 says an unbonded voice may learn, speak and be
paid, and under this section speaking needs no bond at all, since a claim
is escrowed rather than transferred. Bond buys **room** — throughput, the
rate the metabolism governs (§3.3) — and §4 does not say what room an
identity has before it has earned any. The tree currently funds every
arriving identity at the genesis cover so it can bond into room, which,
since identities are free (`MU-RECURSION.md` 47.3), is a faucet: μ⃗ per key.
It is measured — supply grew to three times the seed across two ingests
from two voices — and it is pinned by
`the_kernel_conserves_across_genesis_bond_and_a_settlement` rather than
hidden. The candidates are that an unbonded voice has room for exactly one
claim in flight, or that its room is what its own closed deflation has
earned it. Neither is ruled.

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
chapter 27, T10) — the description the language no longer carries is
**burned**, while the retirer is paid the compression yield. Supply falls
by the freed measure less that yield.

Therefore

```
supply → the description length of the language as it currently stands
```

and not the integral of everything it has ever held. Supply cannot grow
without new observation, and it falls when the engine expresses what it
already knows more compactly. PoUW mints when structure is understood;
PoWC burns when the same meaning is re-expressed over fewer letters. The
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
presented and verified: the reserve paces strokes, bond buys room,
`descends` gates admission. Issuance is deflation closed per unit of
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

A listing is priced by `economy::price` against the book's pressure on each
lane; the pool is a bond, not a price basis. A requester escrows the price;
a claimant's word or solve enters the docket; release on the second organ's
replay mints the deflation and pays it, discharging the escrow (§3.1) — the
escrow was the claim, the mint is the payment. Hyphae are
paid the pool share priced into every listing they home, and accrual on the
temporal lane of demands they keep open, counted in heights of the home's
receipt ledger (`MU-RECURSION.md` 45.4), never in clock time.

**T-accrual is capped by the listing's own magnitude.** A demand nobody
closes would otherwise pay its home forever, which is a schedule in
disguise: standing is a cost the listing carries, so it can accrue at most
what the listing is worth. The cap is the listing's μ⃗ — a measurement, not
a constant.

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
supply = Σ balance + Σ bond
```

Escrow is **not** in the second sum. It is a claim on issuance that has not
happened (§3.1): counting it as supply reports value the mesh has not
created, and the audit fails at the first docket — measured on this tree as
`supply [18, 60, 0, 78]` against carriers `[18, 61, 0, 78]` after two
ingests. A ruling discharges an escrow into a mint, which enters both sums
together; a revocation discharges it into nothing.

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
that spends without earning runs down to the dwell. μ never decays and
never regenerates.

## 8. Policy ratios

There are no monetary constants. The quantities that behave like policy are
ratios the physics already reads:

- price against pressure on a lane;
- bond against cover room;
- pool share to the home against the deflation minted;
- accrual on T against standing extent;
- slash against bond.

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
`RewriteKind::Retire` that 0.G6 built. **E3**'s cap on T-accrual. Then
`price` returning zero when the bond is empty, the Note, voice keys, and
delegation.

Also not landed: the equivocation burn, receipt-indexed Notes,
two-signature releases, FROST voice groups and manifests. Heights in place
of ticks landed with `MU-RECURSION.md` 5.1, and the mind's own K in place
of a mesh height for the dialect and contemplation lifecycles landed with
7.3a/7.3b. The landing order is in `MU-ENGINEERING.md` §10.
