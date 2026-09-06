# Economy — μ, the four-lane currency

Status at f595b4e: balances, stake bond, escrow, per-lane pricing, and the
ledger entries exist in `src/economy.rs`. Issuance as mint, stake for
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
each lane. There is no block reward, no schedule, and no issuance to
operators as such; supply on a lane is the integral of verified work on
that lane.

Two events mint:

- an install: a word, sentence, rule, translation, or rewrite upheld at the
  home and replayed at a witness;
- a solve: a rule spoken onto withheld tests and verified against the
  corpus at the home.

Optimisation moves are work: a condense on G, a torse on H, a refinement on
H, a re-anchoring on S, an opened axis on H, a translation on G each mint
the lane vector read on the carriers before and after, and each is refused
when that vector is zero on every lane.

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
replay pays the escrow to the claimant and mints the deflation. Hyphae are
paid the pool share priced into every listing they home, and accrual on the
temporal lane of demands they keep open, counted in heights of the home's
receipt ledger (`MU-RECURSION.md` 45.4), never in clock time.

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
supply = Σ balance + Σ bond + Σ escrow
```

`Treasury::audit` checks both from the entries and refuses to persist a
snapshot that fails. A testnet differs from the beta only in the first term:
genesis is a finite, signed allocation recorded as its own kind, so seeded
value and earned value are always separable in the ledger.

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

Not landed: mint as issuance (release currently draws from the bond and
pays nothing when the bond is empty), `price` returning zero when the bond
is empty, stake and unstake for voices, the Note, voice keys, genesis,
burn, `supply`, `audit`, T accrual on open demands, delegation. Also not
landed: the equivocation burn, receipt-indexed Notes, two-signature
releases, FROST voice groups and manifests, and heights in place of ticks
(`MU-RECURSION.md` Part IX). The
landing order is in `MU-ENGINEERING.md` §10.
