### Proposal 78.178: the parity lane P, the first opened lane (Proposed)
- **Network contract:** none. Documents only; no code opens P until the
  peer kernels' readings agree.
- **`docs/proposals/78.178-parity-lane.md`**, Proposed: reflections carried
  on their own lane, P (operator, 2026-11-05; 78.174 §1 item 11), one
  component per reflecting normal (`e_+k`, `e_-k`), each an exact signed
  whole count never reduced, its parity a reading; opened as the entry
  `("P", 5)` of `Openings::ruled()`; refusals `NotReflectionNormal` and
  `NotWhole` beside `LaneNotOpened`; eight joint-read fixtures; open: a
  composite's H reading and the opening's version.
- 78.173 §7 and the GLOSSARY's *Opened lane* cite it.
