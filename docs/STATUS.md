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

## Witnessed on the fleet (d2b8a20e)
Minds restarted at 04:15:47 against courts still catching up after their
own restart (a court stroke held its kernel ~146 s): the first `Attest` 35 s
in, 241 in four minutes — a 65, b 14, c 162 — over 19 of the 20 speaker →
hearer pairs, and solve verdicts flowing. On 78e76b11, where each mind waited
on those courts synchronously, the same courts had sealed none in twenty
minutes.

## The tree
- The hypha carrier (d2b8a20e, MU-CLIENT §5.1): the mind's session to the
  nerve on its own thread; the field posts each exchange and takes the answer
  at its next admission; a resting mind is woken by the court's word.
- Each mind thinks in its own field (78e76b11, §4.2): tubes and orbits, lane
  ledgers, lexicon, taught words and parked covers are the mind's
  `MindField`, scoped to its tick; the court books on its shared state.
- Agreement reaches the court (7ddd1a8e): the speech carries the hearer's
  signed HOLD; the court verifies it and seals the hearer's attestation.
- The conversation runs on its own carrier (da04a310).
- The mind proposes Retire rewrites (S3, b2350321).
- The strip, 2cbb6bad … 8f2f062a: about 20 000 lines out; the build carries
  no dead code.
- Parked on `track/modes` (`/opt/mycelium-modes`): 78.162 (4)–(7).

## The sandbox
Hyphae `mycelium-server@{a,b,c}` on 78e76b11 and minds
`mycelium-client@1..5` on d2b8a20e (started, never enabled: a reboot leaves
them down); the seats (`systemd-run`, posted by their houses) await
re-posting.

## Next (MU-CLIENT §7.1)
1. Hearing through the hearer's own shells and pivots (§5.3), replacing the
   flat `read_word`; the per-class algebra built as cognition (§4.2's last
   owed item).
2. `seat_mind` replaced by a seat carrier (§6); the seat surface's waiting
   link (`Entity::relay`) and `pull_task_by_id`'s settle retire with it.
