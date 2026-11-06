### The crossing record, X1: its four parts written as exact values (#160, v0.5.0, draft)
- `anastome::dialogue::crossing_record`: `Crossing { place, content,
  holonomy, clauses }`. The place is the act's TUniversal point and its
  past cone (`parents`). The content is the bodies before and after as
  `TurnBody` (breakpoints per phase, an extent share in `[0, 1]`) and the
  motion (a versor, or turns per phase). The holonomy is one versor per
  ring. The clauses are the clause ids read, the cited final points and the
  signing stamps.
- A record whose content is absent is refused `AddressOnly`.
- `Turn::put` / `Turn::take`: a turn is written `(r, h)` with its winding
  `K` beside it and read back only through `Turn::of`; a winding that is
  not the turn's own refuses.
- MU-HOSTING §5a.10 restated: a turn's residue keeps each side's winding
  (LAW-COORDINATE v4 ruling (c)); `AddressOnly` ruled.
- A wire change (a recut) once each universe act carries the record. That
  carriage is owed in this draft.
