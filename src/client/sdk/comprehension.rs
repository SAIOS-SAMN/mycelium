// 24, 26 — the comprehension verdict a client reads.

use super::*;

/// The comprehension verdict — the C-STACK's terminal (congruence → coherence →
/// cohesion → converse → COMPREHENSION): the received utterance DECOMPOSED over
/// the local genome toward understanding. Never re-cognition (that is the
/// perception stack's overlay), never recall.
#[derive(Clone, Debug)]
pub enum Comprehension {
    /// The utterance decomposed into held words — address and standing
    /// from the working atlas. Never a sentinel index.
    Understood(Vec<(Vec<u8>, Standing)>),
    /// Partially decomposed: `demand` is the EXACT element still missing —
    /// A12's canonical HNF residue against the span of what this mind holds
    /// on the plane, the whole word off it, or the residual chain a listing
    /// left. `understood` is the part that did factor, kept rather than
    /// thrown away with the rest.
    ///
    /// A count of residual links is not what M6 step 5 asks for: "No mind
    /// emits `unknown`; it emits the element it lacks." A count cannot be
    /// docketed by a hypha, cannot be reduced by another mind, and cannot
    /// be replayed — and the remainder A12 computes exactly would be
    /// discarded to produce it.
    Novel { demand: Box<Element>, understood: Vec<(Vec<u8>, Standing)> },
    /// A CONNECTION heard by holding it: the transition to the held word on
    /// its fibre vanished — the same word, glued.
    Held,
    /// A connection heard whose transitions to the words on its fibre did not
    /// vanish: THE OBSTRUCTIONS, keyed by the held word each was measured
    /// against.
    ///
    /// This was one `Box<Element>`. `comprehend` measured a transition against
    /// every word on the fibre and kept the one whose held word had the
    /// LOWEST ADDRESS, discarding the rest — so a mind that could say exactly
    /// how it differed from four held words said how it differed from one of
    /// them, chosen by a byte comparison on an address that carries no
    /// ordering (1.5: an address is an identity, not a rank). Chapter 13 says
    /// the obstructions on a fibre are a COCHAIN: they are all of them, and
    /// which words they stand against is half of what they say.
    Dialect(crate::network::nerve::Keyed<Element>),
    /// A connection on a fibre this genome holds no representative on:
    /// admitted as new geometry.
    Admitted,
    /// A sentence cites forms this mind does not hold.
    Withheld { missing: Vec<Vec<u8>> },
}

impl PartialEq for Comprehension {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Comprehension::Understood(a), Comprehension::Understood(b)) => a == b,
            // Two demands are one demand when they name the same vertex
            // (M1, rule 3), never when two counts happen to match.
            (
                Comprehension::Novel { demand: a, understood: ua },
                Comprehension::Novel { demand: b, understood: ub },
            ) => a.same_vertex(b) && ua == ub,
            (Comprehension::Held, Comprehension::Held) | (Comprehension::Admitted, Comprehension::Admitted) => true,
            // Two cochains are one cochain when they stand against the same
            // words and name the same vertex at each — never when two counts
            // match (rule 2).
            (Comprehension::Dialect(a), Comprehension::Dialect(b)) => {
                a.len() == b.len()
                    && a.scoped().all(|(k, ga)| match b.get(&k) {
                        Some(gb) => ga.same_vertex(gb),
                        None => false,
                    })
            }
            (Comprehension::Withheld { missing: a }, Comprehension::Withheld { missing: b }) => a == b,
            _ => false,
        }
    }
}
impl Eq for Comprehension {}
