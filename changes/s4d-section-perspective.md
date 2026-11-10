### The section is read by sites through its perspective (track 26 S4d, stage 2)
- **Sites, not lines.** A section sample is read through its
  perspective (`SectionSample::perspective`), admitted only where the
  perspective's plane spans every axis of the sample. A site of the
  section, `spelled::SectionSite`, is the sample's coordinate vector
  restricted to that plane. `SectionSample::lines`, `line`, `line_mut`
  and the `[r][c]` index are retired; readers walk `sites`, read `at` and
  write `paint` in place, and a site off the sample carries no symbol.
  `SectionSample::filled` is refused `Unspelled`.
- **Speech writes the sample in place.** `statement::speak_elements`
  paints the input sample, not a raster; the panes, the tiling, the crop
  and the scaling each write a sample. The court's solve replay is keyed
  by each withheld sample's content written whole.
- **The symbol set refuses by name.** `SymbolSet::of_family` returns
  `Result`: a posted test input of another symbol width is refused
  `SymbolRefusal::Width`, never read as an empty set, and the erasure act
  is then unstated.
- Claims `section_site_is_the_sample_coordinate_restricted_by_the_perspective`
  and `symbol_set_refuses_differing_test_width`. No
  wire, task, word or chart address changes; `scripts/check` is green,
  2090 tests passing. Open: the section's planar tuples (`SectionLink`,
  the painted triple, the chart's coordinate pairs).
