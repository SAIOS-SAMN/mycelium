// 78.161 — THE CHATTER FRAME: each mind speaks its dialect to the others. A
// frame is a voice's signed word, its agreement, or its divergence; the
// hypha carries none of it and judges none of it.
//
//! THE WIRE. A frame is `[0x43][sub u8][speaker 32][point: 0 | 1 TPoint
//! 84][fields][sig 64]`, the signature ed25519 over `"mycelium chatter v6" ‖
//! sub ‖ speaker ‖ point ‖ fields`. Sub-kinds:
//! - SAY `[said: len u16, bytes][count u16]` then per ring `[vertex: len u16,
//!   bytes][motion Mv][gauge: count u8, (from, to)…][terms: count u16,
//!   (word Mv, transport Mv)…][vantage: count u16, Mv…]`: what the speaker
//!   said, each ring by its whole Cl(4,1) motion, its hue map, its terms and
//!   the vantage it reads it from — the mathematics a hearer re-derives,
//!   never a tape it would compare; then its EVIDENCE `[0 | 1 scene: len
//!   u16, bytes][word: len u16, bytes]`: for a word spoken under a scene,
//!   the scene's citation and the word's record, so the hearer reads what
//!   the word does on that scene (MU-CLIENT §5.5, real convergence);
//! - HEARD `[said: len u16, bytes][count u16]` then per word `[vertex: len
//!   u16, bytes][derived: 0 | 1 Mv][gauge: count u8, (from, to)…][cites
//!   count u16]` and per citation `[form: len u16, bytes][vertex: len u16,
//!   bytes][lane u8][exponent i64 LE]`, then `[asks: 0 | 1 Mv][held: count
//!   u16, Mv…][vantage: count u16, Mv…]`: how the hearer said each word in
//!   its own language — the motion and hue map its cited generators compose
//!   to, the citations, its question, the rings it sounded with and the
//!   vantage it read from — never a verdict on it;
//! - ACK `[said: len u16, bytes][count u16]` then per word `[vertex: len u16,
//!   bytes][motion: 0 | 1 Mv][hue: 0 | 1, count u8, (from, to)…]`: 78.163,
//!   the speaker's reading of the hearer's thought — each word's LOOP
//!   HOLONOMY, what the round trip said → heard carried it to. The holonomy
//!   is the content: what one mind's language does to the other's thought;
//!   still only where the hearer echoed.
//! - HOLD, the same fields as ACK: the hearer's own reading of every loop,
//!   re-derived from what it heard and what it derived, then the exact
//!   strain per lane.
//!
//! THE FRAMING. A frame travels on its own connection, `[len u32
//! LE][frame]`, within the wire's one budget (`network::wire::BODY_BUDGET`);
//! a question is answered on the same connection in the same framing.

use crate::chronometry::tuniversal::TPoint;
use crate::transducer::cga::Mv;
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub const TAG: u8 = 0x43;
pub const SAY: u8 = 0;
pub const HEARD: u8 = 1;
pub const ACK: u8 = 2;
pub const HOLD: u8 = 3;
const SIGNED: &[u8] = b"mycelium chatter v6";

/// ONE RING SAID (MU-CLIENT §4.9, 78.31): an utterance that returns — its
/// coordinate in the speaker's form, its holonomy `Ω` (the ring's whole
/// motion, what a loop said → heard is read against), its hue map, and its
/// TERMS, each word with the transport to the next, from which the hearer
/// composes the ring again in its own field (`compose`). A single word is a
/// ring of one term, its transport the identity. Its VANTAGE (MU-CLIENT
/// §5.3): the speaker's own known words it reads the ring through — the
/// perspective it offers, so the hearer reads from its own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spoken {
    pub vertex: Vec<u8>,
    pub motion: Mv,
    pub gauge: Vec<(u8, u8)>,
    pub terms: Vec<(Mv, Mv)>,
    pub vantage: Vec<Mv>,
}

/// ONE WORD HEARD, as the hearer said it in its own language: the motion and
/// the hue map its cited generators compose to (`None`: the hearer could not
/// read the word at all), and each citation — the hearer's form and vertex,
/// the lane, the exponent.
///
/// THE QUESTION (78.46, MU-CLIENT §5.3): `asks` is the word's RESIDUE against
/// what the hearer knows — the ring posited over the hearer's known words,
/// and the part none of them reaches, exact — where the hearer does not know
/// the word. A ring whose coordinates do not read is asked whole. The
/// speaker answers by factoring it (`Carrier::heard_back`), the factors in
/// the order they compose to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeardWord {
    pub vertex: Vec<u8>,
    pub derived: Option<Mv>,
    pub gauge: Vec<(u8, u8)>,
    pub cites: Vec<(Vec<u8>, Vec<u8>, u8, i64)>,
    pub asks: Option<Mv>,
    /// THE HEARER'S OWN RINGS the heard ring resonated with — the held
    /// shortcuts of the layer it seated in: its re-derivation, which the
    /// speaker nests in its own rings for MUTUAL balance (MU-CLIENT §4.9).
    pub held: Vec<Mv>,
    /// THE HEARER'S VANTAGE (MU-CLIENT §5.3): its own known words it read the
    /// ring through — empty where it knows the ring through none of its own.
    pub vantage: Vec<Mv>,
}

