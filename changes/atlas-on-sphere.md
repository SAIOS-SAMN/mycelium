### The atlas is read on Sⁿ: charts as caps, transitions on overlaps, gluing on triangles (78.189)

`transducer::stalk::sphere` reads a field's charts as open caps of Sⁿ, each bounded by its body's round (the nucleus):
- `meeting`: two charts overlap by the sign of their rounds' inversive product, through `cga::containment`. The reading is on, inside, wraps, apart or unread, and every conformal map keeps it.
- `overlaps`: one edge per overlapping pair, carrying the motion the physics states between the two bodies. The motion is asked only on an overlap.
- `gluing`: on each triangle of pairwise overlapping charts, the circuit `g̃_ac · g_bc · g_ab` reads `Glued` where it is a scalar, and `Holonomy` where it is not. The holonomy is kept whole.

Each pair and each triangle spends one read of the stroke.

Proposal 78.189 and claims in `stalk/tests/sphere.rs`. GLOSSARY: *Chart* is amended (a chart is now the cap a round bounds) and *Atlas* extended. MU-TOPOLOGY §3 states the atlas on Sⁿ.

**Network contract:** none. This is a new reading only; no chart key, wire or ledger changes.
