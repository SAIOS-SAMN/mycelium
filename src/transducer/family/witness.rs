// 54.7, 58.1 — what a percept's cover witnesses: the certified 2-simplices
// of its nerve on both halves, and the phases its meets share.

use super::covered::MeetChord;
use super::pair::Covered;
use crate::network::nerve::coord;
use crate::transducer::cech::{read_nerve, NerveTotals};

// THE CERTIFIED 2-SIMPLICES OF THIS PERCEPT'S COVER (54.7, ruled
// 2026-09-21; RE-TARGETED TO THE PULLBACK 58.1, ruled 2026-09-22).
//
// IT MUST HAPPEN HERE AND NOWHERE LATER. A nerve's 2-simplex exists
// exactly where `U_i ∩ U_j ∩ U_k ≠ ∅`, which is a fact about the BASE
// and needs every chart's cells in ONE frame. By the time a form is a
// word on the wire its chain is CENTERED — framed with its min-corner
// at the origin (`element::residual`) — so each arrives in its own
// frame, and the only thing relating two of them is the transition
// carriage, which is the very thing a cocycle test evaluates. The
// court would need the frames to agree to find the overlap, and the
// overlap to know whether they agree. Here the footprints are still
// the cover's own absolute `(r, c, hue)`, so the question is
// answerable and is answered once.
//
// THE CHARTS ARE THE PIECES' DOMAINS, AND THAT IS THE WHOLE READING.
// This was built from the chord ends' CODOMAINS and read `0 certified
// triples` on every form of the corpus — which was a property of which
// half was measured, not of the corpus. `Supports` says it in its own
// doc: a deformation is a map OUT OF the input, so the Čech nerve is
// the nerve of the cover of the SOURCE, and "reading only the codomain
// leaves the nerve discrete".
//
// Why it could not be otherwise. On the image side the cover's pieces
// are a PARTITION by construction — `pieces_of` peels disjoint bodies
// and `occ_out` names one piece per site — so `int(B'ᵢ) ∩ int(B'ⱼ) = ∅`
// and a triple intersection asks three solid bodies to occupy one
// voxel. On the source side nothing of the kind holds: pass (a)
// assembles an output piece's preimage out of WHOLE input pieces, so
// one input piece lying under two output pieces stands in both
// preimages, and pass (b) carries one unplaced piece by every placed
// versor, so one domain is the base of as many sections as there are
// versors that reach an image. Both are replication, and replication is
// the only case in which the transition between two copies of one shape
// exists to be read at all.
//
// NO CHART IS FOLDED INTO ANOTHER. The codomain reading deduplicated by
// footprint — "two chord ends standing on the same cells are one chart"
// — which is right for a partition and wrong here: two carriages over
// one source region are two SECTIONS of that region's fibre, `g_ij =
// θ_j θ_i⁻¹ · γ_ij ≠ 1`, and folding them to one chart erases exactly
// the holonomy the 2-cell would carry. The cover is INDEXED by
// carriage, and the nerve of an indexed cover admits repeats.
//
// THE IMAGE READING IS TAKEN TOO, on the same pass, because the claim
// "the codomain is a partition" is now a measurement and not a
// recollection (A2).
//
// The two readings come back as the carriers that hold them — the source's
// and the image's `NerveTotals` — and the line reads them by field.
pub(super) fn cover_nerves(covers: &[Covered]) -> (NerveTotals, NerveTotals) {
    // `support_of` writes the sites of the base — the SPATIAL axes, the
    // third being the corpus's own `e₃`. The hue never reaches here: it is
    // the gauge a piece's laminae carry, a fibre coordinate, and the nerve
    // is read on the base (70.4).
    let cells_of = |sup: &[crate::transducer::base::Site]| -> Vec<coord::Lamina> {
        sup.iter().map(|s| coord::Lamina { site: s.clone() }).collect()
    };
    let mut src = NerveTotals::default();
    let mut img = NerveTotals::default();
    for (ci, c) in covers.iter().enumerate() {
        // A COVER IS ONE DEFORMATION. Its source is that pair's body at
        // frame zero, recentred, so two covers' cells are cells of two
        // different bases: each is read on its own and the readings are
        // summed. Pooling them would read a coincidence of two
        // recentrings as an overlap — the same defect the cover's meet
        // index already avoids by keying its two sides apart.
        // KB1 — EACH CHART BY ITS SECTION'S COORDINATES, the carriage and the
        // footprint as `section_key` writes them, never by its ordinal in the
        // cover: two carriages over one region are two sections because the
        // carriages differ.
        let section = |versor: &crate::transducer::element::Versor, sup: &[crate::transducer::base::Site]| {
            let mut k = Vec::new();
            versor.put_canonical(&mut k);
            for s in sup {
                s.put(&mut k);
            }
            k
        };
        let mut dom: Vec<crate::transducer::cech::Chart> = Vec::new();
        let mut cod: Vec<crate::transducer::cech::Chart> = Vec::new();
        for (_, (_, versor, sup)) in c.cover.carried.scoped() {
            dom.push((section(versor, &sup.domain), cells_of(&sup.domain)));
            cod.push((section(versor, &sup.codomain), cells_of(&sup.codomain)));
        }
        src.absorb(ci, read_nerve(&dom));
        img.absorb(ci, read_nerve(&cod));
    }
    // THE PAIRWISE READING TOO, because "0 triples" has two very
    // different causes and one number cannot tell them apart: a cover
    // whose charts overlap in pairs but never in threes is a genuine
    // 1-complex, while a cover whose charts never overlap AT ALL is a
    // PARTITION — and the nerve of a partition is a set of isolated
    // points, with no edges to climb from. The meets would then be
    // ADJACENCY between disjoint regions, not intersection, and no
    // amount of attaching machinery would find a 2-cell.
    (src, img)
}

// THE SPECTRAL WITNESS (54.7, ruled 2026-09-22): do three tethers share
// a phase, rather than a place?
//
// Every spatial criterion tried before this one — an `ℓ¹` stencil, the
// open star, the Khalimsky corner — asked whether three charts contain
// a mutual POINT, and each reintroduced a lattice to answer it. The
// physical criterion is congruence of PHASE: three currents in laminar
// flow lock into a surface where their compact rotor projections share
// a commensurate cyclotomic period, `ℓ ∈ {1,2,3,4,5,6,8,10,12}` by
// `φ(ℓ) ≤ 4`. Where no mutual period divides them no standing wave can
// form, the boundary never freezes, and the strain stays in the
// 1-skeleton as open kinematic flux.
//
// This needs NO spatial infrastructure: the rotor half is compact, so
// `Rotor::order` answers the period exactly, and the meets are already
// in hand.
pub(super) fn phase_spectrum(meet_chords: &[MeetChord]) -> (usize, usize, usize, [usize; 13]) {
    let mut finite = 0usize;
    let mut infinite = 0usize;
    let mut unread = 0usize;
    // The periods in the cyclotomic order they may occupy, counted at
    // their own index rather than summed into a total (rule 5).
    let mut by_period = [0usize; 13];
    for (_, _, e) in meet_chords.iter() {
        let Some(t) = e.as_ref() else {
            unread += 1;
            continue;
        };
        match t.word.rotor().order() {
            Some(crate::transducer::alphabet::Order::Finite(l)) => {
                finite += 1;
                if let Some(slot) = by_period.get_mut(l as usize) {
                    *slot += 1;
                }
            }
            Some(crate::transducer::alphabet::Order::Infinite) => infinite += 1,
            None => unread += 1,
        }
    }
    (finite, infinite, unread, by_period)
}
