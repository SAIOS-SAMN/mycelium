### A body's nucleus is its second moment read, never a sphere (track 26 S7 stage 3; the ruling of 2026-11-06 on #134)
- **No recut and no network contract.** The ruling makes any word or
  chart address this moves a recut. It moves none: the retired producer
  wrote `X(c) − ½r²e_∞`, and `X(c) = c + ½c²e_∞ + e_o`, so the nucleus
  vector is the same value as before, written from the same exact `r²`.
  No word, chart or task address, tape or wire byte changes, and the
  oracle's golden file reproduces unchanged.
- **The nucleus** (`blade::nucleus_form`): a body's centre and its exact
  polar second moment per unit mass, written where the algebra needs one
  vector as the dual-sphere form `c + ½(c² − r²)e_∞ + e_o`. It needs `r²`
  only, so no root and no gauge are taken. `Blade::nucleus`,
  `vertex_nucleus` and the lift's `spatial` read it, and `Blade::radius2`
  is documented as the second moment, never a radius.
- **Retired:** the crate-private `cga::sphere_at`. The `compile_fail` doc
  test now reads `E0425`.
- **Claim:** `nucleus_is_the_second_moment_read_never_a_sphere`. On an
  L-tromino, whose `r²` is no rational square, `s² = r²`, the centre reads
  back, and the form is at unit weight.
