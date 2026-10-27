// 54.3 — THE INVERSE: `Form::from_address`, A21's inverse and the only
// decoder.
//
//! Split from `chart.rs` at the §1.1 1200-line reading bound; the item is
//! moved, not rewritten (§1.6). The key is written there and read here,
//! and the two are one fact in two directions: every reader below mirrors
//! a writer above it, decision for decision.

use super::form::*;
use super::transport::*;
use crate::network::nerve::{Coordinate, Keyed};
use crate::transducer::alphabet::{Rotor, Word};
use crate::transducer::element::Versor;
use crate::transducer::sentence::Link;
use crate::transducer::stalk;
use super::chart::word_vertex_address;
use crate::transducer::address::take_key_count;

/// A chain's coordinates back out of the key (AT1, 54.5 Road (d)): the flag,
/// then the measure per lane, the pieces and the corner, as
/// `put_chain_at` writes them. `Some(None)` is the empty chain.
fn take_chain_at(k: &[u8], p: &mut usize) -> Option<Option<ChainAt>> {
    let flag = *k.get(*p)?;
    *p += 1;
    match flag {
        0 => Some(None),
        1 => {
            let mut measure = [0u64; 4];
            for lane in measure.iter_mut() {
                *lane = take_u64(k, p)?;
            }
            let pieces = take_u64(k, p)?;
            let r = i64::from_le_bytes(k.get(*p..*p + 8)?.try_into().ok()?);
            let c = i64::from_le_bytes(k.get(*p + 8..*p + 16)?.try_into().ok()?);
            *p += 16;
            Some(Some(ChainAt { measure, pieces, corner: (r, c) }))
        }
        _ => None,
    }
}

