### The crossing record, X1: its four parts written as exact values and carried on every universe act (#160, v0.5.0, draft)
- `anastome::dialogue::crossing_record`: `Crossing { place, content,
  holonomy, clauses }`. The place is the act's TUniversal point and its
  past cone (`parents`). The content is the frame's periods per phase,
  stated once, the bodies before and after (each a `Body`: its origin per
  phase beside its `TurnBody`, breakpoints per phase and an extent share
  in `[0, 1]`) and the motion (a versor, or turns per phase). The holonomy
  is one versor per ring, each ring named by its vertices from the least,
  its orientation kept (`Ring`; a writing off the least start is refused
  `RingStart`). The clauses are the clause ids read, the cited final points
  and the signing stamps.
- A record whose content is absent is refused `AddressOnly`.
- `Turn::put` / `Turn::take`: a turn is written `(r, h)` with its winding
  `K` beside it and read back only through `Turn::of`; a winding that is
  not the turn's own refuses.
- The carriage (wire v3.0, `WIRE_VERSION = 0x0300`, a recut): the fold,
  anchor, map, mouth, conversion out, manifest and statement each carry a
  `Crossing` in their signed bytes (`crossing_record::crossed`: the act's
  own bytes, then the record under its own domain) and on the wire before
  the signature. The act's own hash and id stay its own bytes'. A line's
  fold carried inline and a key's own handoff carry none.
- Admission verifies the record from itself (`Crossing::verify`): no frame
  is `NoFrame`, a body with no origin `NoPlace`, a universe act with no
  record `Absent`, and a motion stated as turns is read by `verify_turns`
  over the record's frame and the bodies' origins (`Turns`, each breach
  named; `Unpaired` where the counts before and after differ). Refused as
  `Boundary::Crossing { act, domain, why }` on each act's reply.
- `mycelium_sdk::forms` re-exports the record beside the `forms` crate.
- MU-HOSTING §5a.10 restated: a turn's residue keeps each side's winding
  (LAW-COORDINATE v4 ruling (c)); `AddressOnly` ruled.
