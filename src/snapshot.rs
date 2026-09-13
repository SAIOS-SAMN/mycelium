// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 3, 28 — THE SEALED DOOR: the one place a Keyed yields every entry,
//! in key order, for the wire. Cognition names `to_bytes` / `snapshot` on
//! the carrier, never this trait's supertrait.

use crate::network::nerve::Keyed;

mod sealed {
    pub trait Sealed {
        type Value;
        fn walk<F: FnMut(&[u8], &Self::Value)>(&self, f: F);
    }
}

/// The only place a Keyed yields all entries. The supertrait is private
/// to this module; cognition cannot name it.
pub trait Persist: sealed::Sealed {
    fn persist<F: FnMut(&[u8], &Self::Value)>(&self, f: F) {
        sealed::Sealed::walk(self, f);
    }
}

impl<V> sealed::Sealed for Keyed<V> {
    type Value = V;
    fn walk<F: FnMut(&[u8], &V)>(&self, mut f: F) {
        crate::network::nerve::keyed_persist_walk(self, &mut f);
    }
}

impl<V> Persist for Keyed<V> {}

/// Walk `k` in key order. Callers live in this module or in a carrier's
/// `to_bytes` that is the wire of that carrier.
pub fn persist<V>(k: &Keyed<V>, f: impl FnMut(&[u8], &V)) {
    <Keyed<V> as Persist>::persist(k, f);
}

/// WRITE A SECTION AND COUNT WHAT IT WROTE, in one act.
///
/// Every section that carries a count wrote `carrier.len()` — a stored
/// counter — and then wrote its content by walking the carrier with
/// [`persist`]. One number, two independent sources, nothing holding them
/// together: law 12 at the door every carrier leaves through.
///
/// They parted in the field. Two of three hyphae held ledgers whose COC1
/// section announced five complexes and carried four; each court's own
/// reader believed the announcement, read a fifth object where the next
/// section began, and refused its whole history. The content was intact.
/// The number written beside it was not a reading of that content.
///
/// So the body is written first and measured as it is written, and the count
/// is that act's own measure. A carrier whose `len` disagrees with its walk
/// can no longer produce a tape that lies about itself — and where those two
/// disagree is a fault in the carrier, to be found there and not papered
/// over by whichever reading the wire happened to take.
/// A RECORD THAT CANNOT BE FRAMED IS NOT WRITTEN, and is not counted.
///
/// `write` returns `None` where a field refuses — [`put_addr`] refuses an
/// address that does not fit its `u16` length, and NEVER TRUNCATES: it
/// writes nothing at all. Twelve callers discarded that refusal, so the
/// record went onto the tape with its address field simply absent and the
/// reader framed the next field as a length. A half-written record is the
/// one thing a counted section cannot survive.
///
/// So the body is rewound to where the record began and the record is
/// dropped whole, which keeps the count a reading of the content — the
/// property this function exists for — and the loss is said out loud rather
/// than being discovered later as a tape that will not read.
pub fn counted<V>(
    out: &mut Vec<u8>,
    carrier: &Keyed<V>,
    mut write: impl FnMut(&mut Vec<u8>, &[u8], &V) -> Option<()>,
) {
    let mut body = Vec::new();
    let mut n = 0u64;
    let mut dropped = 0u64;
    persist(carrier, |key, v| {
        let mark = body.len();
        match write(&mut body, key, v) {
            Some(()) => n = n.saturating_add(1),
            None => {
                body.truncate(mark);
                dropped = dropped.saturating_add(1);
            }
        }
    });
    if dropped > 0 {
        eprintln!(
            "  snapshot: {dropped} records could not be framed and were not written; \
             {n} were"
        );
    }
    framed(out, n, &mut body);
}

/// A SECTION'S FRAME: `count u64 ‖ byte_len u64 ‖ body`, and the length is
/// the half that makes a section a section.
///
/// A COUNT CANNOT DELIMIT ANYTHING. It says how many records to read and
/// nothing about where they end, so a reader recovers the boundary by
/// parsing every record in order and trusting the result. One record whose
/// framing the reader reconstructs differently from the writer and the
/// position is wrong from there on — and every later refusal names a place
/// that has nothing to do with the fault.
///
/// Measured, on a ledger this build wrote and this build then refused:
/// hypha a's DEF1 announced 91 records, the reader framed 84 of them over
/// 298223 bytes and then read an address length of zero with a tag byte of
/// 13. The section's true body was 345929 bytes — 6511624 to FLD1 at
/// 6857553 — which the reader had no way to know, so it could not say that
/// it had lost the frame, could not say where, and refused 8.6 MB of a
/// court's history on the strength of a byte it had mis-seated. Hypha b
/// lost 13.7 MB the same way in the same section.
///
/// With the length written, a reader knows the section's own end before it
/// reads a record. A body that does not consume exactly that many bytes is
/// a refusal AT THAT SECTION, stating the two numbers, and the fault is
/// attributed where it happened rather than wherever the position drifted
/// to. That is a property of the tape and not of the parser's luck.
///
/// Both fields are `u64`. The count was `u32` and each length below it is
/// written `as u32`; a cast that truncates is the same class of defect as
/// the count that lies, and a section is not the place to economise eight
/// bytes.
pub fn framed(out: &mut Vec<u8>, n: u64, body: &mut Vec<u8>) {
    out.extend_from_slice(&n.to_le_bytes());
    out.extend_from_slice(&(body.len() as u64).to_le_bytes());
    out.append(body);
}

/// READ A SECTION'S FRAME — the counterpart of [`framed`], and the same
/// carrier, so a reader cannot drift from the writer (law 12). Returns the
/// record count and the byte the body ENDS at; the caller checks it arrived
/// there.
///
/// `None` when the frame itself does not fit, or when the body it declares
/// runs past the tape. Neither is a record's problem and neither is
/// reported as one.
pub fn take_frame(b: &[u8], pos: &mut usize) -> Option<(usize, usize)> {
    let take = |b: &[u8], p: &mut usize| -> Option<usize> {
        let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        usize::try_from(v).ok()
    };
    let n = take(b, pos)?;
    let len = take(b, pos)?;
    let end = pos.checked_add(len).filter(|e| *e <= b.len())?;
    Some((n, end))
}
