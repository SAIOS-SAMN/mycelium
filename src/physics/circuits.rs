use super::metabolism;

/// A CARRIER'S CIRCUITS (45.7; named so at OR2, 73.7: the circuit is the
/// clock, the orbit is the word): the circuits a stroke ring has completed
/// and the period of the last one in strokes — the kernel's local time
/// beside K, read off the ring and never a clock. One per carrier: the
/// court's docket, the exchange to one peer, the mind's dirty ring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Circuits {
    strokes: u64,
    circuits: u64,
    at: u64,
    period: u64,
}

impl Circuits {
    /// One stroke over the carrier, with the circuits it completed. The
    /// reading — circuits so far and the strokes the last one took — is
    /// returned when a circuit completed, so the caller writes its circuit
    /// line once per circuit (34.2, 34.3) and never per stroke.
    pub fn stroke(&mut self, circuits: u64) -> Option<(u64, u64)> {
        // A count past its carrier is refused and counted, and no circuit is
        // read on the stroke that met it (STD1, rule 11).
        if !metabolism::step_or_refuse(&mut self.strokes, 1) {
            return None;
        }
        let n = std::num::NonZeroU64::new(circuits)?;
        if !metabolism::step_or_refuse(&mut self.circuits, n.get()) {
            return None;
        }
        // A period is strokes since the last circuit. `at` is a stroke this
        // counter has already passed, so the difference is exact; a mark
        // ahead of the count is a broken counter and reports no circuit
        // rather than a period of zero (rule 11).
        self.period = self.strokes.checked_sub(self.at)?;
        self.at = self.strokes;
        Some((self.circuits, self.period))
    }

    /// Circuits completed so far.
    pub fn circuits(&self) -> u64 {
        self.circuits
    }

    /// Strokes the last circuit took; zero before the first.
    pub fn period(&self) -> u64 {
        self.period
    }
}