fn take_u64(k: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(k.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

/// THE INVERSE OF [`form_vertex_key`] (A21 §4, 1.5 as ruled 2026-09-20).
///
/// §1.5 writes the key "so that the word form is recovered from its key",
/// and until now nothing recovered it: `Rotor::from_key`, `Mv::from_key`
/// and `ClassVector::from_key` existed and there was no decoder for
/// `TAG_FORM`. That gap was read as a structural division — "the address
/// names a form and cannot rebuild it; the tape rebuilds a form and does
/// not name it" — which made an unwritten function into an ontology and
/// left the tape carrying the key's content a second time. Measured, that
/// duplication was 68% of every tape.
///
/// WHAT THE KEY DETERMINES, THIS POPULATES. Vertex kinds and their
/// coordinates, the members each acts through, the faces and their
/// incidence, the sections along `e₃`, the opened axes and the frame
/// ratio. Orbits and holonomies are DERIVED and never keyed (§1.5), so
/// they are re-derived here by the form's own reader rather than read.
///
/// WHAT THE KEY DOES NOT DETERMINE IS NOT INVENTED (rule 11). Three
/// things: a `Word` vertex's own address, which names the word the
/// reading located and is a citation rather than a coordinate of this
/// vertex; a `Demand`'s bodies, which 50.5 puts on the tape and not in the
/// key; and the A5 remainder, which is precisely what no generator covers
/// (law 7) and so cannot be derived from the generators. They arrive as
/// `rest`, and a caller that has none passes none — which is exactly a
/// closed, remainder-free form.
/// IT TAKES AN ADDRESS AND NOT A BYTE SLICE (law 2, as ruled 2026-09-20).
///
/// This read `k: &[u8]`, so its interface was a BYTE RUN: any slice in the
/// tree typechecked here, and the signature said nothing about whether what
/// arrived was a coordinate or a tape. That is the same conflation the whole
/// purge is retiring, one level up from the carrier — the decoder is the
/// inverse of `form_vertex_key`, whose codomain is `Address`, so its domain
/// is `Address` and nothing else.
///
/// `Address` is bytes underneath, and that is not the point: it is the type
/// that says these bytes ARE the object's coordinates written out, reached
/// only through a writer that wrote them as such. A caller that has a byte
/// run and wants a form must say so by building an address from it, at its
/// own line, where a reader can see the claim being made.
pub fn from_address(a: &crate::transducer::address::Address) -> Option<Form> {
    let k = a.as_bytes();
    let mut p = 0usize;
    match k {
        [tag, ..] if *tag == crate::transducer::address::TAG_FORM => p += 1,
        _ => return None,
    }
    // 71.2(1), BQ1 — the base's `n`, written first.
    let n = *k.get(p)?;
    p += 1;
    let mut f = Form { n, ..Form::default() };
    // `Form::default()` opens with its Frame vertex; the key writes every
    // vertex the form has, that one included, so the vertices are taken
    // from the key alone and not added to what the default seeded.
    f.vertices = Keyed::new();
    // Every count in the key is read by its own coordinate (STD1 3c-2).
    let nv = take_key_count(k, &mut p)?;
    for _ in 0..nv {
        let kind = *k.get(p)?;
        p += 1;
        let v = match kind {
            0 => Vertex::Frame,
            1 => {
                let rotor = Rotor::from_key(&crate::transducer::address::take_key_bytes(k, &mut p)?)?;
                let acts = take_acts(k, &mut p, true)?;
                let contact = acts.1;
                let derivation =
                    crate::transducer::alphabet::Derivation::spelled(Word::formed(rotor));
                let address = word_vertex_address(&derivation, &acts.0, contact);
                Vertex::Word { address, derivation, acts: acts.0, contact }
            }
            2 => {
                // 54.7 (B5) — THE CITATION'S TARGET IS NOT IN THE KEY, so
                // it is not read from one and none is invented (rule 11).
                // `form_vertex_key` writes the boundary — the chart's rank,
                // the depth, the placement and φ — and WHICH form that
                // boundary mates with is the cover's fact, carried on
                // `Form::cover` and filled by `from_element_form` from the
                // word's `ReceiptsEnvelope`. A form reconstituted from a
                // key alone has no cover, and that is the truth about it.
                let placement = Word::formed(Rotor::from_bytes(k, &mut p)?);
                let depth = take_u16k(k, &mut p)?;
                let carriage = Box::new(Versor::from_bytes(k, &mut p)?);
                let chart = stalk::Invariant::from_key(&crate::transducer::address::take_key_bytes(k, &mut p)?)?;
                Vertex::Glued {
                    chart,
                    magnitude: [0; 4],
                    depth,
                    placement,
                    carriage,
                }
            }
            3 => {
                let by = coord::VertexAt::decode(&crate::transducer::address::take_key_bytes(k, &mut p)?).ok()?;
                let class = coord::Transport::decode(&crate::transducer::address::take_key_bytes(k, &mut p)?).ok()?;
                let acts = take_acts(k, &mut p, false)?;
                Vertex::Rebased { by, class, acts: acts.0 }
            }
            4 => {
                let unfolded = coord::Unfolded::decode(&crate::transducer::address::take_key_bytes(k, &mut p)?).ok()?;
                Vertex::Unfolded { measure: unfolded.0 }
            }
            5 => {
                let kb = crate::transducer::address::take_key_bytes(k, &mut p)?;
                let mut kp = 1usize;
                let kind = crate::transducer::sentence::Demand::from_bytes(&kb, &mut kp)?;
                let chart = stalk::Invariant::from_key(&crate::transducer::address::take_key_bytes(k, &mut p)?)?;
                // AT1, 54.5 ROAD (d) — THE DEMAND'S COORDINATES, not its
                // bodies: the bodies ride the record and the record supplies
                // them (`from_element_form`), as it supplies a citation's
                // cover. A form read from its key alone holds none, and that
                // is the truth about it.
                let mut measure = [0u64; 4];
                for lane in measure.iter_mut() {
                    *lane = take_u64(k, &mut p)?;
                }
                let pieces = take_u64(k, &mut p)?;
                Vertex::Demand { kind, chart, measure, pieces, bodies: Vec::new() }
            }
            _ => return None,
        };
        // THE VERTEX STANDS AT ITS OWN COORDINATE, read off what it is; a
        // record whose coordinate does not read is no vertex (rule 11).
        f.put_vertex(v)?;
    }
    let nf = take_key_count(k, &mut p)?;
    for _ in 0..nf {
        let o = coord::Overlap::decode(&crate::transducer::address::take_key_bytes(k, &mut p)?).ok()?;
        let producer = match *k.get(p)? {
            0 => Producer::Place,
            1 => Producer::Motor,
            _ => return None,
        };
        p += 1;
        let carriage = Versor::from_bytes(k, &mut p)?;
        let gauge = take_gauge(k, &mut p)?;
        // 54.2: the edge's VALUE, as the transport it is.
        let edge = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let word = Word::formed(Rotor::from_bytes(k, &mut p)?);
                let carriage = Versor::from_bytes(k, &mut p)?;
                let gauge = take_gauge(k, &mut p)?;
                Some(Transport { word, carriage, gauge })
            }
            _ => return None,
        };
        // THE PIVOT'S FACTORS, each behind its flag (ruled): absent is the
        // identity, which is what an unpivoted overlap carries.
        let mut factors = [Word::identity(), Word::identity()];
        for factor in factors.iter_mut() {
            match *k.get(p)? {
                0 => p += 1,
                1 => {
                    p += 1;
                    *factor = Word::formed(Rotor::from_bytes(k, &mut p)?);
                }
                _ => return None,
            }
        }
        let [left, right] = factors;
        // BQ5b — the interface, as `put_face` writes it.
        let interface = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                Some(super::Interface::take(k, &mut p)?)
            }
            _ => return None,
        };
        // 78.108 — THE REFRACTION, as `put_face` writes it: a held coset's
        // candidates whole, or a static contact's tether; its poses unread at
        // this vantage.
        let crossed = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let n = take_key_count(k, &mut p)?;
                let mut candidates = Vec::with_capacity(n.min(64));
                for _ in 0..n {
                    let word = Word::formed(Rotor::from_bytes(k, &mut p)?);
                    let carriage = Versor::from_bytes(k, &mut p)?;
                    let gauge = take_gauge(k, &mut p)?;
                    candidates.push(Transport { word, carriage, gauge });
                }
                Some(super::Refraction::Coset(candidates))
            }
            2 => {
                p += 1;
                Some(super::Refraction::Static {
                    tether: Versor::from_bytes(k, &mut p)?,
                    poses: [super::HeldPose::Unread, super::HeldPose::Unread],
                })
            }
            _ => return None,
        };
        // THE DECODE IS THE EXACT INVERSE (law 2: the address IS the form):
        // what crossed is what is read, and nothing is added. A contact the
        // perceiving mind could not refract crosses with none and stays
        // refused at the reader too, as it is at the mind (CT2 constructs it
        // there). 78.107's static read by the interface, added here, broke
        // the round trip and is withdrawn (78.108).
        let refraction = crossed;
        let place = FacePlace { carriage, gauge, edge, refraction };
        f.faces.put(&o, FormFace { place, producer, left, right, interface });
    }
    let mut r = [0u64; 4];
    for x in &mut r {
        *x = u64::from_le_bytes(k.get(p..p + 8)?.try_into().ok()?);
        p += 8;
    }
    // A ZERO DENOMINATOR IS NOT A RATIO (54.7 B6, rule 11). The tape
    // decoder carried this refusal and this one did not, so the purge would
    // have dropped it silently: a bundle whose frame ratio has `hd = 0` or
    // `wd = 0` is no bundle, and reading one back as a form would hand
    // every downstream reader a division it cannot take. Refused, never
    // repaired to 1 — a coordinate this build cannot read is not a
    // coordinate to guess at.
    if r[1] == 0 || r[3] == 0 {
        return None;
    }
    // Each part read in `usize` or the key refused: a ratio this node's
    // carrier cannot hold is not truncated into another ratio.
    let at = |x: u64| usize::try_from(x).ok();
    f.ratio = ((at(r[0])?, at(r[1])?), (at(r[2])?, at(r[3])?));
    // THE SECTIONS ARE NOT IN THE KEY (AT1): each member's frames are in its
    // record, and `Form::sections` reads them off the members.
    let na = take_key_count(k, &mut p)?;
    for _ in 0..na {
        let class = crate::transducer::address::take_key_bytes(k, &mut p)?;
        // The class alone: the chord is the orbit's, found by the class.
        f.axes.put(&coord::Holonomy::decode(&class).ok()?, None);
    }
    // 54.2, AT1 — THE REMAINDER BY ITS COORDINATES; the chain rides the
    // record and `from_element_form` places it from there.
    f.remainder_at = take_chain_at(k, &mut p)?;
    // 78.143 — THE ELEMENTS, where the key carries any, and then nothing: a
    // run of bytes after them is no form this build writes.
    if p < k.len() {
        f.elements = super::form::rule_element::take_elements(k, &mut p)?;
        if p != k.len() {
            return None;
        }
    }
    // ORBITS AND HOLONOMIES ARE DERIVED AND NEVER KEYED (§1.5), so they are
    // re-derived from the incidence just read, by the same reader the
    // extractor uses — not read back from a carrier that would be a second
    // statement of the faces (law 12).
    f.reindex();
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    // AND THE DEPTH, which is `one more than the deepest citation` and so a
    // reading of the `Glued` vertices the key just wrote (law 12). It was
    // left at `Form::default()`'s zero, and the court checks `f.depth ==
    // f.depth_of()` — so every citing form decoded from its address was
    // refused `REFUSE_GLUED_DEPTH` the moment this became the only decoder.
    f.depth = f.depth_of()?;
    // THE FILED TORSION IS NOT DERIVED HERE, AND THE REASON IS A READING
    // AND NOT A PREFERENCE (54.3, measured 2026-09-20).
    //
    // `Form.torsion` is not in the key, so the obvious move is to re-derive
    // it the way the orbits above are re-derived: read each orbit, classify
    // its holonomy, file what closes. That was written and it is WRONG,
    // because filing is not a function of the incidence.
    //
    // `pivot::drive` files an orbit in ONE of its four branches —
    // `NoneDischarged`. An orbit the stroke DISCHARGED leaves no filing
    // because the obstruction is gone; an orbit that went to `Vortex`, and an
    // orbit whose `Starved` budget ran out before its candidates were
    // reached, stand UNFILED with their holonomy intact. So two forms with
    // the same faces, the same vertices and the same orbits carry different
    // filings depending on what a stroke could afford, and a derivation
    // from the incidence alone would hand a starved form the filing its
    // stroke never made. `from_address(address(f)) ≡ f` failed on the very
    // first fixture that has an orbit and has never run a stroke, which is
    // the reading.
    //
    // SO THE KEY DOES NOT DETERMINE THE FILING, and this says so rather
    // than covering it: torsion is `Form ∖ Key`, exactly where the K6.4
    // genome instrument already counts it. Closing that is one of two
    // rulings — torsion enters the key, which is a layout change and
    // therefore a recut, or 54.5's value carries it — and neither is taken
    // by a decoder inventing a theorem (rule 11).
    //
    // What DID move is the filer itself: `file_orbit` no longer takes its
    // period as a parameter, so a screw is no longer filed at its rotor's
    // order. See `the_filer_reaches_the_fixed_point_gate`.
    Some(f)
}

