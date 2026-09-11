// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.8, L11c — THE COVER OF A DEFORMATION, BUILT FROM THE
//! CORRESPONDENCE. The same constructor as the ring's, at the cover
//! level.
//!
//! No single versor carries this body onto its image — that is what a
//! `TopologicalDeformation` says. The cover splits both into PIECES that
//! are each carried, and what is left is the transition between pieces
//! where they meet.
//!
//! THE PIECES ARE PULLED BACK, NEVER COMPARED. Peeling both bodies alone
//! and matching their charts afterwards produces two decompositions with
//! no reason to share one: measured at 2c9fb0c, every failure was a chart
//! miss and the carriage was never consulted once. An output piece's
//! preimage is the set of input pieces standing under its own cells, read
//! per cell from a keyed occupancy — so a piece in place is congruent by
//! construction and the lookup cannot miss. That preimage is ONE set and
//! is carried once: the two peels are under no obligation to agree on
//! granularity, and requiring them to (one input piece against one output
//! piece) refuses every carriage where they differ. A purely positional pullback
//! would find ONLY pieces carried in place, and every transition would be
//! the identity; the second pass by shell kind and hue is what lets a
//! piece that MOVED be found, and the moduli are never the key there,
//! because a deformed piece's moduli differ and that difference is the
//! deformation itself.
//!
//! Nothing is added to the tape. `Form.torsion` already carries a filled
//! cell as `FiledLoop { sigma, period, chord }` and the court already
//! replays it (23.1 step 4).
//!
//! Nothing is keyed by position. A piece is addressed by its content —
//! shell kind, hue, and its support written out — so no arrival index
//! survives into the nerve (rule 6, rule 15).

use super::pieces::{carriage_key, hue_transition, meet_key, pieces_of, preimage_key};
use super::*;
use crate::transducer::alphabet::Rotor;

/// WHAT THE COVERS OF ONE WALK READ. An instrument and nothing else: no
/// cognition reads it, no branch consults it, and it is taken and cleared
/// by the walker that prints it (§8.1 instrumentation).
///
/// It exists because the cover's readings do not survive into the form.
/// A filed class becomes torsion, a refused carriage becomes a demand
/// vertex the soak already counts — but a cover that COMPLETED and found
/// nothing, and the reads a cover spent, leave no trace in content, and
/// without them `cells filled 0` cannot be told from `no cover ran`.
#[derive(Default, Clone, Copy)]
pub struct CoverTally {
    /// Covers that ran to the end of their pieces, and covers whose share
    /// ended first.
    pub measured: u64,
    pub unmeasured: u64,
    /// Reads spent inside covers, apart from the peel's own.
    pub spend: u64,
    /// Carriages placed, and meets by their reading.
    pub carried: u64,
    pub flat: u64,
    pub filed: u64,
    pub standing: u64,
    /// Meets whose transition the seventeen do not spell (37.7).
    pub unspellable: u64,
    /// Pieces carried geometrically whose rotor the seventeen do not
    /// spell — the adjunction candidates the cover used to discard.
    pub refused: u64,
}

thread_local! {
    static TALLY: std::cell::Cell<CoverTally> = std::cell::Cell::new(CoverTally::default());
}

/// Read the walk's covers and clear the count.
pub fn take_cover_tally() -> CoverTally {
    TALLY.with(|t| t.replace(CoverTally::default()))
}

fn tally(f: impl Fn(&mut CoverTally)) {
    TALLY.with(|t| {
        let mut v = t.get();
        f(&mut v);
        t.set(v);
    });
}

