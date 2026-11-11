### One closure, one point (MU-ECONOMY §3.4, §11.4 F2b; #169 step 6)

**Network contract.** A first closure's settling act now carries its point, `Cite::Closure { task, rule }`: the task's citation and the closing rule's address, each written out whole. This is citation tag 18, an additive step.

- **Points held.** A court holds every point its cones hold, those sealed on its own line and on every line it admits (`Court::point_held`, persisted as `LEAF_POINTS`). Points are kept across folds.
- **A held point is closed.** A later closure of it is judged again and mints nothing, whichever court it is presented at, even where that court's own registry would call it first. The door's verdict follows the point.
- **Concurrent seals.** Two seals of one point made before either court held the other's are concurrent (45.3). A court notes a point as it admits the seal and mints no point it holds, so every second seal it meets is concurrent. Both are held as a residue naming them, in the order held, never divided and never ordered by arrival (`Court::concurrent_points`). The court logs `court concurrent closure … held`, and the census counts `points.concurrent`.
- **POW++.** The console's reading says so in place of "no court burns it yet".
