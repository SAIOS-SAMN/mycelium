// 50.3, 52.5 — THE INCIDENCE: what a face between two members measures.
//
//! A member's fiber is one body along `e₃`, and the quotient of its union
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

/// THE QUOTIENT OF A FIBER'S UNION SHAPES (52.5): every lamina the member
/// stands on across its occupied frames, read at its own `(r, c, k)` and
/// never at frame zero, with the versor that carries the one union onto
/// the other — VERIFIED here, on the union, lamina by lamina.
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
/// fiber's moments to re-derive a versor already in hand — measured on the
/// 60-task slice at 4 000 000 stroke: 32.5 s to 38.9 s, +19% wall clock for
/// a reading identical in every count.
///
/// It is verified rather than asserted (rule 11, law 12): every lamina of
/// the union is carried and its image read, which is `carriage`'s own check
/// without `carriage`'s search, and a fiber the versor does not carry
/// whole has no union quotient. `None` also where either side is empty.
pub(crate) fn union_quotient(
    mut ins: Vec<Lamina>,
    mut outs: Vec<Lamina>,
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
    for l in &a.laminae {
        super::perceive::landed_hue(v, &l.site, &b)?;
    }
    Some((a, b, v.clone()))
}

/// THE FRAMES A MEMBER OCCUPIES, read at its own coordinate (52.5 reading
/// 6). `bodies_at` is keyed `member ‖ frame`, so the frames this member
/// stands on are the fiber under its own member prefix and the remaining key
/// IS the frame, by its own spelling (AT1 Q4) — whose byte order is the
/// frames' order, so the trie's walk is the succession along `e₃` and nothing
/// sorts.
///
/// Re-derived from the carrier that already holds the bodies, never stored
/// beside them (law 12).
pub(crate) fn occupied_frames(bodies_at: &Keyed<(Body, Body, Versor)>, v: &coord::VertexAt, ck: &[u8]) -> Vec<u64> {
    let prefix = member_key(v, ck);
    let Some(fiber) = bodies_at.fiber_at(&prefix) else {
        return Vec::new();
    };
    fiber
        .filter_map(|(k, _)| crate::transducer::sentence::frame_of_key(k.get(prefix.len()..)?))
        .collect()
}

/// WHAT HAPPENED TO EVERY FACE THIS FORM WAS OFFERED, by the reader that
/// answered. Each field is one `continue` in the incidence loop; `offered`
/// is what `named_faces` proposed and `built` is what stood. The counts are
/// a measurement and never a gate (§8.1).
#[derive(Default)]
pub(crate) struct FaceDrops {
    pub(crate) offered: usize,
    pub(crate) built: usize,
    /// THE CORNER CONTACTS (70.4) among them: offered by the diagonal, and
    /// built into faces carrying 50.3's edge. Counted apart so a form's line
    /// says whether its bodies relate, not only that faces stood.
    pub(crate) contact_offered: usize,
    pub(crate) contact_built: usize,
    /// BQ5b — contact faces built on WHETHER whose WHAT did not read: the
    /// body pair's nuclei (or their tether's ray) absent, the interface
    /// `None` (A.14).
    pub(crate) unnucleated: usize,
    pub(crate) member_key: usize,
    pub(crate) vertex: usize,
    pub(crate) chart: usize,
    pub(crate) word: usize,
    pub(crate) place: usize,
    /// AT1 — a pair the incidence named that stands on NO FRAME TOGETHER: a
    /// face is one overlap per frame both members hold (Q2), and a pair with
    /// none has no overlap to be.
    pub(crate) frameless: usize,
}

/// WHY NO FRAME PLACES A NAMED PAIR (78.46(5)(d)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FramelessWhy {
    /// The two members hold no frame in common.
    NoCommonFrame,
    /// They hold frames in common, and the edge re-derived on none of them.
    EdgeOnNone,
}

/// A NAMED PAIR NO FRAME PLACES — A REFUSAL THAT CARRIES ITS CONTENT
/// (78.46(5)(d), A.17, law 7): the incidence named the pair, and a face is
/// one overlap per frame both members stand on (AT1, Q2), so with no such
/// frame there is no overlap to be. It is counted and it keeps the pair,
/// both members with the frames each holds, and why; never dropped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Frameless {
    pub(crate) a: (coord::VertexAt, Invariant, Vec<u64>),
    pub(crate) b: (coord::VertexAt, Invariant, Vec<u64>),
    pub(crate) why: FramelessWhy,
}

