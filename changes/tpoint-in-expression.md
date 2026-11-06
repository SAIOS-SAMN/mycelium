### v0.5.0: the TUniversal point moves into the expression; the lane byte is retired
- **Network contract:** changed, a recut. No migration and no legacy
  reader: every hypha, client and kernel moves together, and a peer on
  the old envelope is refused whole. The frame is `[len u32 LE][body]`
  (`wire::HEAD`, `wire::delimited_len`, the bound `wire::DELIMITED_MAX =
  BODY_BUDGET + u16::MAX + 2`); the lane byte, `Lane`, `read_lane`,
  `POINTED`, `split_point`, `encode_at` and `decode_at` are gone. The body
  is an expression, `[act][point][point count u16 LE]`
  (`anastome::dialogue::expression`, `Expression`, `write_act`): the act
  as its codec writes it, tag at offset zero, then the last final
  TUniversal point the speaker read and its count, a count of `0` with no
  point bytes where none was read. No act's own layout, signed byte or
  law version changes. MU-CONCORDANCE 1.11 says the point rides inside
  the expression.
- **The door** reads an envelope's body and hands it to its clock
  (`tuniversal::hear`), which hears the speaker's point on the shelf and
  answers the act; every reply and every ask to a peer is written by
  `tuniversal::express` with the door's final point. The pointed-connection
  book (`note_pointed`, `admit_pointed`, `PointedRefusal`) is retired:
  every speaker writes the slot, so every reply carries the point.
- **The Noise opening** is told from a plaintext envelope by the first
  four bytes, which `channel::Paced::take_head` reads and serves again;
  `noise::OPEN`'s first four read as a length past `DELIMITED_MAX`.
- **The SDK** re-exports `delimited_len`, `HEAD`, `DELIMITED_MAX`,
  `Expression` and `write_act` in place of `Lane`, `read_lane`,
  `split_point` and `POINTED`.
- **The layers gate** (`gate/src/layers.rs`) lists no exception: the
  transport sources name no carrier. MU-ENGINEERING §9.2 closes both rows.
