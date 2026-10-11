// 78.161 — CHATTER: minds speak their dialect to each other. A frame is a
// voice's signed word, its agreement, or its divergence; the hypha carries
// none of it and judges none of it.
//
//! THE WIRE. A frame is `[0x43][sub u8][speaker 32][point: 0 | 1 TPoint
//! 84][fields][sig 64]`, the signature ed25519 over `"mycelium chatter v2" ‖
//! sub ‖ speaker ‖ point ‖ fields`. Sub-kinds:
//! - SAY `[said: len u16, bytes][count u16]` then per word `[vertex: len u16,
//!   bytes][motion Mv][gauge: count u8, (from, to)…]`: what this mind stood,
//!   each word by its whole Cl(4,1) motion and its hue map — the mathematics
//!   a hearer re-derives, never a tape it would compare;
//! - HEARD `[said: len u16, bytes][count u16]` then per word `[vertex: len
//!   u16, bytes][derived: 0 | 1 Mv][gauge: count u8, (from, to)…][cites
//!   count u16]` and per citation `[form: len u16, bytes][vertex: len u16,
//!   bytes][lane u8][exponent i64 LE]`: how the hearer said each word in its
//!   own language — the motion and hue map its cited generators compose to,
//!   and the citations — never a verdict on it;
//! - ACK `[said: len u16, bytes][count u16]` then per word `[vertex: len u16,
//!   bytes][motion: 0 | 1 Mv][hue: 0 | 1, count u8, (from, to)…]`: 78.163,
//!   the speaker's reading of the hearer's thought — each word's LOOP
//!   HOLONOMY, what the round trip said → heard carried it to (`holonomy`,
//!   `hue_holonomy`). The holonomy is the content: what one mind's language
//!   does to the other's thought; still only where the hearer echoed.
//! - HOLD, the same fields as ACK: the hearer's own reading of every loop,
//!   re-derived from what it heard and what it derived. Each mind verifies
//!   the other's by the algebra (`same_loop`); where both hold one loop on
//!   every word, the thought is shared and each mind's ear holds every
//!   curved holonomy as a transport between the two.
//!
//! THE TRANSPORT. Each mind listens on its own address and posts every frame
//! it hears into its inbox, drained by the mind on its stroke. A frame is
//! sent on its own connection, `[len u32 LE][frame]`. Peers are found at a
//! rendezvous directory — each mind files its listening address under its
//! voice's hex — and at `MYCELIUM_CHATTER_PEERS` (comma-separated), for
//! minds on other hosts.

use crate::chronometry::tuniversal::TPoint;
use crate::transducer::cga::Mv;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub const TAG: u8 = 0x43;
pub const SAY: u8 = 0;
pub const HEARD: u8 = 1;
pub const ACK: u8 = 2;
pub const HOLD: u8 = 3;
const SIGNED: &[u8] = b"mycelium chatter v2";

/// ONE WORD SAID: the vertex it is in the speaker's form, its whole motion,
/// its hue map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spoken {
    pub vertex: Vec<u8>,
    pub motion: Mv,
    pub gauge: Vec<(u8, u8)>,
}

/// ONE WORD HEARD, as the hearer said it in its own language: the motion and
/// the hue map its cited generators compose to (`None`: the hearer could not
/// read the word at all), and each citation — the hearer's form and vertex,
/// the lane, the exponent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeardWord {
    pub vertex: Vec<u8>,
    pub derived: Option<Mv>,
    pub gauge: Vec<(u8, u8)>,
    pub cites: Vec<(Vec<u8>, Vec<u8>, u8, i64)>,
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

/// WHAT A FRAME SAYS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Said {
    /// What the speaker stood (`said`, its address), word by word.
    Say { said: Vec<u8>, words: Vec<Spoken> },
    /// What the hearer re-derived of something said, word by word.
    Heard { said: Vec<u8>, words: Vec<HeardWord> },
    /// 78.163 — the speaker's reading of every loop the hearer's thought closed.
    Ack { said: Vec<u8>, words: Vec<AckWord> },
    /// 78.163 — the hearer's own reading of the same loops.
    Hold { said: Vec<u8>, words: Vec<AckWord> },
}

