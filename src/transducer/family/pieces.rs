// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.8, 13.9 — THE PIECES OF A BODY AND THE ADDRESSES OF THE COVER: how a
//! body decomposes, and how a piece, a preimage, a carriage and a meet are
//! each named by their own content.
//!
//! Nothing here is keyed by position. A piece is addressed by its shell
//! kind and its support written out; a preimage by the pieces it
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

/// A piece of a body: one shell, one HUE-BLIND connected component (70.4).
/// Its hues are the gauge its laminae carry, read off them where a reader
/// needs them; a seam between two hues inside a piece is that piece's own
/// gauge section, never a meet.
///
/// The kind is ON ITS ADDRESS and is not carried beside it (law 12): the
/// cover's second pass propagates the placed versors and reads the image
/// by position, so nothing asks a piece what kind it is.
pub(crate) struct Piece {
    pub(crate) body: Body,
}

/// THE ADDRESSES ARE COORDINATES NOW (§4.4 E8, ruled 2026-09-22). A piece,
/// a carriage and a meet each decode back to what they name, and the
/// carriers holding them take the coordinate rather than loose bytes.
///
/// The three functions that wrote these keys by hand are gone. What they
/// wrote is `coord::Piece::encode`, `coord::Carriage::encode` and
/// `coord::Meet::encode`, and what `meet_ends` read back out is
/// `coord::Meet::ends`. There is one producer of each layout (law 12) and
/// it is the one the trie decodes through.
///
/// ONE DEFECT SURFACED IN THE DECLARING. `assembly_key` wrote a ONE-PIECE
/// assembly as that piece's bare address and a many-piece assembly as a run
/// of length-prefixed ones, so the bytes did not say which they were: the
/// collapse existed only so that the pullback (which assembles its domain
/// and names a single output piece) and the propagation (which names a
/// single input piece and assembles its image) would agree on the address
/// of a carriage between two whole pieces. A [`coord::Carriage`] carries a
/// LIST on both sides and the two passes agree because both sides are
/// lists, not because one collapses into the other's shape.
///
/// [`coord::Piece`]: crate::network::nerve::coord::Piece
/// [`coord::Carriage`]: crate::network::nerve::coord::Carriage
/// [`coord::Meet`]: crate::network::nerve::coord::Meet
pub(crate) fn piece_at(
    kind: ShellKind,
    support: &[crate::transducer::base::Site],
) -> crate::network::nerve::coord::Piece {
    crate::network::nerve::coord::Piece { kind, support: support.to_vec() }
}

/// THE PIECES OF A BODY ARE BODIES: its hue-blind connected components
/// (`blade::components` on the three coordinates of the base, which is
/// 51.3's 6-adjacency whatever hues the laminae carry, 70.4), filed by
/// their own address.
///
/// THE COVER DOES NOT DESCEND BELOW A BODY (51.3, 50.8 as ruled
/// 2026-09-18). This peeled the body into concentric shells first and took
/// the components of each shell, so a body no single versor carried whole
/// became one word vertex per shell component rather than one demand: 1 655
/// vertices, 11 762 faces and 10 118 orbits on one 22×22 family of epoch 65.
/// Two readings followed from that shape and neither was a separate defect.
/// The form's address is its vertex key, so 1 655 pieces made it 9 990 312
/// bytes, and every index naming the idea copied it (R6). And a shell
/// component is carried by a translation, whose rotor is the identity and
/// which occupies no coordinate of the class vector, so `admit_words` filed
/// those vertices on no axis and the pivot was offered nothing —
/// `crystal.on_axis` answered 0 on 123 of 123 reads.
///
/// What no single versor carries whole is one demand of the kind its
/// reading types (50.5), declared from `Cover::uncarried` in `extract.rs`,
/// never a listing of pieces. `ShellKind::Whole` is the kind of a body read
/// without a peel — the value, not a placeholder for one.
pub(crate) fn pieces_of(
    laminae: &[Lamina],
) -> crate::network::nerve::Keyed<Piece, crate::network::nerve::coord::Piece> {
    let mut out: crate::network::nerve::Keyed<Piece, crate::network::nerve::coord::Piece> =
        crate::network::nerve::Keyed::new();
    // WHATEVER HUES THE LAMINAE CARRY (70.4): the component is taken over
    // the body's whole support, so a polychrome body is one piece and its
    // colour seams stay inside it as its gauge section.
    for comp in crate::transducer::blade::components(laminae) {
        if comp.is_empty() {
            continue;
        }
        // A component whose moduli are not read at its `n` is no piece
        // (38.2, BQ4); the perceivers count such a body where it arises.
        let Some(body) = body_of(comp) else { continue };
        let k = piece_at(ShellKind::Whole, &support_of(&body));
        out.put(&k, Piece { body });
    }
    out
}
