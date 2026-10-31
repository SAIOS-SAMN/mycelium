# MU-FOLD — eras of the ledger

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents; *worlds* and *planets* are simulated places, and *peoples*, *civilizations* and *ancients* simulated inhabitants, of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the hypha (its ledger, receipt chains and co-signed folds). No game term.

**Status:** specification, **Built**. The fold is specified by `MU-RECURSION.md` 45.8,
45.9, 47.2, 47.3 and 78.65 (finality per item) and by `MU-ECONOMY.md` §2
and §6; where this document and those sections differ, they govern. The
code is `src/economy/fold.rs` (the net and its conservation),
`src/chronometry/era.rs` (an era and its cold segment) and
`src/server/cognition/court/era.rs` (the eras a court holds).

## The problem it answers

`Treasury.entries` and every court's receipt chain are append-only. The
hot ledger grows with everything the mesh ever did, where the value the
mesh holds is the net per identity, the ruled cover and the proofs the
compensation rests on. A late joiner would have to admit the whole
history; a court asked about an old era would have to hold it all.

## When a chain folds

**Finality is per item** (78.65). A fold moves what is closed, and nothing
open holds it back: a listing or a provisional word below the cut stays
citable by the stamp of the act that admitted it, through its era, and an
escrow is a claim carried off the ledger, so none of them pins the fold.
Each closes after the fold at the act that closes it. The fold's floor is
the line's own last close, within one `ACT_RECEIPTS` frame. A fold is
never triggered at a height interval, on memory alone, or on a wall clock
(rule 5, law 9). An era is therefore per chain, as K is; the mesh's shared
coordinate stays the height (45.7).

## What the fold writes

1. The net per identity per lane over the prefix, one `epoch` row per
   identity with a nonzero net (`MU-ECONOMY.md` §2), anchored to the fold
   receipt. Closed circulation rings vanish in the net exactly; that is
   the compression, and it is a sum, never a choice.
2. A receipt of kind `fold` (45.1) citing the archived prefix's head hash;
   heights continue across it and the chain never breaks.
3. The archived prefix into the cold section `ARC1` (28), keyed by the
   fold root, written once and never rewritten.

Both §6 equations hold at the prefix before the fold and at the `epoch`
rows after it, or the fold is `REFUSE_FOLD_INEXACT` and writes nothing.

## Two organs

A witness with bond outside the home's delegation closure fetches the
prefix by K range (the back-fill act of 47.2), re-derives §6 across it and
co-signs the fold root; the home writes the rows and seals the receipt
only with both signatures (47.3). The witness keeps what it fetched, so
every archived prefix has two holders and there is no operator's archive.

## What stays hot

The `epoch` rows, the install receipts (two-signed proofs of what was
paid) and the presentation (the ruled cover). Work receipts — attests,
quotes, offers, pages — go cold. A fold changes no price, extent or
ruling.

## Extraction and joining

Any holder serves an archived prefix by K range; a court replays era N by
fetching and re-deriving. A joiner adopts a fold root the way it adopts a
genesis: on the two signatures and its own §6 re-derivation over the
`epoch` rows it received, seeding its chain from the fold's head and
height; the archive replays on demand, never as a precondition. A chain
is never truncated behind a fold root it did not adopt.

## Refusals and lines

`REFUSE_FOLD_INEXACT` (34.1, `src/court/verdict/refusals.rs`): a fold whose
nets do not conserve writes nothing. `REFUSE_FOLD_OPEN` is still defined
there, but since finality became per item (78.65) no fold path produces
it. The server's log lines (`src/bin/mycelium-server/fold.rs`,
`src/bin/mycelium-server/judge/fold.rs`) are `court fold witnessed …`,
`court fold refused …` and `court fold unmeasured …`.

## Tests of the fold

- `src/economy/tests/fold.rs`:
  `fold_replaces_prefix_with_epoch_rows_and_conserves`, which also
  shows a fold that does not conserve refused `REFUSE_FOLD_INEXACT` with
  nothing written.
- `src/server/cognition/tests/era.rs`:
  - `commit_fold_seals_on_two_signatures`;
  - `verify_fold_second_fold_on_fetched_rows`;
  - `commit_fold_stands_epoch_rows_at_root`;
  - `propose_fold_passes_open_listing_and_escrow`;
  - `propose_fold_at_close_cuts_long_orbit_at_close`;
  - `bind_eras_reads_each_era_own_range`;
  - `receipts_page_from_root_pages_no_earlier_era`;
  - `admit_backfill_admits_citation_of_folded_receipt`;
  - `restore_agrees_with_bound_line`.
- `src/chronometry/tests/shed.rs`:
  `shed_folded_keeps_peer_nodes_own_hot_line_cites`,
  `shed_folded_leaves_hole_backfill_cannot_cross`.
- `src/anastome/dialogue/tests/fold_wire.rs`:
  `fold_witness_and_rows_round_trip`;
  `src/anastome/dialogue/tests/receipts_range.rs`:
  `receipts_request_and_reply_round_trip_within_budget`.
