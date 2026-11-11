// 47.4 — ACT_EQUIVOCATION: a third party presents two receipts under one
// key at one K, and this court burns the signer's bond.
//
//! Nothing is reconstructed. Both prefixes name this node's origin, both
//! signatures verify under the key each receipt names, each receipt is one
//! claim with its prefix and re-derives its hash, and the two differ
//! (`admit_equivocation`); then the burn, once. The reply is `Ruled`.

use crate::context::Context;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Delimited;
use mycelium::network::channel::Channel;

pub(crate) fn judge(s: &mut Channel, delimited: &Delimited, context: &Context) {
    let Some((a, b)) = dialogue::decode_equivocation(&delimited.body) else {
        return;
    };
    let say = |s: &mut Channel, r: dialogue::Ruled| {
        match dialogue::encode_ruled(&r) {
            Some(body) => answer(s, Delimited { body }),
            None => eprintln!("  court equivocation answer refused: the reason passes its u16"),
        }
    };
    let refuse = |s: &mut Channel, why: &str| {
        eprintln!(
            "  court equivocation refused {:02x}{:02x}… K {}: {why}",
            a.receipt.stamp.vk[0], a.receipt.stamp.vk[1], a.receipt.stamp.place().print()
        );
        say(s, dialogue::Ruled::Refused(why.as_bytes().to_vec()));
    };
    // 47.2 steps 1 and 2 on each half, before any carrier is touched. A
    // node that has adopted no origin has none equal to the envelope's.
    let origin = match context.mesh.origin.lock() {
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
    // MU-HOSTING §5a.10: each half's receipt reconstructed from its own
    // coordinates; a half whose chain hash does not re-derive disagrees
    // with itself, `REFUSE_RECEIPT`.
    let reconstructed = mycelium::court::admission::Reconstructed::<mycelium::chronometry::KStepReceipt>::receipt;
    let (Some(ra), Some(rb)) = (reconstructed(&a.receipt), reconstructed(&b.receipt)) else {
        refuse(s, mycelium::court::admission::REFUSE_RECEIPT);
        return;
    };
    let proof = match mycelium::court::admission::admit_equivocation(va, &ra, vb, &rb) {
        Ok(p) => p,
        Err(why) => {
            refuse(s, why);
            return;
        }
    };
    // A court that cannot be read has judged nothing and answers nothing.
    let Ok(mut h) = crate::court_lock::held!(context.court) else {
        eprintln!(
            "  court equivocation unread {:02x}{:02x}… K {}: the court is not readable",
            proof.vk[0], proof.vk[1], proof.at.print()
        );
        return;
    };
    let burned = h.run(|k, _| k.accept_equivocation(proof));
    drop(h);
    // The burn line is the court's (34.2); the reply says whether anything
    // left: the same proof again finds no bond and is inert.
    match burned {
        Some(_) => say(s, dialogue::Ruled::Applied),
        None => say(s, dialogue::Ruled::Inert),
    }
}
