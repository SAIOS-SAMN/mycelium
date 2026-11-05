### LAW-PASSAGE v3: the passage is the pole swap, `FORMS-PASSAGE` v4 (proposal 78.172; Concordance §12)
- **Network contract:** the clause register on `ACT_LAWS` mode 7 serves
  `FORMS-PASSAGE` v4 in place of v3, with new fixture and object tags. A
  passage fixture is `Fixture::Crossed` (tag 8: the plane's two units,
  the frame state and `T`) or `Fixture::Ridden` (tag 9: the two units and
  the riding state); a crossing reaches `Object::Crossing` (tag 5: the
  frame state past the swap, `S` and `T`). Fixture tags 0–3 (v2's
  product passed twice and once, the commutator, the square) are retired
  and read as none. A kernel signing on mode 8 reconstructs the eight
  passes of v4; a signature on v3 no longer names a served clause. No
  act, ledger layout or L0 law changes.
- **The law** (adopted by both operators 2026-11-05; netstratum docs/117
  §10, §12): a frame state `H·n_Z + R·n_Q` (hold and reach) goes by the
  sandwich `x ↦ e₊·x·e₊`, its span `H − R` reversed and its whole kept;
  the riding state goes one-sided, `q ↦ e₊·q`; one passage writes
  `T ↦ T − 2·(H − R)`; two passes are the identity; at a pole the passage
  is one-way. v2's product `pass(v, q)` is retired as the passage; `v_k`
  stays the plane's direction in the advance.
- **`forms::passage`** (`LAW_VERSION` 3 → 4): `Poles::of` (refusing
  `NotUnit`, `WrongSign`, `Mismatched`), `Poles::state`, `read` (to a
  `Frame`; refusing `NotFrame`), `swap`, `cross` (to a `Crossing`) and
  `ride` (refusing `MixedParity`); `Frame::whole`, `span`, `share`
  (refusing `NoWhole`, `NotShare`) and `place` (`Place`, `Pole`).
  `Throat::verify` and `pass_share` stand. `pass`, `NotDirection` and
  `NotEven` are removed.
- **Fixtures:** the five of 78.172 §5 and the roles' exchange
  (`(H, R) = (4/5, 1/5)` to reach `4/5` about hold `1/5`, the whole kept,
  `δ` `4/5 ↦ 1/5`) are claims in `forms/src/tests/crossing.rs`. v2's
  algebra facts stand as claims on `Clifford::product` in
  `src/transducer/cga/tests/product.rs`.
- **SDK:** `signing::reconstruct_fixture` reconstructs the new fixtures
  through `forms::passage::Poles`; `forms::passage::{Poles, Frame,
  Crossing, Place, Pole}` join the surface; `FORM_LAW_IDS` names
  `FORMS-PASSAGE` 4.
- **Documents:** 78.172 amended to v3; Concordance §12 restates draft
  2.11 on v3, held until xylarium reads the fixtures; MU-EQUATIONS §11.
