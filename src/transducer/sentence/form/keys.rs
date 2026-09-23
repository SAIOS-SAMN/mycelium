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

/// 54.7 R5 — AN ORDINAL, NOT A COORDINATE (rule 15), and the type says so.
/// The one writer of these bytes; a carrier keyed by a vertex's position in
/// its own form takes this and no coordinate of any A21 system can reach it.
pub fn vertex_ix_key(i: u16) -> coord::VertexIx {
    coord::VertexIx(i)
}

pub(crate) fn unfolded_key(measure: &Measure) -> coord::Unfolded {
    coord::Unfolded(*measure)
}

/// Hue map as a section coordinate: word conjugacy and the map, never a
/// matching key for correspondence (chapter 21 step 1). Read from the
/// ACTION alone (L14): a word vertex carries its canonical rotor, so
/// being classed resolves no citation.
/// THE STRATUM EVERY HUE SECTION OF ONE TRANSPORT CLASS HANGS UNDER: the
/// hue key's own prefix, up to but not including the gauge. A21 §2 —
/// coordinates are ordered so a prefix reads a stratum, and the stratum here
/// is "every recolouring this class has been spoken with", which is what a
/// caller can actually ask for. The gauge is INSIDE the full key, so a read
/// keyed by the whole thing can only answer a question whose answer the
/// asker already holds.
pub(crate) fn hue_class_prefix_of(
    rotor: &crate::transducer::alphabet::Rotor,
) -> Option<Vec<u8>> {
    let mut buf = vec![crate::transducer::address::TAG_HUE];
    rotor.put_class_coordinates(&mut buf)?;
    Some(buf)
}

pub(crate) fn hue_section_key_of(
    rotor: &crate::transducer::alphabet::Rotor,
    gauge: &[(u8, u8)],
) -> Option<crate::transducer::address::Address> {
    // Built ON the prefix, so the stratum a reader reads and the key a writer
    // files under cannot drift apart (law 12: one fact, one construction).
    let mut buf = hue_class_prefix_of(rotor)?;
    buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
    for &(a, b) in gauge {
        buf.push(a);
        buf.push(b);
    }
    Some(crate::transducer::address::Address::new(buf))
}

pub(crate) fn vertex_transport_key_of(
    rotor: &crate::transducer::alphabet::Rotor,
    gauge: &[(u8, u8)],
) -> Option<crate::transducer::address::Address> {
    let mut buf = vec![crate::transducer::address::TAG_WORD, ShellKind::Whole.as_u8()];
    rotor.put_class_coordinates(&mut buf)?;
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
    // A position past the end of the tape is a malformed tape, not a tape
    // with nothing left (rule 11).
    let left = b.len().checked_sub(pos)?;
    if item != 0 && n > left / item {
        return None;
    }
    Some(n)
}
