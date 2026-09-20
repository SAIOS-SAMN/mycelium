// 19, 28 — THE TAPE: the element as bytes, and the tags that name what a
// tape spells.
//
//! The tape is the coordinate-exact ANASTOME description: the generation
//! magic, the four tags (connection, sentence, translation, rewrite), the
//! codec both ways, and the resynchronisation of the action from the tape.

use super::*;

fn tape_of_versor(_g: &Versor) -> crate::transducer::alphabet::Word {
    crate::transducer::alphabet::Word::identity()
}

fn versor_of_tape(w: &crate::transducer::alphabet::Word) -> Versor {
    match versor_of_word(w) {
        Some(v) => v,
        None => identity_versor(),
    }
}

pub const CONNECTION_TAG: u8 = 0x11;
/// A sentence: a nerve of germs. The tape after this tag is the base
/// connection, then the germ addresses, faces and cycles. Distinct from
/// CONNECTION_TAG so an opened-axis connection is not replayed as a sentence.
pub const SENTENCE_TAG: u8 = 0x12;
/// A translation: the gauge between two sections, not a motion of a body.
/// The court replays it by inversion of the gauge; the syllabus teaches it
/// by incidence. Same payload as a connection, then the two section addresses.
pub const TRANSLATION_TAG: u8 = 0x13;
/// An optimisation: a Rewrite the court replays both ways.
pub const REWRITE_TAG: u8 = 0x14;

impl Element {
    /// GEN3: tapes only. Magic `0xA3`, then description, mag, chain, sections,
    /// torsion, anchor, transitions as letter tapes. Addresses on the tape
    /// are `len16 + bytes`. An older file is refused.
    pub const GEN3_MAGIC: u8 = 0xA3;
    pub const GEN2_MAGIC: u8 = 0xA2;

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![Self::GEN3_MAGIC];
        out.extend_from_slice(&(self.description.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.description);
        // THE ADDRESS RIDES THE RECORD (54.5, as ruled 2026-09-20). It was
        // derived on read — `Element::address` calls `vertex_address`,
        // which parses the tape — so the tape had to be present for a word
        // to know its own name. That is the dependency that kept the tape
        // alive: retiring it inverts the derivation, and the address is
        // what a word IS (law 2) rather than a summary of a byte run.
        //
        // Written for every kind, because a residual's address is just as
        // much its name as a form's.
        let a = self.address();
        out.extend_from_slice(&(a.len() as u32).to_le_bytes());
        out.extend_from_slice(a.as_bytes());
        // THE SPEAKER'S FILING (54.5): class ‖ σ ‖ ℓ ‖ chord, in key order.
        // It is an attestation and travels with the word that makes it, so
        // the court can check a claim against geometry it re-derives rather
        // than asking a form to vouch for itself.
        out.extend_from_slice(&(self.filing.len() as u32).to_le_bytes());
        for (class, f) in self.filing.scoped() {
            out.extend_from_slice(&(class.len() as u32).to_le_bytes());
            out.extend_from_slice(&class);
            out.extend_from_slice(&(f.sigma.len() as u32).to_le_bytes());
            for &(x, y) in &f.sigma {
                out.push(x);
                out.push(y);
            }
            out.extend_from_slice(&f.period.to_le_bytes());
            out.extend_from_slice(&f.chord.to_le_bytes());
        }
        for m in &self.mag {
            out.extend_from_slice(&m.to_le_bytes());
        }
        out.extend_from_slice(&(self.sub.len() as u32).to_le_bytes());
        for (r, c, before, after) in self.sub.as_slice() {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*before);
            out.push(*after);
        }
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        for s in &self.sections {
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            for (b, a) in s {
                out.push(*b);
                out.push(*a);
            }
        }
        out.extend_from_slice(&(self.torsion.len() as u32).to_le_bytes());
        for TorsionCycle { sigma, period: l, fibre } in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        match &self.anchor {
            Some(t) => {
                out.push(1);
                t.to_bytes(&mut out);
            }
            None => out.push(0),
        }
        out.extend_from_slice(&(self.transitions.len() as u32).to_le_bytes());
        for Transition { asg: a, gauge: g, fibre } in &self.transitions {
            out.extend_from_slice(&(a.len() as u32).to_le_bytes());
            for (b, f) in a {
                out.push(*b);
                out.push(*f);
            }
            let tape = tape_of_versor(g);
            let back = versor_of_tape(&tape);
            if back == *g {
                out.push(0);
                tape.to_bytes(&mut out);
            } else {
                out.push(1);
                g.to_bytes(&mut out);
            }
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        out.extend_from_slice(&(self.action.germinated.len() as u32).to_le_bytes());
        for (i, v) in &self.action.germinated {
            out.extend_from_slice(&i.to_le_bytes());
            v.to_bytes(&mut out);
        }
        match &self.action.spatial {
            Some(v) if self.letters().is_none() => {
                out.push(1);
                v.to_bytes(&mut out);
            }
            _ => out.push(0),
        }
        // THE ROTOR RIDES WITH THE ACTION. It is written only where the
        // tape spells no letters — where it is written at all — and a
        // composite that crossed the wire without it would arrive with its
        // rotation flattened onto the sensor plane, which is the fault the
        // field exists to end.
        match &self.action.rotor {
            Some(r) if self.letters().is_none() => {
                out.push(1);
                r.to_bytes(&mut out);
            }
            _ => out.push(0),
        }
        out
    }