/// One carriage the cover placed: the versor, the support it was carried
/// FROM, and the support it landed on.
///
/// Both supports are kept because the nerve reads meets on both. A
/// deformation is a map OUT OF the input, so the Čech nerve is the nerve
/// of the cover of the SOURCE: two carriages sharing a domain overlap on
/// the whole of it, however far apart their codomains land. That is the
/// replication case, and it is the only case in which the transition
/// between two copies of one shape is visible at all. Meets on the
/// codomain are the merge case, where several domains reach one image
/// region. Reading only the codomain — measured with `carried 2` on a
/// family of one input piece and ten output pieces — leaves the nerve
/// discrete: the two copies land apart, no ring touches, no 1-simplex is
/// formed, and no cycle exists for a holonomy to be read around.
struct Placed {
    /// The carriage's ROTOR and its `D(ρ)·T(t)`, kept apart (L11e). They
    /// are not factored here: `word_or_demand` already separated them when
    /// it built the vertex — the derivation's rotor and the vertex's
    /// carriage — so the meet composes each half against its neighbour's
    /// and hands both to `cech::classify`, exactly as `extend` does.
    ///
    /// A versor was kept instead, and the meet re-derived `D·T·R` from it
    /// through a second classifier that could not hold a dual-plane rotor.
    rotor: Rotor,
    carriage: Versor,
    /// The hue map this carriage read cell by cell, which the transition
    /// at a meet composes against its neighbour's.
    sigma: Vec<(u8, u8)>,
    domain: Vec<(i64, i64)>,
    codomain: Vec<(i64, i64)>,
}

/// The rotor and the carriage a word vertex already carries. `None` for a
/// vertex that is not a word, which the cover never places.
fn halves_of(v: &Vertex) -> Option<(Rotor, Versor)> {
    match v {
        Vertex::Word { derivation, carriage, .. } => {
            Some((derivation.rotor(), carriage.as_ref().clone()))
        }
        _ => None,
    }
}

/// What the cover of one deformation reached. Every field is content, and
/// every carrier is keyed by a piece's own address.
pub(crate) struct Cover {
    /// The word vertex of each carried piece and the versor that carried
    /// it, by piece address.
    pub(crate) carried: crate::network::nerve::Keyed<(Vertex, Versor)>,
    /// A meet whose transition has FINITE order: a torsion cycle of this
    /// form (15.5, A17) with the period it closes at, by meet address.
    pub(crate) filed: crate::network::nerve::Keyed<(Rotor, u32)>,
    /// A meet whose transition is NOT a finite-order rotation about a
    /// point: an infinite-order rotor, a placement residue, a dilation
    /// residue, or a product of them. Nothing is filed and the demand
    /// stands on it (50.5) CARRYING THIS CLASS VECTOR, by meet address.
    ///
    /// The class travels with the demand rather than being recomputed by
    /// whoever meets it, which is what lets an arriving generator of the
    /// conjugate class close it with no coordinator directing the match.
    /// A bare `Rotor` here could not carry a placement or a dilation, so
    /// every standing transition that was one read as the identity.
    pub(crate) standing:
        crate::network::nerve::Keyed<crate::transducer::sentence::ClassVector>,
    /// A meet whose transition is a rotation THE SEVENTEEN DO NOT SPELL
    /// (37.7), by meet address. Not a standing class — a standing class is
    /// one the alphabet names and no held generator closes; this is one
    /// the alphabet cannot name. It is where an adjoined generator would
    /// come from at the cover level.
    pub(crate) unspellable: crate::network::nerve::Keyed<Rotor>,
    /// A PIECE THE COVER CARRIED GEOMETRICALLY AND COULD NOT SPELL, by
    /// cover-piece address: `carriage` found the versor and `word_or_demand`
    /// refused its rotor. Kept as the demand the peel typed, so the piece
    /// enters the form as a vertex instead of vanishing.
    ///
    /// It was a `continue`. That discarded, silently, the only place in the
    /// engine where a carriage is CONFIRMED and its rotor is outside the
    /// alphabet — which is 37.7's condition exactly, and so the shortest
    /// route the cover has to an adjoined generator. The cover cannot run
    /// the ring itself: it is inside the memoised percept and 50.2 step 2
    /// forbids the crystal there, so the candidate is carried out to
    /// `derive_against`, which runs per mind.
    pub(crate) refused: crate::network::nerve::Keyed<Demanded>,
    /// What no piece carried, as a VOLUME on four lanes (law 5): the
    /// input's cells no piece carried and the image's cells no piece
    /// reached (50.8 case 4). A count of cells is not this quantity —
    /// cells carry different lane weight — so the dissolution reads the
    /// zero vector and never a cardinality.
    pub(crate) uncarried: [u64; 4],
}

