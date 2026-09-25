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
//! THE PIECES ARE PULLED BACK, NEVER COMPARED. An output piece's preimage
//! is the set of input pieces standing under its own sites, read per site
//! from a keyed occupancy, so a piece in place is congruent by
//! construction. That preimage is ONE set and is carried once: the two
//! peels are under no obligation to agree on granularity, and requiring
//! them to — one input piece against one output piece — refuses every
//! carriage where they differ.
//!
//! A purely positional pullback reaches only pieces carried in place, and
//! every transition would be the identity. The second pass reaches a piece
//! that MOVED, and it does not look for one: a deformation carries its
//! pieces by the SAME versors, so each versor the pullback placed is
//! applied to the unplaced piece and the image says where it landed. That
//! image is ONE set too, assembled and carried once, in the same theorem
//! the preimage stands on.
//!
//! Nothing is added to the tape. `Form.torsion` already carries a filled
//! simplex as `FiledOrbit { sigma, period, chord }` and the court already
//! replays it (23.1 step 4).
//!
//! Nothing is keyed by position. A piece is addressed by its content —
//! shell kind and its support written out, the hue being a gauge the
//! piece's laminae carry and not a coordinate of it (H1b) — so no arrival index
//! survives into the nerve (rule 6, rule 15).

use super::pieces::pieces_of;
use super::*;
use crate::network::nerve::coord;

