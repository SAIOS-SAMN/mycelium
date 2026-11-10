// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-CLIENT §1, §4.9, §7.1 (ruled 2026-11-06; issue #149) — COHESION AS THE
// VISCOSITY READING PER TURNING PLANE, AND TRAVERSAL: per turning plane k,
// the held share c_k = A/(A + Q) = ν(1/t) is the viscosity; the open share
// ν(t) is fluidity. Both are ½ at the throat, exchanged by the passage.
// Never summed across planes.

use super::fixtures::*;
use crate::client::sdk::*;
use crate::core::Q;
use num_traits::{One, Zero};

/// AN UNHELD MIND HAS NO COHESION: with no thoughts held in contemplation,
/// `mind.cohesion()` returns `Ok(None)` without refusing.
#[test]
fn mind_cohesion_none_when_empty() {
    let mind = Mind::open("127.0.0.1:1", Voice::named("cohesion_empty"));
    assert_eq!(mind.cohesion(), Ok(None));
}

/// A HELD THOUGHT READS COHESION PER PLANE, NEVER SUMMED:
/// holding a thought perceives its frames, reading cohesion per plane from
/// its well's inflation on the membrane's frame plane. Cohesion and fluidity
/// sum to 1 per plane, with no total across planes.
#[test]
fn mind_cohesion_read_per_plane_over_held_thoughts() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("cohesion_held"));
    let scene = [42u8; 32];
    let _ = bind_a_thought(&mut mind, &scene);

    let cohesion = mind.cohesion().expect("reading").expect("cohesion stands");
    assert!(!cohesion.planes().is_empty(), "holds planes");

    for (k, c) in cohesion.planes() {
        assert!(c > &Q::zero(), "held share is positive on plane {}", k.written());
        assert!(c <= &Q::one(), "held share does not exceed 1 on plane {}", k.written());
    }

    let fluidity = cohesion.fluidity();
    assert_eq!(fluidity.len(), cohesion.planes().len());
    for (k, nu) in &fluidity {
        let c = cohesion.on(k).expect("matching plane");
        assert_eq!(nu + c, Q::one(), "per plane, c_k + ν_k = 1 on {}", k.written());
    }
}
