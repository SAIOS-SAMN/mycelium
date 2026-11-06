### A claim of factorisation states its perspective; the court reconstructs under it (track 26 S6 stage 2, #32)
- **Network contract (v0.4.0's break).** The claim of factorisation
  (`ACT_SOLVE`, claim kind 1) carries the perspective after its residue,
  `[rank u16][(source u16)(sense u8) · rank][role u8 · rank]`, and is
  signed as `mycelium factorisation v2`. `encode_factorisation` takes the
  perspective; `FactorisationAct.perspective` reads it back. The SDK
  re-exports `Perspective`, `Role` and `SensorMotion` beside it.
- **The court** reads its copy of the posted pair through the stated
  perspective and reconstructs there (`server::factorisation`); a
  perspective of another rank is refused `REFUSE_FACTOR_PERSPECTIVE_RANK`,
  one reading an axis as other than space `REFUSE_FACTOR_PERSPECTIVE_ROLE`.
- **The mind** perceives under a perspective (`TaskPercept::under`) and
  states its percept's perspective on every claim.
- Claim: `claim_states_its_perspective_and_court_reconstructs_under_it`;
  `factorisation_claim_round_trips` reads a turned, recast perspective back.
