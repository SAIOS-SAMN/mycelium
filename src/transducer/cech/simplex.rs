// 13 — THE SIMPLEX: a simplex of the Čech complex, the transition on it,
// and the fill that closes it.
//
//! A transition is the element that carries one chart's section onto
//! another's; a holonomy is the composite around a triangle; a `Fill` is
//! what closes a simplex — a gauge, a class, or nothing.

use super::*;

/// The TRANSITION carrying chart X's view to chart Y's, from each chart's germ for
/// the shared object: `g_XY = g_Y ∘ g_X⁻¹` (the change of view). Antisymmetric by
/// construction — `g_YX = g_XY⁻¹`. `None` if X's germ is not invertible (a
/// coboundary/residual chart has no group view-change; that overlap is handled by
/// the additive coboundary, not this group transition), and `None` where the
/// composite's counts would pass their carrier (STD1, `Element::compose`).
pub fn transition(g_x: &Element, g_y: &Element) -> Option<Element> {
    g_y.compose(&g_x.reverse()?)
}

/// The HOLONOMY around a triangle of charts — the composite transition
/// `g_ab ∘ g_bc ∘ g_ca`. Identity ⇔ the charts GLUE (one sheaf); otherwise it is
/// the exact obstruction (the equivocation) the charts must reconcile. `None`
/// where the composite's counts would pass their carrier (STD1).
pub fn holonomy(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> Option<Element> {
    g_ab.compose(g_bc)?.compose(g_ca)
}

/// Does a triangle of charts GLUE — the Čech cocycle `δg = 0`? True iff the
/// holonomy around the overlap is trivial (the transitions are the coboundary of a
/// consistent global assignment). False = CURVATURE (δσ ≠ 0), a dialect. `None`
/// where the holonomy could not be formed: neither a gluing nor a curvature.
pub fn glues(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> Option<bool> {
    holonomy(g_ab, g_bc, g_ca).map(|h| h.is_identity())
}

/// Realise a group transition as an alphabet rotor. None is a remainder:
/// the edge is honest and no simplex is formed over it.
pub fn rotor_of(g: &Element) -> Option<Rotor> {
    // THE ROTOR THE ELEMENT CARRIES, wherever it carries it. A transition
    // is `g_y ∘ g_x⁻¹` — a composite, which has no tape — so the reading
    // is of the element and never of a tape.
    g.carried_rotor()
}

/// THE SECTION AN ELEMENT CARRIES: its rotation AND its placement, read
/// from one [`Element::letters`] so the two halves are the same element's.
///
/// [`rotor_of`] answers the rotation alone, and a [`Section`] built from
/// that alone has the identity carriage whatever the element's placement
/// is: a Γ that is a translation, a dilation, or a rotation about any
/// point but the origin then reads as its rotor, and every STANDING class
/// at the court is invisible. The reader exists so the caller has the
/// choice.
pub fn section_of(g: &Element) -> Option<(Rotor, Versor)> {
    let letters = g.letters()?;
    Some((letters.spelling.word.rotor(), letters.carriage))
}

pub(crate) fn nonzero(v: &[u64; 4]) -> bool {
    v.iter().any(|&x| x > 0)
}

/// The key of a simplex: its sorted vertices, written out (A21).
pub(crate) fn simplex_key(verts: &[Vec<u8>]) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_SIMPLEX];
    crate::transducer::address::put_key_count(&mut buf, verts.len());
    for v in verts {
        crate::transducer::address::put_key_bytes(&mut buf, v);
    }
    crate::transducer::address::Address::new(buf)
}

pub(crate) fn sorted_verts(verts: impl IntoIterator<Item = impl AsRef<[u8]>>) -> Vec<Vec<u8>> {
    let mut verts: Vec<Vec<u8>> = verts.into_iter().map(|v| v.as_ref().to_vec()).collect();
    verts.sort_unstable();
    verts.dedup();
    verts
}

