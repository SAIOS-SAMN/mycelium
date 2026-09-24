// 47.4 — ACT_EQUIVOCATION: a third party presents two receipts under one
// key at one K, and this court burns the signer's bond.
//
//! Nothing is replayed. Both prefixes name this node's origin, both
//! signatures verify under the key each receipt names, each receipt is one
//! claim with its prefix and re-derives its hash, and the two differ
//! (`admit_equivocation`); then the burn, once. The reply is `Ruled`.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(s: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some((a, b)) = dialogue::decode_equivocation(&frame.body) else {
        return;
    };
    let say = |s: &mut TcpStream, r: dialogue::Ruled| {
        match dialogue::encode_ruled(&r) {
            Some(body) => answer(s, Frame { lane: frame.lane, body }),
            None => eprintln!("  court equivocation answer refused: the reason passes its u16"),
        }
    };
    let refuse = |s: &mut TcpStream, why: &str| {
        eprintln!(
            "  court equivocation refused {:02x}{:02x}… K {}: {why}",
            a.receipt.stamp.vk[0], a.receipt.stamp.vk[1], a.receipt.stamp.place().print()
        );
        say(s, dialogue::Ruled::Refused(why.as_bytes().to_vec()));
    };
    // 47.2 steps 1 and 2 on each half, before any carrier is touched. A
    // node that has adopted no origin has none equal to the frame's.
    let origin = match door.mesh.origin.lock() {
        Ok(o) => *o,
        Err(_) => None,
    };
    let Some(origin) = origin else {
        refuse(s, mycelium::verdict::REFUSE_ORIGIN);
        return;
    };
    let va = match mycelium::court::admission::admit_signature(origin, &a.prefix, &a.receipt.stamp.vk, &a.sig) {
        Ok(v) => v,
        Err(why) => {
            refuse(s, why);
            return;
        }
    };
    let vb = match mycelium::court::admission::admit_signature(origin, &b.prefix, &b.receipt.stamp.vk, &b.sig) {
        Ok(v) => v,
        Err(why) => {
            refuse(s, why);
            return;
        }
    };
    let proof = match mycelium::court::admission::admit_equivocation(va, &a.receipt, vb, &b.receipt) {
        Ok(p) => p,
        Err(why) => {
            refuse(s, why);
            return;
        }
    };
    // A court that cannot be read has judged nothing and answers nothing.
    let Ok(mut k) = door.kernel.lock() else {
        eprintln!(
            "  court equivocation unread {:02x}{:02x}… K {}: the court is not readable",
            proof.vk[0], proof.vk[1], proof.at.print()
        );
        return;
    };
    let burned = k.accept_equivocation(proof);
    drop(k);
    // The burn line is the kernel's (34.2); the reply says whether anything
    // left: the same proof again finds no bond and is inert.
    match burned {
        Some(_) => say(s, dialogue::Ruled::Applied),
        None => say(s, dialogue::Ruled::Inert),
    }
}
