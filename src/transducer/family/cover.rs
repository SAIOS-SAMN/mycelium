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

/// THE ADDRESS OF A COVER PIECE, which is a CARRIAGE and not a piece of a
/// body: the ordered pair of the domain's address in the input and the
/// codomain's address in the image. Ordered, unlike a meet, because a
/// carriage has a direction.
///
/// Keying here by the domain alone would assert that Φ is injective on
/// the input, and keying by the codomain alone that it is injective on
/// the image. Neither holds. A shape occurring once in the input and ten
/// times in the image is one domain carried by ten versors — replication —
/// and a shape occurring many times in the input and once in the image is
/// many domains reaching one codomain — merging. Measured at a35bb9d with
/// both injectivity conditions enforced: sixteen covers over twelve
/// tasks, `filed 0 standing 0` in every one, and on a family with one
/// input piece and ten output pieces exactly ONE carriage was placed. The
/// nine refused were refused by bookkeeping, not by geometry, and with
/// only one piece placed there is no second node in the nerve, so no
/// transition exists to have an order at all.
fn carriage_key(domain: &[u8], codomain: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(domain.len() + codomain.len() + 8);
    k.extend_from_slice(&(domain.len() as u32).to_le_bytes());
    k.extend_from_slice(domain);
    k.extend_from_slice(&(codomain.len() as u32).to_le_bytes());
    k.extend_from_slice(codomain);
    k
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
    by: Versor,
    /// The hue map this carriage read cell by cell, which the transition
    /// at a meet composes against its neighbour's.
    sigma: Vec<(u8, u8)>,
    domain: Vec<(i64, i64)>,
    codomain: Vec<(i64, i64)>,
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
        let Some(Ok(v)) = word_or_demand(&pre, &op.body) else { continue };
        let key = carriage_key(&preimage_key(&parts), ko);
        for &(r, c, _) in pre.cells.iter().chain(op.body.cells.iter()) {
            reached.insert(cell_pos_key(r, c), ());
        }
        for ki in &parts {
            consumed.insert(ki, ());
        }
        placed.insert(
            &key,
            Placed {
                by: car.v.clone(),
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
                return Err(Unmeasured);
            }
            let Some(op) = outs.get(&ko) else { continue };
            let Some(car) = carriage(&ip.body, &op.body) else { continue };
            let Some(Ok(v)) = word_or_demand(&ip.body, &op.body) else { continue };
            let key = carriage_key(ki, &ko);
            for &(r, c, _) in ip.body.cells.iter().chain(op.body.cells.iter()) {
                reached.insert(cell_pos_key(r, c), ());
            }
            consumed.insert(ki, ());
            placed.insert(
                &key,
                Placed {
                    by: car.v.clone(),
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
                    return Err(Unmeasured);
                }
                let (Some(pa), Some(pb)) = (placed.get(&here[a]), placed.get(&here[b])) else {
                    continue;
                };
                let Some(inv) = pa.by.versor_inverse() else {
                    // No versor: a remainder edge (13.9 step 7).
                    continue;
                };
                let gamma = pb.by.geo(&inv);
                // THE GAUGE AROUND THE SEAM, composed and not assumed:
                // σ_a takes the input's hues to a's, σ_b to b's, so the
                // transition's own hue map is σ_b ∘ σ_a⁻¹. Passing the
                // identity here would claim the seam permutes no hue
                // without having composed anything (rule 11).
                let hue = crate::transducer::sentence::CycleType::of(&hue_transition(
                    &pa.sigma, &pb.sigma,
                ));
                // CLASSIFIED ON THE VERSOR (12.6). A `None` is a versor
                // these readers cannot factor, which is unmeasured and
                // neither flat nor standing.
                match crate::transducer::element::meet_class(&gamma, hue) {
                    Some(crate::transducer::element::MeetClass::Flat) => {}
                    Some(crate::transducer::element::MeetClass::Finite { ell, rotor }) => {
                        cover.filed.insert(&key, (*rotor, ell));
                    }
                    Some(crate::transducer::element::MeetClass::Standing { kappa }) => {
                        cover.standing.insert(&key, *kappa);
                    }
                    // 37.7 AT A SEAM. Kept apart from `standing` because
                    // it is a different question: a standing class is one
                    // the alphabet names and no held generator closes, and
                    // this is one the alphabet does not name at all. It is
                    // an adjunction candidate, and Vent 1 measures how many
                    // the cover finds.
                    Some(crate::transducer::element::MeetClass::Unspellable { rotor, .. }) => {
                        cover.unspellable.insert(&key, *rotor);
                    }
                    None => {}
                }
            }
        }
    }
    Ok(cover)
}

// `rotor_of_transition` was here. It classified a meet's transition by the
// transition's sensor-plane block alone, which reads a translation or a
// dilation as the identity and mis-orders a rotation about any point but
// the origin. `element::meet_class` replaces it, on the versor.

/// σ_b ∘ σ_a⁻¹ — the hue map of the transition between two carriages, as
/// a listing. `a` takes the input's hues to a's image and `b` to b's, so
/// crossing the seam is a's map undone and b's applied. A hue `a` does not
/// list is one `a` did not move, so it enters `b` as itself.
fn hue_transition(a: &[(u8, u8)], b: &[(u8, u8)]) -> Vec<(u8, u8)> {
    use crate::network::nerve::{Keyed, ROOT};
    let mut back: Keyed<u8> = Keyed::new();
    for &(h, t) in a {
        back.insert([t], h);
    }
    let mut forward: Keyed<u8> = Keyed::new();
    for &(h, t) in b {
        forward.insert([h], t);
    }
    let mut out: Keyed<u8> = Keyed::new();
    for &(_, t) in a {
        let Some(source) = back.get(&[t]).copied() else { continue };
        // A hue `b` does not list is one `b` leaves alone, which is the
        // listing's own convention and not a default standing in for a
        // missing value: `cycle_lengths` reads absence as a fixed point.
        let image = match forward.get(&[source]) {
            Some(h) => *h,
            None => source,
        };
        out.insert([t], image);
    }
    let mut listing: Vec<(u8, u8)> = out
        .fiber(ROOT)
        .filter_map(|(k, v)| match k.as_slice() {
            [h] => Some((*h, *v)),
            _ => None,
        })
        .collect();
    listing.sort_unstable();
    listing
}
