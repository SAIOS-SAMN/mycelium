### `forms::coordinate` holds each lane as a keyed vector, H in degrees with `K` per component, decides on the caller's intervals and re-origins across a throat: `FORMS-COORDINATE` v3 (proposal 78.173 fixtures 1–16)
- **Network contract: a wire change and a law version.**
  `FORMS-COORDINATE` is v3 in `forms::LAW_IDS`, the SDK's
  `FORM_LAW_IDS` and the clause register, so the clause served on
  `ACT_LAWS` mode 7 and named in a kernel's signing act (mode 8) is
  `FORMS-COORDINATE` v3. The coordinate's writing in a clause changes: a
  local is `[h u64 LE][lane count][(lane name, vector) …][K count][K u64
  LE …]`, a lane name its byte (S 0, G 1, T 2, H 3) or `4`, the name and
  its law version for an opened lane, a vector its components as
  `(key, value)` in canonical key order, a key its axes as `(letter,
  index)`. A lane set or vector out of canonical order is a second writing
  and reads as none. Fixture tags 6 (`within` on a sector at a depth)
  and 7 (a range on one lane) are retired and read as none; `within` on
  stated bounds is tag 10 and a range on one component tag 11. Object tag
  3 (one side) is retired; a side per component is object tag 6, and the
  refusals `ShapesDiffer` and `NoComponent` are object tags 7 and 8. A
  kernel signing v2's clause must reconstruct v3's fixtures to sign
  again. No act or ledger layout changes.
- **Lanes are vectors** (`Vector`, `Key`, `Axis`, `Lanes`): exact signed
  components, each keyed by its axis and held in the order of the keys
  written out. H's components are keyed by generators, the plane named by
  its two axes (`e_Z1Q1`, the turning τ₁₂ as `e_Z1Z2`), stored in degrees,
  with `K_c = ⌊|θ_c| / 360⌋` per component (`DEGREES_PER_TURN`, 360,
  registered `EXTERNAL` as the degree's definition).
- **Open lanes** (`Lane::Opened`, `Openings`): a lane set is a list of
  named lanes; a lane beyond S, G, T and H is admitted only through a
  ruled opening, and is refused `LaneNotOpened` naming its name and law
  version otherwise. None is ruled; the ruled act between kernels is the
  next step.
- **Decisions on the caller's intervals** (`Interval`, `Bounds`, `Sides`,
  `Whole`): `within` reads a side per component against stated bounds,
  within only where every component is; ranges are on one component by
  two bisections (`Bodies::range`), any other component a filter
  (`Neighbours::filter`). `Sector` and `Reading` are the display reading
  only (`BASE` registered `FORMAT`); `Sector::side` is removed.
- **Motion per component** (`Coordinate::advance`, `bell_of`, `merge`,
  `fission`), refused `ShapesDiffer` where two shapes differ. **The
  re-origin** (`Coordinate::pass`, `Through`, `Passed`) through a throat
  by `forms::passage`'s `Poles::cross`: S's span reverses, T gains
  `−2·span`, and the frame reads reach and hold exchanged, `H + R` kept and
  `δ ↦ 1 − δ`; within one enclosure, the face exchange alone. **The flow**
  of two pushes (`flow`), with which dilation invariance is claimed.
- **Fixtures:** 78.173 §5 restates fixtures 1–10 on one-component lanes
  with H in degrees (fixture 7 stands at 2250°, `K = 6`), adds
  netstratum's 11–13 (a two-plane H; a range on one component and a
  filter; a merger and fission per component) and new 14 (re-origin), 15
  (dilation invariance, xylarium's joint fixture) and 16 (the keys, `K`
  per component and an unopened lane refused). Claimed in
  `forms/src/tests/coordinate.rs`, `motion.rs`, `vectors.rs`, `throat.rs`
  and `properties.rs`. The register serves 1a, 2, 4, 5, 6, 11 and 12.
  Fixture 11b waits on netstratum's composition law.
- **Next:** rewire `TPoint` as the form's one-component non-negative case,
  and the ruled act that opens a lane.
