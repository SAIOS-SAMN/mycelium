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

use super::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::peel::ShellKind;

/// A piece of a body: one shell, one hue, one connected component.
pub(crate) struct Piece {
    pub(crate) kind: ShellKind,
    pub(crate) hue: u8,
    pub(crate) body: Body,
}

/// The address of a piece, as content (A21): the shell kind, the hue, and
/// the support written out, length-prefixed. No digest — a hash here
/// would key cognition by bytes and lose the coordinates the nerve reads
/// (rule 2, rule 6).
pub(crate) fn piece_key(kind: ShellKind, hue: u8, support: &[(i64, i64)]) -> Vec<u8> {
    let mut k = vec![kind.as_u8(), hue];
    k.extend_from_slice(&(support.len() as u32).to_le_bytes());
    for &(r, c) in support {
        k.extend_from_slice(&r.to_le_bytes());
        k.extend_from_slice(&c.to_le_bytes());
    }
    k
}

/// The address of a preimage: the pieces it is assembled from, written
/// out length-prefixed in key order. A preimage of ONE piece is that
/// piece and keeps its own address, so the pullback and the structural
/// pass name the same cover piece the same way. Content, not a digest and
/// not an arrival index (rule 2, rule 6, rule 15).
fn preimage_key(parts: &[Vec<u8>]) -> Vec<u8> {
    match parts {
        [only] => only.clone(),
        many => {
            let mut k = Vec::new();
            for p in many {
                k.extend_from_slice(&(p.len() as u32).to_le_bytes());
                k.extend_from_slice(p);
            }
            k
        }
    }
}

/// The unordered pair of two piece addresses: the 1-simplex of the nerve,
/// named by its two ends and never by their positions.
fn meet_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let (lo, hi) = match a <= b {
        true => (a, b),
        false => (b, a),
    };
    let mut k = Vec::with_capacity(lo.len() + hi.len() + 8);
    k.extend_from_slice(&(lo.len() as u32).to_le_bytes());
    k.extend_from_slice(lo);
    k.extend_from_slice(&(hi.len() as u32).to_le_bytes());
    k.extend_from_slice(hi);
    k
}

/// The pieces of a body: its peel shells, and within each shell the
/// one-hue connected components (13.9's split, `sentence::pieces` over
/// 4-adjacency), filed by their own address.
pub(crate) fn pieces_of(
    cells: &[Cell3],
    scale: Option<((usize, usize), (usize, usize))>,
) -> crate::network::nerve::Keyed<Piece> {
    let mut out: crate::network::nerve::Keyed<Piece> = crate::network::nerve::Keyed::new();
    for shell in crate::transducer::peel::peel_body_scaled(cells, scale) {
        // One hue at a time, so a component is a piece of ONE colour: a
        // seam between two hues is a meet, not a body (13.9).
        let mut hues: crate::network::nerve::Keyed<Vec<Cell3>> = crate::network::nerve::Keyed::new();
        for &(r, c, h) in &shell.cells {
            match hues.get_mut(&[h]) {
                Some(v) => v.push((r, c, h)),
                None => {
                    hues.insert([h], vec![(r, c, h)]);
                }
            }
        }
        for (hue_key, of_hue) in hues.fiber(crate::network::nerve::ROOT) {
            // The hue is the KEY this group was filed under, not a cell
            // read out of it by position (rule 15): every cell here has
            // that hue by construction.
            let [hue] = hue_key.as_slice() else { continue };
            let hue = *hue;
            let chain: Vec<crate::transducer::sentence::Cell> =
                of_hue.iter().map(|&(r, c, h)| (r, c, h, h)).collect();
            for comp in crate::transducer::sentence::pieces(&chain) {
                let cells: Vec<Cell3> = comp.iter().map(|&(r, c, h, _)| (r, c, h)).collect();
                if cells.is_empty() {
                    continue;
                }
                let body = body_of(cells);
                let k = piece_key(shell.kind, hue, &support_of(&body));
                out.insert(k, Piece { kind: shell.kind, hue, body });
            }
        }
    }
    out
}

/// One piece the cover placed: the versor that carried it and the support
/// it landed on.
struct Placed {
    by: Versor,
    support: Vec<(i64, i64)>,
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
    /// A meet whose transition has INFINITE order: nothing is filed and
    /// the demand stands on it (50.5), by meet address.
    pub(crate) standing: crate::network::nerve::Keyed<Rotor>,
    /// What no piece carried, as a VOLUME on four lanes (law 5): the
    /// input's cells no piece carried and the image's cells no piece
    /// reached (50.8 case 4). A count of cells is not this quantity —
    /// cells carry different lane weight — so the dissolution reads the
    /// zero vector and never a cardinality.
    pub(crate) uncarried: [u64; 4],
    /// Whether the cover ran to the end of its own pieces. A budget that
    /// ended leaves this false, and an unmeasured cover dissolves
    /// nothing: it is not a smaller cover (rule 11).
    pub(crate) measured: bool,
}

impl Cover {
    fn new() -> Cover {
        Cover {
            carried: crate::network::nerve::Keyed::new(),
            filed: crate::network::nerve::Keyed::new(),
            standing: crate::network::nerve::Keyed::new(),
            uncarried: [0; 4],
            measured: true,
        }
    }

