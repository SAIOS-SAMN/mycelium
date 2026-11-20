### Retire `Constellation.frames` and `frames_stand` in favor of `poses` (#239)

- Renamed `Constellation.frames` to `Constellation.poses` and `frames_stand` to `poses_stand` across `src/physics/equilibrium.rs`, `src/physics/tests/equilibrium.rs`, and `src/transducer/field/cross_scene.rs` per MU-ENGINEERING §9.3 (*A frame of reference: poses*).
