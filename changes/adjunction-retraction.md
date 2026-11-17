### A hypha attaches to a running mesh by adjunction, and detaches by retraction

78.185 (#276), approved 2026-11-17. Two `ACT_DOMAIN` sub-kinds carry it:
- **Adjunction (21):** keyed and signed by the hypha's own key, with its crossing record. It states the hypha's tethers: each node, the motion carrying that node's readings onto the hypha's, and their relative phase state.
- **Retraction (22):** the same keying and signature, stating only the withdrawal.

The court checks each on its nerve: the genesis hyphas, overlapping each other by the identity, plus every attached hypha with its tethers. It uses `forms::cover`, and seals what it admits. Attached hyphas are saved (`LEAF_ATTACHED`, written last) and join the boot's live set, so a restart never burns an attached hypha's stake. A retraction returns the stake to the hypha's balance. Each refusal is named: `Untethered`, `NoNode`, `OpensCycle`, `Cocycle`, `Disconnects`, `Claimed`, `Genesis`, `Held`, `Unsigned`.