    /// The deformation is answered when every cell of both bodies is
    /// carried and no meet of infinite order stands. A finite-order
    /// holonomy is NOT an obstruction: it is filed as torsion and the
    /// sentence carries the class as its theorem (15.5), so a demand kept
    /// alive on it would be a demand for what the form already says.
    pub(crate) fn dissolves(&self) -> bool {
        self.measured
            && self.uncarried == [0; 4]
            && self.standing.fiber(crate::network::nerve::ROOT).next().is_none()
    }
}

/// 50.8 — COVER ONE DEFORMATION.
///
/// Pass (a), THE PULLBACK: an output piece's preimage is the input pieces
/// standing under its own cells, read per cell from the occupancy. Pass
/// (b), BY STRUCTURE: an input piece still unplaced is looked for among
/// the output pieces of its own shell kind and hue — never its moduli,
/// which a deformed piece changes by definition. In both, `carriage`
/// decides and nothing is ranked (rule 4).
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
) -> Cover {
    use crate::network::nerve::ROOT;
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
    let mut taken: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut reached: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let out_keys: Vec<Vec<u8>> = outs.fiber(ROOT).map(|(k, _)| k).collect();

    // (a) THE PULLBACK, in place.
    for ko in &out_keys {
        if taken.contains(ko) {
            continue;
        }
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
            if consumed.contains(&ki) {
                continue;
            }
            if !budget.spend() {
                cover.measured = false;
                return cover;
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
        let Some(Ok(v)) = word_or_demand(&pre, &op.body) else { continue };
        let key = preimage_key(&parts);
        taken.insert(ko, ());
        for &(r, c, _) in pre.cells.iter().chain(op.body.cells.iter()) {
            reached.insert(cell_pos_key(r, c), ());
        }
        for ki in &parts {
            consumed.insert(ki, ());
        }
        placed.insert(&key, Placed { by: car.v.clone(), support: support_of(&op.body) });
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
        for ko in fiber {
            if taken.contains(&ko) {
                continue;
            }
            if !budget.spend() {
                cover.measured = false;
                return cover;
            }
            let Some(op) = outs.get(&ko) else { continue };
            let Some(car) = carriage(&ip.body, &op.body) else { continue };
            let Some(Ok(v)) = word_or_demand(&ip.body, &op.body) else { continue };
            taken.insert(&ko, ());
            for &(r, c, _) in ip.body.cells.iter().chain(op.body.cells.iter()) {
                reached.insert(cell_pos_key(r, c), ());
            }
            consumed.insert(ki, ());
            placed.insert(ki, Placed { by: car.v.clone(), support: support_of(&op.body) });
            cover.carried.insert(ki, (v, car.v));
            break;
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
    let mut occ: crate::network::nerve::Keyed<Vec<Vec<u8>>> = crate::network::nerve::Keyed::new();
    for (k, p) in placed.fiber(ROOT) {
        let cells: Vec<Cell3> = p.support.iter().map(|&(r, c)| (r, c, 0)).collect();
        let ring = crate::transducer::blade::adjacent_ring(&cells);
        for (r, c) in p.support.iter().copied().chain(ring.into_iter().map(|(r, c, _)| (r, c))) {
            let pos = cell_pos_key(r, c);
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
                    cover.measured = false;
                    return cover;
                }
                let (Some(pa), Some(pb)) = (placed.get(&here[a]), placed.get(&here[b])) else {
                    continue;
                };
                let Some(inv) = pa.by.versor_inverse() else {
                    // No versor: a remainder edge (13.9 step 7).
                    continue;
                };
                let gamma = pb.by.geo(&inv);
                let Some(block) = crate::transducer::family::linear_block_of_versor(&gamma) else {
                    continue;
                };
                let Some(g) = rotor_of_transition(&block) else {
                    continue;
                };
                if g.is_identity() {
                    continue;
                }
                match g.order() {
                    Some(crate::transducer::alphabet::Order::Finite(ell)) => {
                        cover.filed.insert(&key, (g, ell));
                    }
                    Some(crate::transducer::alphabet::Order::Infinite) => {
                        cover.standing.insert(&key, g);
                    }
                    // Not a versor: no class (15.3), nothing filed.
                    None => {}
                }
            }
        }
    }
    cover
}

/// The rotor of a transition's plane block, where the seventeen spell it.
/// A transition the plane does not close has no rotor here and files
/// nothing.
fn rotor_of_transition(block: &[[crate::core::Q; 2]; 2]) -> Option<Rotor> {
    use crate::transducer::alphabet::{spell_peel_on, PLANE_COL, PLANE_ROW};
    use crate::transducer::peel::PeelNest;
    let plane = (PLANE_ROW, PLANE_COL);
    match spell_peel_on(plane, block)? {
        (PeelNest::Closed { cores }, _) => {
            let mut w = crate::transducer::alphabet::Word::identity();
            for syllable in cores.iter().rev() {
                for &(id, inv) in syllable {
                    w.push(id, inv);
                }
            }
            Some(w.rotor())
        }
        _ => None,
    }
}
