// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.8, L11c — THE COVER OF A DEFORMATION: the same constructor as the
//! ring's, at the cover level.
//!
//! No single versor carries this body onto its image — that is what a
//! `TopologicalDeformation` says. The cover splits both into PIECES that
//! are each carried, and what is left is the transition between pieces
//! where they meet. A transition of finite order is a class the form
//! FILES as torsion; an infinite one is what the demand still stands on.
//!
//! Nothing is added to the tape. `Form.torsion` already carries a filled
//! cell as `FiledLoop { sigma, period, chord }` and the court already
//! replays it (23.1 step 4), so the cell coordinate the row asks for is
//! the one the form has had since 15.5.
//!
//! Nothing is ranked. Pieces come out in shell order and then key order,
//! and a piece is paired to a piece by the same carriage the whole-body
//! pairing uses; there is no nearest, no best and no first-found.

use super::*;
use crate::transducer::alphabet::Rotor;

/// The pieces of a body: its peel shells, and within each shell the
/// one-hue connected components (13.9's split, `sentence::pieces` over
/// 4-adjacency). Shell order, then the component order the keyed read
/// returns — an order by content, never a ranking (rule 4).
///
/// A body of one hue with no inner shell is one piece, and a cover of one
/// piece has no meet: nothing to fill, and the demand stands as it did.
pub(crate) fn pieces_of(
    cells: &[Cell3],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<Body> {
    let mut out = Vec::new();
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
        for (_, of_hue) in hues.fiber(crate::network::nerve::ROOT) {
            let chain: Vec<crate::transducer::sentence::Cell> =
                of_hue.iter().map(|&(r, c, h)| (r, c, h, h)).collect();
            for comp in crate::transducer::sentence::pieces(&chain) {
                let cells: Vec<Cell3> = comp.iter().map(|&(r, c, h, _)| (r, c, h)).collect();
                if cells.is_empty() {
                    continue;
                }
                out.push(body_of(cells));
            }
        }
    }
    out
}

/// One piece the cover placed: which piece it is, the versor that
/// carried it, and the support it landed on.
struct Placed {
    idx: usize,
    by: Versor,
    support: Vec<(i64, i64)>,
}

/// What the cover of one deformation reached. Every field is content:
/// the pieces carried and how, the classes their meets left, and the
/// cells no piece reached.
pub(crate) struct Cover {
    /// One `Word` vertex per carried piece, with the versor that carried
    /// it, in the order the pieces came out.
    pub(crate) carried: Vec<(Vertex, Versor)>,
    /// Filed classes: a meet whose transition has FINITE order is a
    /// torsion cycle of this form (15.5, A17), with the period it closes
    /// at. The chord is written by the caller, which owns the face
    /// indices.
    pub(crate) filed: Vec<(Rotor, u32)>,
    /// A meet whose transition has infinite order: nothing is filed and
    /// the demand stands on it (50.5). Held as the transition itself, so
    /// the caller can carry its class vector.
    pub(crate) standing: Vec<Rotor>,
    /// Cells of either body no piece carried. The demand dissolves only
    /// when both are empty (50.8).
    pub(crate) uncarried: usize,
}

