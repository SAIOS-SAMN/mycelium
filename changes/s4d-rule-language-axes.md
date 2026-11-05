### The rule language's coordinates named by the plane axis they are read along (track 26 S4d, stage 3)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no clause or word address moves: the clause codec writes
  each coordinate in the byte its planar name was written with
  (`Extent` 1 and 2, `Least` 3 and 4, `Greatest` 5 and 6).
- **SDK surface change, inside v0.4.0's break** (coordinator decision).
  `mycelium_sdk::rule::Coord::Height` and `Width` are
  `Coord::Extent(PlaneAxis)`, `Top` and `Left` are `Coord::Least(PlaneAxis)`,
  and `Bottom` and `Right` are `Coord::Greatest(PlaneAxis)`, with
  `PlaneAxis::First` for `Height`, `Top` and `Bottom` and
  `PlaneAxis::Second` for `Width`, `Left` and `Right`. A client writes
  `Coord::Extent(PlaneAxis::First)` where it wrote `Coord::Height`.
- `PlaneAxis` (`transducer::perspective`, re-exported by
  `mycelium_sdk::rule`) names the first or the second space axis of
  `Perspective::plane` by its place in the plane, never as a row or a
  column. Claim `coord_round_trips_and_reads_its_planar_byte_by_role`.
- **`About::Grid` is `About::Sample`** (SDK surface, `mycelium_sdk::rule::About`):
  a carriage or an echo read about the sample's own frame, beside
  `About::Body`. The clause and element codecs keep their bytes (a
  carriage's `about` 0, an echo's tag 9), so no clause or word address
  moves.
- **The section's planar names renamed by function** (SDK surface):
  `mycelium_sdk::task::TaskRasters` is `TaskSection`;
  `TaskEnvelope::of_rasters` and `PostedTask::of_rasters` are
  `of_section`; `mycelium_sdk::rule::Step::grid_motion` is
  `Step::symmetry_motion`; `seat_mind::task::grid_steps` is
  `symmetry_steps`; `family::grid_of_painted` is `sample_of_painted`;
  `painted_within_grid` is `painted_within_sample`. Tests renamed with
  their units: `symmetry_motion_carries_every_site_to_index_image`,
  `symmetry_motion_exchanges_non_square_shape`,
  `symmetry_steps_reads_task_before_seat`.
- The frame acts' planar fields, `statement::Spoken.frame` and the
  "grid" test names were left open here and are built in stage 2d above.
