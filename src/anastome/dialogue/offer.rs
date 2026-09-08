// 47.3 — ACT_OFFER and ACT_INSTALLED: what a court offers at first
// vantage, and what two organs sign when it installs.

use super::*;

/// Offer: `[ACT_OFFER][addr][elen u32][elem][nv u32][voices][deflation 32][k 8][sig 64][inst_sig 64]`
/// `k` is TUniversal at first vantage, comparable across hyphas. `sig`
/// covers the Offer prefix; `inst_sig` is the home organ's signature over
/// the Installed prefix, so a second vantage can emit Installed without
/// the home key.
pub fn encode_offer(
    addr: impl AsRef<[u8]>,
    elem: &[u8],
    voices: &[[u8; 32]],
    deflation: &[u64; 4],
    k: u64,
    sig: &[u8; 64],
    inst_sig: &[u8; 64],
) -> Vec<u8> {
    let mut out = vec![ACT_OFFER];
    let _ = put_addr(&mut out, addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
    for v in voices {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, deflation);
    out.extend_from_slice(&k.to_le_bytes());
    out.extend_from_slice(sig);
    out.extend_from_slice(inst_sig);
    out
}

pub fn decode_offer(body: &[u8]) -> Option<Offer> {
    if body.first() != Some(&ACT_OFFER) {
        return None;
    }
    let mut p = 1usize;
    let addr = take_addr(body, &mut p)?;
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
    Some(Offer { addr, elem, voices, deflation, k, sig, install_sig: inst_sig })
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

/// Bytes of an Offer/Installed body that the signature covers (everything after the act except the trailing sigs).
pub fn offer_signed_prefix(addr: impl AsRef<[u8]>, elem: &[u8], voices: &[[u8; 32]], deflation: &[u64; 4], k: u64) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = put_addr(&mut out, addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out.extend_from_slice(&(voices.len() as u32).to_le_bytes());
    for v in voices {
        out.extend_from_slice(v);
    }
    push_mu(&mut out, deflation);
    out.extend_from_slice(&k.to_le_bytes());
    out
}

pub fn installed_signed_prefix(addr: impl AsRef<[u8]>, elem: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let _ = put_addr(&mut out, addr);
    out.extend_from_slice(&(elem.len() as u32).to_le_bytes());
    out.extend_from_slice(elem);
    out
}