/// 50.8 — COVER ONE DEFORMATION. Both bodies are split into pieces; each
/// input piece is paired to the output piece the SAME carriage rule the
/// whole-body pairing uses admits (`carriage`, exact or nothing); the
/// transition between two incident carried pieces is read by its own
/// rotor.
///
/// The four readings, and none of them is a choice:
///
/// - identity: the pieces agree on their overlap and there is nothing to
///   fill.
/// - no versor between two pieces: a remainder edge (13.9, 50.3). Not a
///   demand — the split is Mayer–Vietoris and no loop through that edge
///   is evaluated.
/// - finite order `ℓ`: the class is FILED as torsion at its period, and
///   the cell is filled.
/// - infinite order: the demand stands, carrying that class.
///
/// The loop over pieces spends the caller's `Budget` (rule 10): a cover
/// that ran out of budget carried fewer pieces, so cells stay uncarried
/// and the demand stands — which is the same answer an incomplete cover
/// gives for any other reason, and never a claim that it closed.
pub(crate) fn cover_deformation(
    input: &Body,
    image: &Body,
    scale: Option<((usize, usize), (usize, usize))>,
    budget: &mut crate::physics::Budget,
) -> Cover {
    let ins = pieces_of(&input.cells, scale);
    let outs = pieces_of(&image.cells, scale);
    let mut cover =
        Cover { carried: Vec::new(), filed: Vec::new(), standing: Vec::new(), uncarried: 0 };

    // Which cells any piece carried, so what is left is read from content
    // rather than counted down from a total.
    let mut reached: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // The pieces this cover placed, for the meets below.
    let mut placed: Vec<Placed> = Vec::new();

    // THE OUTPUT PIECES BY CHART, once. The read is a keyed fiber at the
    // input piece's chart and never a walk of the output pieces (rule 1:
    // no pair loop over ins × outs).
    //
    // The PEELED chart, as `group_by_chart` uses for the whole-body
    // pairing: a carriage between two pieces does not require their
    // moduli to be equal — `pair_rank` reads that equality as a rank and
    // not as a gate — so grouping on the exact chart would refuse pairs
    // the whole-body path admits.
    let mut by_chart: crate::network::nerve::Keyed<Vec<usize>> = crate::network::nerve::Keyed::new();
    for (j, op) in outs.iter().enumerate() {
        let k = op.moduli.peel().key();
        match by_chart.get_mut(&k) {
            Some(v) => v.push(j),
            None => {
                by_chart.insert(k, vec![j]);
            }
        }
    }
    let mut taken: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for ip in &ins {
        if !budget.spend() {
            break;
        }
        // The fiber of output pieces on this piece's own chart, in key
        // order. Carrying is exact, so the one that carries is read out
        // rather than chosen among candidates (rule 4).
        let Some(fiber) = by_chart.get(&ip.moduli.peel().key()) else {
            // L11d: NO OUTPUT PIECE SHARES THIS PIECE'S PEELED CHART. The
            // pairing never even reaches a carriage, so nothing about the
            // carriage's strictness is being measured here.
            eprintln!(
                "  cover piece no-fiber charts_out {} axes_of_piece {}",
                by_chart.fiber(crate::network::nerve::ROOT).count(),
                ip.moduli.0.len()
            );
            continue;
        };
        let mut carried_here = false;
        for &j in fiber {
            if taken.contains(&j.to_le_bytes()) {
                continue;
            }
            let Some(op) = outs.get(j) else {
                continue;
            };
            let Some(car) = carriage(ip, op) else {
                // L11d: WHY THIS PIECE DID NOT CARRY. `versor_between`
                // reads the shapes alone; `carriage` then reads the hue
                // map cell by cell. Which of the two refused is the whole
                // question, and it is written out rather than inferred.
                let shaped = crate::transducer::blade::versor_between(&ip.shape, &op.shape).is_some();
                eprintln!(
                    "  cover piece refused axes {:?} versor {} at {}",
                    crate::transducer::family::differing_axes(&ip.moduli, &op.moduli),
                    shaped,
                    match shaped {
                        true => "the hue map or the cell walk",
                        false => "the shapes",
                    }
                );
                continue;
            };
            let Some(Ok(v)) = crate::transducer::family::word_or_demand(ip, op) else {
                continue;
            };
            taken.insert(j.to_le_bytes(), ());
            for &(r, c, _) in &ip.cells {
                reached.insert(cell_pos_key(r, c), ());
            }
            for &(r, c, _) in &op.cells {
                reached.insert(cell_pos_key(r, c), ());
            }
            placed.push(Placed { idx: placed.len(), by: car.v.clone(), support: support_of(op) });
            cover.carried.push((v, car.v));
            carried_here = true;
            break;
        }
        if !carried_here {
            // No output piece on this piece's own chart carried it at
            // all: the fiber was empty, or every member refused above.
            eprintln!("  cover piece uncarried fiber {}", fiber.len());
        }
    }

    // THE MEETS, read from a keyed occupancy rather than from a pair loop
    // (rule 1): each placed piece's support and its ring are written into
    // one cell-keyed carrier, and a cell holding two or more pieces IS a
    // meet. The bound is the pieces' own cells — their boundary — and no
    // pair is ever formed that a cell did not name.
    let mut occ: crate::network::nerve::Keyed<Vec<usize>> = crate::network::nerve::Keyed::new();
    for p in &placed {
        let (idx, support) = (&p.idx, &p.support);
        let cells: Vec<Cell3> = support.iter().map(|&(r, c)| (r, c, 0)).collect();
        let ring = crate::transducer::blade::adjacent_ring(&cells);
        for (r, c) in support.iter().copied().chain(ring.into_iter().map(|(r, c, _)| (r, c))) {
            let k = cell_pos_key(r, c);
            match occ.get_mut(&k) {
                Some(v) => {
                    if !v.contains(idx) {
                        v.push(*idx);
                    }
                }
                None => {
                    occ.insert(k, vec![*idx]);
                }
            }
        }
    }
    let mut met: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for (_k, here) in occ.fiber(crate::network::nerve::ROOT) {
        // A cell's OWN occupants: rule 1 allows this walk, because the
        // cell is the boundary the pair is read out of.
        for a in 0..here.len() {
            for b in (a + 1)..here.len() {
                let key = pair_key(here[a] as u16, here[b] as u16);
                if met.contains(&key) {
                    continue;
                }
                met.insert(key, ());
                if !budget.spend() {
                    break;
                }
                let (Some(pa), Some(pb)) = (placed.get(here[a]), placed.get(here[b])) else {
                    continue;
                };
                let Some(inv) = pa.by.versor_inverse() else {
                    // No versor between them: a remainder edge, and no
                    // loop through it is evaluated (13.9 step 7).
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
                        cover.filed.push((g, ell));
                    }
                    Some(crate::transducer::alphabet::Order::Infinite) => cover.standing.push(g),
                    // Not a versor: no class (15.3), nothing filed and
                    // nothing claimed.
                    None => {}
                }
            }
        }
    }

    for &(r, c, _) in input.cells.iter().chain(image.cells.iter()) {
        if !reached.contains(&cell_pos_key(r, c)) {
            cover.uncarried += 1;
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
