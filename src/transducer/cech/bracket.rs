// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.26(2), CB1; MU-CLIENT §4.9 — THE CROSS-CHAMBER BRACKET. A sentence
//! spanning more than one chamber carries generators in each, and where two
//! chambers' generators fail to commute their bracket lives in neither
//! chamber's algebra by construction of neither. Comprehension across
//! chambers reads it: for each pair of chambers meeting at a body (the
//! boundary between them), each pair of their read circuits' actions is
//! composed both ways, and a commutator that does not vanish is the
//! bracket, held as content. It is HELD where an algebra the sentence holds
//! contains it: one of the two chambers, or an enclosure, a chamber of the
//! sentence whose bodies wrap both (EN1, NB1); it is LOST where none does.
//! Chambers sharing no body commute by the locality theorem (MU-CLIENT §4.5)
//! and are not read; a chamber and its own face are one enclosing the other,
//! not two chambers. This is the reading before any fix: no enclosure is
//! minted here, and nothing is decided (78.26(2)).

use super::orbit_algebra::{algebra_contains, ChamberAlgebra};
use crate::network::nerve::Keyed;
use crate::transducer::cga::LinearFunction;

/// WHAT HOLDS ONE BRACKET: the chamber of the pair whose own algebra
/// contains it, or an enclosure's, each by its place among the sentence's
/// chambers; or nothing the sentence holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bracketed {
    Chamber(usize),
    Enclosure(usize),
    Lost,
}

/// ONE CROSS-CHAMBER BRACKET: the two chambers and, in each, the read
/// circuit whose action it brackets; the commutator `A∘B − B∘A`, exact; and
/// what holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossBracket {
    pub chambers: (usize, usize),
    pub circuits: (usize, usize),
    pub bracket: LinearFunction,
    pub held: Bracketed,
}

/// WHY A PAIR OF CHAMBERS WAS NOT READ, by name: a chamber whose read
/// circuits' actions did not read, two chambers acting in algebras of
/// different width, or the stroke spent at the pair's bracket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossUnread {
    Actions { chamber: usize },
    Width { chambers: (usize, usize) },
    Spent { chambers: (usize, usize) },
}

/// THE SENTENCE'S CROSS-CHAMBER BRACKETS, and each pair it could not read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CrossBrackets {
    pub brackets: Vec<CrossBracket>,
    pub unread: Vec<CrossUnread>,
}

impl CrossBrackets {
    /// The brackets held, and the brackets lost: two counts of content, read
    /// off it and never stored beside it.
    pub fn held_lost(&self) -> (usize, usize) {
        let lost = self.brackets.iter().filter(|b| b.held == Bracketed::Lost).count();
        (self.brackets.len() - lost, lost)
    }
}

/// Whether every body of `inner` is a body of `outer`.
fn wraps(outer: &[Vec<u8>], inner: &[Vec<u8>]) -> bool {
    inner.iter().all(|b| outer.contains(b))
}

/// 78.26(2), CB1 — READ THE CROSS-CHAMBER BRACKETS of a sentence's chambers,
/// each given by its bodies' coordinates and its orbit algebra, in the
/// chambers' own order. The pairs are read off a keyed index of the bodies,
/// two chambers per body they share, never a product of every chamber with
/// every other; the enclosures of a pair are the chambers in that index
/// whose bodies wrap both. One read of the budget per pair of actions.
pub fn cross_brackets(chambers: &[(&[Vec<u8>], &ChamberAlgebra)], budget: &mut crate::physics::Budget) -> CrossBrackets {
    let mut by_body: Keyed<Vec<usize>> = Keyed::new();
    for (k, (bodies, _)) in chambers.iter().enumerate() {
        for b in *bodies {
            match by_body.get_mut(b) {
                Some(at) => at.push(k),
                None => {
                    by_body.insert(b, vec![k]);
                }
            }
        }
    }
    // Each meeting pair once, by its two places.
    let mut pairs: Keyed<(usize, usize)> = Keyed::new();
    for (_, at) in by_body.scoped() {
        for (i, &a) in at.iter().enumerate() {
            for &b in &at[i + 1..] {
                let (ba, bb) = (chambers[a].0, chambers[b].0);
                if wraps(ba, bb) || wraps(bb, ba) {
                    continue;
                }
                let mut key = Vec::new();
                crate::transducer::address::put_key_count(&mut key, a);
                crate::transducer::address::put_key_count(&mut key, b);
                pairs.insert(key, (a, b));
            }
        }
    }
    let mut out = CrossBrackets::default();
    for (_, &(a, b)) in pairs.scoped() {
        let (Some(xs), Some(ys)) = (chambers[a].1.actions.as_deref(), chambers[b].1.actions.as_deref()) else {
            let chamber = if chambers[a].1.actions.is_none() { a } else { b };
            out.unread.push(CrossUnread::Actions { chamber });
            continue;
        };
        if xs.iter().chain(ys).map(LinearFunction::n).collect::<Vec<_>>().windows(2).any(|w| w[0] != w[1]) {
            out.unread.push(CrossUnread::Width { chambers: (a, b) });
            continue;
        }
        // The enclosures of the pair: chambers on a body of `a` whose
        // bodies wrap both.
        let (ba, bb) = (chambers[a].0, chambers[b].0);
        let on_a = match ba {
            [body, ..] => by_body.get(body),
            [] => None,
        };
        let enclosures: Vec<usize> = match on_a {
            Some(at) => at.iter().copied().filter(|&k| k != a && k != b && wraps(chambers[k].0, ba) && wraps(chambers[k].0, bb)).collect(),
            None => Vec::new(),
        };
        'pair: for (i, x) in xs.iter().enumerate() {
            for (j, y) in ys.iter().enumerate() {
                if !budget.spend() {
                    out.unread.push(CrossUnread::Spent { chambers: (a, b) });
                    break 'pair;
                }
                let bracket = x.after(y).minus(&y.after(x));
                if bracket.is_zero() {
                    continue;
                }
                let holds = |k: usize| algebra_contains(chambers[k].1, &bracket) == Some(true);
                let held = match [a, b].into_iter().find(|&k| holds(k)) {
                    Some(k) => Bracketed::Chamber(k),
                    None => match enclosures.iter().copied().find(|&k| holds(k)) {
                        Some(k) => Bracketed::Enclosure(k),
                        None => Bracketed::Lost,
                    },
                };
                out.brackets.push(CrossBracket { chambers: (a, b), circuits: (i, j), bracket, held });
            }
        }
    }
    out
}
