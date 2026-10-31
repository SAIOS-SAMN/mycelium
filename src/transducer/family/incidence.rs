// 50.3, 52.5 — THE INCIDENCE: what a face between two members measures.
//
//! A member's fibre is one body along `e₃`, and the quotient of its union
//! shapes is what a face above it carries; the frames it stands on are read
//! at its own coordinate, and the per-frame readings are the section. What
//! a face was offered and what became of it is counted here and nowhere
//! else (§8.1).

use super::perceive::{edge_transport, EdgeTally};
use super::*;
use crate::network::nerve::Keyed;
use crate::network::nerve::coord;
use crate::transducer::sentence::{section_of_frames, Transport};

/// Two transports are one edge: the word projectively, the carriage as a
/// versor, the gauge as a listing (rule 3).
fn same_transport(a: &Transport, b: &Transport) -> bool {
    a.word.rotor().projective_eq(&b.word.rotor())
        && element::same_versor(&a.carriage, &b.carriage)
        && a.gauge == b.gauge
}

/// THE QUOTIENT OF A FIBRE'S UNION SHAPES (52.5): every compartment the member
/// stands on across its occupied frames, read at its own `(r, c, k)` and
/// never at frame zero, with the versor that carries the one union onto
/// the other — VERIFIED here, on the union, compartment by compartment.
///
/// A quotient measured at ONE frame is a motion of that frame's slice: its
/// bodies all sit at a single `k`, so `versor_between` has no `e₃` extent
/// to read. The union has that extent by construction (51.3: the family is
/// one body on `(r, c, k)`), and it is what the face above this member is
/// measured on.
///
/// THE VERSOR IS NOT SEARCHED FOR AGAIN. The member's frames are exactly
/// the frames that read `v` — that is what makes them one member, and the
/// grouping upstream has already decided it. `v` carries no `e₃`
/// component, so the frame it acts at rides along and `v` carries `a_k`
/// onto `b_k` at every `k` the member stands on, hence the union onto the
/// union. Calling `carriage` here would run `versor_between` over the whole
/// fibre's moments to re-derive a versor already in hand — measured on the
/// 60-task slice at 4 000 000 stroke: 32.5 s to 38.9 s, +19% wall clock for
/// a reading identical in every count.
///
/// It is verified rather than asserted (rule 11, law 12): every compartment of
/// the union is carried and its image read, which is `carriage`'s own check
/// without `carriage`'s search, and a fibre the versor does not carry
/// whole has no union quotient. `None` also where either side is empty.
pub(crate) fn union_quotient(
    mut ins: Vec<Compartment>,
    mut outs: Vec<Compartment>,
    v: &Versor,
) -> Option<(Body, Body, Versor)> {
    if ins.is_empty() || outs.is_empty() {
        return None;
    }
    ins.sort_unstable();
    ins.dedup();
    outs.sort_unstable();
    outs.dedup();
    let a = body_of(ins)?;
    let b = body_of(outs)?;
    for l in &a.compartments {
        super::perceive::landed_hue(v, &l.site, &b)?;
    }
    Some((a, b, v.clone()))
}

/// THE FRAMES A MEMBER OCCUPIES, read at its own coordinate (52.5 reading
/// 6). `bodies_at` is keyed `member ‖ frame`, so the frames this member
/// stands on are the fibre under its own member prefix and the remaining key
/// IS the frame, by its own spelling (AT1 Q4) — whose byte order is the
/// frames' order, so the trie's walk is the succession along `e₃` and nothing
/// sorts.
///
/// Re-derived from the carrier that already holds the bodies, never stored
/// beside them (law 12).
pub(crate) fn occupied_frames(bodies_at: &Keyed<(Body, Body, Versor)>, v: &coord::VertexAt, ck: &[u8]) -> Vec<u64> {
    let prefix = member_key(v, ck);
    let Some(fibre) = bodies_at.fibre_at(&prefix) else {
        return Vec::new();
    };
    fibre
        .filter_map(|(k, _)| crate::transducer::sentence::frame_of_key(k.get(prefix.len()..)?))
        .collect()
}

/// A member's input body, output body and the versor carrying one onto the
/// other.
type BodiesCarried = (Body, Body, Versor);