/// State of a simplex: open work, a coboundary gauge, a finite-order
/// congruence, a class at this degree, or a rotation the alphabet does not
/// spell.
#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Open,
    /// The gauge that closed the simplex: its rotor and its carriage,
    /// because a transition is `D(ρ)·T(t)·R` and a gauge that is a pure
    /// placement has the identity rotor and is not the identity. Both
    /// boxed, as every other variant's carriers are: a `Rotor` is sixteen
    /// rationals and an unboxed one here sized the whole enum.
    Filled(Box<Rotor>, Box<Versor>),
    /// An orbit that CLOSES, held with its class vector and the period it
    /// closes at. Three objects, because 11.3 holds an `H¹` orbit "with its
    /// class vector (12.6)" and 3.9 reads the candidate pivots off the
    /// axes that vector occupies — so `(rotor, ℓ)` alone was an orbit that
    /// filed and then had nothing to say. The period is the TORSION PART
    /// of 11.3, filed beside the coordinates and never instead of them,
    /// and it is itself an axis with a move of its own (the fibre row of
    /// the 3.9 table): filing at ℓ opens a candidate, it does not close
    /// the orbit.
    Congruent(Box<crate::transducer::sentence::ClassVector>, Box<Rotor>, u32),
    /// A standing class: the class vector of 12.6 AND the rotor it was read
    /// from. Both, because they are different objects — the vector holds
    /// the rotor's CONJUGACY INVARIANTS together with the placement and
    /// dilation residues, and the rotor is the element, which those
    /// invariants do not determine. The horizon keys pivots by a rotor's
    /// class and cannot be served by the vector alone; law 12 is untouched,
    /// since neither is derivable from the other.
    Class(Box<crate::transducer::sentence::ClassVector>, Box<Rotor>),
    /// 37.7 AT A SIMPLEX: the transition acts on the sensor plane and the
    /// peel stands on a prime outside the seventeen. Not a class — a class
    /// is one the alphabet names and no held generator closes; this is one
    /// the alphabet cannot name, and it is where an adjoined generator
    /// comes from at the court.
    ///
    /// A rotor that acts on ANOTHER plane is not this: it is spelled by a
    /// letter of the seventeen and carries a class with no sensor section.
    Unspellable(Box<Rotor>),
    /// 78.26(5) — EVALUATED AND NOT MEASURED, by why: the transition's rotor
    /// is no versor (no conjugacy invariants, so no class), or a rotation it
    /// was composed from is none — a section's, or the simplex's own
    /// coefficient. Never `Open`, which is the state of a simplex not yet
    /// evaluated (rule 11).
    Unread(FillUnread),
}

/// WHY A TRANSITION WAS NOT MEASURED (78.26(5)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillUnread {
    /// The transition's rotor has no conjugacy invariants: no versor.
    RotorNotVersor,
    /// A section's rotation is no versor, so it has no reverse.
    SectionNotVersor,
    /// The simplex's own coefficient is no versor, so it has no reverse.
    CoefficientNotVersor,
}

impl FillUnread {
    pub const ALL: [FillUnread; 3] =
        [FillUnread::RotorNotVersor, FillUnread::SectionNotVersor, FillUnread::CoefficientNotVersor];
}

impl Fill {
    /// NOT SETTLED: not yet asked, or asked and not measured. Either way the
    /// simplex recorded no reading — it pins no coface, is owed as unfilled,
    /// and is asked again.
    pub fn unsettled(&self) -> bool {
        matches!(self, Fill::Open | Fill::Unread(_))
    }
}

/// THE OUTCOME OF ATTEMPTING TO FILL A SIMPLEX — a transition result, and
/// never a stored state (54.7, ruled 2026-09-21).
///
/// `Fill` carried an `Already` variant whose own documentation said "not
/// stored on the simplex; the return of `fill` when there is no new
/// state" — an operational answer living in a persistent enum. It could
/// not be written to the tape, so the codec aliased it onto `Open`'s tag,
/// and every consumer carried a defensive `Open | Already` arm for a case
/// that could never arrive from storage.
///
/// `Fill::Open` was doing three jobs besides: the stored state of an
/// unfilled simplex, the answer when NO SUCH SIMPLEX exists, and the
/// answer when a section the fill needed was missing. A cell that is not
/// there and a cell that is there and unfilled are different facts, and
/// returning one value for both is a refusal reported as a measurement
/// (rule 11).
#[derive(Clone, Debug)]
pub enum CoboundaryOutcome {
    /// The attempt completed and the simplex now holds this state.
    /// `Attached(Fill::Open)` is a genuine reading: the boundary was
    /// evaluated and did not close, so the cell stands open.
    Attached(Fill),
    /// The simplex was not `Open`, so nothing was attempted and nothing
    /// changed.
    AlreadyFilled,
    /// No such simplex, or a reading the fill required was not there.
    /// This is the case that used to answer `Open`.
    Absent,
}

