// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! TRANSDUCTION IS FACTORISATION (MU-SENSING §3.9; ROADMAP track 26 S4b):
//! a pair's deformation split into what the held algebra carries and what
//! it does not,
//!
//! ```text
//! deformation  =  motion  ∘  residue
//! ```
//!
//! - **Motion:** a signed permutation of the input's phases and the
//!   re-period of each moved phase onto the output's enclosure: the turns
//!   carried exactly, every band boundary landing on a boundary of the
//!   output or the motion refused by name (`OffSite`). The versors of the
//!   alphabet act on turns as signed permutations at the seams
//!   (`SignedPermutation::of_word`), so this is the motion the transduction
//!   carries at any rank.
//! - **Residue:** the incidences the motion does not carry: every output
//!   site where the motion's speech and the output differ, as links of the
//!   pair's own rank and width (`sentence::Link`), each with the symbol the
//!   motion speaks there and the symbol observed. Applying the residue to
//!   the motion's speech gives the output exactly.
//!
//! Every motion that lands gives a factorisation, and every factorisation
//! that holds is held: several are a VORTEX (MU-SENSING §3.8), in the order
//! the group is generated (an order of content), never reduced to the
//! "smallest" residue by a scalar. A court reconstructs a claim's stated
//! motion through [`Factorisation::of`], the one function the mind reads
//! the residue through, and admits the claim where the residue it derives
//! is the residue claimed (`court::verdict::factorisation`).

use super::{need, Enclosed, Enclosure, SignedPermutation, Transduction, TransductionRefusal};
use crate::transducer::sample::Sample;
use crate::transducer::sentence::Link;

/// A MOTION THE TRANSDUCTION CARRIES: a signed permutation of the input's
/// phases, then the re-period of each moved phase onto the enclosure
/// `onto` (the output's periods).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Motion {
    /// The signed permutation of the phases.
    pub permutation: SignedPermutation,
    /// The enclosure the moved frame is re-periodded onto.
    pub onto: Enclosure,
}

impl Motion {
    /// THE RE-PERIOD OF EACH PHASE the motion carries from a frame of `from`:
    /// `p′/p` per moved phase in lowest terms; `None` where the ranks differ.
    pub fn reperiod(&self, from: &Enclosure) -> Option<Vec<(usize, usize)>> {
        self.permutation.enclosure(from).ok()?.reperiod(&self.onto)
    }

    /// THE MOTION'S SPEECH OF `x`: the frame moved by the permutation and
    /// spoken onto `onto`, exact or refused by name (`Rank`, `OffSite`).
    pub fn spoken(&self, x: &Sample) -> Result<Sample, TransductionRefusal> {
        self.permutation.spoken_onto(&Enclosed::of(x), &self.onto)
    }
}

/// ONE FACTORISATION OF A PAIR: the motion, the bodies it carries whole
/// (input body of the moved frame onto output body, or the refusal that
/// names why a body is not carried), and the residue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factorisation {
    pub motion: Motion,
    /// Per body of the moved frame: the output body it is carried onto, or
    /// `Mixed` where it is split or merged.
    pub carried: Vec<Result<usize, TransductionRefusal>>,
    /// The incidences the motion does not carry, in canonical address
    /// order: at each output site where the motion's speech differs from
    /// the output, the symbol spoken and the symbol observed.
    pub residue: Vec<Link>,
}

/// THE FACTORISATIONS OF A PAIR, typed: never a boolean, never a choice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Factored {
    /// Exactly one motion lands.
    One(Factorisation),
    /// Several hold, each held, in the group's generated order.
    Vortex(Vec<Factorisation>),
    /// No motion lands: each signed permutation refused, named.
    None(Vec<(SignedPermutation, TransductionRefusal)>),
}

impl Factored {
    /// Every factorisation held, in order: the one, the vortex's sections,
    /// or none.
    pub fn held(&self) -> &[Factorisation] {
        match self {
            Factored::One(f) => std::slice::from_ref(f),
            Factored::Vortex(fs) => fs,
            Factored::None(_) => &[],
        }
    }
}

impl Factorisation {
    /// THE FACTORISATION OF `(x, y)` UNDER `motion`: the motion's speech of
    /// `x`, then the record of that speech against `y` on one enclosure
    /// (`Transduction::of` under the caller's stroke, which [`need_of_motion`]
    /// sizes), its carriages and its projected change as the residue.
    /// Refused by name where the motion does not land (`Rank`, `OffSite`),
    /// where it is spoken onto another enclosure than `y`'s (`Rank` on rank,
    /// `OntoAnother` on the periods), or where the stroke ends (`Spent`).
    pub fn of(x: &Sample, y: &Sample, motion: &Motion, budget: &mut crate::physics::Budget) -> Result<Factorisation, TransductionRefusal> {
        let to = Enclosure::of(y);
        if motion.onto.rank() != to.rank() {
            return Err(TransductionRefusal::Rank { from: motion.onto.rank(), to: to.rank() });
        }
        if motion.onto != to {
            return Err(TransductionRefusal::OntoAnother);
        }
        let spoken = motion.spoken(x)?;
        let t = Transduction::of(&spoken, y, budget)?;
        let residue = t.projected()?.iter().map(Link::of_change).collect();
        Ok(Factorisation { motion: motion.clone(), carried: t.relations(), residue })
    }

