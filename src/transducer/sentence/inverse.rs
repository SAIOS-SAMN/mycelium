// 54.3 — THE INVERSE: `Form::from_address`, A21's inverse and the only
// decoder.
//
//! Split from `chart.rs` at the §1.1 1200-line reading bound; the item is
//! moved, not rewritten (§1.6). The key is written there and read here,
//! and the two are one fact in two directions: every reader below mirrors
//! a writer above it, decision for decision.

use super::form::*;
use super::transport::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{Rotor, Word};
use crate::transducer::element::Versor;
use crate::transducer::sentence::Link;
use crate::transducer::stalk;
use super::chart::word_vertex_address;

/// The chain back out of its address, AND THE CORNER IT SAT AT.
///
/// The address names the chain up to A5's centring; the corner beside it
/// is where it stood. `from_residual` is the one reader of a residual
/// address and this uses it rather than restating the layout (law 12).
fn take_chain_address(k: &[u8], p: &mut usize) -> Option<Vec<Link>> {
    // THE SHAPE LEADS: no chain, a chain by its address, or a chain A5
    // refuses an address and the key writes out (54.5).
    let shape = *k.get(*p)?;
    *p += 1;
    match shape {
        0 => Some(Vec::new()),
        1 => {
            let a = take_bytes(k, p)?;
            let mut out = from_residual(&a)?;
            let cr = i64::from_le_bytes(k.get(*p..*p + 8)?.try_into().ok()?);
            let cc = i64::from_le_bytes(k.get(*p + 8..*p + 16)?.try_into().ok()?);
            *p += 16;
            for l in &mut out {
                l.0 += cr;
                l.1 += cc;
            }
            Some(out)
        }
        2 => {
            let n = take_u32(k, p)? as usize;
            let mut out = Vec::with_capacity(n.min(k.len()));
            for _ in 0..n {
                let r = i64::from_le_bytes(k.get(*p..*p + 8)?.try_into().ok()?);
                let c = i64::from_le_bytes(k.get(*p + 8..*p + 16)?.try_into().ok()?);
                out.push((r, c, *k.get(*p + 16)?, *k.get(*p + 17)?));
                *p += 18;
            }
            Some(out)
        }
        _ => None,
    }
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
/// ratio. Loops and holonomies are DERIVED and never keyed (§1.5), so
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
pub fn from_address(k: &[u8]) -> Option<Form> {
    let mut p = 0usize;
    match k {
        [tag, ..] if *tag == crate::transducer::address::TAG_FORM => p += 1,
        _ => return None,
    }
    let mut f = Form::default();
    // `Form::default()` opens with its Frame vertex; the key writes every
    // vertex the form has, that one included, so the vertices are taken
    // from the key alone and not appended to what the default seeded.
    f.vertices.clear();
    let nv = take_u32(k, &mut p)? as usize;
    for _ in 0..nv {
        let kind = *k.get(p)?;
        p += 1;
        let v = match kind {
            0 => Vertex::Frame,
            1 => {
                let rotor = Rotor::from_key(&take_bytes(k, &mut p)?)?;
                let acts = take_acts(k, &mut p, true)?;
                let contact = acts.1;
                let derivation =
                    crate::transducer::alphabet::Derivation::spelled(Word::formed(rotor));
                let address = word_vertex_address(&derivation, &acts.0, contact);
                Vertex::Word { address, derivation, acts: acts.0, contact }
            }
            2 => {
                let address = crate::transducer::address::Address::new(take_bytes(k, &mut p)?);
                let placement = Word::formed(Rotor::from_bytes(k, &mut p)?);
                let depth = take_u16k(k, &mut p)?;
                let carriage = Box::new(Versor::from_bytes(k, &mut p)?);
                let chart = stalk::Invariant::from_key(&take_bytes(k, &mut p)?)?;
                Vertex::Glued {
                    address,
                    chart,
                    magnitude: [0; 4],
                    depth,
                    placement,
                    carriage,
                }
            }
            3 => {
                let by = take_u16k(k, &mut p)?;
                let of = take_u16k(k, &mut p)?;
                let acts = take_acts(k, &mut p, false)?;
                Vertex::Rebased { by, of, acts: acts.0 }
            }
            4 => {
                let m = *k.get(p)?;
                p += 1;
                let measure = match m {
                    0 => Measure::Occupied,
                    1 => Measure::Remainder,
                    2 => Measure::Ring { anchor: take_u16k(k, &mut p)? },
                    _ => return None,
                };
                Vertex::Unfolded { measure }
            }
            5 => {
                let kb = take_bytes(k, &mut p)?;
                let mut kp = 1usize;
                let kind = crate::transducer::sentence::Demand::from_bytes(&kb, &mut kp)?;
                let chart = stalk::Invariant::from_key(&take_bytes(k, &mut p)?)?;
                // A demand whose bodies the caller did not supply is not a
                // demand standing for nothing — it is a form that cannot be
                // rebuilt from what was given, and it refuses (rule 11).
                // 54.2: the remainder is in the key, as its chain's address.
                let bodies = take_chain_address(k, &mut p)?;
                Vertex::Demand { kind, chart, bodies }
            }
            _ => return None,
        };
        f.vertices.push(v);
    }
    let nf = take_u32(k, &mut p)? as usize;
    for _ in 0..nf {
        let i = take_u16k(k, &mut p)?;
        let j = take_u16k(k, &mut p)?;
        let word = Word::formed(Rotor::from_bytes(k, &mut p)?);
        let n = take_u16k(k, &mut p)? as usize;
        let mut at = Keyed::new();
        for _ in 0..n {
            let pk = take_bytes(k, &mut p)?;
            let carriage = Versor::from_bytes(k, &mut p)?;
            let gauge = take_gauge(k, &mut p)?;
            // 50.3's edge: its PRESENCE is on the key and its value is
            // not, because the court re-measures it. An absent value is
            // not a zero edge; the face records that one stood and the
            // court supplies what it measures.
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
            at.insert(&pk, FacePlace { carriage, gauge, edge });
        }
        let pairs = match *k.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let m = take_u16k(k, &mut p)? as usize;
                let mut ps = Keyed::new();
                for _ in 0..m {
                    ps.insert(take_frame_key(k, &mut p)?, ());
                }
                Some(ps)
            }
            _ => return None,
        };
        f.faces.push(FormFace { i, j, word, at, pairs });
    }
    let mut r = [0u64; 4];
    for x in &mut r {
        *x = u64::from_le_bytes(k.get(p..p + 8)?.try_into().ok()?);
        p += 8;
    }
    f.ratio = ((r[0] as usize, r[1] as usize), (r[2] as usize, r[3] as usize));
    let ns = take_u32(k, &mut p)? as usize;
    for _ in 0..ns {
        let class = take_bytes(k, &mut p)?;
        let m = take_u16k(k, &mut p)? as usize;
        let mut members = Keyed::new();
        for _ in 0..m {
            members.insert(take_frame_key(k, &mut p)?, ());
        }
        f.sections.insert(&class, members);
    }
    let na = take_u32(k, &mut p)? as usize;
    for _ in 0..na {
        let class = take_bytes(k, &mut p)?;
        f.axes.insert(&class, 0);
    }
    // 54.2: the remainder is identity and is in the key.
    // `place_chain` is the tape decoder's own reader of a placed chain —
    // the same one, not a second that agrees with it (law 12). An empty
    // chain places nothing and leaves the default.
    let rem = take_chain_address(k, &mut p)?;
    if !rem.is_empty() {
        f.remainder = crate::transducer::element::place_chain(&rem)?;
    }
    // LOOPS AND HOLONOMIES ARE DERIVED AND NEVER KEYED (§1.5), so they are
    // re-derived from the incidence just read, by the same reader the
    // extractor uses — not read back from a carrier that would be a second
    // statement of the faces (law 12).
    f.reindex();
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    // AND THE DEPTH, which is `one more than the deepest citation` and so a
    // reading of the `Glued` vertices the key just wrote (law 12). It was
    // left at `Form::default()`'s zero, and the court checks `f.depth ==
    // f.depth_of()` — so every citing form decoded from its address was
    // refused `REFUSE_GLUED_DEPTH` the moment this became the only decoder.
    f.depth = f.depth_of();
    // THE FILED TORSION IS NOT DERIVED HERE, AND THE REASON IS A READING
    // AND NOT A PREFERENCE (54.3, measured 2026-09-20).
    //
    // `Form.torsion` is not in the key, so the obvious move is to re-derive
    // it the way the loops above are re-derived: walk each loop, classify
    // its holonomy, file what closes. That was written and it is WRONG,
    // because filing is not a function of the incidence.
    //
    // `pivot::drive` files a loop in ONE of its four branches —
    // `NoneDischarged`. A loop the stroke DISCHARGED leaves no filing
    // because the obstruction is gone; a loop that went to `Vortex`, and a
    // loop whose `Starved` budget ran out before its candidates were
    // reached, stand UNFILED with their holonomy intact. So two forms with
    // the same faces, the same vertices and the same loops carry different
    // filings depending on what a stroke could afford, and a derivation
    // from the incidence alone would hand a starved form the filing its
    // stroke never made. `from_address(address(f)) ≡ f` failed on the very
    // first fixture that has a loop and has never run a stroke, which is
    // the reading.
    //
    // SO THE KEY DOES NOT DETERMINE THE FILING, and this says so rather
    // than covering it: torsion is `Form ∖ Key`, exactly where the K6.4
    // genome instrument already counts it. Closing that is one of two
    // rulings — torsion enters the key, which is a layout change and
    // therefore a recut, or 54.5's value carries it — and neither is taken
    // by a decoder inventing a theorem (rule 11).
    //
    // What DID move is the filer itself: `file_loop` no longer takes its
    // period as a parameter, so a screw is no longer filed at its rotor's
    // order. See `the_filer_reaches_the_fixed_point_gate`.
    Some(f)
}

