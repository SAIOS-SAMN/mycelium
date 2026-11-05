### `forms::coordinate` holds LAW-COORDINATE v1, the coordinate a kernel addresses its bodies by (proposal 78.173; track 24 B3)
- **Network contract:** no wire byte, act, ledger layout or reading
  changes. A new shell, `FORMS-COORDINATE` v1, joins `forms::LAW_IDS`
  (and the SDK's `FORM_LAW_IDS`).
- **The coordinate** `origin / h : S:G:T:H : K`: `Coordinate`, `Origin`
  (the genesis and epoch, then the enclosures written out as
  coordinates, never a digest) and `Local` (the bell, four signed exact
  lanes S, G, T, H, and `K`). Its derived order is the canonical key, and
  its writing is TPoint's writing where there is no enclosure and every
  lane is non-negative.
- **The sexagesimal reading:** `Sector`, the exact interval at a depth in
  base 60 (`BASE`, classed `EXTERNAL` by the numbers gate), nesting by
  depth, with places and a writing (`0;08,34,17`) for display only.
  `Coordinate::within` decides on the sector's exact bounds and refuses
  two origins `OriginsDiffer`.
- **Neighbours:** `Bodies` holds coordinates in canonical order, and
  `Bodies::range` returns a range on one lane by two bisections,
  claimed equal to the test of every body.
- **Next and open:** `TPoint` is not rewired onto the form in this
  change. The signed-to-issuance map at the vertex is open.
