### The seated-mind harness: `mycelium-client --seat` end to end against a live house (#133)
- `sdk/examples/seats.rs`, a house on the SDK that seats a mind: it founds
  its domain and designates its reserve where it has not, lists seat 1
  (`FN_SOLVE`, `FN_DECLINE`), answers a take with TAKEN, sets the court's
  tasks before the holder one at a time as percepts (record kind 4), and
  answers each call with an OUTCOME. Exit 0 on a seated `FN_SOLVE`, 3 where
  every task set was declined.
- `scripts/quickstart` stage 8 runs it on hypha c with
  `mycelium-client --seat <domain> 1`: sit, percept, call, outcome, then
  the court's judged reading read for the solve under the seated mind's
  own key (Concordance 6.2a). The run fails where any of these is missing.
  `MYCELIUM_SEAT_TASKS` (default 30), `MYCELIUM_SEAT_WAIT` and
  `MYCELIUM_JUDGED_WAIT` bound it.
- Defect found by the harness, fixed: a seated solve was held for the
  return of a thought of the task's scene, which a seated mind need never
  make. At K 19 the solve of task 1bcfb847 stayed held while the house had
  already been told `FN_SOLVE`, and no court judged it. A seat's solve now
  leaves at the mind's next return, as a seat's call does
  (`Mind::submit_solve_signed` takes the scene; claim
  `seated_mind_solve_paid_to_its_own_key` restated).
