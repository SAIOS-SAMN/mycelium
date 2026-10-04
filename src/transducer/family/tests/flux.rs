// 78.91 — THE MIND PERCEIVES WHAT IS NO MOTION AS FLUX (`family/flux.rs`):
// the percept's non-motion demands read as the fluxes their links carry and
// put to the law register; motion demands are not its to read.

use crate::physics::laws::Flow;
use crate::transducer::family::flux::{flux_tally, perceive_fluxes, Named};
use crate::transducer::sentence::Demand;

/// A creation is perceived as a source, a vanishing as a sink, a deformation
/// at constant mass as no flux; a demand of another kind is not read. The
/// tally is the process's, so it is read as the difference across the call.
#[test]
fn creation_and_vanishing_are_perceived_as_lawful_source_and_sink() {
    let chart = crate::transducer::stalk::Invariant::default();
    let demands: Vec<Named> = vec![
        (Demand::Creation, chart.clone(), vec![(0, 0, 0, 3), (0, 2, 0, 3)]),
        (Demand::TopologicalDeformation { image: chart.clone(), axes: Vec::new() }, chart.clone(), vec![(1, 1, 2, 0)]),
        (Demand::TopologicalDeformation { image: chart.clone(), axes: Vec::new() }, chart.clone(), vec![(1, 1, 2, 0), (1, 2, 0, 2)]),
        (Demand::RotorPrime { plane: (0, 1), prime: 3, witness: Box::default() }, chart.clone(), vec![(0, 0, 0, 1)]),
    ];
    let before = flux_tally();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let fluxes = perceive_fluxes(&demands, &mut budget);
    let flows: Vec<Flow> = fluxes.iter().map(|f| f.flow).collect();
    assert_eq!(flows, vec![Flow::Source, Flow::Sink], "a source, a sink, a reshaping that is no flux");
    let after = flux_tally();
    assert!(after.read >= before.read + 3, "three non-motion demands read");
    assert!(after.emitted >= before.emitted + 2 && after.absorbed > before.absorbed);
    assert!(after.reshaped > before.reshaped);
}