/// THE COVER DID NOT RUN TO THE END OF ITS OWN PIECES, because its share
/// of the stroke ended first. It is not a smaller cover (rule 11).
///
/// This is a TYPE and not a flag on the cover, because the alternative
/// leaves a half-built `Cover` in existence: `carried` partly populated,
/// and `uncarried` sitting at `[0; 4]` — a zero that was never measured
/// and that reads exactly like a cover which reached everything. A
/// boolean guarding that zero protects only the readers that remember to
/// consult it. There is now no such value to read.
#[derive(Debug)]
pub(crate) struct Unmeasured;

impl Cover {
    fn new() -> Cover {
        Cover {
            carried: crate::network::nerve::Keyed::new(),
            filed: crate::network::nerve::Keyed::new(),
            standing: crate::network::nerve::Keyed::new(),
            unspellable: crate::network::nerve::Keyed::new(),
            refused: crate::network::nerve::Keyed::new(),
            uncarried: [0; 4],
        }
    }

    /// The deformation is answered when every cell of both bodies is
    /// carried and no meet of infinite order stands. A finite-order
    /// holonomy is NOT an obstruction: it is filed as torsion and the
    /// sentence carries the class as its theorem (15.5), so a demand kept
    /// alive on it would be a demand for what the form already says.
    ///
    /// A `Cover` exists only where the cover completed, so this reads the
    /// measurement and never a flag saying whether there was one.
    pub(crate) fn dissolves(&self) -> bool {
        self.uncarried == [0; 4]
            && self.standing.fiber(crate::network::nerve::ROOT).next().is_none()
            && self.unspellable.fiber(crate::network::nerve::ROOT).next().is_none()
            && self.refused.fiber(crate::network::nerve::ROOT).next().is_none()
    }
}

