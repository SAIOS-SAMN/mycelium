### Align open vent and `Physics::impulse` with `forms::passage` (#353)

- Added `Physics::impulse_draw` and `Physics::impulse` in `src/physics/stroke.rs`, sizing per-lane read budgets via `forms::passage::PassageState` (`1 - δ = need / (need + hold)`).
- Sized `Mind::wind`'s open vent (`vent`) in `src/client/sdk/mind/winding.rs` from actual pending `need` via `Physics::impulse` rather than a fixed `unit_spend()`.