/// WHAT A `drain` DID — the conjugate of [`CoboundaryOutcome`], and a
/// transition result rather than a stored state for the same reason (62.1).
///
/// Four outcomes and no two of them are the same fact. `Vacuum` and
/// `Cauterized` are both successful releases and differ in whether anything
/// was carried; `Pinned` is a refusal the caller can act on by descending
/// first; `Absent` is no such 1-cell, which is not a refusal at all.
#[derive(Clone, Debug)]
pub enum Drained {
    /// The tether carried identity transport. It returned to the vacuum
    /// with no topological consequence and nothing is filed.
    Vacuum,
    /// The tether carried `g_ab ≠ 1` and was cauterized at dimension 1.
    /// The reading travels out; strain is never dropped (rule 11).
    Cauterized(Fill),
    /// The edge lies in the boundary of a simplex that has been READ, so
    /// draining it would retroactively invalidate a recorded class. The
    /// coface must `descend` first.
    Pinned,
    /// No 1-cell at this key.
    Absent,
}

/// 78.74 — WHAT RELEASING ONE VERTEX DID, cell by cell: the chambers crossed
/// (their classes kept as holes, `kept`) or unsealed, the holes on it
/// cauterized (`scarred`: a vertex whose word left reads no class back),
/// the edges drained to vacuum or cauterized, and what stood pinned, so the
/// vertex stood.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Release {
    pub kept: u64,
    pub scarred: u64,
    pub unsealed: u64,
    pub vacuum: u64,
    pub cauterized: u64,
    pub pinned: u64,
}

/// WHAT A `descend` DID — THE CROSS (ruled 2026-10-20: no
/// descend without its dipole ascend, its antipoles condense and drain, and
/// the puncture).
///
/// `Unsealed` and `Throat` are both crossings and differ in whether the
/// chamber had ever sealed a circulation. One standing unsettled recorded
/// nothing, so the membrane goes and nothing is kept; one carrying a fill
/// was the witness that a boundary holonomy CLOSED, and its class is never
/// handed out alone: the throat opens.
#[derive(Clone, Debug)]
pub enum Descended {
    /// The chamber stood unsettled — it sealed no circulation, so the
    /// membrane goes and no class is kept or handed down.
    Unsealed,
    /// THE THROAT: the chamber is punctured and its class KEPT as a hole,
    /// its content — the strain its seal wound in — handed down to the
    /// field (unwound on its line), its boundary locked under the hole. Only
    /// `seal`, ascend's read-back, returns the class to a chamber.
    Throat(Fill),
    /// A 3-cell above this one has been read, so descending would
    /// retroactively invalidate it. That coface must descend first.
    Pinned,
    /// No 2-cell at this key.
    Absent,
    /// 78.99 (2) — a hole stands here; its conjugate is `seal`.
    Hole,
}

/// WHAT AN `ascend` DID — [`Descended`]'s inverse on content (AD1, 62.5 as
/// amended).
#[derive(Clone, Debug)]
pub enum Ascended {
    /// The boundary closes and the cell is sealed with this fill.
    Sealed(Fill),
    /// The boundary was read and does not close: the cell stands open.
    Open,
    /// The cell was already sealed; nothing changed.
    AlreadySealed,
    /// No boundary stands to seal over.
    Absent,
    /// 78.99 (2) — a hole stands here; only `seal` caps it.
    Hole,
}

impl Descended {
    /// The class the throat kept as a hole, where there was one.
    pub fn kept(&self) -> Option<&Fill> {
        match self {
            Descended::Throat(f) => Some(f),
            _ => None,
        }
    }

    /// Did the chamber leave the cells? Both crossings did; no refusal.
    pub fn opened(&self) -> bool {
        matches!(self, Descended::Unsealed | Descended::Throat(_))
    }
}

impl Drained {
    /// The class a strained dissolution filed, where there was one.
    pub fn cauterized(&self) -> Option<&Fill> {
        match self {
            Drained::Cauterized(f) => Some(f),
            _ => None,
        }
    }

    /// Did the cell leave the complex? Every release did; neither refusal.
    pub fn released(&self) -> bool {
        matches!(self, Drained::Vacuum | Drained::Cauterized(_))
    }
}

impl CoboundaryOutcome {
    /// The state the simplex holds after the attempt, where the attempt
    /// reached one. `None` for both non-attempts, which are different
    /// facts and are kept apart at the call sites that care.
    pub fn attached(&self) -> Option<&Fill> {
        match self {
            CoboundaryOutcome::Attached(f) => Some(f),
            _ => None,
        }
    }
}