    /// THE DEFORMATION RECOMPOSED: the motion's speech of `x` with the
    /// residue applied, site by site: each residue link's site takes its
    /// `after` symbol. Exactly `y` where the factorisation is `y`'s; refused
    /// by name where a link does not stand on the motion's speech
    /// (`OffSpeech`: a site outside the enclosure, or a symbol of another
    /// width).
    #[cfg(test)]
    pub fn recomposed(&self, x: &Sample) -> Result<Sample, TransductionRefusal> {
        let spoken = self.motion.spoken(x)?;
        let width = spoken.width();
        // The speech's sites in canonical address order, each with its
        // symbol; a residue link replaces the symbol at its site.
        let mut at: Vec<(crate::transducer::base::Site, Vec<u8>)> =
            spoken.sites().zip(spoken.symbols()).map(|(s, y)| (s, y.bytes().to_vec())).collect();
        for link in &self.residue {
            if link.after.len() != width {
                return Err(TransductionRefusal::OffSpeech);
            }
            let k = at.binary_search_by(|(s, _)| s.cmp(&link.site)).map_err(|_| TransductionRefusal::OffSpeech)?;
            if let Some((_, symbol)) = at.get_mut(k) {
                symbol.clone_from(&link.after);
            }
        }
        let symbols: Vec<u8> = at.into_iter().flat_map(|(_, s)| s).collect();
        Sample::new(spoken.extents().to_vec(), width, symbols, spoken.place().clone()).map_err(TransductionRefusal::Sample)
    }
}

/// THE STROKE A FACTORISATION UNDER `motion` NEEDS: the record of the
/// motion's speech against the output, both on the output's enclosure
/// ([`need`]). `None` where the enclosure's need passes `usize`.
pub fn need_of_motion(motion: &Motion) -> Option<usize> {
    need(&motion.onto, &motion.onto)
}

/// THE STROKE [`factorisations`] NEEDS FOR `pairs` PAIRS OF RANK `rank`:
/// one read per signed permutation of the phases per pair, `pairs · 2ⁿ ·
/// n!`, read off the group's order and never a guess. `None` past `usize`.
pub fn factorisations_need(rank: usize, pairs: usize) -> Option<usize> {
    let mut order = 1usize;
    for k in 1..=rank {
        order = order.checked_mul(k)?.checked_mul(2)?;
    }
    order.checked_mul(pairs)
}

/// EVERY FACTORISATION OF A PAIR (MU-SENSING §3.9): one per signed
/// permutation of the phases, in [`SignedPermutation::all`]'s generated
/// order, each spoken onto the output's enclosure. A motion that does not
/// land is kept with its refusal; the rest are all held, as one or as a
/// vortex. `Rank` where the two frames differ in rank: no motion of one
/// rank carries the other (MU-SENSING §5). Each factorisation is read
/// under a stroke its own need sizes, drawn from `budget` one read per
/// factorisation; `Spent` where `budget` ends first.
pub fn factorisations(x: &Sample, y: &Sample, budget: &mut crate::physics::Budget) -> Result<Factored, TransductionRefusal> {
    let (from, to) = (Enclosure::of(x), Enclosure::of(y));
    if from.rank() != to.rank() {
        return Err(TransductionRefusal::Rank { from: from.rank(), to: to.rank() });
    }
    let mut held = Vec::new();
    let mut refused = Vec::new();
    for permutation in SignedPermutation::all(from.rank()) {
        if !budget.spend() {
            return Err(TransductionRefusal::Spent { need: None });
        }
        let motion = Motion { permutation: permutation.clone(), onto: to.clone() };
        let Some(stroke_need) = need_of_motion(&motion) else { return Err(TransductionRefusal::Spent { need: None }) };
        let mut stroke = crate::physics::Physics::default().stroke_n(stroke_need);
        match Factorisation::of(x, y, &motion, &mut stroke) {
            Ok(f) => held.push(f),
            Err(why) => refused.push((permutation, why)),
        }
    }
    Ok(match held.len() {
        0 => Factored::None(refused),
        1 => match held.pop() {
            Some(f) => Factored::One(f),
            None => Factored::None(refused),
        },
        _ => Factored::Vortex(held),
    })
}
