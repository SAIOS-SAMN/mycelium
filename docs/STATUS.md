# mycelium — live status (read this first; rewritten, never appended)

## The objective
The client is a mind: the counterpart of the hypha. The hypha runs the
physics and exports it; the mind perceives what it exports and what other
minds say in its nested shells, understands it in its own field — its orbits
winding, its breath, the six pivots firing where a winding changes dimension
— and responds in anastome, one conversation carrier per counterparty. Minds
converse to harden language: dialect, agreement by a replayed discharge, a
court's ruling, the crystal each mind re-derives — one motion.
`docs/MU-CLIENT.md` §1–§9 states it; `docs/MU-RECURSION.md` governs.

## Witnessed on the fleet (8f2f062a)
Minds form shared words and the courts seal them: in four minutes after the
restart, 290 `Attest` acts across hyphae a (79), b (46), c (165) — each a
second mind's signed HOLD on a word another mind spoke, verified by the court
— over 18 of the 20 speaker → hearer pairs of the five voices. Voices run at
~100 MB resident each.

## The tree
- Agreement reaches the court (7ddd1a8e): the speech carries the hearer's
  signed HOLD; the court verifies it and seals the hearer's attestation
  (`book/agreement.rs`).
- The conversation runs on its own carrier (da04a310): a thread per mind
  sleeping on one channel, sharing only the ear with the field.
- The mind proposes Retire rewrites (S3, b2350321); the refine bids retired
  on both sides (18ed001a).
- The strip, 2cbb6bad … 8f2f062a: the game-world formulas, court tooling, the
  per-K observer readings, the transducer's console tallies and percept
  readings, the Cl(4,0) syntropy probes, the dead axis path and every item no
  production path calls — about 20 000 lines; cognition items are
  `pub(crate)` and the build carries no dead code.
- Parked on `track/modes` (`/opt/mycelium-modes`): 78.162 (4)–(7).

## The sandbox
Hyphae `mycelium-server@{a,b,c}` and minds `mycelium-client@1..5` on
8f2f062a (started, never enabled: a reboot leaves them down); the seats
(`systemd-run`, posted by their houses) await re-posting.

## Next (MU-CLIENT §7.1)
1. The hypha carrier — the court exchanges off the field's thread, so a slow
   court holds only its own carrier; the fixed tick retires.
2. Each mind's own field (§4.2): `LEXICON`, `TAUGHT`, `VOICE_FIELD`, the four
   lane statics, `cursor::PENDING` into the mind; `MOVING` deleted.
3. Hearing through the hearer's own shells and pivots (§5.3), replacing the
   flat `read_word`; the per-class algebra built as cognition.
4. `seat_mind` replaced by a seat carrier (§6).