/// 78.163 — ONE WORD'S LOOP, ACKNOWLEDGED: the holonomy of said → heard on
/// the motion (`None`: no loop closed — nothing derived, or no versor said)
/// and on the hue map (`None`: the said map is no permutation, so the loop
/// cannot close on G).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AckWord {
    pub vertex: Vec<u8>,
    pub motion: Option<Mv>,
    pub hue: Option<Vec<(u8, u8)>>,
}

/// MU-CLIENT §5.5 — A WORD'S EVIDENCE: the citation of the scene it was
/// spoken under and the word's own record, so the hearer decompresses the
/// word onto that scene (`family::local::word_local`) and reads what it does
/// there — the effect two different constructions converge on, never the
/// word's bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evidence {
    pub scene: Vec<u8>,
    pub word: Vec<u8>,
}

/// WHAT A FRAME SAYS.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "a frame is decoded, read once and dropped; the hold's exact strain per lane is its content, not a field to box away")]
pub enum Said {
    /// What the speaker stood (`said`, its address), word by word, and its
    /// evidence where it is a word spoken under a scene.
    Say { said: Vec<u8>, words: Vec<Spoken>, evidence: Option<Evidence> },
    /// What the hearer re-derived of something said, word by word.
    Heard { said: Vec<u8>, words: Vec<HeardWord> },
    /// 78.163 — the speaker's reading of every loop the hearer's thought closed.
    Ack { said: Vec<u8>, words: Vec<AckWord> },
    /// 78.163 — the hearer's own reading of the same loops, and (78.112 H5)
    /// the exact strain they carried, per lane: how far the hearer's language
    /// moved the thought, priced in its own field.
    Hold { said: Vec<u8>, words: Vec<AckWord>, deviation: [num_rational::BigRational; 4] },
}

/// ONE FRAME: who said it, at which final point, and what.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chatter {
    pub speaker: [u8; 32],
    pub point: Option<TPoint>,
    pub said: Said,
}

fn put_short(out: &mut Vec<u8>, b: &[u8]) -> Option<()> {
    out.extend_from_slice(&u16::try_from(b.len()).ok()?.to_le_bytes());
    out.extend_from_slice(b);
    Some(())
}

fn take_short(b: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let n = usize::from(u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?));
    let out = b.get(*p + 2..*p + 2 + n)?.to_vec();
    *p += 2 + n;
    Some(out)
}

fn put_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u16::try_from(n).ok()?.to_le_bytes());
    Some(())
}

fn take_count(b: &[u8], p: &mut usize) -> Option<usize> {
    let n = usize::from(u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?));
    *p += 2;
    Some(n)
}

fn put_gauge(out: &mut Vec<u8>, g: &[(u8, u8)]) -> Option<()> {
    out.push(u8::try_from(g.len()).ok()?);
    for &(a, b) in g {
        out.push(a);
        out.push(b);
    }
    Some(())
}

fn take_gauge(b: &[u8], p: &mut usize) -> Option<Vec<(u8, u8)>> {
    let k = usize::from(*b.get(*p)?);
    *p += 1;
    let mut g = Vec::with_capacity(k);
    for _ in 0..k {
        g.push((*b.get(*p)?, *b.get(*p + 1)?));
        *p += 2;
    }
    Some(g)
}

fn put_mv(out: &mut Vec<u8>, m: Option<&Mv>) {
    match m {
        Some(m) => {
            out.push(1);
            m.to_bytes(out);
        }
        None => out.push(0),
    }
}

fn put_mvs(out: &mut Vec<u8>, ms: &[Mv]) -> Option<()> {
    put_count(out, ms.len())?;
    for m in ms {
        m.to_bytes(out);
    }
    Some(())
}

fn take_mvs(b: &[u8], p: &mut usize) -> Option<Vec<Mv>> {
    (0..take_count(b, p)?).map(|_| Mv::from_bytes(b, p)).collect()
}

