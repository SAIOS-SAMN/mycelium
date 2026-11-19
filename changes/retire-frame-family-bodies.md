### Migrate internal `Frames` usages to `FamilyBodies` (#239)

- Replaced internal references to the `Frames` alias with `FamilyBodies` across `src/transducer/family/`, `src/transducer/field/`, and `src/client/sdk/mind/` (retaining `pub type Frames = FamilyBodies` for `mycelium-sdk` compatibility).
- Renamed `Thought.frames`, `PairPercept.frames`, and `ascertain_with_frames` to `Thought.bodies`, `PairPercept.bodies`, and `ascertain_with_bodies` in `src/client/sdk/mind/`.
