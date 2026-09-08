// 47.3 — ACT_OFFER and ACT_INSTALLED: what a court offers at first
// vantage, and what two organs sign when it installs.

use super::*;

/// Offer: `[ACT_OFFER][addr][home 32][elen u32][elem][nv u32][voices][deflation 32][k 8][sig 64][inst_sig 64]`
/// `k` is TUniversal at first vantage, comparable across hyphas. `sig`
/// covers the Offer prefix; `inst_sig` is the home organ's signature over
/// the Installed prefix, so a second vantage can emit Installed without
/// the home key.
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
    out.extend_from_slice(&o.k.to_le_bytes());
    out.extend_from_slice(&o.sig);
    out.extend_from_slice(&o.install_sig);
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
    let k = u64::from_le_bytes(body.get(p..p + 8)?.try_into().ok()?);
    p += 8;
    let sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    let inst_sig: [u8; 64] = body.get(p..p + 64)?.try_into().ok()?;
    Some(Offer { addr, home, elem, voices, deflation, k, sig, install_sig: inst_sig })
}

/// The witness's answer: `[glued u8][vk 32][rlen u32][reached]`. A form the
/// witness did not reach is a zero length, never the offered tape echoed.
pub fn encode_offer_reply(w: &Witnessed) -> Vec<u8> {
    let mut out = vec![w.glued as u8];
    out.extend_from_slice(&w.vk);
    out.extend_from_slice(&(w.reached.len() as u32).to_le_bytes());
    out.extend_from_slice(&w.reached);
    out
}

pub fn decode_offer_reply(body: &[u8]) -> Option<Witnessed> {
    let glued = *body.first()? != 0;
    let vk: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let rlen = u32::from_le_bytes(body.get(33..37)?.try_into().ok()?) as usize;
    let reached = body.get(37..37 + rlen)?.to_vec();
    Some(Witnessed { glued, vk, reached })
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

/// Bytes of an Offer body that the signature covers: everything after the
/// act byte except the two trailing signatures.
pub fn offer_signed_prefix(o: &Offer) -> Vec<u8> {
    let body = encode_offer(o);
    body[1..body.len() - 128].to_vec()
}

pub fn installed_signed_prefix(addr: impl AsRef<[u8]>, elem: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = put_addr(&mut out, addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out
}
