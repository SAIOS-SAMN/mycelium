### Rank-n perception and transduction as factorisation (track 26 S4b)
- **No axis is fixed in the code.** `GRID_AXES`, `GRID_PLANE`,
  `is_grid_axis`, `FRAME_AXIS`, `GRID_N`, `PLANE_ROW` and `PLANE_COL`
  are gone. The roles of a sample's axes come from a perspective
  (`transducer::perspective`): the default reads every axis as space,
  and a site of a family's base carries the sample's place in its family
  after its axes. The sensor plane is a perspective's first two space
  axes. `BASE_AXES` is derived from the alphabet's slots.
- **The alphabet stays at `n = 3`, derived.** `ALPHABET_BASE_N =
  LETTER_RANK − 1`; a sample of a rank the letters are not written on is
  refused by name (`AlphabetRank`), never truncated. Generalising the
  letters past `n = 3` is the next phase.
- **D2.5 O3 derived.** `STROKES_PER_RESERVE = 1 << ALPHABET_BASE_N = 8`:
  the vertices of the unit cell of the base the founding motions act on.
  No task's rank enters the process's capacity. The value is unchanged.
- **A link carries a site of rank `n`** (`sentence::Link`: a `Site` and
  the two symbols at the source's width). The word, chart and field
  carriers read `SectionLink`, the former two-coordinate tuple, reached
  from a link only through `Link::section`, refused by name.
- **Connection by grade.** Bodies connect through a shared cell of grade
  `n − 1` at every rank (`incidence_of_sites`); the fixed 4-adjacency and
  the rule language's offset tables are generated (`faces_of`,
  `surrounding`, `by_grade`).
- **Transduction as factorisation.** Every pair factors as motion ∘
  residue (`transduce::factorisations`, `Factorisation`): the motion a
  signed permutation of the phases with the re-period onto the output's
  enclosure, the residue the incidences it does not carry, as rank-`n`
  links. Every motion that lands is held, several as a vortex
  (`Factored::Vortex`). A court derives the residue from a claim's stated
  motion and admits on a match (`court::verdict::factorisation`).
- **A task of any rank is perceived.** The mind perceives every task it
  takes at its own rank (`transduce::TaskPercept`, held under the task's
  citation) before the word path. A task off the alphabet's section stops
  `BeyondAlphabet`.
- **Network contract.**
  - No wire byte, act or ledger layout changes. No task address changes:
    a task's identity is its samples' content, untouched.
  - No word or chart address changes in this phase. The section's links
    and every chain written to a tape are unchanged, and the oracle's
    golden reading reproduces unchanged.
  - SDK surface: `task::NotRaster` is replaced by `task::SectionRefusal`,
    and `TaskEnvelope::rasters` / `PostedTask::rasters` are renamed
    `section`.
  - The mind's typed task reading (`observe::TaskSeen`) may carry the
    new stop code 17 ("perceived beyond the alphabet"). A seat answers
    such a task at once and sends no declination for it.
- **Changed result at a base of two coordinates.** A base of two
  coordinates is now a rank-1 sample and its place, so its second
  coordinate takes no half in a compartment's vertex point
  (`torsion_quotient::vertex_point_equals_sites_centroid`). Every rank-2
  reading is unchanged.
- **Not in this phase:**
  - the rank-2 carrier the word, chart and rule-language machinery
    spells (`family::Grid`), kept as the alphabet's section;
  - the hypha's registry and solve replay reading posted tasks of any
    rank (built in S4c, above);
  - a wire act for a claim of factorisation (built in S4c, above).
