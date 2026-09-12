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
    /// MEETS THAT BECAME A FACE CARRYING 50.3's EDGE — the transition
    /// between the two pieces' own frames. Counted because `flat` alone
    /// cannot tell a loop that was evaluated and found flat from a loop
    /// whose faces carry no edge to evaluate: without this, wiring the
    /// edge and not wiring it read the same on the line.
    pub edges: u64,
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

pub(crate) fn tally(f: impl Fn(&mut CoverTally)) {
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
    /// THE 1-SIMPLICES OF THE NERVE, by the meet's own address. The address
    /// IS the unordered pair of carriage keys, so the ends are read back
    /// with `meet_ends` and not stored a second time beside it (law 12).
    /// No transition and no class — the face between two vertices is the
    /// form's to compute and the cycle's holonomy is the form's to read.
    pub(crate) meets: crate::network::nerve::Keyed<()>,
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
    use super::cursor;
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
    // THE PARK, written once: everything the stroke MEASURED, and nothing
    // about where it was. It is taken by value, so there is one place a
    // half-run cover is stored and no site can forget a carrier.
    macro_rules! park {
        () => {{
            tally(|t| t.unmeasured += 1);
            cursor::park(&ckey, cursor::Pending { cover, placed, consumed, reached });
            return Err(Unmeasured);
        }};
    }

    // (a) THE PULLBACK, in place. A CARRIAGE ALREADY PLACED IS SKIPPED
    // BECAUSE IT IS PLACED, read at its own address, and not because an
    // index passed it. That reading is intrinsic to the cover's own state,
    // so a stroke that resumes needs no memory of the order a previous
    // stroke walked in.
    //
    // THE TEST IS THE CARRIAGE'S ADDRESS AND NOT THE PIECE'S CELLS. Asking
    // instead whether every cell of the image piece is already `reached`
    // refuses REPLICATION by bookkeeping: a piece reached from one domain
    // would never be offered to a second, and one image piece carried from
    // two domains is exactly the transition 50.8 exists to read. Measured
    // on the twelve at 1<<20: `carried 161 → 158` and `flat 3444 → 3387`,
    // three carriages and fifty-seven cycles lost to the wrong reading.
    // `carriage_key` was introduced for this and the test now uses it.
    //
    // A piece with no carriage is read again next stroke and fails again.
    // That is not a queue position missed: it is the uncarried support, a
    // standing defect, and `uncarried` below is where it reports.
    for ko in &out_keys {
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
        // SETTLED? The preimage is a function of this piece's own cells, so
        // the carriage's address is too, and it is the same address a
        // previous stroke would have written. Placed or refused, this piece
        // is read and is not read again — and the assembly that got here
        // costs nothing, being keyed reads of structure already in hand.
        let settled = carriage_key(&preimage_key(&parts), ko);
        if cover.carried.contains(&settled) || cover.refused.contains(&settled) {
            break 'piece;
        }
        // ONE READ PER CARRIAGE ATTEMPT, charged where the attempt is.
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
            park!();
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
    // A CONSUMED INPUT PIECE IS SKIPPED BECAUSE IT IS CONSUMED. This
    // reader was already intrinsic before any cursor was written, and the
    // cursor was bolted over it.
    for ki in &in_keys {
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
            let Some(op) = outs.get(&ko) else { continue };
            // A carriage already placed between these two is not attempted
            // again: the cover's own `carried` answers it, keyed by exactly
            // this ordered pair.
            if cover.carried.contains(&carriage_key(ki, &ko)) {
                continue;
            }
            // One read per carriage ATTEMPT, charged where the attempt is.
            if !budget.spend() {
                park!();
            }
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
    // addressed by its two ends; a cell holding k of them is k−1 meets.
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
                    // One piece's cells are offered contiguously — the walk
                    // above is per piece — and a piece reaches one position
                    // more than once only through its own ring, so a repeat
                    // can only be the key just pushed. This was
                    // `!v.contains(&k)`, a walk of the cell's occupants on
                    // every offer, comparing whole addresses: k² address
                    // comparisons on the same cell whose pairing below was
                    // already k².
                    Some(v) => match v.as_slice() {
                        [.., last] if *last == k => {}
                        _ => v.push(k.clone()),
                    },
                    None => {
                        occ.insert(pos, vec![k.clone()]);
                    }
                }
            }
        }
    }
    // A MEET ALREADY RECORDED IS NOT RECORDED AGAIN, and that reader —
    // `cover.meets.contains(&key)` below — was in the tree before any
    // cursor was. The cell needs no position of its own.
    for (_pos, here) in occ.fiber(ROOT) {
        // ONE READ PER CELL THAT YIELDS A MEET, charged on the first meet
        // it yields. A cell that yields nothing is not work and costs
        // nothing; charged unconditionally at the top instead, every
        // single-occupant cell of every placed support was paid for —
        // measured on the twelve, `measured 13 → 11` and `carried 4 → 0`.
        //
        // `paid` is bookkeeping about this stroke's payment, not a reading
        // of the cell: nothing in the cover branches on it.
        let mut paid = false;
        // A cell's occupants are a simplex, not a clique: they all contain
        // this cell, so every cycle among them bounds, and k occupants are
        // k−1 meets. Written as k(k−1)/2, each meet became a face and a loop
        // of the pair form — 1 806 of one form's 1 989 faces, 1 901 loops.
        // The cycles that carry a cover's holonomy span DISTINCT cells and
        // are untouched.
        for w in here.windows(2) {
            let [ka, kb] = w else { continue };
            'meet: {
                let key = meet_key(ka, kb);
                if cover.meets.contains(&key) {
                    break 'meet;
                }
                if !paid {
                    if !budget.spend() {
                        park!();
                    }
                    paid = true;
                }
                // THE MEET IS RECORDED AND NOT READ. Two pieces overlap;
                // that is a 1-simplex of the nerve and nothing more. The
                // transition between them is the FACE of the two vertices
                // they become, and the class of a cycle of faces is the
                // form's loop holonomy — both the form's to compute, once,
                // over the whole cycle rather than edge by edge here.
                cover.meets.insert(&key, ());
            }
        }
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