#[cfg(test)]
impl Frameless {
    /// The refusal as the census line reads it: each member by its
    /// vertex's coordinate and its chart, with its frames, and the reason.
    pub(crate) fn line(&self) -> String {
        use crate::network::nerve::Coordinate;
        let hx = |b: &[u8]| -> String {
            let mut s: String = b.iter().take(2).map(|x| format!("{x:02x}")).collect();
            s.push('…');
            s
        };
        // The chart WHOLE: two charts of one vertex share a prefix, and a
        // prefix that cannot tell them apart is no reading of either (78.45).
        let whole = |b: &[u8]| -> String { b.iter().map(|x| format!("{x:02x}")).collect() };
        let member = |(_, c, ks): &(coord::VertexAt, Invariant, Vec<u64>)| {
            format!("chart {} on frames {ks:?}", whole(c.key().as_bytes()))
        };
        let vertices = match self.a.0 == self.b.0 {
            true => format!("one vertex {}", hx(self.a.0.encode().as_bytes())),
            false => format!("vertices {} and {}", hx(self.a.0.encode().as_bytes()), hx(self.b.0.encode().as_bytes())),
        };
        let why = match self.why {
            FramelessWhy::NoCommonFrame => "no frame both stand on",
            FramelessWhy::EdgeOnNone => "the edge re-derived on no frame both stand on",
        };
        format!("frameless pair at {vertices}: {} ↔ {}: {why}", member(&self.a), member(&self.b))
    }
}

/// THE EDGE BETWEEN TWO MEMBERS (50.3), MEASURED ON THEIR UNION SHAPES
/// (52.5), and the frames on which the per-frame reading re-derives one
/// edge — the face's section along `e₃`.
///
/// THE FACE IS ON THE UNION SHAPES, not on a slice. The edge was measured
/// on the two members' bodies AT THE LEAST FRAME BOTH STAND ON: every
/// lamina of that reading sits at one `k`, so `edge_transport`'s union
/// `(a_i ∪ a_j)` has no `e₃` extent and `versor_between` on it cannot
/// return a transport with `(r,k)` or `(c,k)` content whatever the family
/// does along the frame axis. That is the shape of the flatness measured
/// at epoch 70 — 292 of 292 orbits flat — and no amount of classifying
/// downstream recovers a bivector the reading never had. The union of a
/// member's fiber has the extent by construction (51.3: the family is one
/// body on `(r, c, k)`), and it is what the face carries here.
///
/// The per-frame readings below stay, and are the SECTION: the frames on
/// which one edge re-derives. They are compared with each other and not
/// with the union, because a union edge is placed in the union's own
/// coordinates and a frame's is placed in that frame's — asking a slice to
/// equal the whole would report every face as claiming nothing.
///
/// A pair whose fibers have no union between them — a cover piece stands
/// at one frame and has none — falls to the per-frame edge, and a pair
/// with no common frame and no union carries none.
pub(crate) fn edge_between(
    bodies_at: &Keyed<(Body, Body, Versor)>,
    unions: &Keyed<(Body, Body, Versor)>,
    n_reached: usize,
    (i, cik): (&coord::VertexAt, &[u8]),
    (j, cjk): (&coord::VertexAt, &[u8]),
    tally: &mut EdgeTally,
) -> (Option<Transport>, Option<Keyed<()>>) {
    let at = |v: &coord::VertexAt, ck: &[u8], k: u64| -> Option<&(Body, Body, Versor)> {
        let mut bk = member_key(v, ck);
        bk.extend_from_slice(&crate::transducer::sentence::frame_key(k));
        bodies_at.get(&bk)
    };
    let union_edge = match (unions.get(&member_key(i, cik)), unions.get(&member_key(j, cjk))) {
        (Some((a_i, b_i, v_i)), Some((a_j, b_j, _))) => {
            tally.take(Some(edge_transport(v_i, a_i, a_j, b_i, b_j)))
        }
        _ => None,
    };
    let mut edge: Option<Transport> = None;
    let mut agreeing: Vec<u64> = Vec::new();
    let mut common = 0usize;
    // SUCCESSION ALONG e₃ IS THE NEXT FRAME OCCUPIED, never `k + 1` (52.5
    // reading 6, ruled 2026-09-18). This read `for k in 0..n`, probing every
    // integer up to the family's pair count and `continue`-ing on each miss:
    // a linear scan over a range, where the frames a member actually stands
    // on are named by its own fiber of `bodies_at`. The occupied set may
    // skip, repeat or reorder in ℤ, and the integers between it are not
    // coordinates of anything.
    for k in occupied_frames(bodies_at, i, cik) {
        let (Some((a_i, b_i, v_i)), Some((a_j, b_j, _))) = (at(i, cik, k), at(j, cjk, k)) else {
            continue;
        };
        common += 1;
        let measured = edge_transport(v_i, a_i, a_j, b_i, b_j);
        match &edge {
            None => {
                edge = tally.take(Some(measured));
                if edge.is_some() {
                    agreeing.push(k);
                }
            }
            Some(e) => {
                if measured.as_ref().is_ok_and(|t| same_transport(t, e)) {
                    agreeing.push(k);
                }
            }
        }
    }
    let pairs = match (&edge, common) {
        (Some(_), _) if agreeing.len() < n_reached => Some(section_of_frames(agreeing)),
        _ => None,
    };
    // The union edge is the face's where the two fibers have one; the
    // per-frame edge stands where they do not.
    (union_edge.or(edge), pairs)
}