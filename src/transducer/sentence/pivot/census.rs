// MD1, MR1 — AN ORBIT'S MODES, read off its holonomy's own action.

use super::*;

/// WHAT THE MODE SPLIT READ OFF AN ORBIT, each arm by name (rule 11).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModesRead {
    /// `(class, planes, order)` per mode, the order where an elliptic mode
    /// closes; a parabolic or hyperbolic mode, or a rosette, reads `None`.
    Split(Vec<(crate::network::nerve::coord::BivectorKind, usize, Option<u32>)>),
    /// The motion's modes lie in an extension of ℚ, or its action does not
    /// split into commuting rays that multiply back to it.
    Undecomposed(crate::transducer::cga::Undecomposed),
    /// The element is not a versor: it has no action to read.
    Unread,
}

impl ModesRead {
    /// THE ONE READER of an orbit's modes, from its holonomy's action (MR1),
    /// under the stroke's budget (BD1).
    pub fn of(m: &element::Versor, budget: &mut crate::physics::Budget) -> ModesRead {
        match crate::transducer::cga::motion_modes(m, budget) {
            None => ModesRead::Unread,
            Some(Ok(ms)) => ModesRead::Split(ms.iter().map(|x| (x.kind, x.planes, x.order)).collect()),
            Some(Err(e)) => ModesRead::Undecomposed(e),
        }
    }
}
