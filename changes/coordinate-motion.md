### `forms::coordinate` advances a body in the bell, solves an event's bell, and merges and splits two bodies exactly: `FORMS-COORDINATE` v2 (proposal 78.173 fixtures 7–10)
- **Network contract:** no wire layout, act or ledger layout changes. A
  law version changes: `FORMS-COORDINATE` is v2 in `forms::LAW_IDS`, the
  SDK's `FORM_LAW_IDS` and the clause register, so the clause served on
  `ACT_LAWS` mode 7 and named in a kernel's signing act (mode 8) is
  `FORMS-COORDINATE` v2. Its fixtures are unchanged (1a, 2, 4, 5, 6).
- **The advance** (`Coordinate::advance`, `Rates`): each lane
  `Lᵢ(t) = Lᵢ(h) + rᵢ·(t − h)`, `K` re-read as `⌊|L_H(t)|⌋`; it composes,
  and `t < h` is refused `Backward`.
- **An event's bell** (`Coordinate::bell_of`, `EventBell`): `h + (v − L)/r`,
  exact and possibly between whole bells; none on an unmoving lane
  (`Unmoving`) or behind the body's bell (`Behind`).
- **The merger** (`merge`, `Merger`): the holding-weighted lanes at one
  origin and one bell, `K` re-read, with each body's offset per lane,
  whose holding-weighted sum is zero; refused `OriginsDiffer`,
  `BellsDiffer` or `NoHolding` (`MotionRefusal`).
- **Fission** (`fission`): the merged point, the holdings and the offsets
  give both bodies back; offsets no merger left are refused `Unbalanced`
  with their sum per lane.
- **Fixtures:** 78.173 §5 states fixtures 7–10 as exact inputs and
  outputs, their statements agreed with netstratum on 2026-11-05;
  claimed in `forms/src/tests/motion.rs`, with the composition and the
  inverse over generated cases in `forms/src/tests/properties.rs`. The
  register does not serve them: its fixture form has no operation or
  object for them, and adding one is a wire change left to the clause's
  drafting.
- **Open:** re-origin across a throat, and gaps (they wait on the passage
  flux on T).
