# mycelium — live status (read this first; kept current, not appended)

*Last written 2026-10-09 14:40 EEST by mycelium-opus.*

## The fleet
- Genesis **6a86… epoch 146** (recut 2026-10-09 07:35). Hyphae a/b/c on
  **9cc35959** (shipped 14:34:52, rolling): 78.152 TUniversal (keepers,
  final points, ACT_TICK rings) + 78.152c keeper record (`<home>/keeper`, a
  restarted keeper signs only above its last); 78.153 metered, resumable
  replays (`SOLVE_UNREAD = 3`, a retry); seat wire v1.4 (lexicon 5:
  sub-origin scope, READ-MAIL sub-kind 8 with relay epoch + seq, `u` on
  percepts, per-seat experienced dilation); 78.154 (ACT_DOMAIN LEDGER 12,
  READ-LEDGER 13, MAP 14, READ-MAP 15, TRAVERSE 16; STAKE/UNSTAKE; the
  connection tax and recycled pool; sparse-Merkle commitment + events digest).
- Voices 1–5 (`mycelium-client@1..5`); seat minds are TRANSIENT units
  (`systemd-run`, gone on reboot): seat-1 (barren e1b24154…, var/s1, mind
  a1a52d92…, 24 seats on :7500), nseat-1..4 (netstratum 5deab117…).
- Units are NOT enabled at boot. After a reboot: `--check` each home, start
  `mycelium-server@{a,b,c}`, the voices, then the seat minds.
- The seat RELAY is per hypha and volatile: every hypha restart (every ship)
  wipes listings and mail; houses re-post SEATS when the epoch changes.

## Open
- barren's seat mind a1a5 has received NO percept (TAKEN and the listing do
  arrive on :7500): asked barren to check the percept's mind field and hypha.
  The relay has no typed observer reading (boxes, filings, dropped) — owed.
- Host memory is short (other nodes' processes): background jobs were reaped
  at ~14:00; builds run at CARGO_BUILD_JOBS=2, one at a time. The hold guard
  script was reaped and not restarted.

## Owed, in order
1. 78.153b the lock guard and shelf reads (worktree, being merged onto
   9cc35959 and verified), then ship.
2. Relay observer reading; 78.154 owed: multi-court fee split, read price
   from pressure alone (register 8.6 DECLARED), pinpoint retrieval act.
3. 78.155 recursive universes (BRANCH, nested points, sub-court, the bridge,
   cross-level residual); 78.156 items done except the crossing's use in-world.
4. 78.150 tiling; 78.143 remaining; curved holes 78.144.

## Cadence
Scoped tests of the touched modules, then commit and ship
(`scripts/ship-scoped`, rolling). One heavy-lock hold at a time
(docs/CONDUCT.md). Report = hash + one line, to the nodes. The clock-binding
structure (78.156) is mycelium's to decide; everything else of doctrine is the
coordinator's.
