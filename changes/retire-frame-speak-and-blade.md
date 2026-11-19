### Retire `frame` names in `family::speak` and `blade::continuous` (#239)

- Renamed `Speech.frame` to `Speech.extents`, `output_frame` to `output_extents`, and `grows_to_frame` to `grows_to_enclosure` in `src/transducer/family/speak.rs` and `extract.rs`.
- Renamed `NotCarried::Frames` to `NotCarried::Bases` in `src/transducer/blade/continuous.rs`.
