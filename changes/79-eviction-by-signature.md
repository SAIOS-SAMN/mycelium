### Eviction by signature, a bounded held queue, a waiting fold root (track 24, #79)
- **Network contract:** no wire byte, act or ledger layout changes. A
  new name joins 47.2's admission refusals,
  `REFUSE_UNREACHED_PAST_MEMORY`, used only inside back-fill admission.
  A line frame names the nodes that left before the nodes filed, so a
  node evicted and another filed at its stamp read back in order; the
  records are unchanged.
- A signed frame that contradicts its signer's line where it is held
  unreached (at its own place, or at its predecessor's) evicts the
  line's unreached top, and is then admitted. A forged genesis stored
  first gives way to the signer's own genesis. A reached receipt is
  never evicted.
- The unreached queue holds at most the process's capacity
  `C = ⌊M_proc / b⌋` receipts (MU-DERIVATIONS D2.5); a back-filled
  receipt past it is refused by name and not stored.
- A back-filled `Fold` files its root once a signature reaches it, and a
  restore files only the reached folds' roots.
