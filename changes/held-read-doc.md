### READ-MAIL documented as a held read with no time bound (#133)
- `sdk/src/house.rs` and MU-HOSTING §3.1 step 6 now state the following.
  A house's READ-MAIL whose box holds nothing past its cursor is answered
  at the court's next event: a seat post filed on the relay, or a keeper
  reading signed. The hold reads no clock (78.146 as amended, 2026-10-17).
  A caller sets no read timeout on it, or treats a timeout as "nothing
  yet" and asks again.
- Found by the #133 run against xylarium's house: its own 30 s read
  timeout surfaced as `os error 11` on a quiet relay. No behaviour changes.