/// WHAT THE COVERS OF ONE WALK READ. An instrument and nothing else: no
/// cognition reads it, no branch consults it, and it is taken and cleared
/// by the walker that prints it (§8.1 instrumentation).
///
/// It exists because the cover's readings do not survive into the form.
/// A filed class becomes torsion, a refused carriage becomes a demand
/// vertex the soak already counts — but a cover that COMPLETED and found
/// nothing, and the reads a cover spent, leave no trace in content, and
/// without them `simplices filled 0` cannot be told from `no cover ran`.
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
    /// CARRIAGE ATTEMPTS MADE BY PROPAGATION — pass (b). One per (distinct
    /// placed versor, unplaced piece) whose image names one output piece,
    /// so it is bounded by `placed × unplaced` and is zero where the
    /// pullback placed nothing. The pass it replaces tried `ins × outs`
    /// and this counter is how that cannot come back unseen.
    pub propagated: u64,
    /// Carriages placed and 1-simplices of the nerve recorded.
    pub carried: u64,
    pub meets: u64,
    /// MEETS THAT BECAME A FACE CARRYING 50.3's EDGE — the transition
    /// between the two pieces' own frames. Counted because `flat` alone
    /// cannot tell an orbit that was evaluated and found flat from an orbit
    /// whose faces carry no edge to evaluate: without this, wiring the
    /// edge and not wiring it read the same on the line.
    pub edges: u64,
    /// AXIOM B — INTERFACES WHOSE TRANSITION WAS TAKEN AS A GEODESIC
    /// TRANSVECTION between the two frames' own base points on the sheet,
    /// and those where a frame did not stand on it.
    ///
    /// Counted because the wiring is an `if let`: a frame off the chart
    /// falls through to `edge_of`'s conformal carriage, and a fallback that
    /// is not counted is a refusal reported as nothing (rule 11). Without
    /// these two, Axiom B being live and Axiom B never firing read the same
    /// on the line — which is the defect `edges` above already exists to
    /// prevent one level down.
    pub geodesic: u64,
    pub off_sheet: u64,
    /// THE FOOTPRINT'S OWN DISPLACEMENT, offered and reaching the operator
    /// (54.7, ruled 2026-09-21).
    ///
    /// `chords_offered` is every chord the gate emitted; `tau_active` is
    /// those whose two footprints have DIFFERENT basepoints, so
    /// `τ = p(D_b) − p(D_a) ≠ 0` and the chord's transport is the
    /// composition `Ω_frame · T(τ)` rather than the frame transvection
    /// alone.
    ///
    /// Counted because `R = 1 − v_b v_a` has `⟨R⟩₂ = v_a ∧ v_b`, a wedge of
    /// two vectors and therefore SIMPLE with `I₂ = 0` identically — so a
    /// corpus of pure transvections reads `non-simple 0` whether or not the
    /// footprint contributes. Only a composition of two non-parallel boosts
    /// is non-simple, so without this pair the reading `non-simple 4 of
    /// 545` cannot distinguish "τ is absent" from "τ is collinear".
    pub chords_offered: u64,
    pub tau_active: u64,
    /// THE THREE WAYS `τ` DOES NOT REACH THE OPERATOR, kept apart because
    /// they are three different facts and one count of "inactive" reports
    /// them as one. `tau_unfooted` is a chord end carrying no footprint at
    /// all; `tau_met` is two footprints standing on one basepoint, so
    /// `τ = 0` and the identity is the honest transport; `tau_refused` is a
    /// τ the Cayley transform could not carry, which is a defect and not a
    /// reading. Only the last is a refusal (rule 11).
    /// WHETHER THE INTERFACE CARRIED A FRAME (56.5). `framed` is a meet
    /// whose two supports are related by a proper turn, so `g = θ·γ`;
    /// `unframed` is one no turn relates — a reflection, a difference of
    /// shape, or a 45° relation refused as irrational — where `γ` stands
    /// alone. Counted apart because "the frame never fired" and "the
    /// frame fired and was the identity" would otherwise read the same,
    /// which is the defect that let `ℓ = 1` pass as a measurement.
    pub framed: u64,
    pub unframed: u64,
    /// A frame whose half turn no even moment can close and whose
    /// supports differ in cardinality, so no cell test can either (56.5).
    /// Counted apart from `framed` because it is a PROJECTIVE class, not
    /// a determined rotor, and reporting the two as one would be the same
    /// defect as reporting "never fired" and "fired and was identity"
    /// together.
    pub frame_half_open: u64,
    /// FR1 — the pieces whose own pose was read on the tube, the domain or
    /// the codomain extending off the sensor's plane. The regime, beside the
    /// three outcomes above, not a fourth.
    pub pose_tube: u64,
    pub tau_unfooted: u64,
    pub tau_met: u64,
    pub tau_refused: u64,
    /// THE ORBITS THE FORM CLASSIFIED over those carriages, by their
    /// reading. They are counted where they are read — on the form, over a
    /// whole cycle — and not in the cover, which classifies nothing.
    pub flat: u64,
    pub filed: u64,
    pub standing: u64,
    /// Orbits whose holonomy the seventeen do not spell (37.7).
    pub unspellable: u64,
    /// 78.26(5) — orbits evaluated and not measured: a rotor or a rotation it
    /// was composed from is no versor (`Fill::Unread`).
    pub orbit_unread: u64,
    /// Pieces carried geometrically whose rotor the seventeen do not
    /// spell — the adjunction candidates the cover used to discard.
    pub refused: u64,
    /// 53.6 item 3 — WHAT EACH MEET DID AT THE LIFT TO CLASSES.
    ///
    /// A meet relates two PIECES; the scene holds classes, so a meet
    /// becomes incidence the scene can carry only through
    /// `transport_class`. Three outcomes and they mean different things,
    /// which is why they are three counters and not one:
    ///
    ///   `chords` — the two pieces carry DISTINCT classes, so the meet
    ///   names a chord between two of the scene's vertices. These are the
    ///   pairs that span distinct sites, which `named_faces` says
    ///   in its own comment are "the cycles that carry a class" — and which
    ///   nothing offered until 54.7.
    ///
    ///   `within_class` — both pieces are one class. The meet is real and
    ///   closes nothing the scene can see; skipping it is correct.
    ///
    ///   `unclassed` — a piece carries no transport class at all, so the
    ///   meet cannot be lifted whatever its geometry. `transport_class`
    ///   answers only for `Vertex::Word`, and it gates the orbits' ENDS by
    ///   the same predicate, so this counter reads a blockage that stops
    ///   the chord and the torsion chord together.
    ///
    /// Counted here rather than printed, because a count recovered by
    /// matching a rendered line is a second carrier of one fact: the text
    /// can drift from the writer and nothing makes them agree (law 12).
    /// The line that prints them is a VIEW of this carrier.
    pub chords: u64,
    pub within_class: u64,
    pub unclassed: u64,
    /// AT1, ruled (a) on the chords — TWO DISTINCT PIECES AT ONE PIECE-CHART:
    /// one member at one frame, equal nucleus and equal shape moments, and
    /// yet two carried entries of the cover. The chart's key is fixed-size by
    /// ruling and is never widened to tell them apart; the second is a
    /// finding, counted here, and meets nothing as a node of its own.
    pub piece_collisions: u64,
    /// §4.4 E8 — A KEY THIS READER WROTE AND COULD NOT READ BACK.
    ///
    /// The cover's three carriers are keyed by [`coord::Piece`],
    /// [`coord::Carriage`] and [`coord::Meet`], each of which decodes or
    /// refuses. A refusal is this reader failing on its own layout, which
    /// is a defect and not a reading of the corpus, so it is counted apart
    /// from every "nothing was there" above: absorbed into one of those it
    /// would read as an empty cover.
    pub undecodable: u64,
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

