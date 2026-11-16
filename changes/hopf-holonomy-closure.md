### Two-tube Hopf holonomy closure on open demands (#349)

- Added `Contemplation::hopf_return_holonomy` in `src/transducer/field/plan.rs` and wired it into `note_ground` (`src/transducer/field/cross_scene.rs`) so an open `NoReturn` arc on a chart can close against a prior open arc via the two-tube return loop $\tilde{R}_2 R_1$ or commutator $R_1 R_2 \tilde{R}_1 \tilde{R}_2$.
- Updated `Mind::drain_demands` (`src/client/sdk/mind/think.rs`) to advance `by_chart` as a rolling boundary chain and fall back to the two-tube return holonomy $\tilde{R}_i R_{i-1}$ when consecutive legs commute.
