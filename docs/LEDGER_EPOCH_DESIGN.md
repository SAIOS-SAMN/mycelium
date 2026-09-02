# SNF Ledger Epoching — design draft (Item 4a)

Status: DRAFT — staged behind the 0x09 refinement bid. Registry persistence
(the security-critical half of Item 4a) is already landed: settled states
snapshot to `<state>.registry` on every bounty and overlay after corpus
re-registration at boot.

## The pressure

`Treasury.entries` is an append-only journal: every enroll, settle, recall
fee, door fee, tithe, solve, and refine appends a row. Under registered-work
throughput (1,120 tasks dispatching continuously, plus the market's own
settle traffic) the journal grows without bound — the same leak class the
demand market had before thermodynamic capacity. Restart replay time and
resident memory both scale with journal length, not with economic state.

## The invariant that must survive

Conservation: `pool + Σ balances = Σ stakes` at every prefix of the journal.
Any compaction must preserve (a) this equation, (b) per-identity balances
exactly, and (c) the K-step receipt chain's authority (the chronometric
spine is content-addressed; folding must not orphan receipts the court can
still be asked to replay).

## The fold (Smith Normal Form framing)

The journal is a 1-chain of transfers over the identity lattice: each entry
is a signed edge `who --μ⃗--> who'` (pool is an identity). Compaction is the
boundary fold: present the epoch's entries as the integer matrix `D` of the
transfer complex and reduce `D → SNF = diag(d₁…dₖ)`; the epoch's NET effect
on every identity is the image — a single balance 0-chain `B₀`. Everything
in `ker` (closed circulation loops: A→B→A, tithe round-trips, door-fee
refunds) vanishes exactly. That is the compression: closed chains carry no
economic information forward.

An epoch closes when the journal's length crosses the same thermodynamic
capacity read the market uses (MemAvailable/κ — no static row cap, per the
no-static-constants ruling). On close:

1. Freeze the epoch prefix; compute `B₀` (per-identity net) + verify
   conservation against the live pool.
2. Write one `epoch` row per identity with nonzero net (kind `"epoch"`,
   orientation 0 — a fold is neither work nor spend), anchored to the LAST
   K-step receipt of the folded prefix so the chronometric chain stays
   connected.
3. Archive the folded prefix to `<state>.epoch-N` (cold, append-only,
   auditable — never deleted; the court can still replay any epoch by
   loading its archive).
4. Truncate the live journal to `[epoch rows] + [post-freeze tail]`.

## Restore path

Boot restore reads epoch rows exactly like any other entry — no special
case: the fold IS ordinary journal content. A hypha that never epochs and
one that epochs every hour converge to identical balances (the fold is
semantically invisible — only the resident footprint differs).

## Provisional escrow interaction

Escrowed (provisional) awards must NOT fold: they are open 1-cells awaiting
a court verdict; folding them would erase the escrow the verdict releases
or slashes. The freeze line therefore slides back to exclude the oldest
open provisional — an epoch can only close behind fully-final prefix.

## Not in scope here

- Registry archive (settled snapshots are already tiny: 37 B/task).
- Cross-hypha ledger reconciliation (each hypha folds its own journal;
  escrow-gossip already carries the inter-node stakes).