/// 78.163 — THE LOOP'S HOLONOMY ON THE MOTION: what the round trip said →
/// heard carries the said motion `m` to, `derived · m̃` — the hearer's
/// derivation composed with the reverse of what was said, projectively
/// `derived · m⁻¹` (1.5). Two minds are two charts, and carrying a thought
/// across them is transport on a curved space: the holonomy is the content —
/// what one mind's language does to the other's thought. Still only where
/// the hearer echoed. `None` where `m` is no versor (`m m̃` no non-zero
/// scalar): no loop closes on it.
pub fn holonomy(said: &Mv, derived: &Mv) -> Option<Mv> {
    let norm = said.geo(&said.reverse());
    let s = norm.scalar_part();
    if num_traits::Zero::is_zero(&s) || !norm.sub(&Mv::scalar(s)).is_zero() {
        return None;
    }
    Some(derived.geo(&said.reverse()))
}

/// 78.163 — THE LOOP'S HOLONOMY ON THE HUE MAP: `derived ∘ said⁻¹`, the hue
/// map the round trip leaves. `None` where the said map is no permutation of
/// the hues it lists — a merge has no inverse, and the loop cannot close on
/// G.
pub fn hue_holonomy(said: &[(u8, u8)], derived: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut from: Vec<u8> = said.iter().map(|&(a, _)| a).collect();
    let mut to: Vec<u8> = said.iter().map(|&(_, b)| b).collect();
    from.sort_unstable();
    to.sort_unstable();
    let distinct = from.windows(2).all(|w| w[0] != w[1]);
    if !distinct || from != to {
        return None;
    }
    let inverse: Vec<(u8, u8)> = said.iter().map(|&(a, b)| (b, a)).collect();
    Some(crate::transducer::sentence::transport::compose_gauge(derived, &inverse))
}

/// The loop is STILL — the motion's holonomy still and the hue map's the
/// identity: the hearer echoed, and the loop carries no content.
pub fn still(a: &AckWord) -> bool {
    let motion = a.motion.as_ref().is_some_and(crate::transducer::sentence::presentation::is_still);
    let hue = a.hue.as_ref().is_some_and(|h| crate::transducer::sentence::transport::gauge_eq(h, &[]));
    motion && hue
}

/// ONE WORD'S LOOP read: its holonomy on the motion and on the hue map, from
/// what was said and what was derived — the one reader both minds use.
pub fn loop_of(said: &Spoken, heard: &HeardWord) -> AckWord {
    AckWord {
        vertex: said.vertex.clone(),
        motion: heard.derived.as_ref().and_then(|d| holonomy(&said.motion, d)),
        hue: heard.derived.as_ref().and_then(|_| hue_holonomy(&said.gauge, &heard.gauge)),
    }
}

/// Two acknowledgements of one word name one loop: their motions
/// projectively one (1.5), their hue holonomies one map.
pub fn same_loop(a: &AckWord, b: &AckWord) -> bool {
    let motion = match (&a.motion, &b.motion) {
        (Some(x), Some(y)) => x.projective_eq(y),
        (None, None) => true,
        _ => false,
    };
    let hue = match (&a.hue, &b.hue) {
        (Some(x), Some(y)) => crate::transducer::sentence::transport::gauge_eq(x, y),
        (None, None) => true,
        _ => false,
    };
    a.vertex == b.vertex && motion && hue
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
            Said::Say { said, words } => {
                put_short(&mut out, said)?;
                put_count(&mut out, words.len())?;
                for w in words {
                    put_short(&mut out, &w.vertex)?;
                    w.motion.to_bytes(&mut out);
                    put_gauge(&mut out, &w.gauge)?;
                }
            }
            Said::Ack { said, words } | Said::Hold { said, words } => {
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
    pub fn open(frame: &[u8]) -> Option<Chatter> {
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
                    words.push(Spoken { vertex, motion, gauge });
                }
                Said::Say { said, words }
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
                    _ => Said::Hold { said, words },
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
                    words.push(HeardWord { vertex, derived, gauge, cites });
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

/// THE POST: this mind's listening address and the frames heard on it,
/// waiting for the mind's stroke.
pub struct Post {
    addr: String,
    inbox: Arc<Mutex<std::collections::VecDeque<Vec<u8>>>>,
}

/// One frame off a connection: `[len u32 LE][frame]`, within the wire's one
/// budget, or nothing.
fn read_frame(s: &mut TcpStream) -> io::Result<Vec<u8>> {
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

impl Post {
    /// LISTEN at `listen` (`127.0.0.1:0` takes any free port): every
    /// connection is read on its own thread, so a peer that stalls holds
    /// nothing else up.
    pub fn open(listen: &str) -> io::Result<Post> {
        let listener = TcpListener::bind(listen)?;
        let addr = listener.local_addr()?.to_string();
        let inbox: Arc<Mutex<std::collections::VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(std::collections::VecDeque::new()));
        let into = Arc::clone(&inbox);
        std::thread::Builder::new().name("chatter-post".into()).spawn(move || {
            for conn in listener.incoming() {
                let Ok(mut s) = conn else { continue };
                let into = Arc::clone(&into);
                let _ = std::thread::Builder::new().name("chatter-conn".into()).spawn(move || {
                    if let Ok(frame) = read_frame(&mut s) {
                        if let Ok(mut held) = into.lock() {
                            held.push_back(frame);
                        }
                    }
                });
            }
        })?;
        Ok(Post { addr, inbox })
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// Every frame heard since the last take, in the order heard.
    pub fn take(&self) -> Vec<Vec<u8>> {
        match self.inbox.lock() {
            Ok(mut held) => std::mem::take(&mut *held).into(),
            Err(_) => Vec::new(),
        }
    }

    /// THE NEXT FRAME heard, the earliest first; the rest stay in the post
    /// for the mind's next tick — a stroke that ran out hears them later,
    /// never drops them.
    pub fn next(&self) -> Option<Vec<u8>> {
        self.inbox.lock().ok()?.pop_front()
    }
}

/// SEND one frame to a peer, on its own connection.
pub fn send(addr: &str, frame: &[u8]) -> io::Result<()> {
    let mut s = TcpStream::connect(addr)?;
    let n = u32::try_from(frame.len()).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "frame length"))?;
    s.write_all(&n.to_le_bytes())?;
    s.write_all(frame)?;
    s.flush()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// FILE THIS MIND'S ADDRESS at the rendezvous, under its voice.
pub fn publish(dir: &std::path::Path, voice: &[u8; 32], addr: &str) -> io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(".{}.tmp", hex(voice)));
    std::fs::write(&tmp, addr.as_bytes())?;
    std::fs::rename(tmp, dir.join(hex(voice)))
}

