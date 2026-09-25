// 47.3 — ACT_OFFER and ACT_INSTALLED: what a court offers at first
// vantage, and what two organs sign when it installs.

use super::*;

/// Offer: `[ACT_OFFER][addr][home 32][elen u32][elem][nv u32][voices][deflation 32][47.1 receipt section]`
///
/// The trailing section is 47.1 whole — the signed prefix carrying the
/// home's causal position with the offered ADDRESS as its payload, the
/// signature over exactly those bytes, and the receipt the prefix names —
/// so the witness runs 47.2 before touching a carrier. It carried `k`, a
/// bespoke `sig` and an `inst_sig` instead, all three written as zeroes at
/// the only call site and read by nobody. `None` when the section cannot
/// be framed: an offer without its signed position is not spoken.
pub fn encode_offer(o: &Offer) -> Option<Vec<u8>> {
    let mut out = vec![ACT_OFFER];
    put_addr(&mut out, &o.addr)?;
    out.extend_from_slice(&o.home);
    crate::transducer::address::put_bytes(&mut out, &o.elem)?;
    crate::transducer::address::put_u32_count(&mut out, o.voices.len())?;
    for v in &o.voices {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, &o.deflation);
    out.extend_from_slice(&super::encode_receipt_section(
        &o.section.prefix,
        &o.section.sig,
        &o.section.receipt,
    )?);
    Some(out)
}

pub fn decode_offer(body: &[u8]) -> Option<Offer> {
    if !act_is(body, ACT_OFFER) {
        return None;
    }
    let mut p = 1usize;
    let addr = take_addr(body, &mut p)?;
    let home: [u8; 32] = body.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let elen = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let elem = body.get(p..p + elen)?.to_vec();
    p += elen;
    let nv = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let mut voices = Vec::with_capacity(nv.min(16));
    for _ in 0..nv {
        voices.push(body.get(p..p + 32)?.try_into().ok()?);
        p += 32;
    }
    let deflation = read_mu(body, &mut p)?;
    let section = super::decode_receipt_section(body, p)?;
    Some(Offer { addr, home, elem, voices, deflation, section })
}

/// The witness's answer: `[arm u8][vk 32][payload]` — `0` unmeasured with
/// no payload, `1` reached with `len32 + form`, `2` refused with
/// `len16 + reason`. The arm is a tag the home matches on. `None` where
/// the reason passes its u16: the reply is refused, never wrapped (LD0).
pub fn encode_offer_reply(w: &Witnessed) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    match w {
        Witnessed::Unmeasured { vk } => {
            out.push(0);
            out.extend_from_slice(vk);
        }
        Witnessed::Reached { vk, form } => {
            out.push(1);
            out.extend_from_slice(vk);
            crate::transducer::address::put_bytes(&mut out, form)?;
        }
        Witnessed::Refused { vk, why } => {
            out.push(2);
            out.extend_from_slice(vk);
            crate::transducer::address::put_u16_count(&mut out, why.len())?;
            out.extend_from_slice(why);
        }
    }
    Some(out)
}

pub fn decode_offer_reply(body: &[u8]) -> Option<Witnessed> {
    let arm = act_tag(body)?;
    let vk: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let p = 33usize;
    match arm {
        0 => Some(Witnessed::Unmeasured { vk }),
        1 => {
            let n = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
            Some(Witnessed::Reached { vk, form: body.get(p + 4..p + 4 + n)?.to_vec() })
        }
        2 => {
            let n = u16::from_le_bytes(body.get(p..p + 2)?.try_into().ok()?) as usize;
            Some(Witnessed::Refused { vk, why: body.get(p + 2..p + 2 + n)?.to_vec() })
        }
        // An arm the layout does not define is refused, never guessed at
        // (rule 11, A21 1: the decoder is total on well-formed keys and
        // refuses otherwise).
        _ => None,
    }
}

/// Installed: `[ACT_INSTALLED][addr][elen u32][elem][sig_home 64][sig_peer 64]`.
pub fn decode_installed(body: &[u8]) -> Option<Installed> {
    if !act_is(body, ACT_INSTALLED) {
        return None;
    }
    let mut p = 1usize;
    let addr = take_addr(body, &mut p)?;
    let elen = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let elem = body.get(p..p + elen)?.to_vec();
    p += elen;
    let sig_home: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    let sig_peer: [u8; 64] = body.get(p + 64..p + 128)?.try_into().ok()?;
    Some(Installed { addr, elem, sig_home, sig_peer })
}

// `offer_signed_prefix` is deleted. 47.1: "Every signature in the tree is
// over exactly these bytes, in this order", and 26: "Every signed frame
// signs the receipt prefix of 47.1". A bespoke per-act prefix is a second
// signing rule, and this one had no production caller at all -- only its
// own test -- so nothing ever signed an offer and nothing ever verified
// one. The offer's signature is the section's, over 47.1's prefix, whose
// payload is the offered address (A21), which binds the form by its
// content and not by the bytes it happened to be spelled with (law 2).
