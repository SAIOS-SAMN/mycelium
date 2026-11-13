### Gate chatter speech to one in flight on hypha carrier (#318, MU-CLIENT §5.1)

- **One speech ask in flight at a time (MU-CLIENT §5.1, #318):**
  - Added `speaking_agreed: bool` to `CourtPace` in `src/client/sdk/mind/court.rs` to track in-flight speech from chatter.
  - In `src/client/sdk/mind/chatter.rs`, `read_hearings` gates incoming `FieldWork::Speak`: if a speech ask is already in flight, subsequent speech stays in `field_waiting` under MU-CLIENT §4.13 return lapse physics instead of flooding `hypha.carrier.landings`.
  - Clears `speaking_agreed = false` in `speak_agreed`'s landing callback, and drains waiting speech on subsequent windings.
- **Clock height advancement tracking in conversation:**
  - Added `told_h` to `Chat` in `src/client/sdk/mind/chatter.rs` so that `tell_conversation_the_clock` forwards `Inflow::Clock` when the court's height advances, even on quiescent minds whose orbit count $K$ has not yet moved.
- **Verification:**
  - Added unit test `read_hearings_gates_speech_to_one_in_flight` to `src/client/sdk/tests/conversation.rs`.
