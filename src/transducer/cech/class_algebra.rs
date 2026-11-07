// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-CLIENT §3.2 (the holonomic shell), §4.2, §4.11, §1a step 4 (#143) —
// THE ALGEBRA HELD PER CLASS in the mind's own field.
//
//! Each ring the mind takes in has a curvature, its holonomy `Ω`, and that
//! curvature generates an algebra: its flow generators (`flow_generators`),
//! closed under the commutator with what the mind already holds for the
//! ring's CLASS (`lie_close`). The court holds this algebra per chart
//! (`lie_basis`). The mind holds it per class, because a class is
//! node-independent and a chart is one node's coordinate (§3.3, 78.44).
//!
//! THE CLASS is the holonomy rotor's conjugacy invariants `(a²/w, p²/w,
//! kind)` written out (`Rotor::class_key`, 15.3), which is the first segment
//! of the class vector and the stratum every court asks by (§4.6). Its value
//! is unchanged under `Ω ↦ Γ Ω Γ⁻¹`. So two minds that hear one word in two
//! frames file it under one class, and each holds the algebra on its own
//! planes. The conjugating element `Γ` carries the one basis onto the
//! other's span, and it is the translation between their perspectives
//! (§4.11(2); 23.9).
//!
//! WHAT LIES OUTSIDE THE HELD SPAN is what the mind newly understands. It is
//! read per axis as each generator's coset (`quotient::Outside`, its
//! coordinate `Span::residue`). Only that part joins the class's algebra, so
//! the algebra grows only where a generator lies outside what is held. The
//! reading is returned to the caller and never stored beside the basis
//! (law 12).

use super::quotient::{quotient, Quotient};
use crate::network::nerve::Keyed;
use crate::transducer::address::Address;
use crate::transducer::cga::Mv;

/// THE MIND'S ALGEBRA PER CLASS: for each class it has heard, a basis of the
/// bracket-closed algebra its rings generated, keyed by the class's
/// coordinates written out. A class never heard holds the zero algebra.
#[derive(Clone, Debug, Default)]
pub struct ClassAlgebras {
    held: Keyed<Vec<Mv>>,
}

/// ONE RING HEARD AGAINST ITS CLASS'S ALGEBRA: the class it was filed
/// under, and its generators read against what the class held before it.
/// The `outside` part is what the mind newly understands, each generator
/// with its coset's coordinate. An empty `outside` means the class's
/// algebra did not grow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeardInClass {
    pub class: Address,
    pub quotient: Quotient,
}

/// WHY A RING WAS NOT HEARD IN ANY CLASS. Each refusal leaves the held
/// algebra as it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassRefusal {
    /// The holonomy has no rotor at its poles, or the rotor has no
    /// conjugacy class (odd, or of zero weight).
    Unclassed,
    /// The holonomy has no generators to read: odd, no versor, or modes
    /// the stroke could not split.
    Unread,
    /// The stroke ran out while closing the algebra under the commutator.
    Spent,
}

/// THE CLASS A RING IS FILED UNDER: its holonomy rotor's conjugacy
/// invariants, written out. `None` where the holonomy reads no rotor at
/// its poles or the rotor has no class.
pub fn class_of(ring: &Mv) -> Option<Address> {
    crate::transducer::element::base_rotor_of(ring).ok()?.class_key()
}

impl ClassAlgebras {
    pub fn new() -> ClassAlgebras {
        ClassAlgebras::default()
    }

    /// The basis held for `class`, the held basis first in the order it
    /// closed. Empty for a class never heard.
    pub fn basis(&self, class: &Address) -> &[Mv] {
        self.held.get(class.as_bytes()).map_or(&[], Vec::as_slice)
    }

    /// How many classes hold an algebra.
    pub fn classes(&self) -> usize {
        self.held.len()
    }

    /// RING `ring` HEARD IN ITS CLASS: its generators read against the
    /// class's held algebra, and the class's algebra closed with those that
    /// lie outside it. Where every generator is inside, nothing is closed
    /// and the basis stands as it was.
    pub fn hear(&mut self, ring: &Mv, budget: &mut crate::physics::Budget) -> Result<HeardInClass, ClassRefusal> {
        let class = class_of(ring).ok_or(ClassRefusal::Unclassed)?;
        let held = self.basis(&class).to_vec();
        let read = quotient(&held, ring, budget).map_err(|_| ClassRefusal::Unread)?;
        if !read.spanned() {
            let outside: Vec<Mv> = read.outside.iter().map(|o| o.generator.clone()).collect();
            let closed = super::lie_close(&held, &outside, budget).map_err(|_| ClassRefusal::Spent)?;
            self.held.insert(class.as_bytes(), closed);
        }
        Ok(HeardInClass { class, quotient: read })
    }
}
