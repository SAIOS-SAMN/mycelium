### Replication by reconstruction (track 24, B0; #13)
- **Network contract:** no wire byte, act or ledger layout changes. Each
  hypha's domain registries (founding, tiers, reserve, origin) are now a
  reconstruction of the chain it holds, so a domain founded at one hypha
  is founded at every hypha of the mesh. The interim contract "a kernel
  addresses the hypha that founded its domain" is withdrawn.
- A peer's `Cite::Domain`, `Cite::Reserve` and `Cite::Origin` receipts
  are reconstructed once the signer's own signature reaches them: at once
  for a receipt admitted under its signature, and for a back-filled
  receipt when a frame its signer signed at or above it is admitted.
  Each citation is applied by the founding hypha's rules and refusals,
  once by content. The same content held already changes nothing. Other
  content is refused `AlreadyFounded`, `TierHeld` or `AlreadyOriginated`,
  held by the receipt's stamp (`Court::founding_refusal`), and the
  registries keep what they held. A reserve or an origin whose domain is
  not yet founded at the hypha waits for that founding. A fold of a
  reserve tier passes its era anchor at every hypha, as on the court that
  sealed it.
- The carrier reading counts `founding.unsigned`, `founding.unfounded`
  and `founding.refused`.
- Open: a founding sealed inside a peer's era folded before the hypha
  caught up (a catch-up from a fold root delivers no receipt below the
  root); a version-2 origin's geometry, which crosses only as its
  statement's hash; the three waiting carriers are volatile, so a
  founding back-filled but not yet signed when a process stops is not
  read after the restart.
