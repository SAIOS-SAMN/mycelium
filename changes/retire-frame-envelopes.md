### Retire transport `frame` names in chatter and peer receipts in favor of `envelope` (#239)

- Renamed `Inflow::Frame`, `FieldWork::Answered.frame`, `Answer::Agree.frame`, `Question.frames`, `Closed.frames`, `fn frame`, `hear_frame`, `read_frame`, `hold_answer_frame`, and `keep_closed_frames` to their `envelope` / `envelopes` equivalents across `src/client/sdk/mind/chatter.rs`, `src/client/sdk/mind/peers.rs`, and `src/client/sdk/tests/peers.rs` (Issue #239, Sense 5).
