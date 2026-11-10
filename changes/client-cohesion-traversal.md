### Client: cohesion as the viscosity reading over held thoughts (MU-CLIENT §4.9, §7.1; #149)
- **Mind and Contemplation:**
  - `Contemplation::cohesion(&self) -> Result<Option<Cohesion>, FieldRefusal>`
    reads cohesion per plane from held wells' inflations on their membranes'
    frame planes (`e_+k-k`), never summed and never stored.
  - `Contemplation::cohesion_of(&self, address: &[u8]) -> Option<Result<Cohesion, FieldRefusal>>`
    reads a specific thought's cohesion.
  - `Mind::cohesion(&self)` and `Mind::cohesion_of(&self, address: &[u8])`
    expose the reading on the client SDK surface.
  - `Cohesion` re-exported in `crate::client::sdk::`.
- **Claims and Verification:**
  - `contemplation_cohesion_reads_per_plane_over_held_wells` and
    `contemplation_cohesion_none_when_empty` in `transducer::field::tests::shares`.
  - `mind_cohesion_read_per_plane_over_held_thoughts` and
    `mind_cohesion_none_when_empty` in `client::sdk::tests::cohesion`.
  - Cohesion and fluidity sum to 1 per plane (`c_k + ν_k = 1`), with no total across planes.
