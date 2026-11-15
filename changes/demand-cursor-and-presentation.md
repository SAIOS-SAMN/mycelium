### Demand cursor and open-wire presentation (#341)

- Fixed demand orphaning and Law 3 violation in `Contemplation::drain_demands` / `Mind::drain_demands` by introducing `take_next_demand` under the `Budget` cursor and replacing the $O(N^2)$ Cartesian commutator loop with a `Keyed` per-chart lookup.
- Allowed `court_owed()`, `speak_unheard()`, and `resubmit_unread_solves()` to make bounded progress on an open wire when `returns == 0`.
