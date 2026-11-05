### A gap past the capacity closes through a fold root (track 24, #88)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes. The catch-up ask and the root answer of 78.147 carry it.
- A fold root is admitted only as a `Reconstructed<RootProof>`
  (`Reconstructed::root`): both signed positions verify, and each one's
  commitment binds the root. A forged root is refused by name, counted
  `root_refused`, and adopts, sheds and evicts nothing.
- A back-fill page that stops on the held bound `C` asks the range again
  from below the signer's unreached floor, which a court holding the
  root's proof answers with the root.
- The root's signatures reach the receipt held unreached at its place
  where it is the one the root names, and every receipt held below it;
  they are released and read in line order. Receipts held unreached that
  a root's shed takes off the chain leave the signed-reach gate with it.
- A peer behind by more than `C` receipts reaches the signed head and
  never holds more than `C` unreached.
