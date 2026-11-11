### The engineering numbers stand in a register of their own (78.182 item 20, D1.4 O2)

- **`OPEN_PHYSICS` keeps only the physical invariants** (`OpenKind::Physics`: the throat grid, the candidates per pair).
- **Its sibling, `OPEN_ENGINEERING`,** holds the engineering and protocol numbers: the pool divisor, the body budget, the account shards and the soak instruments' strokes.
- **The numbers gate reads both alike** (N5), and `open_physics_holds_physical_invariants_only` holds the split.
