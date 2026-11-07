### The cross-chamber bracket is read: held by a chamber or an enclosure, or lost (#148; CB1, 78.26(2); MU-CLIENT §4.9)
- **Network contract:** none. This is a reading in the mind's cognition and on
  the pivot's tally. No wire, verdict, tape or ledger layout changes.
- `cech::cross_brackets` (`transducer/cech/bracket.rs`) reads each pair of a
  sentence's chambers that meet at a body. Pairs come from a keyed index of
  the bodies, and a chamber and its own face are not paired. For each pair of
  the two chambers' read actions it composes the commutator `A∘B − B∘A`,
  exactly. A commutator that does not vanish is a `CrossBracket` with its
  chambers, circuits and content, marked `Bracketed::Chamber` where one of the
  two chambers' algebras holds it, `Bracketed::Enclosure` where a chamber
  whose bodies include both holds it, and `Bracketed::Lost` where none does.
  Pairs that are not read are named (`CrossUnread::Actions`, `Width`,
  `Spent`). Nothing is minted for a lost bracket.
- `PivotCells.brackets` carries the reading beside the cells' algebras
  (`family::pivot_cells`). `PivotTally` counts the brackets held, lost and
  unread once per fresh reading of a form's cells, on its line.
- Claims `cross_brackets_hold_in_enclosure_and_lose_without`,
  `cross_brackets_read_nothing_where_generators_commute_or_chambers_part`,
  `cross_brackets_name_a_chamber_with_no_actions`.
- Owed: what comprehension constructs from a lost bracket (an enclosure
  wrapping chambers that no cell of the sentence holds) waits on a ruling.
