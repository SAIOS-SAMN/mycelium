// 24.1 — THE TRANSPORT APERTURE: how open the contact to one peer is, read
// from its round trips in strokes.
//
//! Every reading here is in STROKES (law 9): the rtt an aperture holds is a
//! stroke count and never a duration, and occlusion is a full stroke-cycle
//! of consecutive failures rather than a timeout.


/// The transport aperture — the perceived state of the wire to the server
/// (distinct from the epigenome's chromatic `Aperture`). A timeout or
/// refusal is not an anonymous error: it is a measured physical condition
/// (an impedance mismatch — a closed door or a saturated manifold). The
/// entity perceives it, folds it into its metabolic pacing, and adapts its
/// speech instead of redialing blind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportAperture {
    /// The last exchange passed — the wire is clear.
    Open,
    /// Resistance measured: `z` consecutive failed exchanges. Speech
    /// continues, paced by the impedance (the dwell scales by 1 + z).
    Impeded { z: u64 },
    /// Sustained saturation — a full stroke-cycle of consecutive failures
    /// (the strokes-per-reserve ratio). The entity turns inward this
    /// stroke: internal study and dialect work instead of speech.
    Occluded { z: u64 },
}
