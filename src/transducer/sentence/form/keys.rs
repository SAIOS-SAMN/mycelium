// A21 — the keys a form is indexed by: the vertex, the transport, the hue
// section, the unfolded measure. Each writes its coordinates out.

use super::*;

pub(crate) fn spell_versor(v: &Versor) -> Option<Word> {
    let id = alphabet::Word::identity();
    let held = element::conformal_of(&id, &element::identity_versor())?;
    if element::same_versor(&held, v) {
        return Some(id);
    }
    None
}

/// Transport class of a conformal versor: the spelled word's class key.
pub fn class_key_of_versor(v: &Versor) -> Option<crate::transducer::address::Address> {
    spell_versor(v)?.class_key()
}

pub fn vertex_ix_key(i: u16) -> Vec<u8> {
    let mut k = vec![0u8];
    k.extend_from_slice(&i.to_le_bytes());
    k
}

pub(crate) fn unfolded_key(measure: &Measure) -> Vec<u8> {
    let mut k = vec![4u8];
    match measure {
        Measure::Occupied => k.push(0),
        Measure::Remainder => k.push(1),
        Measure::Ring { anchor } => {
            k.push(2);
            k.extend_from_slice(&anchor.to_le_bytes());
        }
    }
    k
}

/// Hue map as a section coordinate: word conjugacy and the map, never a
/// matching key for correspondence (chapter 21 step 1).
pub(crate) fn hue_section_key(word: &Word, gauge: &[(u8, u8)]) -> Option<crate::transducer::address::Address> {
    let mut buf = vec![crate::transducer::address::TAG_HUE];
    crate::transducer::address::put_bytes(&mut buf, word.class_key()?.as_bytes());
    buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
    for &(a, b) in gauge {
        buf.push(a);
        buf.push(b);
    }
    Some(crate::transducer::address::Address::new(buf))
}

pub(crate) fn vertex_transport_key(word: &Word, gauge: &[(u8, u8)]) -> Option<crate::transducer::address::Address> {
    let mut buf = vec![crate::transducer::address::TAG_WORD, ShellKind::Whole.as_u8()];
    crate::transducer::address::put_bytes(&mut buf, word.class_key()?.as_bytes());
    buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
    for &(b, a) in gauge {
        buf.push(b);
        buf.push(a);
    }
    Some(crate::transducer::address::Address::new(buf))
}

pub(crate) fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

pub(crate) fn take_count(b: &[u8], pos: usize, n: usize, item: usize) -> Option<usize> {
    if item == 0 {
        return Some(n);
    }
    let left = b.len().saturating_sub(pos);
    if item != 0 && n > left / item {
        return None;
    }
    Some(n)
}
