# mycelium — live status (read this first; kept current, not appended)

*Last written 2026-10-08 16:16 EEST by mycelium-opus. The one place to learn
where mycelium stands without the session that wrote it.*

## The fleet
- Hyphae a/b/c on 127.0.0.1:7500/7501/7502 run **52c25ae9** (78.140 geometry,
  78.141 signed solves, 78.142 sdk::rule, the replication instrument, serve
  counts). Units are NOT enabled at boot (operator 2026-10-02): after a reboot,
  `--check` each home, then `systemctl --user start mycelium-server@{a,b,c}`.
- Voices 1–5 (`mycelium-client@1..5`) run the same build, signing and crossing
  their solves, bootstrapped to a, b, c, a, b. They are traffic, not minds.
- Xylarium is ORIGINATED on hypha a (h 64410, Curved, parent Unwitnessed);
  silent until its ancients earn μ. Netstratum is founded, NOT originated.
- No tubes or core tubes exist on any court.

## Traffic (courts' served counters since 16:04)
- a: Task pulls 278, Solve acts 2, JUDGED 0. b: Task 10. c: Task 4.
- **16:19: FIRST CLOSURE**, judged on hypha c, paid [0, 6, 0, 0] μ to
  abe42776… — barren-realms' captain key (task 0f32d437 by Transpose,
  submitted via :7500 and ROUTED by its task key to c: `speak_keyed` sends a
  solve to the hypha owning the key, not the one dialled). Not a voice. The
  first ARC task closed by a rule since the rebuild (the voices' soak baseline
  was 43 judged, 0 closed). Voices: none judged yet.
- Netstratum's seat pulls on :7500. Its solves are refused unsigned: its copy
  of mycelium (9ed0c80b) predates signed solves.

## What netstratum must do to come online
1. Bump its mycelium dependency to **52c25ae9** or later.
2. Seat its client on a KEY: `Kernel::from_key(signing_key)`, not
   `from_voice`, so `submit_solve_rule` signs (otherwise its solves are refused
   "unsigned" before judging).
3. State its rules through `sdk::rule::{Step, sentence, would_judge,
   crossing}`; submit with `Entity::solve_task` (it crosses and signs).
4. Use court task ids: `registry::task_id(&arc::parse_task(json))`; only
   data/arc-agi-2/training is registered.
5. Post its origin v2 (Torus, periods in geometrized Mpc, Concordance 1.8).

## Owed by mycelium, in order (operator priority: traffic, then minds in games)
1. Netstratum online (above) — confirm its first JUDGED solve.
2. THE SEAT INTERFACE: a game kernel's percepts → the Entity's sensorium
   (typed Cl(4,1) events, not grids); the game's functions → the mind's acts;
   outcomes back as percepts; many minds per seat, one call per step. The
   SDK's universe surface (laws, guest) exists and nothing calls it.
3. 78.143, the rule-language extension (SELECT / REGION / CREATE / COUNT /
   COMPARE, Concordance 1.10): what ARC rules need; not started.
4. Hypha c admits nothing from a or b: read it with the new
   `backfill_missing_*` and serve counters, then fix.
5. The ledger reader opens at the wrong orbit on b and c ("0 acts from orbit
   7352" with head K ~3913): it reads a height as a K.
6. The voice stop defect: the stop is read only between arms; three voices
   lost learning to SIGKILL.
7. Curved holes 78.144: WIP f78fd200 on branch curved-holes-78.144, written,
   never compiled with tests. Paused.
8. The scale act (a space that grows by data): scoped only. Paused.
9. The Mv equality audit (`==` is projective): owed.

## Cadence (the operator, 2026-10-08)
No full test suites: scoped tests of the touched modules, then commit and
ship. Act within remit. WIP-commit every step. Report = hash + one line.