/// A run of members as the key writes them: the chart, the carriage, the
/// contact where the vertex carries one, and the sorted gauge.
///
/// The contact is the VERTEX's and the writer repeats it at every member,
/// so it rides beside the run rather than inside it.
struct Acts(Keyed<Acting>, Option<crate::transducer::alphabet::Contact>);

fn take_acts(k: &[u8], p: &mut usize, contact: bool) -> Option<Acts> {
    let n = match contact {
        true => take_u32(k, p)? as usize,
        false => take_u16k(k, p)? as usize,
    };
    let mut out = Keyed::new();
    let mut got: Option<Option<crate::transducer::alphabet::Contact>> = None;
    for _ in 0..n {
        let ck = take_bytes(k, p)?;
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
        out.insert(&ck, Acting { chart, carriage: Box::new(carriage), gauge });
    }
    Some(Acts(out, got.flatten()))
}

/// A FRAME AS ITS OWN KEY, the two bytes `frame_key` writes. Read as bytes
/// and not through a `u16`, because `frame_key` is big-endian on purpose —
/// the trie's order is then the frame's order — and routing it through
/// `take_u16k`, which is little-endian, is what gave a frame two spellings
/// and the sections run two addresses (54.5).
fn take_frame_key(k: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let b = k.get(*p..*p + 2)?.to_vec();
    *p += 2;
    Some(b)
}

