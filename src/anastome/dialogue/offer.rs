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
/// the only call site and read by nobody.
pub fn encode_offer(o: &Offer) -> Vec<u8> {
    let mut out = vec![ACT_OFFER];
    let _ = put_addr(&mut out, &o.addr);
    out.extend_from_slice(&o.home);
    out.extend_from_slice(&(o.elem.len() as u32).to_le_bytes());
    out.extend_from_slice(&o.elem);
    out.extend_from_slice(&(o.voices.len() as u32).to_le_bytes());
    for v in &o.voices {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, &o.deflation);
    out.extend_from_slice(&super::encode_receipt_section(
        &o.section.prefix,
        &o.section.sig,
        &o.section.receipt,
    ));
    out
}

pub fn decode_offer(body: &[u8]) -> Option<Offer> {
    if body.first() != Some(&ACT_OFFER) {
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

/// The witness's answer: `[vk 32][flen u16][refused][rlen u32][reached]`.
/// A form the witness did not reach is a zero length, never the offered
/// tape echoed; a replay that refused says which refusal.
pub fn encode_offer_reply(w: &Witnessed) -> Vec<u8> {
    let mut out = Vec::with_capacity(38 + w.refused.len() + w.reached.len());
    out.extend_from_slice(&w.vk);
    out.extend_from_slice(&(w.refused.len() as u16).to_le_bytes());
    out.extend_from_slice(&w.refused);
    out.extend_from_slice(&(w.reached.len() as u32).to_le_bytes());
    out.extend_from_slice(&w.reached);
    out
}

pub fn decode_offer_reply(body: &[u8]) -> Option<Witnessed> {
    let vk: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32usize;
    let flen = u16::from_le_bytes(body.get(p..p + 2)?.try_into().ok()?) as usize;
    p += 2;
    let refused = body.get(p..p + flen)?.to_vec();
    p += flen;
    let rlen = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
    p += 4;
    let reached = body.get(p..p + rlen)?.to_vec();
    Some(Witnessed { vk, reached, refused })
}

/// Installed: `[ACT_INSTALLED][addr][elen u32][elem][sig_home 64][sig_peer 64]`
pub fn encode_installed(addr: impl AsRef<[u8]>, elem: &[u8], sig_home: &[u8; 64], sig_peer: &[u8; 64]) -> Vec<u8> {
    let mut out = vec![ACT_INSTALLED];
    let _ = put_addr(&mut out, addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out.extend_from_slice(sig_home);
    out.extend_from_slice(sig_peer);
    out
}

pub fn decode_installed(body: &[u8]) -> Option<Installed> {
    if body.first() != Some(&ACT_INSTALLED) {
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

pub fn installed_signed_prefix(addr: impl AsRef<[u8]>, elem: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = put_addr(&mut out, addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out
}
