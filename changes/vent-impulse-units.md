### Scale `Physics::impulse_items` in read units and hand over vent budget (#353)

- Added `Physics::impulse_items` in `src/physics/stroke.rs` so pending item counts (`items * unit_spend()`) and reserve holding `H` stand in the same read units on `forms::passage::PassageState`.
- Updated `Mind::stroke_in_field`, `take_docketed_demands`, `speak_owed`, and open-wire `speak_unheard` to draw via `impulse_items` and hand over remaining vent reads via `Budget::hand_over`.