/// 50.8 — COVER ONE DEFORMATION.
///
/// Pass (a), THE PULLBACK: an output piece's preimage is the input pieces
/// standing under its own cells, read per cell from the occupancy. Pass
/// (b), BY STRUCTURE: an input piece the pullback placed nowhere is tried
/// against EVERY output piece of its own shell kind and hue — never its
/// moduli, which a deformed piece changes by definition. In both,
/// `carriage` decides and nothing is ranked (rule 4).
///
/// Neither pass is injective in either direction, because Φ is not. A
/// cover piece is a carriage — a domain, a codomain, a versor — keyed by
/// `carriage_key`, so one domain carried to several codomains is several
/// cover pieces and several nodes of the nerve. Both passes are bounded
/// by the budget spent per attempt and by nothing else (rule 10).
///
/// The readings at a meet, and none of them is a choice:
///
/// - identity: the pieces agree across the seam and there is nothing to
///   fill.
/// - no versor: a remainder edge (13.9 step 7, 50.3), and no loop through
///   it is evaluated.
/// - finite order `ℓ`: the class is FILED as torsion at its period.
/// - infinite order: the demand stands, carrying that class.
pub(crate) fn cover_deformation(
    input: &Body,
    image: &Body,
    scale: Option<((usize, usize), (usize, usize))>,
    budget: &mut crate::physics::Budget,
) -> Result<Cover, Unmeasured> {
    use crate::network::nerve::ROOT;
    let opened = budget.remaining();
    let mut flat = 0u64;
    let ins = pieces_of(&input.cells, scale);
    let outs = pieces_of(&image.cells, scale);
    let mut cover = Cover::new();

    // Cell position → the input piece holding it. This is Φ: the
    // positional correspondence over the two supports, which is what a
    // deformation-in-place has in place of a whole-body versor.
    let mut occ_in: crate::network::nerve::Keyed<Vec<u8>> = crate::network::nerve::Keyed::new();
    for (k, p) in ins.fiber(ROOT) {
        for &(r, c, _) in &p.body.cells {
            occ_in.insert(cell_pos_key(r, c), k.clone());
        }
    }

    let mut placed: crate::network::nerve::Keyed<Placed> = crate::network::nerve::Keyed::new();
    let mut consumed: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut reached: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let out_keys: Vec<Vec<u8>> = outs.fiber(ROOT).map(|(k, _)| k).collect();

    // (a) THE PULLBACK, in place.
    for ko in &out_keys {
        let Some(op) = outs.get(ko) else { continue };
        // The preimage: the input pieces named under this piece's own
        // cells, deduped by their address and read in key order.
        let mut named: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for &(r, c, _) in &op.body.cells {
            if let Some(ki) = occ_in.get(&cell_pos_key(r, c)) {
                named.insert(ki, ());
            }
        }
        // Φ⁻¹(V) IS ONE SET. It is assembled whole and carried once, and
        // it is closed up to whole pieces so no piece is cut across the
        // preimage boundary. Measured before this line stood: tried piece
        // by piece instead, 24 input pieces against 1 output piece carried
        // NOTHING — a 3-cell piece has no versor onto a 60-cell image, so
        // every carriage refused on granularity and the cover was empty.
        // The granularity of the two peels is not required to agree; that
        // it need not is the whole reason the preimage is taken.
        let mut cells: Vec<Cell3> = Vec::new();
        let mut parts: Vec<Vec<u8>> = Vec::new();
        for (ki, ()) in named.fiber(ROOT) {
            if !budget.spend() {
                tally(|t| t.unmeasured += 1);
                return Err(Unmeasured);
            }
            let Some(ip) = ins.get(&ki) else { continue };
            cells.extend_from_slice(&ip.body.cells);
            parts.push(ki);
        }
        // No preimage is no piece to carry, not a piece that failed to
        // carry (rule 11): nothing is reached and nothing is consumed.
        let Some(pre) = (!cells.is_empty()).then(|| body_of(cells)) else {
            continue;
        };
        let Some(car) = carriage(&pre, &op.body) else { continue };
        let key = carriage_key(&preimage_key(&parts), ko);
        // The carriage is CONFIRMED here — a versor exists and carries the
        // preimage onto this piece. If its rotor does not spell, that is
        // 37.7 and not a failure to carry, so the piece is kept as the
        // demand the peel typed rather than dropped (rule 11: a refusal is
        // a reading).
        let v = match word_or_demand(&pre, &op.body) {
            Some(Ok(v)) => v,
            Some(Err(demanded)) => {
                cover.refused.insert(&key, demanded);
                continue;
            }
            // No sensor-plane block at all: the versor is not one this
            // reader can factor, which is unmeasured and not a demand.
            None => continue,
        };
        let Some((rotor, carriage)) = halves_of(&v) else { continue };
        for &(r, c, _) in pre.cells.iter().chain(op.body.cells.iter()) {
            reached.insert(cell_pos_key(r, c), ());
        }
        for ki in &parts {
            consumed.insert(ki, ());
        }
        placed.insert(
            &key,
            Placed {
                rotor,
                carriage,
                sigma: car.sigma.clone(),
                domain: support_of(&pre),
                codomain: support_of(&op.body),
            },
        );
        cover.carried.insert(&key, (v, car.v));
    }

    // (b) BY STRUCTURE, for what the pullback left. The correspondence is
    // the shell kind and the hue — what the piece IS — and never the
    // moduli, which the deformation changed.
    let mut by_kind_hue: crate::network::nerve::Keyed<Vec<Vec<u8>>> =
        crate::network::nerve::Keyed::new();
    for ko in &out_keys {
        let Some(op) = outs.get(ko) else { continue };
        let k = vec![op.kind.as_u8(), op.hue];
        match by_kind_hue.get_mut(&k) {
            Some(v) => v.push(ko.clone()),
            None => {
                by_kind_hue.insert(k, vec![ko.clone()]);
            }
        }
    }
    let in_keys: Vec<Vec<u8>> = ins.fiber(ROOT).map(|(k, _)| k).collect();
    for ki in &in_keys {
        if consumed.contains(ki) {
            continue;
        }
        let Some(ip) = ins.get(ki) else { continue };
        let Some(fiber) = by_kind_hue.get(&[ip.kind.as_u8(), ip.hue]).cloned() else { continue };
        // EVERY output of this kind and hue is tried, and one that carries
        // is placed — not the first one only. A shape occurring once in
        // the input and several times in the image is ONE piece carried by
        // SEVERAL versors, and the transitions between those copies are
        // the holonomy this cover exists to read. Stopping at the first
        // success places one copy and refuses the rest by bookkeeping
        // rather than by geometry, and then no transition can be anything
        // but the identity.
        for ko in fiber {
            if !budget.spend() {
                tally(|t| t.unmeasured += 1);
                return Err(Unmeasured);
            }
            let Some(op) = outs.get(&ko) else { continue };
            let Some(car) = carriage(&ip.body, &op.body) else { continue };
            let key = carriage_key(ki, &ko);
            // As in the pullback: a confirmed carriage whose rotor does not
            // spell is 37.7, and it is carried out rather than dropped.
            let v = match word_or_demand(&ip.body, &op.body) {
                Some(Ok(v)) => v,
                Some(Err(demanded)) => {
                    cover.refused.insert(&key, demanded);
                    continue;
                }
                None => continue,
            };
            let Some((rotor, carriage)) = halves_of(&v) else { continue };
            for &(r, c, _) in ip.body.cells.iter().chain(op.body.cells.iter()) {
                reached.insert(cell_pos_key(r, c), ());
            }
            consumed.insert(ki, ());
            placed.insert(
                &key,
                Placed {
                    rotor,
                    carriage,
                    sigma: car.sigma.clone(),
                    domain: support_of(&ip.body),
                    codomain: support_of(&op.body),
                },
            );
            cover.carried.insert(&key, (v, car.v));
        }
    }

    // (c) WHAT IS LEFT, AS A VOLUME. Both bodies (50.8 case 4): the
    // input's cells no piece carried, and the image's cells no piece
    // reached.
    let mut left: Vec<crate::transducer::sentence::Cell> = Vec::new();
    for &(r, c, h) in &input.cells {
        if !reached.contains(&cell_pos_key(r, c)) {
            left.push((r, c, h, 0));
        }
    }
    for &(r, c, h) in &image.cells {
        if !reached.contains(&cell_pos_key(r, c)) {
            left.push((r, c, 0, h));
        }
    }
    cover.uncarried = element::measure_lanes(&left);

    // (d) THE MEETS, from a keyed occupancy of the placed supports and
    // their rings (rule 1: never a pair loop over the pieces). A cell
    // holding two or more placed pieces IS a meet, and the meet is
    // addressed by its two ends.
    // The two sides are filed under DISTINCT keys — a side byte ahead of
    // the position — so a domain cell and a codomain cell that happen to
    // sit at one grid position are not read as a meet. They are cells of
    // two different spaces that the correspondence relates; only the
    // versor relates them, and a shared coordinate is a coincidence of the
    // embedding, not an overlap.
    let mut occ: crate::network::nerve::Keyed<Vec<Vec<u8>>> = crate::network::nerve::Keyed::new();
    for (k, p) in placed.fiber(ROOT) {
        for (side, support) in [(0u8, &p.domain), (1u8, &p.codomain)] {
            let cells: Vec<Cell3> = support.iter().map(|&(r, c)| (r, c, 0)).collect();
            let ring = crate::transducer::blade::adjacent_ring(&cells);
            for (r, c) in support.iter().copied().chain(ring.into_iter().map(|(r, c, _)| (r, c))) {
                let mut pos = vec![side];
                pos.extend_from_slice(&cell_pos_key(r, c));
                match occ.get_mut(&pos) {
                    Some(v) => {
                        if !v.contains(&k) {
                            v.push(k.clone());
                        }
                    }
                    None => {
                        occ.insert(pos, vec![k.clone()]);
                    }
                }
            }
        }
    }
    for (_pos, here) in occ.fiber(ROOT) {
        // A cell's OWN occupants: rule 1 allows this walk, because the
        // cell is the boundary the pair is read out of.
        for a in 0..here.len() {
            for b in (a + 1)..here.len() {
                let key = meet_key(&here[a], &here[b]);
                if cover.filed.contains(&key) || cover.standing.contains(&key) {
                    continue;
                }
                if !budget.spend() {
                    tally(|t| t.unmeasured += 1);
                return Err(Unmeasured);
                }
                let (Some(pa), Some(pb)) = (placed.get(&here[a]), placed.get(&here[b])) else {
                    continue;
                };
                // THE TRANSITION, ON BOTH HALVES. Each is composed against
                // its neighbour's in its own algebra — the rotors in
                // Cl(4,0) over all six planes, the carriages in Cl(3,1) —
                // exactly as `extend` composes a cell's. Nothing is
                // re-factored: `word_or_demand` separated them when it
                // built the vertex.
                let (Some(ri), Some(ci)) =
                    (pa.rotor.inverse(), pa.carriage.versor_inverse())
                else {
                    // No inverse: a remainder edge (13.9 step 7).
                    continue;
                };
                let gamma_r = pb.rotor.geo(&ri);
                let gamma_c = pb.carriage.geo(&ci);
                // THE GAUGE AROUND THE SEAM, composed and not assumed:
                // σ_a takes the input's hues to a's, σ_b to b's, so the
                // transition's own hue map is σ_b ∘ σ_a⁻¹. Passing the
                // identity here would claim the seam permutes no hue
                // without having composed anything (rule 11).
                let hue = crate::transducer::sentence::CycleType::of(&hue_transition(
                    &pa.sigma, &pb.sigma,
                ));
                // ONE CLASSIFIER FOR BOTH ORGANS (L23b). The court reaches
                // it through `extend`; the voice reaches it here. A second
                // reader stood beside it, on a versor, which could not hold
                // a dual-plane rotor and re-derived the factorisation this
                // call is handed already separated.
                match crate::transducer::cech::classify(&gamma_r, &gamma_c, hue) {
                    crate::transducer::cech::Fill::Filled(..) => {
                        // Held locally, not tallied here. A cover that
                        // returns `Unmeasured` must contribute NOTHING —
                        // counting the flat meets it happened to reach
                        // before its share ended reports readings from a
                        // cover that did not complete, which is the same
                        // half-measure `Result<Cover, Unmeasured>` exists
                        // to make unrepresentable. Measured: `carried 0`
                        // beside `flat 2`, which cannot both be true of
                        // any completed cover, since a meet exists only
                        // where two pieces were placed.
                        flat += 1;
                    }
                    crate::transducer::cech::Fill::Congruent(rotor, ell) => {
                        cover.filed.insert(&key, (rotor, ell));
                    }
                    crate::transducer::cech::Fill::Class(kappa, _) => {
                        cover.standing.insert(&key, *kappa);
                    }
                    // 37.7 AT A SEAM. Kept apart from `standing` because
                    // it is a different question: a standing class is one
                    // the alphabet names and no held generator closes, and
                    // this is one the alphabet does not name at all. It is
                    // an adjunction candidate, and Vent 1 measures how many
                    // the cover finds.
                    crate::transducer::cech::Fill::Unspellable(rotor) => {
                        cover.unspellable.insert(&key, *rotor);
                    }
                    // `Open` is a transition these readers could not
                    // factor: unmeasured, and neither flat nor standing.
                    // `Already` never comes from `classify`.
                    crate::transducer::cech::Fill::Open
                    | crate::transducer::cech::Fill::Already => {}
                }
            }
        }
    }
    // Exact, not saturating: a budget's remaining only falls, so the
    // opening read is never below the closing one (rule 11).
    let spent = (opened - budget.remaining()) as u64;
    tally(|t| {
        t.measured += 1;
        t.spend += spent;
        t.flat += flat;
        t.carried += cover.carried.fiber(ROOT).count() as u64;
        t.filed += cover.filed.fiber(ROOT).count() as u64;
        t.standing += cover.standing.fiber(ROOT).count() as u64;
        t.unspellable += cover.unspellable.fiber(ROOT).count() as u64;
        t.refused += cover.refused.fiber(ROOT).count() as u64;
    });
    Ok(cover)
}

// `rotor_of_transition` was here. It classified a meet's transition by the
// transition's sensor-plane block alone, which reads a translation or a
// dilation as the identity and mis-orders a rotation about any point but
// the origin. `cech::classify` replaces it — the same reader the court
// uses, taking the rotor and the carriage rather than a versor.
