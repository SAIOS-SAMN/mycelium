### Perspective as a sensor motion; the perspectives of one sample as a cover (track 26 S6, #32)
- **No network contract.** No wire, tape, task, word or chart address
  changes.
- **The pose** (`transducer::perspective`): a `Perspective` holds the signed
  permutation of the sample's phases the sensor reads through beside its
  roles, the roles those of the axes as read. The default is the identity
  pose with every axis space, as before.
- **A shift of perspective is a motion of the sensor** (`SensorMotion`):
  `Pose(σ)` moves the pose to `pose ∘ σ` and the roles with their axes;
  `Recast` reads one axis in another role. `Perspective::read` and `frame`
  read a sample through the pose, its place kept; the sample is never
  rewritten. `turn_to` is the sensor's turn between two perspectives.
  Refusals are named (`PerspectiveRefusal`).
- **The perspectives of one sample are a cover**
  (`transducer::perspective::cover`): each perspective's 0-cochain is the
  contacts it reads (neighbouring sites of one symbol along an axis it
  reads as space); an overlap holds the turn between two perspectives and
  their agreement, congruent or the residue of contacts each alone reads,
  held and never a gate; the cocycle closes where the turns compose around
  a triple, and otherwise names the turn around it (MU-CLIENT 24.4).
- **The frame fixing is a perspective** (`raster::frame_fixing`): the
  moments name a pose, and `frame_fixed` reads the task through it; the
  local reorientation (`reoriented`) is removed. Its readings are unchanged.
- Claims: `perspective_shift_is_a_sensor_motion`,
  `two_perspectives_of_one_sample_compared_as_a_cover`,
  `frame_fixing_is_a_perspective_reading`.
