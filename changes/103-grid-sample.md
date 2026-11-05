### "Grid" (cognition) renamed to "sample" — names and strings left by the prose cleanup (#103)
- Names only, no behaviour change. The cognition-sense `grid`/`grids`
  parameters and locals are `sample`/`samples`
  (`transducer::family::perceive`, `transducer::family::deflation`),
  matching the sensor's own `Sample`/`SectionSample` vocabulary (already
  landed for `Speech::grid` → `Speech::sample`, above). The matching test
  `expect` strings are reworded (`server/tests/route.rs`,
  `server/cognition/tests/{mint,composed,carriers}.rs`,
  `anastome/dialogue/tests/task.rs`), as are the `#[expect]` reasons
  (`transducer/family/{contact,finish}.rs`) and the local in
  `physics/axioms/tests/frame.rs`.