/// Count one orbit the FORM classified over the cover's carriages. Called
/// where the reading happens, which is no longer here.
pub(crate) fn tally_orbit(fill: &crate::transducer::cech::Fill) {
    use crate::transducer::cech::Fill;
    tally(|t| match fill {
        Fill::Filled(..) => t.flat += 1,
        Fill::Congruent(..) => t.filed += 1,
        Fill::Class(..) => t.standing += 1,
        Fill::Unspellable(..) => t.unspellable += 1,
        Fill::Unread(_) => t.orbit_unread += 1,
        // A classified orbit is never unevaluated: `classify` answers every
        // arm above, and `Open` is the state of a simplex not yet asked.
        Fill::Open => {}
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
/// Reading only the codomain leaves the nerve discrete.
#[derive(Clone)]
pub(crate) struct Supports {
    pub(crate) domain: Vec<crate::transducer::base::Site>,
    pub(crate) codomain: Vec<crate::transducer::base::Site>,
}

/// ONE PIECE THE COVER CARRIED: the word vertex it became, the versor
/// that carried it, and BOTH of its supports. They ride along because the
/// form needs the landing for the vertex's place and the cover has
/// already computed both.
///
/// IT CARRIED ONLY THE CODOMAIN (56.5, ruled 2026-09-22). `placed` held
/// `Supports { domain, codomain }` and was consumed locally to build the
/// meets index, so the domain was computed and dropped. A chart's FRAME
/// is how that chart turned under carrying — `θ = relative_pose(domain,
/// codomain)`, the piece against ITSELF — and with only the landing in
/// hand no interface could ask it. Comparing one piece's landing to
/// another's asks whether two different pieces are congruent, which is a
/// different question and usually answers no.
pub(crate) type CarriedPiece = (Vertex, Versor, Supports);

/// WHAT THE COVER OF ONE DEFORMATION REACHED. Every field is content, and
/// every carrier is keyed by a piece's own address.
///
/// THE COVER CLASSIFIES NOTHING (L23c). It carried pieces, read their
/// meets, and then classified each meet on its own — filing torsion and
/// standing classes into carriers that `pair.rs` never read, so the whole
/// reading died here and only a tally survived it. A transition between
/// two pieces is a FACE of the form they are vertices of, and the class of
/// a cycle of faces is the form's orbit holonomy. The cover hands over the
/// pieces and the meets; the form makes them vertices and faces, runs
/// `orbits_of`, and classifies each orbit once.
///
/// X3: A COVER IS ONE DEFORMATION'S, and its carriers are read with
/// `scoped`. `carried`, `meets` and `refused` hold what THIS pass placed,
/// met and could not spell; a read of them is a read of one pair's
/// reading, and there is no coordinate above a cover for a prefix to
/// follow. The
/// same holds for the sets each pass builds as it goes — `occ_in`,
/// `occ_out`, `named`, `versors`, `occ` — which are read back by the pass
/// that filled them.
pub(crate) struct Cover {
    /// The word vertex of each carried piece, the versor that carried it,
    /// and the support it landed on — by the [`coord::Carriage`] it is.
    /// The support rides along because the form needs it for the vertex's
    /// place and the cover has already computed it.
    pub(crate) carried: crate::network::nerve::Keyed<CarriedPiece, coord::Carriage>,
    /// THE 1-SIMPLICES OF THE NERVE, by the meet's own coordinate. The
    /// coordinate IS the unordered pair of carriages, so the ends are read
    /// back with [`coord::Meet::ends`] and not stored a second time beside
    /// it (law 12). No transition and no class — the face between two
    /// vertices is the form's to compute and the cycle's holonomy is the
    /// form's to read.
    pub(crate) meets: crate::network::nerve::Keyed<(), coord::Meet>,
    /// A PIECE THE COVER CARRIED GEOMETRICALLY AND COULD NOT SPELL, by
    /// cover-piece address: `carriage` found the versor and `word_or_demand`
    /// refused its rotor. Kept as the demand the peel typed, so the piece
    /// enters the form as a vertex instead of vanishing.
    ///
    /// A `continue` here would discard, silently, the only place in the
    /// engine where a carriage is CONFIRMED and its rotor is outside the
    /// alphabet — which is 37.7's condition exactly, and so the shortest
    /// route the cover has to an adjoined generator. The cover cannot run
    /// the ring itself: it is inside the memoised percept and 50.2 step 2
    /// forbids the crystal there, so the candidate is carried out to
    /// `derive_against`, which runs per mind.
    pub(crate) refused: crate::network::nerve::Keyed<Demanded, coord::Carriage>,
    /// What no piece carried, as a VOLUME on four lanes (law 5): the
    /// input's laminae no piece carried and the image's laminae no piece
    /// reached (50.8 case 4). A count of laminae is not this quantity —
    /// laminae carry different lane weight — so the dissolution reads the
    /// zero vector and never a cardinality.
    pub(crate) uncarried: [u64; 4],
    /// THE SITES A CARRIAGE REACHED, on either body, by their base key: the
    /// reading the demand for what was NOT reached is typed from (50.5),
    /// so the links a demand stands for are the laminae no piece carried
    /// and never a count of them.
    pub(crate) reached: crate::network::nerve::Keyed<()>,
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
            reached: crate::network::nerve::Keyed::new(),
        }
    }
}

/// 50.8 — COVER ONE DEFORMATION.
///
/// Pass (a), THE PULLBACK: an output piece's preimage is the input pieces
/// standing under its own sites, read per site from the occupancy. Pass
/// (b), BY PROPAGATION: an input piece the pullback placed nowhere is
/// CARRIED by each versor pass (a) already placed, and the output piece
/// its image lands in is read by position. In both, `carriage` decides and
/// nothing is ranked (rule 4), and neither pass looks for a piece.
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
/// - no versor: a remainder edge (13.9 step 7, 50.3), and no orbit through
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
    let opened = budget.remaining();
    let ins = pieces_of(&input.laminae);
    let outs = pieces_of(&image.laminae);
    // WHERE THIS COVER STOPPED, if a stroke stopped it (L20, S13.3). A
    // fresh deformation starts at the pullback's first piece; one a stroke
    // parked starts where that stroke left off, with what it had placed.
    let ckey = cursor::deformation_key(&input.laminae, &image.laminae, scale);
    let mut st = match cursor::take(&ckey) {
        Some(p) => p,
        None => cursor::Pending::new(),
    };
    let mut cover = std::mem::replace(&mut st.cover, Cover::new());

    // Site → the input piece holding it. This is Φ: the positional
    // correspondence over the two supports, which is what a
    // deformation-in-place has in place of a whole-body versor.
    //
    // THE OCCUPANT IS THE PIECE'S COORDINATE, not its bytes (§4.4 E8): the
    // decode happens once, here, where the pieces index is walked, and the
    // pullback below reads a [`coord::Piece`] back rather than re-deriving
    // one from a key it was handed.
    let mut occ_in: crate::network::nerve::Keyed<coord::Piece> =
        crate::network::nerve::Keyed::new();
    let (in_pieces, in_refused) = ins.decoded();
    for (k, p) in &in_pieces {
        for l in &p.body.laminae {
            occ_in.insert(l.site.key(), k.clone());
        }
    }

    let mut placed = std::mem::replace(&mut st.placed, crate::network::nerve::Keyed::new());
    let mut consumed = std::mem::replace(&mut st.consumed, crate::network::nerve::Keyed::new());
    let mut reached = std::mem::replace(&mut st.reached, crate::network::nerve::Keyed::new());
    // A KEY THAT DID NOT DECODE IS A READING (rule 11). Both peels write
    // their own [`coord::Piece`] keys, so a refusal here is this reader
    // failing to read back what it wrote and is counted where it happens —
    // never absorbed into "no pieces", which is a different fact.
    let (out_pieces, out_refused) = outs.decoded();
    let undecodable = (in_refused.len() + out_refused.len()) as u64;
    if undecodable > 0 {
        tally(|t| t.undecodable += undecodable);
    }
    let out_keys: Vec<coord::Piece> = out_pieces.iter().map(|(k, _)| k.clone()).collect();
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

    // (a) THE PULLBACK, in place (50.8). A carriage already placed is
    // skipped at its own address, so a stroke that resumes carries no
    // memory of the order a previous one read in.
    //
    // The address is the carriage's, never the image piece's sites: one
    // image piece carried from two domains is REPLICATION, and 50.8 exists
    // to read the transition between the two copies.
    //
    // A piece with no carriage is read again next stroke and refuses
    // again; it is uncarried support, and `uncarried` below is where it
    // reports (50.8 case 4).
    for ko in &out_keys {
        'piece: {
        let Some(op) = outs.at(ko) else { break 'piece };
        // The preimage: the input pieces named under this piece's own
        // sites, deduped by their address and read in key order.
        let mut named: crate::network::nerve::Keyed<(), coord::Piece> =
            crate::network::nerve::Keyed::new();
        for l in &op.body.laminae {
            if let Some(ki) = occ_in.get(&l.site.key()) {
                named.put(ki, ());
            }
        }
        // Φ⁻¹(V) IS ONE SET: assembled whole and carried once, closed up to
        // whole pieces so none is cut across the preimage boundary. The two
        // peels are under no obligation to agree on granularity, and this
        // is why they need not (50.8).
        let mut laminae: Vec<Lamina> = Vec::new();
        let mut parts: Vec<coord::Piece> = Vec::new();
        let (named_pieces, named_refused) = named.decoded();
        if !named_refused.is_empty() {
            tally(|t| t.undecodable += named_refused.len() as u64);
        }
        for (ki, ()) in named_pieces {
            let Some(ip) = ins.at(&ki) else { continue };
            laminae.extend_from_slice(&ip.body.laminae);
            parts.push(ki);
        }
        // No preimage is no piece to carry, not a piece that failed to
        // carry (rule 11): nothing is reached and nothing is consumed.
        let Some(pre) = (!laminae.is_empty()).then(|| body_of(laminae)).flatten() else {
            break 'piece;
        };
        // SETTLED? The preimage is a function of this piece's own sites, so
        // the carriage's coordinate is too: placed or refused, this piece is
        // read once however many strokes the cover takes.
        let key = coord::Carriage { domain: parts.clone(), codomain: vec![ko.clone()] };
        if cover.carried.holds(&key) || cover.refused.holds(&key) {
            break 'piece;
        }
        // ONE READ PER CARRIAGE ATTEMPT, charged at the attempt and not at
        // the top of the piece: a piece with no preimage is not work, and
        // the assembly above is keyed reads of structure already in hand
        // rather than reads of the scene (rule 10).
        if !budget.spend() {
            park!();
        }
        let Some(car) = carriage_with_core(&pre, &op.body) else { break 'piece };
        // The carriage is CONFIRMED here — a versor exists and carries the
        // preimage onto this piece. If its rotor does not spell, that is
        // 37.7 and not a failure to carry, so the piece is kept as the
        // demand the peel typed rather than dropped (rule 11: a refusal is
        // a reading).
        let v = match word_or_demand(&pre, &op.body) {
            Some(Ok(v)) => v,
            Some(Err(demanded)) => {
                cover.refused.put(&key, demanded);
                break 'piece;
            }
            // No sensor-plane block at all: the versor is not one this
            // reader can factor, which is unmeasured and not a demand.
            None => break 'piece,
        };
        for l in pre.laminae.iter().chain(op.body.laminae.iter()) {
            reached.insert(l.site.key(), ());
        }
        for ki in &parts {
            consumed.put(ki, ());
        }
        let supports = Supports { domain: support_of(&pre), codomain: support_of(&op.body) };
        placed.put(&key, supports.clone());
        cover.carried.put(&key, (v, car.v, supports));
        }
    }

    // (b) BY PROPAGATION, for what the pullback left. A deformation carries
    // its pieces by the SAME versors, and pass (a) has them: the unplaced
    // piece is not looked for, it is CARRIED by what is already placed and
    // the image says where it landed.
    //
    // Each distinct placed versor is applied to the piece's own laminae, the
    // image positions are read from the output occupancy by a keyed read
    // per site (rule 1: never a whole read of the output pieces), the pieces they
    // name are assembled into one image, and `carriage` confirms that one
    // candidate. The attempt count is `placed × unplaced`, and a family
    // whose pullback placed nothing attempts nothing here.
    //
    // A piece no placed versor lands on a piece is UNCARRIED and reports in
    // `uncarried` below: the deformation has a part the rest of it does not
    // explain, which is a demand (50.8 case 4).
    let mut occ_out: crate::network::nerve::Keyed<coord::Piece> =
        crate::network::nerve::Keyed::new();
    for (k, p) in &out_pieces {
        for l in &p.body.laminae {
            occ_out.insert(l.site.key(), k.clone());
        }
    }
    // THE DISTINCT PLACED VERSORS, by their own address. Two carriages that
    // carry by one versor propagate once, and a versor this reader cannot
    // address is not propagated: it could not be told from another, so the
    // cover could not say it had already been applied (W6, rule 15).
    let mut versors: crate::network::nerve::Keyed<Versor> = crate::network::nerve::Keyed::new();
    for (_, (_, v, _)) in cover.carried.scoped() {
        if let Some(vk) = v.key() {
            versors.insert(&vk, v.clone());
        }
    }
    let propagate: Vec<Versor> = versors.scoped().map(|(_, v)| v.clone()).collect();
    let in_keys: Vec<coord::Piece> = in_pieces.iter().map(|(k, _)| k.clone()).collect();
    // A consumed input piece is skipped at its own address, as a placed
    // carriage is in (a): the cover's state answers it, not a cursor.
    for ki in &in_keys {
        'piece: {
        if consumed.holds(ki) {
            break 'piece;
        }
        let Some(ip) = ins.at(ki) else { break 'piece };
        for pv in &propagate {
            // WHERE THIS VERSOR PUTS THE PIECE. Each lamina's center is
            // carried and the output piece holding it is read by position;
            // a lamina whose image the lattice does not hold is one this
            // reader did not carry (W6) and names nothing.
            let mut named: crate::network::nerve::Keyed<(), coord::Piece> =
                crate::network::nerve::Keyed::new();
            for l in &ip.body.laminae {
                let Some(t) = element::carry_site(pv, &l.site) else { continue };
                if let Some(ko) = occ_out.get(&t.key()) {
                    named.put(ko, ());
                }
            }
            // Φ(U) IS ONE SET, as Φ⁻¹(V) is in (a). The two peels are under
            // no obligation to agree on granularity, so an image that lies
            // across several output pieces is assembled whole and carried
            // once; one candidate, not several, and no piece cut across the
            // image boundary.
            let mut laminae: Vec<Lamina> = Vec::new();
            let mut parts: Vec<coord::Piece> = Vec::new();
            let (named_pieces, named_refused) = named.decoded();
            if !named_refused.is_empty() {
                tally(|t| t.undecodable += named_refused.len() as u64);
            }
            for (ko, ()) in named_pieces {
                let Some(op) = outs.at(&ko) else { continue };
                laminae.extend_from_slice(&op.body.laminae);
                parts.push(ko);
            }
            // No image is no piece to carry by this versor, not a piece that
            // failed to carry (rule 11).
            let Some(img) = (!laminae.is_empty()).then(|| body_of(laminae)).flatten() else {
                continue;
            };
            // A carriage already placed between this piece and this image is
            // not attempted again: the cover's own carriers answer it at the
            // address the attempt would write.
            // BOTH SIDES ARE LISTS (§4.4 E8). The pullback names one output
            // piece and assembles its domain; this pass names one input
            // piece and assembles its image. Each side is a list of pieces,
            // so a carriage between two WHOLE pieces has one coordinate
            // whichever pass placed it — where `assembly_key` had to
            // collapse a one-piece assembly to that piece's bare bytes to
            // get the same agreement, at the cost of an address that did
            // not say what it was.
            let key =
                coord::Carriage { domain: vec![ki.clone()], codomain: parts.clone() };
            if cover.carried.holds(&key) || cover.refused.holds(&key) {
                continue;
            }
            // One read per carriage ATTEMPT, charged where the attempt is.
            if !budget.spend() {
                park!();
            }
            tally(|t| t.propagated += 1);
            let Some(car) = carriage(&ip.body, &img) else { continue };
            // As in the pullback: a confirmed carriage whose rotor does not
            // spell is 37.7, and it is carried out rather than dropped.
            let v = match word_or_demand(&ip.body, &img) {
                Some(Ok(v)) => v,
                Some(Err(demanded)) => {
                    cover.refused.put(&key, demanded);
                    continue;
                }
                None => continue,
            };
            for l in ip.body.laminae.iter().chain(img.laminae.iter()) {
                reached.insert(l.site.key(), ());
            }
            consumed.put(ki, ());
            let supports =
                Supports { domain: support_of(&ip.body), codomain: support_of(&img) };
            placed.put(&key, supports.clone());
            cover.carried.put(&key, (v, car.v, supports));
        }
        }
    }

    // (c) WHAT IS LEFT, AS A VOLUME. Both bodies (50.8 case 4): the
    // input's laminae no piece carried, and the image's laminae no piece
    // reached.
    let mut left: Vec<Link> = Vec::new();
    for l in &input.laminae {
        if let (false, Some((r, c))) = (reached.contains(&l.site.key()), on_grid(&l.site)) {
            left.push((r, c, l.hue, 0));
        }
    }
    for l in &image.laminae {
        if let (false, Some((r, c))) = (reached.contains(&l.site.key()), on_grid(&l.site)) {
            left.push((r, c, 0, l.hue));
        }
    }
    cover.uncarried = element::measure_lanes(&left);
    cover.reached = reached.clone();

    // (d) THE MEETS, from a keyed occupancy of the placed supports (rule 1:
    // never a pair loop over the pieces). A site holding two or more placed
    // pieces IS a meet, and the meet is addressed by its two ends; a site
    // holding k of them is k−1 meets. The two sides are filed under
    // DISTINCT keys — a side byte ahead of the position — so a domain site
    // and a codomain site that happen to sit at one grid position are not
    // read as a meet. They are sites of two different spaces that the
    // correspondence relates; only the versor relates them, and a shared
    // coordinate is a coincidence of the embedding, not an overlap.
    //
    // THE RING IS GONE, AND WITH IT THE LAST STENCIL IN THE NERVE (58.3,
    // ruled 2026-09-22). This read `support ∪ ring_of_support(support)`,
    // so a 1-simplex existed wherever two supports came within one 4-step
    // of each other while a 2-simplex needed three of them on one cell. A
    // Cech nerve is functorial over ONE cover:
    //
    //     σ = [v₀ … v_k] ∈ N_k(𝒰)  ⟺  ⋂ᵢ U_i ≠ ∅
    //
    // and evaluating the 1-skeleton over `𝒰_thick = {S ∪ ring(S)}` while
    // the 2-skeleton reads `𝒰_bare = {S}` is two competing producers of one
    // incidence (law 12) — the 1-cells permitted to bridge a gap by raster
    // dilation, the 2-cells starved unless three solid bodies occupy one
    // discrete cell. A complex cannot have its dimensions evaluated against
    // different topologies.
    //
    // EXTENDING THE RING UPWARD WOULD HAVE BEEN WORSE. `ring_of_support` is
    // an extrinsic Manhattan 4-neighbourhood dilation on ℤ², an ad-hoc grid
    // stencil simulating a systole the physics never performed — the same
    // harness defect already purged from `extract.rs`. It also resurrects
    // the pore case this nerve exists to refuse: three mutual neighbours
    // around a void have overlapping RINGS inside the hole, so a thickened
    // cover would certify a 2-cell over empty space and kill a real `H₁`
    // class by fiat.
    //
    // WHAT CONTACT IS INSTEAD. Two adjacent bodies do touch, and that is
    // not a non-empty pixel intersection: under A.10 it is a co-dimensional
    // tether governed by the phase gradient `R = 1 − v_b v_a`. Contact
    // between disjoint regions cannot be manufactured by pretending their
    // coordinates overlap, and this reading now says so.
    let mut occ: crate::network::nerve::Keyed<Vec<coord::Carriage>> =
        crate::network::nerve::Keyed::new();
    let (placed_carriages, placed_refused) = placed.decoded();
    if !placed_refused.is_empty() {
        tally(|t| t.undecodable += placed_refused.len() as u64);
    }
    for (k, p) in &placed_carriages {
        for (side, support) in [(0u8, &p.domain), (1u8, &p.codomain)] {
            for s in support.iter() {
                let mut pos = vec![side];
                s.put(&mut pos);
                match occ.get_mut(&pos) {
                    // `support_of` dedups, and one piece's sites are offered
                    // contiguously — the walk above is per piece — so a
                    // repeat can only be the key just pushed (rule 15: the
                    // slice pattern reads the value this loop wrote).
                    Some(v) => match v.as_slice() {
                        [.., last] if last == k => {}
                        _ => v.push(k.clone()),
                    },
                    None => {
                        occ.insert(pos, vec![k.clone()]);
                    }
                }
            }
        }
    }
    // A meet already recorded is not recorded again, read at its own
    // address: the site needs no position of its own.
    for (_pos, here) in occ.scoped() {
        // ONE READ PER SITE THAT YIELDS A MEET, charged on the first meet
        // it yields (rule 10): a site that yields nothing is not work.
        // `paid` is this stroke's payment and not a reading of the site —
        // nothing in the cover branches on it.
        let mut paid = false;
        // A site's occupants are a SIMPLEX, not a clique: they all contain
        // this site, so every cycle among them bounds and k occupants are
        // k−1 meets. The cycles that carry a cover's holonomy span DISTINCT
        // sites and are untouched by this.
        for w in here.windows(2) {
            let [ka, kb] = w else { continue };
            'meet: {
                let key = coord::Meet::of(ka.clone(), kb.clone());
                if cover.meets.holds(&key) {
                    break 'meet;
                }
                if !paid {
                    if !budget.spend() {
                        park!();
                    }
                    paid = true;
                }
                // THE MEET IS RECORDED AND NOT READ (L23c). Two pieces
                // overlap; that is a 1-simplex of the nerve and nothing
                // more. The transition between them is the FACE of the two
                // vertices they become, and the class of a cycle of faces
                // is the form's orbit holonomy — both the form's to compute,
                // over the whole cycle rather than edge by edge here.
                cover.meets.put(&key, ());
            }
        }
    }
    // Exact, not saturating: a budget's remaining only falls, so the
    // opening read is never below the closing one (rule 11).
    let spent = (opened - budget.remaining()) as u64;
    tally(|t| {
        t.measured += 1;
        t.spend += spent;
        t.carried += cover.carried.scoped().count() as u64;
        t.meets += cover.meets.scoped().count() as u64;
        t.refused += cover.refused.scoped().count() as u64;
    });
    Ok(cover)
}