/// ONE MIND'S ADDRESS at the rendezvous, by its voice: where an answer to it
/// is sent. `None` where it filed none.
pub fn peer_of(dir: &std::path::Path, voice: &[u8; 32]) -> Option<String> {
    let addr = std::fs::read_to_string(dir.join(hex(voice))).ok()?;
    let addr = addr.trim().to_string();
    (!addr.is_empty()).then_some(addr)
}

/// A RENDEZVOUS ENTRY NOTHING LISTENS AT is removed: the mind that filed
/// it is gone (a refused connection says so exactly), and a frame sent there
/// again would only be refused again.
pub fn forget(dir: &std::path::Path, addr: &str) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        if std::fs::read_to_string(e.path()).is_ok_and(|a| a.trim() == addr) {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

/// THE PEERS: every other mind's address at the rendezvous, and the
/// addresses `MYCELIUM_CHATTER_PEERS` names, each once, in key order.
pub fn peers(dir: &std::path::Path, voice: &[u8; 32]) -> Vec<String> {
    let own = hex(voice);
    let mut at: crate::network::nerve::Keyed<String> = crate::network::nerve::Keyed::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name == own || name.starts_with('.') {
                continue;
            }
            if let Ok(addr) = std::fs::read_to_string(e.path()) {
                let addr = addr.trim().to_string();
                if !addr.is_empty() {
                    at.insert(addr.clone().as_bytes(), addr);
                }
            }
        }
    }
    if let Ok(list) = std::env::var("MYCELIUM_CHATTER_PEERS") {
        for addr in list.split(',').map(str::trim).filter(|a| !a.is_empty()) {
            at.insert(addr.as_bytes(), addr.to_string());
        }
    }
    at.scoped().map(|(_, a)| a.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(b: u8) -> ed25519_dalek::SigningKey {
        ed25519_dalek::SigningKey::from_bytes(&[b; 32])
    }

    #[test]
    fn every_sub_kind_reads_back_signed_and_a_forged_one_does_not() {
        let k = key(7);
        let speaker = k.verifying_key().to_bytes();
        let point = TPoint { h: 9, lanes: [1, 2, 3, 4], k: 5, ..TPoint::default() };
        let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
        for said in [
            Said::Say { said: vec![1, 2, 3], words: vec![Spoken { vertex: vec![4; 40], motion: one.clone(), gauge: vec![(1, 2), (2, 1)] }] },
            Said::Heard {
                said: vec![6; 10],
                words: vec![
                    HeardWord { vertex: vec![4; 40], derived: Some(one.clone()), gauge: vec![(3, 4)], cites: vec![(vec![7; 33], vec![8; 34], 1, -3)] },
                    HeardWord { vertex: vec![5; 3], derived: None, gauge: vec![], cites: vec![] },
                ],
            },
            Said::Ack {
                said: vec![2; 7],
                words: vec![
                    AckWord { vertex: vec![4; 40], motion: Some(one.clone()), hue: Some(vec![]) },
                    AckWord { vertex: vec![5; 3], motion: None, hue: None },
                ],
            },
            Said::Hold { said: vec![2; 7], words: vec![AckWord { vertex: vec![4; 40], motion: Some(one.clone()), hue: None }] },
        ] {
            let c = Chatter { speaker, point: Some(point), said };
            let f = c.sign(&k).expect("signed");
            assert_eq!(Chatter::open(&f), Some(c.clone()));
            let mut forged = f.clone();
            let mid = forged.len() / 2;
            forged[mid] ^= 1;
            assert_eq!(Chatter::open(&forged), None, "a changed byte does not verify");
        }
        // Signed by another key than the speaker it names: refused at signing.
        let c = Chatter { speaker, point: None, said: Said::Say { said: vec![], words: vec![] } };
        assert_eq!(c.sign(&key(8)), None);
    }

    /// 78.163 — THE LOOP said → heard, BY THE ALGEBRA: a hearer that echoed
    /// the turn closes it still; one that said the turn twice leaves the said
    /// turn itself as the holonomy — what its language did to the thought; a
    /// swap heard as the swap closes the hue loop still, heard as nothing
    /// leaves the swap; a merge has no inverse and the hue loop stays open; a
    /// word the hearer could not read closes nothing; and both minds read one
    /// loop, projectively.
    #[test]
    fn the_loop_said_heard_carries_what_the_hearers_language_did_to_the_thought() {
        use crate::transducer::cga::{E1, E2};
        let q = |n: i64| num_rational::BigRational::from_integer(n.into());
        let turn = Mv::scalar(q(1)).add(&Mv::blade(E1 | E2, q(1))).widened(3);
        let swap = vec![(1, 2), (2, 1)];
        let said = Spoken { vertex: vec![1], motion: turn.clone(), gauge: swap.clone() };
        let heard = |derived: Option<Mv>, gauge: Vec<(u8, u8)>| HeardWord { vertex: vec![1], derived, gauge, cites: vec![] };
        let exact = loop_of(&said, &heard(Some(turn.clone()), swap.clone()));
        assert!(still(&exact), "an echo carries nothing: {exact:?}");
        let twice = loop_of(&said, &heard(Some(turn.geo(&turn)), swap.clone()));
        assert!(!still(&twice));
        let h = twice.motion.clone().expect("a loop closes on a versor");
        assert!(h.projective_eq(&turn), "what did not cross is the said turn itself");
        let hueless = loop_of(&said, &heard(Some(turn.clone()), vec![]));
        assert!(!still(&hueless), "the swap heard as nothing leaves the swap");
        assert!(hueless.hue.as_ref().is_some_and(|m| crate::transducer::sentence::transport::gauge_eq(m, &swap)));
        let merge = Spoken { vertex: vec![1], motion: turn.clone(), gauge: vec![(3, 0)] };
        assert_eq!(loop_of(&merge, &heard(Some(turn.clone()), vec![(3, 0)])).hue, None, "a merge has no inverse: G stays open");
        let unread = loop_of(&said, &heard(None, vec![]));
        assert_eq!((unread.motion, unread.hue), (None, None), "nothing derived closes nothing");
        assert!(same_loop(&exact, &loop_of(&said, &heard(Some(turn.scale(&q(3))), swap))), "one loop, read projectively by both minds");
    }

    #[test]
    fn a_frame_sent_to_a_post_is_heard_there() {
        let post = Post::open("127.0.0.1:0").expect("listen");
        let k = key(3);
        let c = Chatter { speaker: k.verifying_key().to_bytes(), point: None, said: Said::Say { said: vec![9; 100], words: vec![] } };
        send(post.addr(), &c.sign(&k).expect("signed")).expect("sent");
        let mut heard = Vec::new();
        for _ in 0..200 {
            heard = post.take();
            if !heard.is_empty() {
                break;
            }
            std::thread::yield_now();
            std::thread::park_timeout(std::time::Duration::from_millis(5));
        }
        assert_eq!(heard.len(), 1);
        assert_eq!(Chatter::open(&heard[0]), Some(c));
    }

    #[test]
    fn the_rendezvous_names_every_other_mind() {
        let dir = std::env::temp_dir().join(format!("chatter-rv-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        publish(&dir, &[1; 32], "127.0.0.1:1").expect("published");
        publish(&dir, &[2; 32], "127.0.0.1:2").expect("published");
        assert_eq!(peers(&dir, &[1; 32]), vec!["127.0.0.1:2".to_string()]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
