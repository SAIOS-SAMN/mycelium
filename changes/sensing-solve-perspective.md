### A solve of outputs and speech state their perspective; the court reconstructs under it (track 26 S6 stage 3, #32)
- **Network contract (v0.4.0's break).** The solve of outputs (`ACT_SOLVE`,
  claim kind 0) carries the perspective its outputs were perceived under
  right after them, written as the claim of factorisation writes it, and
  is signed as `mycelium solve v5`. `encode_solve_spoken` takes the
  perspective; `encode_solve` and `encode_solve_with_rule` state the
  default of the outputs' rank; `SolveAct.perspective` reads it back. Each
  sentence of a speech carries the perspective it was spoken under,
  `carries` bit 3 after the ground (`Spoken.perspective`,
  `SpeechAct.perspectives`).
- **The court** reads its copy of the task's pairs through the stated
  perspective (`cognition::stated_perspective`): the train and withheld
  pairs for a solve, and the bound family for a sentence, held for the
  whole adjudication and a waiting sentence's re-hearing
  (`Court::hear_speech_under`, `family_heard`). Another rank is refused
  `REFUSE_PERSPECTIVE_RANK`, a non-space role `REFUSE_PERSPECTIVE_ROLE`. A
  sentence stating none is heard on the family as posted.
- **The mind** states the section's perspective on every solve and
  sentence (`SectionSample::section_perspective`).
- Claim: `solve_and_speech_state_their_perspective_and_court_reconstructs_under_it`.
- Open: the seat's solve call; a word's install after its hearing reads the
  family as posted.