fn take_mv(b: &[u8], p: &mut usize) -> Option<Option<Mv>> {
    let flag = *b.get(*p)?;
    *p += 1;
    match flag {
        0 => Some(None),
        1 => Some(Some(Mv::from_bytes(b, p)?)),
        _ => None,
    }
}

impl Chatter {
    fn sub(&self) -> u8 {
        match self.said {
            Said::Say { .. } => SAY,
            Said::Heard { .. } => HEARD,
            Said::Ack { .. } => ACK,
            Said::Hold { .. } => HOLD,
        }
    }

    /// The signed bytes after the tag: sub, speaker, point, fields. `None`
    /// where a field passes its length's carrier (rule 11, never widened).
    fn body(&self) -> Option<Vec<u8>> {
        let mut out = vec![self.sub()];
        out.extend_from_slice(&self.speaker);
        match &self.point {
            Some(p) => {
                out.push(1);
                p.put(&mut out);
            }
            None => out.push(0),
        }
        match &self.said {
            Said::Say { said, words, evidence } => {
                put_short(&mut out, said)?;
                put_count(&mut out, words.len())?;
                for w in words {
                    put_short(&mut out, &w.vertex)?;
                    w.motion.to_bytes(&mut out);
                    put_gauge(&mut out, &w.gauge)?;
                    put_count(&mut out, w.terms.len())?;
                    for (word, transport) in &w.terms {
                        word.to_bytes(&mut out);
                        transport.to_bytes(&mut out);
                    }
                    put_mvs(&mut out, &w.vantage)?;
                }
                match evidence {
                    Some(e) => {
                        out.push(1);
                        put_short(&mut out, &e.scene)?;
                        put_short(&mut out, &e.word)?;
                    }
                    None => out.push(0),
                }
            }
            Said::Ack { said, words } | Said::Hold { said, words, .. } => {
                put_short(&mut out, said)?;
                put_count(&mut out, words.len())?;
                for w in words {
                    put_short(&mut out, &w.vertex)?;
                    put_mv(&mut out, w.motion.as_ref());
                    match &w.hue {
                        Some(h) => {
                            out.push(1);
                            put_gauge(&mut out, h)?;
                        }
                        None => out.push(0),
                    }
                }
                // The deviation, each lane an exact rational written as the
                // scalar the algebra's own codec carries.
                if let Said::Hold { deviation, .. } = &self.said {
                    for d in deviation {
                        Mv::scalar(d.clone()).to_bytes(&mut out);
                    }
                }
            }
            Said::Heard { said, words } => {
                put_short(&mut out, said)?;
                put_count(&mut out, words.len())?;
                for w in words {
                    put_short(&mut out, &w.vertex)?;
                    put_mv(&mut out, w.derived.as_ref());
                    put_gauge(&mut out, &w.gauge)?;
                    put_count(&mut out, w.cites.len())?;
                    for (form, vertex, lane, e) in &w.cites {
                        put_short(&mut out, form)?;
                        put_short(&mut out, vertex)?;
                        out.push(*lane);
                        out.extend_from_slice(&e.to_le_bytes());
                    }
                    put_mv(&mut out, w.asks.as_ref());
                    put_mvs(&mut out, &w.held)?;
                    put_mvs(&mut out, &w.vantage)?;
                }
            }
        }
        Some(out)
    }

    /// THE FRAME, signed by the speaker's key, which must be the speaker's.
    pub fn sign(&self, key: &ed25519_dalek::SigningKey) -> Option<Vec<u8>> {
        use ed25519_dalek::Signer;
        if key.verifying_key().to_bytes() != self.speaker {
            return None;
        }
        let body = self.body()?;
        let mut signed = SIGNED.to_vec();
        signed.extend_from_slice(&body);
        let sig = key.sign(&signed);
        let mut out = vec![TAG];
        out.extend_from_slice(&body);
        out.extend_from_slice(&sig.to_bytes());
        Some(out)
    }