/// A run of members as the key writes them: the chart, the carriage, the
/// contact where the vertex carries one, and the sorted gauge.
///
/// The contact is the VERTEX's and the writer repeats it at every member,
/// so it rides beside the run rather than inside it.
struct Acts(
    Keyed<Acting, crate::network::nerve::coord::Section>,
    Option<crate::transducer::alphabet::Contact>,
);

fn take_acts(k: &[u8], p: &mut usize, contact: bool) -> Option<Acts> {
    let n = take_key_count(k, p)?;
    let mut out = Keyed::new();
    let mut got: Option<Option<crate::transducer::alphabet::Contact>> = None;
    for _ in 0..n {
        let ck = crate::transducer::address::take_key_bytes(k, p)?;
        let chart = stalk::Invariant::from_key(&ck)?;
        let carriage = Versor::from_bytes(k, p)?;
        if contact {
            // THE CONTACT IS READ, NOT SKIPPED (54.5, as ruled 2026-09-20).
            // It sits between the carriage and the gauge on every Word
            // member, `put_word_vertex` writes it once per member, and this
            // stepped over the bytes and answered `None` — so a vertex with
            // a stop ring decoded WITHOUT one and re-keyed to a different
            // address. `from_address` was not a retract for any form
            // carrying a contact.
            //
            // No fixture carried one — `closed_word_form(w, None)` — so the
            // property passed over the hole for the whole of K6. The second
            // form the extractor built from the corpus found it
            // (`from_address_retracts_every_corpus_form`), which is what
            // reading real data is for.
            //
            // It is the same value on every member of one vertex, because
            // the writer writes the vertex's one contact into each; the
            // last read wins and they agree by construction.
            match *k.get(*p)? {
                0 => {
                    *p += 1;
                    got = Some(None);
                }
                1 => {
                    *p += 1;
                    let anchor = u16::from_le_bytes(k.get(*p..*p + 2)?.try_into().ok()?);
                    let ring = *k.get(*p + 2)?;
                    *p += 3;
                    got = Some(Some(crate::transducer::alphabet::Contact { anchor, ring }));
                }
                _ => return None,
            }
        }
        let gauge = take_gauge(k, p)?;
        // AT1 — THE MEMBER'S FRAMES, each by its own spelling (Q4).
        let nf = take_key_count(k, p)?;
        let mut frames = Keyed::new();
        for _ in 0..nf {
            let f = coord::take_frame(k, p)?;
            frames.insert(frame_key(f), ());
        }
        // THE MEMBER NAMES ITSELF (54.7 R1, law 12): `Acting::key` is the
        // one writer of its key.
        let member = Acting { chart, carriage: Box::new(carriage), gauge, frames };
        out.put(&member.key(), member);
    }
    Some(Acts(out, got.flatten()))
}

