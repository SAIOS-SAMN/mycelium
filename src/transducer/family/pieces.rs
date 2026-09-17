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
///
/// The kind and the hue are ON ITS ADDRESS and are not carried beside it
/// (law 12): the cover's second pass propagates the placed versors and
/// reads the image by position, so nothing asks a piece what kind it is.
pub(crate) struct Piece {
    pub(crate) body: Body,
}

/// The address of a piece, as content (A21): the shell kind, the hue, and
/// the support on the three coordinates of the base written out,
/// length-prefixed.
pub(crate) fn piece_key(kind: ShellKind, hue: u8, support: &[(i64, i64, i64)]) -> Vec<u8> {
    let mut k = vec![kind.as_u8(), hue];
    k.extend_from_slice(&(support.len() as u32).to_le_bytes());
    for &(r, c, f) in support {
        k.extend_from_slice(&r.to_le_bytes());
        k.extend_from_slice(&c.to_le_bytes());
        k.extend_from_slice(&f.to_le_bytes());
    }
    k
}

/// THE ADDRESS OF AN ASSEMBLED SET OF PIECES: the pieces it is assembled
/// from, written out length-prefixed in key order. A set of ONE piece is
/// that piece and keeps its own address, so a carriage onto a whole piece
/// is named the same way whichever pass placed it.
///
/// Both passes assemble, and in opposite directions: the pullback takes
/// Φ⁻¹(V) over the input pieces under an output piece's sites, and the
/// propagation takes Φ(U) over the output pieces under a carried piece's
/// image. Neither requires the two peels to agree on granularity, which
/// is why each is assembled whole and carried once (50.8).
pub(crate) fn assembly_key(parts: &[Vec<u8>]) -> Vec<u8> {
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
/// many domains reaching one codomain — merging. Under either injectivity
/// condition a family with one input piece and ten output pieces places
/// exactly ONE carriage, the other nine refused by bookkeeping and not by
/// geometry, and with one piece placed there is no second node in the
/// nerve and no transition to have an order at all.
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

/// The two ends of a meet, read back out of its own address. The key is
/// the pair, so the nerve stores nothing beside it (law 12).
pub(crate) fn meet_ends(k: &[u8]) -> Option<(&[u8], &[u8])> {
    let (n, rest) = k.split_at_checked(4)?;
    let lo_len = u32::from_le_bytes(n.try_into().ok()?) as usize;
    let (lo, rest) = rest.split_at_checked(lo_len)?;
    let (m, rest) = rest.split_at_checked(4)?;
    let hi_len = u32::from_le_bytes(m.try_into().ok()?) as usize;
    let (hi, tail) = rest.split_at_checked(hi_len)?;
    tail.is_empty().then_some((lo, hi))
}

/// The pieces of a body: its peel shells, and within each shell the
/// one-hue connected components (13.9's split, `blade::components` on the
/// three coordinates of the base), filed by their own address.
pub(crate) fn pieces_of(
    laminae: &[Lamina],
    scale: Option<((usize, usize), (usize, usize))>,
) -> crate::network::nerve::Keyed<Piece> {
    let mut out: crate::network::nerve::Keyed<Piece> = crate::network::nerve::Keyed::new();
    for shell in crate::transducer::peel::peel_body_scaled(laminae, scale) {
        // One hue at a time, so a component is a piece of ONE colour: a
        // seam between two hues is a meet, not a body (13.9).
        let mut hues: crate::network::nerve::Keyed<Vec<Lamina>> = crate::network::nerve::Keyed::new();
        for &(r, c, k, h) in &shell.laminae {
            match hues.get_mut(&[h]) {
                Some(v) => v.push((r, c, k, h)),
                None => {
                    hues.insert([h], vec![(r, c, k, h)]);
                }
            }
        }
        for (hue_key, of_hue) in hues.scoped() {
            // The hue is the KEY this group was filed under, not a lamina
            // read out of it by position (rule 15): every lamina here has
            // that hue by construction.
            let [hue] = hue_key.as_slice() else { continue };
            let hue = *hue;
            for comp in crate::transducer::blade::components(of_hue) {
                if comp.is_empty() {
                    continue;
                }
                let body = body_of(comp);
                let k = piece_key(shell.kind, hue, &support_of(&body));
                out.insert(k, Piece { body });
            }
        }
    }
    out
}