/// CLASSIFY A TRANSITION (12.6). One reader for both organs: the court
/// reaches it through [`extend`], the voice through its cover's meets.
///
/// A transition is `D(ρ)·T(t)·R`, and it arrives here already separated —
/// the rotor in `Cl(4,0)` over all six planes, the carriage `D·T` in
/// `Cl(3,1)`. Nothing is projected and nothing is re-factored.
///
/// `Congruent` requires a FIXED POINT, not a flat carriage. A 90° turn
/// carried along a translation has the same rotor order as one about a
/// fixed point and is not the same class: filing it at period four would
/// put a theorem on the simplex that the seam does not satisfy, because
/// the seam does not return after four turns — its orbit never closes. But a rotation
/// about a point away from the origin is `T_p R T_p⁻¹ = T(p − Rp)·R`, and
/// that DOES return after four turns: it is conjugate to `R`, at `R`'s
/// order, and its translation is where it stands rather than a residue.
/// [`crate::transducer::element::turn_base_point`] reads `p` exactly or
/// refuses, so the two are told apart by solving for the fixed point and
/// never by the size of anything.
///
/// It read `flat` for both and filed every turn as a standing class. The
/// note here said such a turn stood "until the tree gauge of
/// [`solve_gauge`] conjugates the base-point translation away", and that
/// gauge never conjugated: it composes placements along a spanning tree
/// and returns them, so at degree ≥ 2 a solved simplex fills `Filled` and
/// an unsolved one classifies on its ungauged carriage. Neither road
/// reached `Congruent` while a body turned about its own centroid, which
/// is how every transport a voice measures arrives.
///
/// The 12.6 order of the standing class: the rotor's conjugacy invariants
/// first, then the placement and dilation residues, then the plane blocks.
pub fn classify(
    rotor: &Rotor,
    carriage: &Versor,
    hue: crate::transducer::sentence::CycleType,
) -> Fill {
    use crate::transducer::alphabet::Order;
    use crate::transducer::element::{identity_versor, same_versor, turn_base_point};
    let flat = same_versor(carriage, &identity_versor());
    if flat && rotor.is_identity() {
        return Fill::Filled(Box::new(rotor.clone()), Box::new(identity_versor()));
    }
    // The transport's FIXED POINT: the origin when the carriage is flat,
    // the turn's own base point when the carriage is exactly `T(p − Rp)`.
    // A screw and a dilation have neither and still stand.
    let fixed = flat || turn_base_point(rotor, carriage).is_some();
    // A17: an order of zero is not an order — a congruence is believed only
    // where the rotation CLOSES at ℓ — so it is not filed as one, and every
    // reader of a filed period may take `ℓ − 1` whole.
    if let (true, Some(Order::Finite(ell @ 1..))) = (fixed, rotor.order()) {
        // The class vector is read HERE, on the way to filing, so the
        // filed orbit carries the axes 3.9 instantiates its candidate moves
        // from. A rotor with no conjugacy invariants is not a versor
        // (15.3) and files nothing: it falls through to the readings
        // below rather than filing a period with no coordinates.
        if let Some(kappa) = crate::transducer::sentence::ClassVector::of_rotor(
            rotor,
            carriage.clone(),
            hue.clone(),
        ) {
            return Fill::Congruent(Box::new(kappa), Box::new(rotor.clone()), ell);
        }
    }
    // 37.7's condition, discriminated from an off-plane rotor rather than
    // conflated with it: the rotor DOES act on the sensor plane (it has a
    // block there) and the peel of that block stands, which is a prime
    // outside the seventeen. A rotor with no sensor block is not
    // unspellable — it is a letter acting elsewhere, and it carries a class
    // with no sensor section.
    let plane = (
        crate::transducer::alphabet::PLANE_ROW,
        crate::transducer::alphabet::PLANE_COL,
    );
    let acts_on_sensor = crate::transducer::alphabet::rotor_block_on(rotor, plane).is_some();
    if acts_on_sensor && crate::transducer::alphabet::word_of_rotor(rotor).is_none() {
        return Fill::Unspellable(Box::new(rotor.clone()));
    }
    match crate::transducer::sentence::ClassVector::of_rotor(rotor, carriage.clone(), hue) {
        Some(kappa) => Fill::Class(Box::new(kappa), Box::new(rotor.clone())),
        // A rotor with no conjugacy invariants is not a versor (15.3): no
        // class, and unmeasured by name rather than flat (rule 11).
        None => Fill::Unread(FillUnread::RotorNotVersor),
    }
}

