### The chart's pairs, the anchor and the clause tables held as section sites (track 26 S4d, stage 2c)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no task, word, chart or clause address moves. Every codec
  and key writes a section site's coordinates in the plane's order,
  `i64` each (`SectionSite::put_canonical`, `take_canonical`): the bytes
  a planar pair was written in.
- **The chart's supports.** `element::positions` returns section sites,
  and `pullback`, `blade::shape_of_section`, `section_moduli`,
  `sentence::incidence_of`, `orbit::positions_of`, `ring_dist_peel` and
  the confluence reader (`gathered`, `carried_onto`, `read_pair`) take
  them. The blade's sections hold section sites; its plane readers are
  `on_plane` and `off_plane`.
- **`on_grid` retired.** A site of the family's base is read onto the
  section by `SectionSite::of_family`; `scene::restrict` lays a region
  from its least corner by `SectionSite::span_of`. The test
  `on_grid_refuses_site_without_grid_plane` is
  `of_family_refuses_site_without_plane`.
- **The anchor's selection algebra, together.** `anchor::Evidence` and
  `EvidenceOf` hold the footprint as section sites and the scene as
  painted sites; `Selection::eval`, the derive, the carrying set,
  `candidates::signature`, `beam_minima`, `element::measure_lanes`,
  `measure_supports`, `keys::footprint_key` and the evidence reply read
  them. The face neighbours and the extremal codes are generated from
  the plane's rank; `Ext`'s code names an axis of the plane and an order,
  as its byte always read. Claim `section_site_round_trips`.
- **The clause reader's tables.** `statement::Seen` holds each body's
  painted sites and the owner of each site of the sample at its offset in
  canonical address order (`SectionSample::offset_of`), never a table of
  rows. `extent_of` is retired for `statement::corners_of`, the least and
  the greatest corner over `SectionSite::span_of`; the landed lists, the
  regions, the joins and the crop read section sites, and a line of the
  plane is read along each axis, the last first. `acts::pattern_key` and
  `stamped_key` write a centred pattern of painted sites. Claim
  `corners_of_reads_least_and_greatest_on_each_axis`.
- **Open:** the act reading's own index cells (`acts::cells_of`, the
  residue's pieces, `owners`, `step`, the echoes and the bends) read
  extents through `statement::corners_of` and keep `(row, column)`
  indices inside the act reading; the frame acts (panes, tiles, the
  square's symmetries, the scaling) stay planar by their acts.
