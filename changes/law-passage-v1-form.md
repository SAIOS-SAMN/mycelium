### LAW-PASSAGE v1 held as a form (`forms::passage`, track 24 B3)
- `forms::passage` holds the linear passage exact over ℚ: `Throat::pass`
  gives `2H − R`, signed and never clamped; `pass_share` gives `1 − δ`
  and refuses a value outside `[0, 1]` `NotShare`; `Throat::verify`
  reconstructs the passage and refuses any other stated reach
  `NotLawful`, naming the claim against the reconstruction. The shell id
  is `FORMS-PASSAGE` v1 in `forms::LAW_IDS`.
- The six joint-read fixtures of proposal 78.172 §5 (the sixth, `H = 1`,
  `R = 3` gives `−1`, added with netstratum) are claims in
  `forms/src/tests/passage.rs`, with the involution, the fixed throat
  and the throat sphere's `H²/R` refused wherever it moves the reach.
- No wire or layout change. The admission at the vertex, and a kernel
  signing by reconstructing the fixtures (track 24 B2), are still open.
