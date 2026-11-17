### Unify open-wire court presentation in `act_on_returns`

- Replaced the `if / else if` `returns == 0` split in `Mind::act_on_returns` (`src/client/sdk/mind/winding.rs`) with `present_pending()`, driving `court_owed()`, `speak_unheard()` (via `Physics::impulse_items`), and `resubmit_unread_solves()` directly from their pending carriers on an open wire while keeping `comprehend_waiting()` and `self.at_return` gated on thought returns.