    /// Deserialize GEN3 only. An older file is refused, never migrated.
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) == Some(&Self::GEN3_MAGIC) {
            return Self::from_bytes_gen3(b, pos);
        }
        None
    }

    fn from_bytes_gen3(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) != Some(&Self::GEN3_MAGIC) {
            return None;
        }
        *pos += 1;
        let u32_at = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let dlen = u32_at(b, pos)? as usize;
        let description = b.get(*pos..*pos + dlen)?.to_vec();
        *pos += dlen;
        // THE ADDRESS AS THE RECORD CARRIES IT (54.5). Read here and set on
        // the element below, so `Element::address` answers it rather than
        // re-deriving it from a tape — which is what lets the tape go.
        let alen = u32_at(b, pos)? as usize;
        let address = b.get(*pos..*pos + alen)?.to_vec();
        *pos += alen;
        let nfil = u32_at(b, pos)? as usize;
        let mut filing = crate::network::nerve::Keyed::new();
        for _ in 0..nfil {
            let clen = u32_at(b, pos)? as usize;
            let class = b.get(*pos..*pos + clen)?.to_vec();
            *pos += clen;
            let nsig = u32_at(b, pos)? as usize;
            let mut sigma = Vec::with_capacity(nsig.min(b.len()));
            for _ in 0..nsig {
                sigma.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                *pos += 2;
            }
            let period = u32_at(b, pos)?;
            let chord = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            filing.insert(
                &class,
                crate::transducer::sentence::FiledLoop { sigma, period, chord },
            );
        }
        let mut mag = [0u64; 4];
        for m in &mut mag {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        // A position past the end of the tape is a malformed tape, not a
        // tape with nothing left (rule 11).
        let remaining = |b: &[u8], p: &usize| b.len().checked_sub(*p);
        let nsub = u32_at(b, pos)? as usize;
        if nsub > remaining(b, pos)? / 18 {
            return None;
        }
        let mut sub = Vec::with_capacity(nsub);
        for _ in 0..nsub {
            let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            let c = i64::from_le_bytes(b.get(*pos + 8..*pos + 16)?.try_into().ok()?);
            let before = *b.get(*pos + 16)?;
            let after = *b.get(*pos + 17)?;
            *pos += 18;
            sub.push((r, c, before, after));
        }
        // A5 STEP 3 AT THE TAPE: a chain that says two things at one site has
        // no canonical form, so the element it would make has no address
        // (law 2). The refusal travels here, where the bytes arrive, and
        // what comes out is the accepted chain itself — the field's type is
        // A5's proof, so the reader that later asks for the key has nothing
        // left to check.
        let sub = crate::transducer::element::canonical_chain(&sub)?;
        let nsec = u32_at(b, pos)? as usize;
        if nsec > remaining(b, pos)? / 4 {
            return None;
        }
        let mut sections = Vec::with_capacity(nsec);
        for _ in 0..nsec {
            let slen = u32_at(b, pos)? as usize;
            if slen > remaining(b, pos)? / 2 {
                return None;
            }
            let mut s = Vec::with_capacity(slen);
            for _ in 0..slen {
                let bb = *b.get(*pos)?;
                let aa = *b.get(*pos + 1)?;
                *pos += 2;
                s.push((bb, aa));
            }
            sections.push(s);
        }
        let ntor = u32_at(b, pos)? as usize;
        if ntor > remaining(b, pos)? / 8 {
            return None;
        }
        let mut torsion = Vec::with_capacity(ntor);
        for _ in 0..ntor {
            let slen = u32_at(b, pos)? as usize;
            if slen > remaining(b, pos)? / 2 {
                return None;
            }
            let mut sigma = Vec::with_capacity(slen);
            for _ in 0..slen {
                let f = *b.get(*pos)?;
                let t = *b.get(*pos + 1)?;
                *pos += 2;
                sigma.push((f, t));
            }
            let l = u32_at(b, pos)?;
            // A17: a cycle is believed only when the loop CLOSES at its
            // period, so a filed period of zero is not a cycle with no
            // extent — it is a malformed tape, and every reader downstream
            // may then take `ℓ − 1` whole.
            if l == 0 {
                return None;
            }
            let fibre = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            torsion.push(TorsionCycle { sigma, period: l, fibre });
        }
        let anchor = match b.get(*pos) {
            Some(1) => {
                *pos += 1;
                Some(crate::transducer::anchor::Sel::from_bytes(b, pos)?)
            }
            Some(0) => {
                *pos += 1;
                None
            }
            _ => None,
        };
        // A tape that ends here carries no transitions; the count is read
        // or it is absent, never a zero put in place of one.
        let ntr = match u32_at(b, pos) {
            Some(n) => n as usize,
            None => 0,
        };
        let mut transitions = Vec::with_capacity(ntr.min(64));
        for _ in 0..ntr {
            let alen = u32_at(b, pos)? as usize;
            if alen > remaining(b, pos)? / 2 {
                return None;
            }
            let mut a = Vec::with_capacity(alen);
            for _ in 0..alen {
                let bb = *b.get(*pos)?;
                let f = *b.get(*pos + 1)?;
                *pos += 2;
                a.push((bb, f));
            }
            let tag = *b.get(*pos)?;
            *pos += 1;
            let g = if tag == 0 {
                let w = crate::transducer::alphabet::Word::from_bytes(b, pos)?;
                versor_of_tape(&w)
            } else {
                Versor::from_bytes(b, pos)?
            };
            let fibre = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            transitions.push(Transition { asg: a, gauge: g, fibre });
        }
        let mut germinated = Vec::new();
        if let Some(ng) = u32_at(b, pos) {
            for _ in 0..ng as usize {
                let i = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
                *pos += 2;
                let v = Versor::from_bytes(b, pos)?;
                germinated.push((i, v));
            }
        }
        let spatial = match b.get(*pos) {
            Some(1) => {
                *pos += 1;
                Some(Versor::from_bytes(b, pos)?)
            }
            Some(0) => {
                *pos += 1;
                None
            }
            _ => None,
        };
        let rotor = match b.get(*pos) {
            Some(1) => {
                *pos += 1;
                Some(crate::transducer::alphabet::Rotor::from_bytes(b, pos)?)
            }
            Some(0) => {
                *pos += 1;
                None
            }
            _ => None,
        };
        let mut e = Element {
            description,
            action: Action {
                spatial,
                gauge: Vec::new(),
                temporal: mag[2],
                shell: mag[3],
                germinated,
                rotor,
            },
            mag,
            sub,
            sections,
            torsion,
            filing,
            transitions,
            address: std::sync::OnceLock::new(),
            profiles: std::sync::OnceLock::new(),
            anchor,
        };
        e.sync_action_from_tape();
        // THE NAME THE RECORD CARRIED. An empty one is a record written
        // before the address rode it, and it is refused rather than
        // re-derived at a guess (rule 11) — a word whose name this build
        // cannot read is not a word this build holds.
        if address.is_empty() {
            return None;
        }
        let _ = e.address.set(crate::transducer::address::Address::new(address));
        Some(e)
    }

    pub(crate) fn sync_action_from_tape(&mut self) {
        let Some(letters) = self.letters() else {
            return;
        };
        self.action.spatial = conformal_of(&letters.spelling.word, &letters.carriage);
        self.action.gauge = letters.sigma;
    }

}
