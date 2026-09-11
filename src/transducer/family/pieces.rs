// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.8, 13.9 — THE PIECES OF A BODY AND THE ADDRESSES OF THE COVER: how a
//! body decomposes, and how a piece, a preimage, a carriage and a meet are
//! each named by their own content.
//!
//! Nothing here is keyed by position. A piece is addressed by its shell
//! kind, its hue and its support written out; a preimage by the pieces it
//! is assembled from; a carriage by its domain and codomain in that order,
//! because a carriage has a direction; a meet by its two ends unordered,
//! because a seam does not. No digest appears in any of them — a hash
//! would key cognition by bytes and lose the coordinates the nerve reads
//! (rule 2, rule 6, rule 15).
//!
//! Split from `cover.rs` at the §1.1 600-line reading bound; the items are
//! moved, not rewritten (§1.6).

use super::*;
use crate::transducer::peel::ShellKind;

/// A piece of a body: one shell, one hue, one connected component.
pub(crate) struct Piece {
    pub(crate) kind: ShellKind,
    pub(crate) hue: u8,
    pub(crate) body: Body,
}

/// The address of a piece, as content (A21): the shell kind, the hue, and
/// the support written out, length-prefixed.
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
/// pass name the same cover piece the same way.
pub(crate) fn preimage_key(parts: &[Vec<u8>]) -> Vec<u8> {
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
pub(crate) fn carriage_key(domain: &[u8], codomain: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(domain.len() + codomain.len() + 8);
    k.extend_from_slice(&(domain.len() as u32).to_le_bytes());
    k.extend_from_slice(domain);
    k.extend_from_slice(&(codomain.len() as u32).to_le_bytes());
    k.extend_from_slice(codomain);
    k
}

/// The unordered pair of two piece addresses: the 1-simplex of the nerve,
/// named by its two ends and never by their positions.
pub(crate) fn meet_key(a: &[u8], b: &[u8]) -> Vec<u8> {
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

// `hue_transition` was here: σ_b ∘ σ_a⁻¹, the hue map of the transition
// between two carriages. The cover composed it to classify a meet; the
// cover classifies nothing now (L23c), and the cycle type of a LOOP is
// read off the loop's own vertices where the form classifies it.