/// Transition between two sections on a simplex, classified by
/// [`classify`].
///
/// Both halves of the transition are composed: the rotors in `Cl(4,0)` and
/// the carriages in `Cl(3,1)`. A Γ that is a translation or a dilation is
/// the carriage half, so a reading of the rotor alone takes it for the
/// identity, files finite rotor orders correctly and sees no standing
/// class at all.
///
/// THE HUE IS THE SECTION'S, AND A SECTION CARRIES NONE. `CycleType::of`
/// an empty listing is `Identity`, and a section holds a representative
/// word and its gauge with no hue map beside it, so the empty listing is
/// what this simplex actually has rather than a claim about it. A section
/// that should carry a gauge is a row, not a default written here.
pub fn extend(section_a: &Section, section_b: &Section, simplex: &Simplex) -> Fill {
    #[cfg(test)]
    crate::network::nerve::work::count_extend();
    use crate::transducer::element::{identity_versor, same_versor};
    let hue = crate::transducer::sentence::CycleType::Identity;
    // Each section's reverse: rays (78.32(1)).
    let Some(inv_r) = section_a.gamma.is_versor().then(|| section_a.gamma.reverse()) else {
        return Fill::Unread(FillUnread::SectionNotVersor);
    };
    let inv_c = section_a.carriage.reverse();
    let trans_r = section_b.gamma.geo(&inv_r);
    let trans_c = section_b.carriage.geo(&inv_c);
    if simplex.degree() <= 1 {
        let Some(cr) = simplex.coefficient.is_versor().then(|| simplex.coefficient.reverse()) else {
            return Fill::Unread(FillUnread::CoefficientNotVersor);
        };
        let cc = simplex.carriage.reverse();
        let (g_r, g_c) = (trans_r.geo(&cr), trans_c.geo(&cc));
        // The sections agree with what the simplex expected: the simplex
        // is extended BY THE TRANSITION, which is the gauge it was waiting
        // for.
        if g_r.is_identity() && same_versor(&g_c, &identity_versor()) {
            return Fill::Filled(Box::new(trans_r), Box::new(trans_c));
        }
        return classify(&g_r, &g_c, hue);
    }
    classify(&simplex.coefficient, &simplex.carriage, hue)
}

/// One simplex of the Čech complex. Keyed by its sorted vertices, written
/// out (A21).
#[derive(Clone, Debug)]
pub struct Simplex {
    pub vertices: Vec<Vec<u8>>,
    /// The rotation the simplex expects, over all six planes of `Cl(4,0)`.
    pub coefficient: Rotor,
    /// The placement and dilation the simplex expects, `D(ρ)·T(t)` in
    /// `Cl(3,1)`. It is a SECOND field and not a wider coefficient: the
    /// conformal algebra of a two-dimensional base holds one rotation
    /// plane, so a versor coefficient would carry the residues and lose
    /// five of the rotor's planes — `cech/tests/bianchi` builds its
    /// degree-3 class from a rotor on `e₁∧e₂` that has no versor at all.
    pub carriage: Versor,
    pub volume: [u64; 4],
    pub fill: Fill,
}

impl Simplex {
    /// A simplex on n vertices is an (n−1)-simplex. One on NONE is no
    /// simplex and is never remembered by the complex, so n ≥ 1 wherever
    /// this is asked and the subtraction is total.
    pub fn degree(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn key(&self) -> crate::transducer::address::Address {
        simplex_key(&self.vertices)
    }

    pub fn contains(&self, v: &[u8]) -> bool {
        self.vertices.iter().any(|x| x.as_slice() == v)
    }

    /// 78.75 — THE CELL'S CONTENT AS ONE MOTION OF `Cl(4,1)`: its carriage
    /// composed with its rotor carried into the conformal algebra
    /// (`Rotor::lift`, the exact embedding), `carriage · lift(rotor)` — the
    /// transition `D(ρ)·T(t)·R` as the one element it is. The two fields stay
    /// as stored (the persisted layout is the recut's); every reader of a
    /// cell's content reads this, by its modes.
    pub fn motion(&self) -> crate::transducer::cga::Mv {
        self.carriage.geo(&self.coefficient.lift())
    }
}

/// Degree-0 section: the representative word after in-hypha gluing, and its gauge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub addr: crate::transducer::address::Address,
    /// The chart's rotation, over all six planes.
    pub gamma: Rotor,
    /// The chart's placement and dilation (L11e). `extend` composes both
    /// halves, and a Γ that is a translation, a dilation, or a rotation
    /// about any point but the origin is this one: without it the
    /// transition reads as its rotor and every STANDING class at the court
    /// is invisible, while finite rotor orders still file, since
    /// conjugation by a translation preserves the rotor class.
    pub carriage: Versor,
}

impl Section {
    /// A section at a rotation with no placement and no dilation.
    pub fn at(addr: crate::transducer::address::Address, gamma: Rotor) -> Section {
        Section { addr, gamma, carriage: crate::transducer::element::identity_versor() }
    }
}
