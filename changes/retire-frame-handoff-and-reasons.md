### Retire `handoff::frames_hold` and `WordReason.frames` (#239)

- Renamed `Mouth::frames_hold` to `Mouth::charts_hold` in `src/anastome/dialogue/handoff.rs`, `src/server/cognition/domain_passage.rs`, and `src/anastome/dialogue/tests/handoff.rs`.
- Renamed `WordReason.frames` to `WordReason.samples` in `src/transducer/sentence/reasons.rs`.
