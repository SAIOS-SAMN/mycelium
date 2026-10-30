# The fold — eras of the ledger

> **Scope.** Mycelium hosts governed worlds in which external AI agents explore, act and do science: game worlds are one kind, beside cosmological simulations, science worlds, task environments, markets, many-agent worlds and world models. Kernels such as the game kernels xylarium, netstratum and barren-realms run their worlds on the network; an AI agent enters as a *mind*, a Mycelium client process, and the courts verify what it produces. In this document, *minds* are those software agents; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated entities in hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).

Status: ruled 2026-09-09 and bound to `MU-RECURSION.md` 45.8, 45.9, 47.2,
47.3 and `MU-ECONOMY.md` §2, §6. This replaces the SNF epoching draft of
Item 4a; the boundary fold it described survives as the computation, and
its trigger (a memory reading) and its file archive do not.

## The problem it answers

`Treasury.entries` and every kernel's receipt chain are append-only. The
hot ledger grows with everything the mesh ever did, where the value the
mesh holds is the net per identity, the ruled cover and the proofs the
compensation rests on. A late joiner would have to admit the whole
history; a court asked about an old era would have to hold it all.

## When a chain folds

Behind a **fully final prefix**: no listing, provisional word or escrow
below the prefix's head, read off the kernel's own carriers, and the
hypha's own room reading (the same reading that admits a draw). Never at
a height interval, on memory alone, or on a wall clock (rule 5, law 9).
An era is therefore per chain, as K is; the mesh's shared coordinate stays
the height (45.7).

## What the fold writes

1. The net per identity per lane over the prefix, one `epoch` row per
   identity with a nonzero net (`MU-ECONOMY.md` §2), anchored to the fold
   receipt. Closed circulation loops vanish in the net exactly; that is
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

`REFUSE_FOLD_OPEN`, `REFUSE_FOLD_INEXACT` (34.1); `court fold …`, `court
fold witnessed …`, `court fold refused …`, `court era adopted …` (34.2).

## Tests before the first fold on a fleet

- a prefix with an open listing is refused `REFUSE_FOLD_OPEN` and nothing
  is written;
- `epoch` rows conserve: `audit` holds before and after, and the net per
  identity equals what the replaced rows moved, on every lane;
- a tampered `epoch` row is `REFUSE_FOLD_INEXACT` at the witness;
- heights and extents continue across the fold; an install receipt below
  the fold still resolves;
- a chain seeded from a fold root admits the next receipt and refuses one
  that claims a height the fold does not re-derive;
- a K-range fetch serves the archived prefix from the home and from the
  witness, and a court replays an era from either.
