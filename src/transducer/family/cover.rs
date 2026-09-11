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

use super::pieces::{carriage_key, meet_key, pieces_of, preimage_key};
use super::*;

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
    /// COVERS STILL PARKED at the end of the walk (L20): a stroke ended
    /// inside them and the placements are held for the next one. Read off
    /// the cursor's own carrier at the end and counted nowhere, so it
    /// cannot disagree with what is actually held. An `unmeasured` that
    /// leaves nothing pending was re-covered from the first piece; one
    /// that leaves a pending entry will resume.
    pub pending: usize,
    /// Reads spent inside covers, apart from the peel's own.
    pub spend: u64,
    /// Carriages placed and 1-simplices of the nerve recorded.
    pub carried: u64,
    pub meets: u64,
    /// THE LOOPS THE FORM CLASSIFIED over those carriages, by their
    /// reading. They are counted where they are read — on the form, over a
    /// whole cycle — and not in the cover, which classifies nothing.
    pub flat: u64,
    pub filed: u64,
    pub standing: u64,
    /// Loops whose holonomy the seventeen do not spell (37.7).
    pub unspellable: u64,
    /// Pieces carried geometrically whose rotor the seventeen do not
    /// spell — the adjunction candidates the cover used to discard.
    pub refused: u64,
}

thread_local! {
    static TALLY: std::cell::Cell<CoverTally> = std::cell::Cell::new(CoverTally::default());
}

/// Read the walk's covers and clear the count. `pending` is read from the
/// cursor's own carrier here and not accumulated as the walk goes: what is
/// parked at the end IS what is parked, and a counter beside it could say
/// otherwise (law 12).
pub fn take_cover_tally() -> CoverTally {
    let mut t = TALLY.with(|t| t.replace(CoverTally::default()));
    t.pending = super::cursor::parked();
    t
}

/// Count one loop the FORM classified over the cover's carriages. Called
/// where the reading happens, which is no longer here.
pub(crate) fn tally_loop(fill: &crate::transducer::cech::Fill) {
    use crate::transducer::cech::Fill;
    tally(|t| match fill {
        Fill::Filled(..) => t.flat += 1,
        Fill::Congruent(..) => t.filed += 1,
        Fill::Class(..) => t.standing += 1,
        Fill::Unspellable(..) => t.unspellable += 1,
        Fill::Open | Fill::Already => {}
    });
}

fn tally(f: impl Fn(&mut CoverTally)) {
    TALLY.with(|t| {
        let mut v = t.get();
        f(&mut v);
        t.set(v);
    });
}

/// The two supports of one placed carriage: what it was carried FROM and
/// what it landed on.
///
/// Both, because the nerve reads meets on both. A deformation is a map OUT
/// OF the input, so the Čech nerve is the nerve of the cover of the
/// SOURCE: two carriages sharing a domain overlap on the whole of it,
/// however far apart their codomains land. That is the replication case,
/// and the only case in which the transition between two copies of one
/// shape is visible at all. Meets on the codomain are the merge case.
/// Reading only the codomain — measured with `carried 2` on a family of
/// one input piece and ten output pieces — leaves the nerve discrete.
pub(crate) struct Supports {
    pub(crate) domain: Vec<(i64, i64)>,
    pub(crate) codomain: Vec<(i64, i64)>,
}

/// ONE PIECE THE COVER CARRIED: the word vertex it became, the versor
/// that carried it, and the support it landed on. The support rides along
/// because the form needs it for the vertex's place and the cover has
/// already computed it.
pub(crate) type CarriedPiece = (Vertex, Versor, Vec<(i64, i64)>);

