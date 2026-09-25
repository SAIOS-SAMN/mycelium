// 19, 28 — THE ELEMENT'S RECORD (TN1, 78.34: "tape" names a word's letter
// tape and nothing else; this module was `element/tape.rs`): the element
// as bytes under `RECORD_MAGIC`, and the tags that name what it spells.
//
//! The tape is the coordinate-exact ANASTOME description: the generation
//! magic, the four tags (connection, sentence, translation, rewrite), the
//! codec both ways, and the resynchronisation of the action from the tape.

use super::*;

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
    /// THE ELEMENT RECORD (LD0 step 6(b)): magic `0xA4`, then the address,
    /// the description, the filing, the receipts envelope, μ⃗, the chain,
    /// sections, torsion, anchor, transitions — each gauge as its canonical
    /// ray — and the action. Every word inside the record is written whole
    /// (`Word::put_whole`), a formed word's action with it.
    pub const RECORD_MAGIC: u8 = 0xA4;
    /// THE RECORD BEFORE 6(b), refused BY NAME and never migrated: its words
    /// carry letters alone, so under this reader a contact byte or a
    /// carriage's first byte would be read as the whole-word tag — a 0 or 1
    /// continuing into a misparse. One equality refuses it; there is no
    /// reader for it.
    pub const PRE_WHOLE_MAGIC: u8 = 0xA3;
    /// GEN2, older still, and refused the same way.
    pub const GEN2_MAGIC: u8 = 0xA2;

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![Self::RECORD_MAGIC];
        // THE ADDRESS LEADS THE RECORD, AND THE KIND DECIDES THE REST
        // (54.7 B6 Site 4, as ruled 2026-09-21).
        //
        // It was written after the description, which meant the reader had
        // to consume a byte buffer before it could learn what kind of
        // object it was reading — position determined the shape. The tag is
        // the GRADE-0 COORDINATE of an address: it names the algebraic
        // species before any later stratum describes the cells. Read first,
        // it tells the reader the shape of everything after it, and the
        // record becomes self-describing by a coordinate rather than by an
        // order somebody chose.
        //
        // THE ADDRESS RIDES THE RECORD (54.5, as ruled 2026-09-20). It was
        // derived on read — `Element::address` called `vertex_address`,
        // which parsed the tape — so the tape had to be present for a word
        // to know its own name. That is the dependency that kept the tape
        // alive; retiring it inverts the derivation, and the address is
        // what a word IS (law 2) rather than a summary of a byte run.
        //
        // Written for every kind, because a residual's address is just as
        // much its name as a form's.
        let a = self.address();
        out.extend_from_slice(&(a.len() as u32).to_le_bytes());
        out.extend_from_slice(a.as_bytes());
        // AND A SENTENCE WRITES NO DESCRIPTION FIELD AT ALL — not a tape,
        // and not a zero length standing in for one. A form's content is a
        // cell complex and its key writes that complex out in full, so a
        // description beside it would be a second statement of the same
        // object (law 12). Words, connections and residual chains keep
        // theirs: for them the description IS the content, read by
        // `letters`, `rewrite` and `residual` as the object itself.
        //
        // §10 — a layout change is a recut, and B7 is the recut.
        if a.tag() != Some(crate::transducer::address::TAG_FORM) {
            out.extend_from_slice(&(self.description.len() as u32).to_le_bytes());
            out.extend_from_slice(&self.description);
        }
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
            f.periods.to_bytes(&mut out);
            out.extend_from_slice(&f.chord.to_le_bytes());
        }
        // THE RECEIPTS ENVELOPE (54.5 Road (d)). The GEN3 record is
        // `Key → ⟨μ⃗, ReceiptsEnvelope⟩` as ruled 2026-09-20: the form
        // is the manifold and its address IS its structure, but an
        // utterance is a speech act and carries the evidence to prove
        // itself. The genome is a client's cache of adjudicated
        // utterances, not a dictionary of forms, so the envelope
        // persists here and a restarted voice does not lose its
        // certificates, its citations or its derivations.
        //
        // Written but not yet read back into the readers (B1/B2 of the
        // glide path); `from_bytes` below restores it, and the three
        // verifiers take it at B3.
        self.receipts.to_block(&mut out);
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
        for TorsionCycle { sigma, period: l, fibre, remainder } in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
            out.extend_from_slice(&fibre.to_le_bytes());
            // RM1 — the remainder the cycle closed up to, flagged where it
            // was not measured.
            match remainder {
                Some(r) => {
                    out.push(1);
                    r.put(&mut out);
                }
                None => out.push(0),
            }
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
            // The gauge as its canonical ray (A4). A tag-0 arm stood here
            // that wrote a letter tape when the versor round-tripped
            // through `tape_of_versor` — which answered the identity for
            // every versor, so it only ever wrote the identity word.
            g.put_canonical(&mut out);
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

    /// Deserialize the record this build writes. An older one is refused,
    /// never migrated; the layout before 6(b) is named by its constant,
    /// [`Element::PRE_WHOLE_MAGIC`]. A codec prints nothing: the reader
    /// that opens a genome reports its refusals, once, typed.
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Element> {
        match b.get(*pos) {
            Some(&Self::RECORD_MAGIC) => Self::from_bytes_record(b, pos),
            _ => None,
        }
    }

    fn from_bytes_record(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) != Some(&Self::RECORD_MAGIC) {
            return None;
        }
        *pos += 1;
        let u32_at = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        // THE ADDRESS FIRST, AND THE KIND IT OPENS WITH DECIDES THE REST
        // (54.7 B6 Site 4). Read here and set on the element below, so
        // `Element::address` answers it rather than re-deriving it from a
        // tape — which is what lets the tape go.
        let alen = u32_at(b, pos)? as usize;
        let address = b.get(*pos..*pos + alen)?.to_vec();
        *pos += alen;
        // A SENTENCE CARRIES NO DESCRIPTION FIELD. The tag is the grade-0
        // coordinate and it has already been read, so the reader knows the
        // shape of what follows without consuming anything to find out.
        let description = if crate::transducer::address::tag_of(&address)
            == Some(crate::transducer::address::TAG_FORM)
        {
            Vec::new()
        } else {
            let dlen = u32_at(b, pos)? as usize;
            let d = b.get(*pos..*pos + dlen)?.to_vec();
            *pos += dlen;
            d
        };
        let nfil = u32_at(b, pos)? as usize;
        let mut filing: crate::network::nerve::Keyed<
            crate::transducer::sentence::FiledOrbit,
            crate::network::nerve::coord::Holonomy,
        > = crate::network::nerve::Keyed::new();
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
            let periods = crate::transducer::sentence::ToralPeriod::from_pair(u32_at(b, pos)?, u32_at(b, pos)?);
            let chord = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            filing.put(
                &<crate::network::nerve::coord::Holonomy
                    as crate::network::nerve::Coordinate>::decode(&class)
                    .ok()?,
                crate::transducer::sentence::FiledOrbit { sigma, periods, chord },
            );
        }
        // THE RECEIPTS ENVELOPE (54.5 Road (d)), framed, so a malformed one
        // refuses the record rather than eating the fields after it.
        let receipts = crate::transducer::element::ReceiptsEnvelope::from_block(b, pos)?;
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
            // A17: a cycle is believed only when the orbit CLOSES at its
            // period, so a filed period of zero is not a cycle with no
            // extent — it is a malformed tape, and every reader downstream
            // may then take `ℓ − 1` whole.
            if l == 0 {
                return None;
            }
            let fibre = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            let remainder = match b.get(*pos)? {
                0 => {
                    *pos += 1;
                    None
                }
                1 => {
                    *pos += 1;
                    Some(super::Remainder::take(b, pos)?)
                }
                _ => return None,
            };
            torsion.push(TorsionCycle { sigma, period: l, fibre, remainder });
        }
        // STD1 — a holonomic lane with no room for the sections and cycles
        // the record carries is refused where it arrives (LD0): the word
        // this build would hold reads its magnitude total or not at all.
        if !super::carrier::holonomic_fits(mag[3], sections.len(), torsion.len()) {
            return None;
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
            let g = Versor::from_bytes(b, pos)?;
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
            receipts,
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
