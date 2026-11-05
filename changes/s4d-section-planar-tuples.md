### The section's planar tuples held as section sites (track 26 S4d, stage 2b)
- **`SectionLink` is a named type.** The tuple `(i64, i64, u8, u8)` is
  now `sentence::SectionLink { site, before, after }`: a section site
  (`spelled::SectionSite`) and the two symbols. It orders by the site,
  then the symbols, as the tuple did. Every carrier that spelled the
  tuple reads it: the element's chain (`CanonicalChain`, `sub`), the
  receipts, the form's demands and remainder, the field, the court, the
  mind and the soak. The keys of a section site (`family::site_key`,
  `sentence::section_site_key`, `field::remainder_site_key`) take the
  site. A chain's least corner is one reader, `element::corner_of`,
  and `ChainAt::corner` is a section site. Claim
  `section_link_holds_section_site_and_transition`. Every codec writes
  the coordinates in the plane's order and then the two symbols, so no
  wire, tape, task, word or chart address changes.
- **`PaintedSite` holds the painted triple.** `(i64, i64, u8)`, its
  alias `family::Painted` and `incongruence::Cell` are now
  `spelled::PaintedSite { site, symbol }`, ordered as the triple was.
  `raster::painted_sites` is the sensor's one reading and returns it;
  `raster::painted_of` is retired. The shells, the layers, the orbit,
  speech (`Speech.painted`, `remainder`, `statement::Spoken.painted`),
  the germ (`germinate`, `apply`), the local section, the incongruence
  reader and `cognition::frame_of` read it, and a scene's span is
  `SectionSite::span_of`. Test fixtures write a scene once through
  `sentence::tests::fixtures::painted`. Claim
  `painted_sites_round_trip_through_their_span`; the test
  `painted_of_omits_ground` is renamed `painted_sites_omits_ground`
  with its unit. No wire, tape, task, word or chart address changes.
- **Open:** the chart's coordinate pairs, the anchor's selection
  algebra (footprint and scene together) and the clause reader's planar
  tables. Stage 3 proposes naming the rule language's `Coord` variants
  by perspective role (ROADMAP track 26 S4d).
