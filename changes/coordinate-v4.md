### H stands only on turning planes: `FORMS-COORDINATE` v4 (proposal 78.173, agreed with netstratum 2026-11-05, docs/117 §10.3)
- **Network contract: a law version and the register's fixtures.**
  `FORMS-COORDINATE` is v4 in `forms::LAW_IDS`, the SDK's
  `FORM_LAW_IDS` and the clause register, so the clause served on
  `ACT_LAWS` mode 7 and named in a kernel's signing act (mode 8) is
  `FORMS-COORDINATE` v4, still at kernel standing (signable and
  vetoable). The coordinate's layout in a clause is unchanged; a key's
  axis may now be a frame unit, written with its sign as the letter byte
  (`+` for `e₊ₖ`, `-` for `e₋ₖ`) and its plane's index, and an H key
  reads back only where its two units square alike. The served fixtures
  1a, 2, 4, 5, 6, 11 and 12 key H on `e₊₂e₊₃` and `e₊₃e₊₁` in place of
  v3's `e_Z1Q1` and `e_Z2Q2`, every value kept, so their addresses
  change. A kernel signing v3's clause must reconstruct v4's fixtures to
  sign again. No act or ledger layout changes.
- **H key validation** (`Lanes::of`): an H component is keyed by a
  generator of two frame units of one square, the continuum's `e₊ₖe₊ₗ`
  or the vacuum's `e₋ₖe₋ₗ` (`Plane::Turning`). A frame plane's own
  `e₊ₖe₋ₖ` (`Plane::Frame`), a mixed pair `e₊ₖe₋ₗ` (`Plane::Mixed`) and a
  generator of axes named by letters (`Plane::Unsigned`) are refused on H
  `NotTurningPlane`, naming the key and the plane. S's keys may name
  boost planes. New: `Axis::unit`, `Square`, `Key::plane`, `Plane`.
- **Fixtures:** 78.173 §5 restates fixtures 7 and 11–16 on the new keys,
  each value kept: v3's `e_Z1Q1` is `e₊₂e₊₃`, `e_Z2Q2` is `e₊₃e₊₁` and
  τ₁₂'s `e_Z1Z2` is `e₊₁e₊₂`. Fixtures 7 and 11–15 reach v3's objects;
  fixture 16's key order follows its keys (`(-1e/1,190/1,438/1)`,
  `K = (0, 1, 3)`), and 16c (the turning-plane refusals) is new. Claimed
  in `forms/src/tests/vectors.rs` (`h_keys_are_turning_planes`).
