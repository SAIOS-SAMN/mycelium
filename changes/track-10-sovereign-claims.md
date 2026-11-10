### Client: track 10 — sovereign mind claims & amendment held beside (MU-CLIENT §4.10, §4.12, §7.1, #135)
- **Amendment held beside:**
  - Implemented `Amendment` constitutive responses (`Elastic`, `Derivable`, `HeldBeside`, `Fracture`) in `src/client/sdk/mind/amend.rs`.
  - Implemented `evaluate_amendment` computing the commutator $C = g_1 g_2 \tilde{g}_1 \tilde{g}_2$ of old and amended constructors: identity retires old by derivability; non-trivial commutator holds the amended constructor beside the original at conjugate grade, separated by $C$.
  - Added `Mind::amend_constructor` and `Mind::revise_perception_by_amendment` tracking amendments held beside.
- **Track 10 sovereign mind claims:**
  - Added and verified the five Track 10 claims in `src/client/sdk/tests/track10.rs`:
    1. `mind_idles_with_no_escaping_orbit`: idle mind in equilibrium moves no lanes and fires no returns.
    2. `task_arrival_moves_no_orbit`: task arrival and intake onto perception boundary does not advance $K$ or fire escaping returns.
    3. `mind_declines_a_scene_its_field_does_not_bind`: uncoupled offer is left unperceived, crosses returns, and lapses without binding to the field.
    4. `mind_acts_at_its_returns`: speech held for a scene waits until a thought of that scene completes its orbital return.
    5. `mind_revises_a_perception_on_a_court_ruling_by_amendment`: plastic amendment re-derives constructors, retiring commuting ones by derivability and holding non-commuting ones beside separated by commutator.