fn take_gauge(k: &[u8], p: &mut usize) -> Option<Vec<(u8, u8)>> {
    let n = take_key_count(k, p)?;
    let mut g = Vec::with_capacity(n.min(k.len()));
    for _ in 0..n {
        g.push((*k.get(*p)?, *k.get(*p + 1)?));
        *p += 2;
    }
    Some(g)
}

fn take_u16k(k: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(k.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

use crate::network::nerve::coord;

/// 54.3 — THE INVERSE OF A RESIDUAL ADDRESS: the chain a non-sentence word
/// stands for, read back out of the coordinate it is keyed at.
///
/// `from_address` and this are the language's two decoders and they REFUSE
/// EACH OTHER. A form is a closed cell complex written in the vertex-key
/// layout; a residual is an unclosed 1-chain, `∂c ≠ 0`, written as its own
/// bytes. They shared `TAG_FORM` until `8b861ea`, so a reader could not
/// tell which layout it held and `Element::address` fell through from the
/// first to the second whenever a sentence's tape did not decode — which
/// is how a soak's genome reported a 0.9 KB address against a 46 KB tape.
/// Two kinds, two tags, two totals, no fallback between them.
///
/// The KIND byte says which of the four shapes follows, and the chain is
/// at the end of every shape that has one. A REWRITE has no chain: it
/// names the forms it rewrites and nothing else, so this refuses it rather
/// than returning an empty chain, which would read as "a word that says
/// nothing" instead of "not a chain" (rule 11).
pub fn from_residual(addr: &[u8]) -> Option<Vec<Link>> {
    let mut p = 0usize;
    match addr {
        [tag, ..] if *tag == crate::transducer::address::TAG_RESIDUAL => p += 1,
        // A form's address is not a chain's, and reading one as the other
        // is the collision this pair exists to end.
        _ => return None,
    }
    let block = crate::transducer::address::take_key_bytes(addr, &mut p)?;
    let mut q = 0usize;
    // The shape leads, and the slice pattern reads that coordinate rather
    // than taking a member by position (rule 15).
    let [kind, ..] = block.as_slice() else { return None };
    let kind = *kind;
    q += 1;
    match kind {
        // A rewrite carries no chain.
        crate::transducer::element::vertex::KIND_REWRITE => return None,
        crate::transducer::element::vertex::KIND_LETTERS => {
            // M1's rotor, the A4 carriage, then σ.
            Rotor::from_bytes(&block, &mut q)?;
            Versor::from_bytes(&block, &mut q)?;
            let n = take_key_count(&block, &mut q)?;
            q = q.checked_add(n.checked_mul(2)?)?;
        }
        crate::transducer::element::vertex::KIND_SPATIAL => {
            Versor::from_bytes(&block, &mut q)?;
        }
        crate::transducer::element::vertex::KIND_SPATIAL_ROTOR => {
            Versor::from_bytes(&block, &mut q)?;
            Rotor::from_bytes(&block, &mut q)?;
        }
        crate::transducer::element::vertex::KIND_CHAIN => {}
        _ => return None,
    }
    let n = take_key_count(&block, &mut q)?;
    let mut out = Vec::with_capacity(n.min(block.len()));
    for _ in 0..n {
        let r = i64::from_le_bytes(block.get(q..q + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(block.get(q + 8..q + 16)?.try_into().ok()?);
        out.push((r, c, *block.get(q + 16)?, *block.get(q + 17)?));
        q += 18;
    }
    Some(out)
}
