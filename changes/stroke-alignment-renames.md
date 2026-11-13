### The time, loop and park senses of "stroke" renamed (78.183 §7.1)

The renames that follow the 2026-11-13 ruling, with no change of behaviour:
- the court's pass is a cycle (`court_cycle`, `court_cycle_on`);
- a carrier's step is a cycle (`Circuits::cycle`);
- the mind's pass is a winding (`winding_begins`, `winding_began`, `winding_boundary`, `returns_of_winding`, `owes_windings`, `WindingOutcome`);
- horizons and stalls counted in passes are `Lambda::Cycles` and `Stall::Cycles`;
- the scheduler's bound is the park floor (`park_floor`, `ParkFloor`);
- the calibration's refusal is `FootprintUnread`.

"Stroke" in code now names only the reserve's motion for one act, still the scalar `Physics::stroke` until #308 makes it per lane.
