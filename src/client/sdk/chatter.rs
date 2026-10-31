// 78.161 — CHATTER, THE MIND'S SIDE: each mind speaks its dialect to the
// others. The frame and its framing are Anastome's
// (`anastome::dialogue::chatter`); here are the loop readers and the post.
//
//! THE LOOP. ACK and HOLD carry 78.163's reading of each word's LOOP
//! HOLONOMY, what the round trip said → heard carried it to (`holonomy`,
//! `hue_holonomy`). Each mind verifies the other's by the algebra
//! (`same_loop`); where both hold one loop on every word, the thought is
//! shared, and what the two minds' resonance leaves is held in each mind's
//! field — never by a nerve.
//!
//! THE TRANSPORT. Each mind listens on its own address and posts every frame
//! it hears into its inbox, drained by the mind on its stroke. A frame is
//! sent on its own connection (`send`). Peers are found at a rendezvous
//! directory — each mind files its listening address under its voice's hex
//! — and at `MYCELIUM_CHATTER_PEERS` (comma-separated), for minds on other
//! hosts.

pub use crate::anastome::dialogue::chatter::*;
use crate::transducer::cga::Mv;
use std::io;
use std::net::TcpListener;

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

/// Two acknowledgements of one word name one loop: the two motions
/// projectively one (1.5), the two hue holonomies one map.
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

/// THE POST: this mind's listening address. Every frame heard on it is
/// delivered at once to the carrier that converses (MU-CLIENT §5.1), which
/// sleeps until one arrives — no inbox polled, no clock.
pub struct Post {
    endpoint: String,
}

impl Post {
    /// LISTEN at `listen` (`127.0.0.1:0` takes any free port), each frame
    /// handed to `deliver` as it is read: every connection is read on its
    /// own thread, so a peer that stalls holds nothing else up. A frame that
    /// is a QUESTION `answer` takes is answered on the same connection, in
    /// the same framing, and delivered nowhere: the observer asks the
    /// running mind and never waits on its conversation.
    pub fn listen<D, A>(listen: &str, deliver: D, answer: A) -> io::Result<Post>
    where
        D: Fn(Vec<u8>) + Send + Clone + 'static,
        A: Fn(&[u8]) -> Option<Vec<u8>> + Send + Clone + 'static,
    {
        let listener = TcpListener::bind(listen)?;
        let endpoint = listener.local_addr()?.to_string();
        std::thread::Builder::new().name("chatter-post".into()).spawn(move || {
            for conn in listener.incoming() {
                let Ok(mut s) = conn else { continue };
                let (deliver, answer) = (deliver.clone(), answer.clone());
                let _ = std::thread::Builder::new().name("chatter-conn".into()).spawn(move || {
                    let Ok(frame) = read_frame(&mut s) else { return };
                    match answer(&frame) {
                        Some(reply) => {
                            let _ = write_frame(&mut s, &reply);
                        }
                        None => deliver(frame),
                    }
                });
            }
        })?;
        Ok(Post { endpoint })
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// FILE THIS MIND'S ADDRESS at the rendezvous, under its voice.
pub fn publish(dir: &std::path::Path, voice: &[u8; 32], endpoint: &str) -> io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(".{}.tmp", hex(voice)));
    std::fs::write(&tmp, endpoint.as_bytes())?;
    std::fs::rename(tmp, dir.join(hex(voice)))
}

/// ONE MIND'S ADDRESS at the rendezvous, by its voice: where an answer to it
/// is sent. `None` where it filed none.
pub fn peer_of(dir: &std::path::Path, voice: &[u8; 32]) -> Option<String> {
    let endpoint = std::fs::read_to_string(dir.join(hex(voice))).ok()?;
    let endpoint = endpoint.trim().to_string();
    (!endpoint.is_empty()).then_some(endpoint)
}

/// A RENDEZVOUS ENTRY NOTHING LISTENS AT is removed: the mind that filed
/// it is gone (a refused connection says so exactly), and a frame sent there
/// again would only be refused again.
pub fn forget(dir: &std::path::Path, endpoint: &str) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        if std::fs::read_to_string(e.path()).is_ok_and(|a| a.trim() == endpoint) {
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
            if let Ok(endpoint) = std::fs::read_to_string(e.path()) {
                let endpoint = endpoint.trim().to_string();
                if !endpoint.is_empty() {
                    at.insert(endpoint.clone().as_bytes(), endpoint);
                }
            }
        }
    }
    if let Ok(list) = std::env::var("MYCELIUM_CHATTER_PEERS") {
        for endpoint in list.split(',').map(str::trim).filter(|a| !a.is_empty()) {
            at.insert(endpoint.as_bytes(), endpoint.to_string());
        }
    }
    at.scoped().map(|(_, a)| a.clone()).collect()
}
