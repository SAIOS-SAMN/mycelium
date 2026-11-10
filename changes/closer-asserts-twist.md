### Client: closer asserts the closed twist under the family (78.177 §4a(4), #144)
- 78.177 §4a(4): when a held closed form reproduces a listed demand's bodies, the closer asserts the closed twist under the demand's family (E1); the asker, the listing's speaker, is paid nothing.
- `Mind::assert_closed_twist`: recovers the scene's family from the demand's links (`scene_at_of_chain`), matches the scene citation from `Scenes::scene_of_family`, and asserts the closed word under the family.
- `Mind::answer_demands`: invokes `assert_closed_twist` upon `ProbeClosure::Closed`.
- `Contemplation::take_twists` and `Contemplation.twists`: carry the perceived family with each twist returning from an orbit (`close_thought`), transcribed under that family (`transcribe_closed`).
- Claim `closer_asserts_closed_twist_under_the_family` verified in `src/client/sdk/tests/listed.rs`.