    /// A FRAME READ AND ITS SIGNATURE VERIFIED against the speaker it names:
    /// whole, or `None`.
    pub fn verify(frame: &[u8]) -> Option<Chatter> {
        use ed25519_dalek::Verifier;
        let (&tag, rest) = frame.split_first()?;
        if tag != TAG || rest.len() < 64 {
            return None;
        }
        let (body, sig) = rest.split_at(rest.len() - 64);
        let mut p = 0usize;
        let (&sub, _) = body.split_first()?;
        p += 1;
        let speaker: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        let point = match *body.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                Some(TPoint::take(body, &mut p)?)
            }
            _ => return None,
        };
        let said = match sub {
            SAY => {
                let said = take_short(body, &mut p)?;
                let n = take_count(body, &mut p)?;
                let mut words = Vec::new();
                for _ in 0..n {
                    let vertex = take_short(body, &mut p)?;
                    let motion = Mv::from_bytes(body, &mut p)?;
                    let gauge = take_gauge(body, &mut p)?;
                    let m = take_count(body, &mut p)?;
                    let mut terms = Vec::new();
                    for _ in 0..m {
                        let word = Mv::from_bytes(body, &mut p)?;
                        let transport = Mv::from_bytes(body, &mut p)?;
                        terms.push((word, transport));
                    }
                    let vantage = take_mvs(body, &mut p)?;
                    words.push(Spoken { vertex, motion, gauge, terms, vantage });
                }
                let flag = *body.get(p)?;
                p += 1;
                let evidence = match flag {
                    0 => None,
                    1 => Some(Evidence { scene: take_short(body, &mut p)?, word: take_short(body, &mut p)? }),
                    _ => return None,
                };
                Said::Say { said, words, evidence }
            }
            k @ (ACK | HOLD) => {
                let said = take_short(body, &mut p)?;
                let n = take_count(body, &mut p)?;
                let mut words = Vec::new();
                for _ in 0..n {
                    let vertex = take_short(body, &mut p)?;
                    let motion = take_mv(body, &mut p)?;
                    let flag = *body.get(p)?;
                    p += 1;
                    let hue = match flag {
                        0 => None,
                        1 => Some(take_gauge(body, &mut p)?),
                        _ => return None,
                    };
                    words.push(AckWord { vertex, motion, hue });
                }
                match k {
                    ACK => Said::Ack { said, words },
                    _ => {
                        let mut lane = || Mv::from_bytes(body, &mut p).map(|m| m.scalar_part());
                        let deviation = [lane()?, lane()?, lane()?, lane()?];
                        Said::Hold { said, words, deviation }
                    }
                }
            }
            HEARD => {
                let said = take_short(body, &mut p)?;
                let n = take_count(body, &mut p)?;
                let mut words = Vec::new();
                for _ in 0..n {
                    let vertex = take_short(body, &mut p)?;
                    let derived = take_mv(body, &mut p)?;
                    let gauge = take_gauge(body, &mut p)?;
                    let m = take_count(body, &mut p)?;
                    let mut cites = Vec::new();
                    for _ in 0..m {
                        let form = take_short(body, &mut p)?;
                        let v = take_short(body, &mut p)?;
                        let lane = *body.get(p)?;
                        p += 1;
                        let e = i64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
                        p += 8;
                        cites.push((form, v, lane, e));
                    }
                    let asks = take_mv(body, &mut p)?;
                    let held = take_mvs(body, &mut p)?;
                    let vantage = take_mvs(body, &mut p)?;
                    words.push(HeardWord { vertex, derived, gauge, cites, asks, held, vantage });
                }
                Said::Heard { said, words }
            }
            _ => return None,
        };
        if p != body.len() {
            return None;
        }
        let vk = ed25519_dalek::VerifyingKey::from_bytes(&speaker).ok()?;
        let sig = ed25519_dalek::Signature::from_bytes(sig.try_into().ok()?);
        let mut signed = SIGNED.to_vec();
        signed.extend_from_slice(body);
        vk.verify(&signed, &sig).ok()?;
        Some(Chatter { speaker, point, said })
    }
}

/// One frame off a connection: `[len u32 LE][frame]`, within the wire's one
/// budget, or nothing.
pub fn read_frame(s: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut len = [0u8; 4];
    s.read_exact(&mut len)?;
    let n = usize::try_from(u32::from_le_bytes(len)).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "length"))?;
    if n > crate::network::wire::BODY_BUDGET {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "chatter frame over the wire budget"));
    }
    let mut frame = vec![0u8; n];
    s.read_exact(&mut frame)?;
    Ok(frame)
}

/// One frame onto a connection: `[len u32 LE][frame]`.
pub fn write_frame(s: &mut TcpStream, frame: &[u8]) -> io::Result<()> {
    let n = u32::try_from(frame.len()).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "frame length"))?;
    s.write_all(&n.to_le_bytes())?;
    s.write_all(frame)?;
    s.flush()
}

/// SEND one frame to a peer, on its own connection.
pub fn send(endpoint: &str, frame: &[u8]) -> io::Result<()> {
    let mut s = TcpStream::connect(endpoint)?;
    write_frame(&mut s, frame)
}

/// ASK a mind's post one question and read its answer, on one connection.
pub fn ask(endpoint: &str, question: &[u8]) -> io::Result<Vec<u8>> {
    let mut s = TcpStream::connect(endpoint)?;
    write_frame(&mut s, question)?;
    read_frame(&mut s)
}