/// WHAT THE COVER OF ONE DEFORMATION REACHED. Every field is content, and
/// every carrier is keyed by a piece's own address.
///
/// THE COVER CLASSIFIES NOTHING (L23c). It carried pieces, read their
/// meets, and then classified each meet on its own — filing torsion and
/// standing classes into carriers that `pair.rs` never read, so the whole
/// reading died here and only a tally survived it. A transition between
/// two pieces is a FACE of the form they are vertices of, and the class of
/// a cycle of faces is the form's loop holonomy. The cover hands over the
/// pieces and the meets; the form makes them vertices and faces, runs
/// `loops_of`, and classifies each loop once.
pub(crate) struct Cover {
    /// The word vertex of each carried piece, the versor that carried it,
    /// and the support it landed on — by `carriage_key(domain, codomain)`.
    /// The support rides along because the form needs it for the vertex's
    /// place and the cover has already computed it.
    pub(crate) carried: crate::network::nerve::Keyed<CarriedPiece>,
    /// THE 1-SIMPLICES OF THE NERVE: the unordered pair of carriage keys
    /// that meet, by the meet's own address. No transition and no class —
    /// the face between two vertices is the form's to compute and the
    /// cycle's holonomy is the form's to read.
    pub(crate) meets: crate::network::nerve::Keyed<(Vec<u8>, Vec<u8>)>,
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
    pub(crate) fn new() -> Cover {
        Cover {
            carried: crate::network::nerve::Keyed::new(),
            meets: crate::network::nerve::Keyed::new(),
            refused: crate::network::nerve::Keyed::new(),
            uncarried: [0; 4],
        }
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
    use super::cursor::{self, Stage};
    use crate::network::nerve::ROOT;
    let opened = budget.remaining();
    let ins = pieces_of(&input.cells, scale);
    let outs = pieces_of(&image.cells, scale);
    // WHERE THIS COVER STOPPED, if a stroke stopped it (L20, S13.3). A
    // fresh deformation starts at the pullback's first piece; one a stroke
    // parked starts where that stroke left off, with what it had placed.
    let ckey = cursor::deformation_key(&input.cells, &image.cells, scale);
    let mut st = match cursor::take(&ckey) {
        Some(p) => p,
        None => cursor::Pending::new(),
    };
    let mut cover = std::mem::replace(&mut st.cover, Cover::new());

    // Cell position → the input piece holding it. This is Φ: the
    // positional correspondence over the two supports, which is what a
    // deformation-in-place has in place of a whole-body versor.
    let mut occ_in: crate::network::nerve::Keyed<Vec<u8>> = crate::network::nerve::Keyed::new();
    for (k, p) in ins.fiber(ROOT) {
        for &(r, c, _) in &p.body.cells {
            occ_in.insert(cell_pos_key(r, c), k.clone());
        }
    }

    let mut placed = std::mem::replace(&mut st.placed, crate::network::nerve::Keyed::new());
    let mut consumed = std::mem::replace(&mut st.consumed, crate::network::nerve::Keyed::new());
    let mut reached = std::mem::replace(&mut st.reached, crate::network::nerve::Keyed::new());
    let out_keys: Vec<Vec<u8>> = outs.fiber(ROOT).map(|(k, _)| k).collect();
    // THE PARK, written once: everything the stroke has and where it
    // stopped. It is taken by value at each site, so there is one place a
    // half-run cover is stored and no site can forget a carrier.
    macro_rules! park {
        ($stage:expr) => {{
            tally(|t| t.unmeasured += 1);
            cursor::park(
                &ckey,
                cursor::Pending { cover, placed, consumed, reached, stage: $stage },
            );
            return Err(Unmeasured);
        }};
    }

    // (a) THE PULLBACK, in place. Resumed at the piece after the last one
    // a stroke FINISHED — key order, never a count of how many were done.
    // `done_a` only advances when a piece is complete, so a piece the
    // budget cut in half is redone whole next stroke; every write inside
    // one is keyed, so redoing it writes the same entries at the same
    // addresses and the result does not depend on where a stroke fell.
    let (in_a, mut done_a) = match &st.stage {
        Stage::Pullback(after) => (true, after.clone()),
        _ => (false, None),
    };
    for ko in out_keys.iter().filter(|_| in_a) {
        if done_a.as_ref().is_some_and(|after| ko <= after) {
            continue;
        }
        // THE PIECE IS ONE UNIT OF WORK. Every way out of it but the park
        // leaves it FINISHED — a piece with no preimage, no carriage or no
        // spelling is a piece read to its end — so the cursor advances
        // once, below, and no exit can forget to move it.
        'piece: {
        let Some(op) = outs.get(ko) else { break 'piece };
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
            let Some(ip) = ins.get(&ki) else { continue };
            cells.extend_from_slice(&ip.body.cells);
            parts.push(ki);
        }
        // No preimage is no piece to carry, not a piece that failed to
        // carry (rule 11): nothing is reached and nothing is consumed.
        let Some(pre) = (!cells.is_empty()).then(|| body_of(cells)) else {
            break 'piece;
        };
        // ONE READ PER CARRIAGE ATTEMPT, charged where the attempt is.
        //
        // THE CHARGE AND THE CURSOR MUST BE THE SAME UNIT, or a stroke that
        // can pay for one unit does not advance by one. This charged once
        // per PART of a preimage while resuming per PIECE: measured, a
        // four-read stroke on the notched fixture then never finished a
        // piece and never advanced — 4 096 strokes re-running the first
        // one. That is the starvation the cursor exists to end, reproduced
        // by the cursor. A piece that reaches here costs exactly one, so a
        // stroke with one read advances by one piece.
        //
        // It is charged HERE and not at the top of the piece: a piece with
        // no preimage is not work, and charging for it made the machine
        // stroke pay for every empty piece of every peel — measured,
        // `carried 4 → 0` on the twelve.
        //
        // `ins.get(&ki)` in the assembly above is a keyed read of a
        // structure already in hand, not a read of the scene; charging for
        // it charged the index walk rather than the geometry.
        if !budget.spend() {
            park!(Stage::Pullback(done_a));
        }
        let Some(car) = carriage(&pre, &op.body) else { break 'piece };
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
                break 'piece;
            }
            // No sensor-plane block at all: the versor is not one this
            // reader can factor, which is unmeasured and not a demand.
            None => break 'piece,
        };
        for &(r, c, _) in pre.cells.iter().chain(op.body.cells.iter()) {
            reached.insert(cell_pos_key(r, c), ());
        }
        for ki in &parts {
            consumed.insert(ki, ());
        }
        let codomain = support_of(&op.body);
        placed.insert(&key, Supports { domain: support_of(&pre), codomain: codomain.clone() });
        cover.carried.insert(&key, (v, car.v, codomain));
        }
        done_a = Some(ko.clone());
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
    // Resumed like the pullback: after the last input piece a stroke
    // finished. Arriving from a completed (a), the cursor is at the start
    // of (b) and nothing is skipped.
    let mut done_b = match &st.stage {
        Stage::Structure(after) => after.clone(),
        _ => None,
    };
    let in_b = !matches!(&st.stage, Stage::Meets(_));
    for ki in in_keys.iter().filter(|_| in_b) {
        // The input piece the cursor stopped in is RE-ENTERED, not skipped:
        // the attempt inside it resumes at the output piece after the last
        // one paid for.
        if done_b.as_ref().is_some_and(|(after_in, _)| ki < after_in) {
            continue;
        }
        let resume_out = match &done_b {
            Some((after_in, after_out)) if ki == after_in => Some(after_out.clone()),
            _ => None,
        };
        'piece: {
        if consumed.contains(ki) {
            break 'piece;
        }
        let Some(ip) = ins.get(ki) else { break 'piece };
        let Some(fiber) = by_kind_hue.get(&[ip.kind.as_u8(), ip.hue]).cloned() else {
            break 'piece;
        };
        // EVERY output of this kind and hue is tried, and one that carries
        // is placed — not the first one only. A shape occurring once in
        // the input and several times in the image is ONE piece carried by
        // SEVERAL versors, and the transitions between those copies are
        // the holonomy this cover exists to read. Stopping at the first
        // success places one copy and refuses the rest by bookkeeping
        // rather than by geometry, and then no transition can be anything
        // but the identity.
        for ko in fiber {
            if resume_out.as_ref().is_some_and(|after| &ko <= after) {
                continue;
            }
            let Some(op) = outs.get(&ko) else { continue };
            // One read per carriage ATTEMPT, charged where the attempt is,
            // and the cursor records exactly this pair.
            if !budget.spend() {
                park!(Stage::Structure(done_b));
            }
            done_b = Some((ki.clone(), ko.clone()));
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
            for &(r, c, _) in ip.body.cells.iter().chain(op.body.cells.iter()) {
                reached.insert(cell_pos_key(r, c), ());
            }
            consumed.insert(ki, ());
            let codomain = support_of(&op.body);
            placed.insert(
                &key,
                Supports { domain: support_of(&ip.body), codomain: codomain.clone() },
            );
            cover.carried.insert(&key, (v, car.v, codomain));
        }
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
    // Resumed after the last CELL a stroke finished. A cell is one unit of
    // work here for the same reason a piece is above: every pair in it is
    // keyed, so redoing a cell writes the same meets at the same
    // addresses.
    let mut done_d = match &st.stage {
        Stage::Meets(after) => after.clone(),
        _ => None,
    };
    for (_pos, here) in occ.fiber(ROOT) {
        if done_d.as_ref().is_some_and(|after| &_pos <= after) {
            continue;
        }
        // ONE READ PER CELL THAT YIELDS A MEET, charged on the first meet
        // it yields. The cursor resumes at a cell, so a cell is what a
        // stroke pays for — and a cell that yields nothing is not work, so
        // it costs nothing. Charged unconditionally at the top instead,
        // every single-occupant cell of every placed support was paid for:
        // measured on the twelve, `measured 13 → 11` and `carried 4 → 0`.
        //
        // `paid` is bookkeeping about this stroke's payment, not a reading
        // of the cell: nothing in the cover branches on it.
        let mut paid = false;
        // A cell's OWN occupants: rule 1 allows this walk, because the
        // cell is the boundary the pair is read out of.
        for a in 0..here.len() {
            for b in (a + 1)..here.len() {
                let key = meet_key(&here[a], &here[b]);
                if cover.meets.contains(&key) {
                    continue;
                }
                if !paid {
                    if !budget.spend() {
                        park!(Stage::Meets(done_d));
                    }
                    paid = true;
                }
                // THE MEET IS RECORDED AND NOT READ. Two pieces overlap;
                // that is a 1-simplex of the nerve and nothing more. The
                // transition between them is the FACE of the two vertices
                // they become, and the class of a cycle of faces is the
                // form's loop holonomy — both the form's to compute, once,
                // over the whole cycle rather than edge by edge here.
                cover.meets.insert(&key, (here[a].clone(), here[b].clone()));
            }
        }
        done_d = Some(_pos.clone());
    }
    // Exact, not saturating: a budget's remaining only falls, so the
    // opening read is never below the closing one (rule 11).
    let spent = (opened - budget.remaining()) as u64;
    tally(|t| {
        t.measured += 1;
        t.spend += spent;
        t.carried += cover.carried.fiber(ROOT).count() as u64;
        t.meets += cover.meets.fiber(ROOT).count() as u64;
        t.refused += cover.refused.fiber(ROOT).count() as u64;
    });
    Ok(cover)
}

// `rotor_of_transition` was here. It classified a meet's transition by the
// transition's sensor-plane block alone, which reads a translation or a
// dilation as the identity and mis-orders a rotation about any point but
// the origin. `cech::classify` replaces it — the same reader the court
// uses, taking the rotor and the carriage rather than a versor.