fn take_gauge(k: &[u8], p: &mut usize) -> Option<Vec<(u8, u8)>> {
    let n = take_u16k(k, p)? as usize;
    let mut g = Vec::with_capacity(n.min(k.len()));
    for _ in 0..n {
        g.push((*k.get(*p)?, *k.get(*p + 1)?));
        *p += 2;
    }
    Some(g)
}

fn take_u32(k: &[u8], p: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(k.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

fn take_u16k(k: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(k.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

fn take_bytes(k: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let n = take_u32(k, p)? as usize;
    let b = k.get(*p..*p + n)?.to_vec();
    *p += n;
    Some(b)
}

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
    let block = take_bytes(addr, &mut p)?;
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
            let n = u16::from_le_bytes(block.get(q..q + 2)?.try_into().ok()?) as usize;
            q += 2 + n * 2;
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
    let n = u32::from_le_bytes(block.get(q..q + 4)?.try_into().ok()?) as usize;
    q += 4;
    let mut out = Vec::with_capacity(n.min(block.len()));
    for _ in 0..n {
        let r = i64::from_le_bytes(block.get(q..q + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(block.get(q + 8..q + 16)?.try_into().ok()?);
        out.push((r, c, *block.get(q + 16)?, *block.get(q + 17)?));
        q += 18;
    }
    Some(out)
}
