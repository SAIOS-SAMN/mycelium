### A fold root's signatures cover its place and height (track 24, #91)
- **Network contract**, inside v0.4.0's break:
  - **The root's signing domain changes.** The home's and the witness's
    signed positions over a fold root (47.3) carry, as the payload of
    their 47.1 prefix, the root written out as coordinates (A21):
    `"mycelium root v2" ‖ K u64 ‖ i u64 ‖ height u64 ‖ chain hash 32`,
    `K ‖ i` and the height big-endian (`Root::signed`). Version 1,
    unnamed, signed the bare 32-byte chain hash, and no longer verifies:
    a v1 root proof is refused `REFUSE_RECEIPT`, and a v1 co-signature
    seals no fold.
  - **No wire shape changes.** `ACT_FOLD`'s proposal, the witness's
    co-signature, the root answer of `ACT_RECEIPTS` and the `LEAF_ROOTS`
    leaf keep their layouts: each section's payload is length-prefixed,
    and only its content changes, from 32 bytes to 72. Root proofs
    persisted under v1 do not verify; a recut wipes them.
- `Reconstructed::root` reads the root back from each signed position
  (`Root::of_signed`), compares the place and the height the proof
  states with the signed ones as coordinates, and binds the hash. A root
  restated at another place, height or hash is refused `REFUSE_RECEIPT`,
  counted `root_refused`, and adopts nothing, whether or not the court
  holds a receipt at that place. The home (`Court::commit_fold`) and the
  witness (`ACT_FOLD`) check each other's position against the proposed
  root the same way.
- Claims `root_at_another_place_refuses_without_held_receipt`,
  `root_at_another_height_refuses`; the #88 claims stand.
