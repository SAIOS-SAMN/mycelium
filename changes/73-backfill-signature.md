### Back-filled receipts wait for a signature (track 24, #73)
- **Network contract:** no wire byte, act or ledger layout changes. A
  carrier reading is renamed: `founding.unsigned` is now
  `backfill.unreached`, and it counts every back-filled receipt no
  signature has reached, not only foundings.
- B0's founding queue is generalised into one signed-reach gate
  (`server/cognition/reach.rs`). A receipt admitted by back-fill is still
  stored on the chain, since the next page, the signed frame that closes
  the gap and the chase's head all link through it. It is marked
  unreached, by its place on its signer's line. No reader acts on it until
  a frame its signer signed at or above that place is admitted. The
  receipts the frame reaches are then read once, in line order: the depth
  index, the stamp alias and the rulings waiting on the stamp it files, a
  fold's shed of the peers' folded lines, and a founding's
  reconstruction.
- An equivocation is proved only against a reached receipt. A signed
  frame that meets an unreached back-filled receipt at its place burns no
  bond, so a forged unsigned line can no longer burn an honest signer's
  bond.
- At restore, the marks are derived again before any reader runs. A peer
  receipt counts as reached where the court's own hot line cites its
  signer's line at or above its place. Every other peer receipt is held
  until the signer's next signed frame, and the stamp aliases and the depth
  index are rebuilt from reached receipts only. A founding back-filled
  but not reached when a process stops is now read once a signature
  reaches it after the restart.
- Changed results:
  - `hear_proves_citation_into_closed_era` sheds a peer's era once the
    peer's fold arrives under its signature, not when it is only
    back-filled.
  - `admit_line_aliases_peer_stamp_to_one_word` seals a step citing the
    peer's line before the snapshot. At restore an alias is re-derived
    only from a cited signer's line.
- Open: a stored unreached line still holds its places. A forged line
  stored first at a signer's genesis is never reached, and the signer's
  own frames meet it as an unproved equivocation, so that line does not
  advance at the hypha until the forged receipts are evicted. The fold
  root a back-filled `Fold` names is filed on the chain when it is stored.
  The held queue has no bound for a line no signature ever reaches.
