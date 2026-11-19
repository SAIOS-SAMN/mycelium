### Retire `FramePlanes` in favor of `PolePlanes` (#239)

- Renamed `transducer::field::carrier::FramePlanes` to `PolePlanes` and `FieldRefusal::PastFrame` to `FieldRefusal::PastPolePlane` across the field carrier and well modules, adopting the approved pole-plane / pole-unit terminology (Issue #239).
- Renamed `Lifted.frame` and `Atlas::frame()` to `Lifted.poles` and `Atlas::poles()`.