/// THE EDGE BETWEEN TWO MEMBERS (50.3), MEASURED ON THE MEMBERS' UNION SHAPES
/// (52.5), and the frames on which the per-frame reading re-derives one
/// edge — the face's section along `e₃`.
///
/// THE FACE IS ON THE UNION SHAPES, not on a slice. The edge was measured
/// on the two members' bodies AT THE LEAST FRAME BOTH STAND ON: every
/// compartment of that reading sits at one `k`, so `edge_transport`'s union
/// `(a_i ∪ a_j)` has no `e₃` extent and `versor_between` on it cannot
/// return a transport with `(r,k)` or `(c,k)` content whatever the family
/// does along the frame axis. That is the shape of the flatness measured
/// at epoch 70 — 292 of 292 orbits flat — and no amount of classifying
/// downstream recovers a bivector the reading never had. The union of a
/// member's fibre has the extent by construction (51.3: the family is one
/// body on `(r, c, k)`), and it is what the face carries here.
///
/// The per-frame readings below stay, and are the SECTION: the frames on
/// which one edge re-derives. They are compared with each other and not
/// with the union, because a union edge is placed in the union's own
/// coordinates and a frame's is placed in that frame's — asking a slice to
/// equal the whole would report every face as claiming nothing.
///
/// A pair whose fibres have no union between them — a cover piece stands
/// at one frame and has none — falls to the per-frame edge, and a pair
/// with no common frame and no union carries none.
pub(crate) fn edge_between(
    (bodies_at, unions, motions): (&Keyed<BodiesCarried>, &Keyed<BodiesCarried>, &Keyed<Versor>),
    n_reached: usize,
    (i, cik): (&coord::VertexAt, &[u8]),
    (j, cjk): (&coord::VertexAt, &[u8]),
    tally: &mut EdgeTally,
    budget: &mut crate::physics::Budget,
) -> (Option<Transport>, Option<Keyed<()>>) {
    let at = |v: &coord::VertexAt, ck: &[u8], k: u64| -> Option<&(Body, Body, Versor)> {
        let mut bk = member_key(v, ck);
        bk.extend_from_slice(&crate::transducer::sentence::frame_key(k));
        bodies_at.get(&bk)
    };
    let (union_edge, union_held) = match (unions.get(&member_key(i, cik)), unions.get(&member_key(j, cjk))) {
        (Some((a_i, b_i, v_i)), Some((a_j, b_j, v_j))) => {
            (tally.take_carried(edge_transport((v_i, v_j), a_i, a_j, b_i, b_j, budget)), true)
        }
        _ => (None, false),
    };
    let mut edge: Option<Transport> = None;
    let mut agreeing: Vec<u64> = Vec::new();
    let mut common = 0usize;
    // SUCCESSION ALONG e₃ IS THE NEXT FRAME OCCUPIED, never `k + 1` (52.5
    // reading 6, ruled 2026-09-18). This read `for k in 0..n`, probing every
    // integer up to the family's pair count and `continue`-ing on each miss:
    // a linear scan over a range, where the frames a member actually stands
    // on are named by its own fibre of `bodies_at`. The occupied set may
    // skip, repeat or reorder in ℤ, and the integers between it are not
    // coordinates of anything.
    for k in occupied_frames(bodies_at, i, cik) {
        let (Some((a_i, b_i, v_i)), Some((a_j, b_j, v_j))) = (at(i, cik, k), at(j, cjk, k)) else {
            continue;
        };
        common += 1;
        let measured = edge_transport((v_i, v_j), a_i, a_j, b_i, b_j, budget);
        match &edge {
            None => {
                edge = tally.take_carried(measured);
                if edge.is_some() {
                    agreeing.push(k);
                }
            }
            Some(e) => {
                if measured.as_ref().is_ok_and(|(t, _)| same_transport(t, e)) {
                    agreeing.push(k);
                }
            }
        }
    }
    let pairs = match (&edge, common) {
        (Some(_), _) if agreeing.len() < n_reached => Some(section_of_frames(agreeing)),
        _ => None,
    };
    // 78.103 — A PAIR WITH NO BODIES TO MEASURE (no union, no frame both
    // stand on — a cover's piece holds its motion and no body) is related by
    // the two motions alone, `ṽᵢ·vⱼ`. Only a member with no motion filed has
    // nothing to read an edge from: counted, never silent.
    if !union_held && common == 0 {
        return match (motions.get(&member_key(i, cik)), motions.get(&member_key(j, cjk))) {
            (Some(v_i), Some(v_j)) => (tally.take_carried(super::perceive::relative_edge(v_i, v_j)), None),
            // A star face's transport is its placement (the circuit reads
            // the member's own carriage there), never an edge asked here.
            _ => (None, None),
        };
    }
    // The union edge is the face's where the two fibres have one; the
    // per-frame edge stands where they do not.
    (union_edge.or(edge), pairs)
}