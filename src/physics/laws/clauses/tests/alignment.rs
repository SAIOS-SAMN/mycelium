// CONCORDANCE §0, ROADMAP track 24 B2 — THE ALIGNMENT READING, derived from
// signing events: the latest event per kernel and clause version reads, a
// refusal is a collision carrying its residue, a clause no event names is
// unread, and the reading, the history and the ledger's signing citation each
// read back as the same exact objects.

use crate::chronometry::{Cite, Place, Stamp};
use crate::physics::laws::clauses::alignment::{put_alignment, take_alignment};
use crate::physics::laws::clauses::*;
use crate::physics::laws::codec::*;

fn ids() -> Vec<ConcordanceClauseId> {
    register().expect("the register stands").into_iter().map(|r| r.clause.id).collect()
}

fn at(h: u64) -> Stamp {
    Stamp::new(h, [3; 32], Place { k: 0, i: h })
}

fn admitted(ids: &[ConcordanceClauseId], assent: Assent) -> Verdict {
    Ok(ids.iter().map(|id| Admitted { id: id.clone(), assent }).collect())
}

fn event(h: u64, kernel: u8, verdict: Verdict) -> SigningEvent {
    SigningEvent { at: at(h), kernel: [kernel; 32], reserve: None, verdict }
}

/// THE LATEST EVENT NAMING A CLAUSE READS: an admitted act converses on
/// each clause; a later refusal naming one clause makes it collide with the
/// refusal as residue, the others still read the earlier act; a later
/// re-signing converses again. History stays: the events are all given.
#[test]
fn read_alignment_takes_the_latest_event_per_clause() {
    let ids = ids();
    let residue = SigningRefusal::NetworkVetoed { id: ids[0].clone() };
    let events = vec![event(1, 7, admitted(&ids, Assent::Enabled)), event(2, 7, Err(residue.clone()))];
    let [k]: [KernelAlignment; 1] = read_alignment(&ids, &events, None).try_into().expect("one kernel");
    assert_eq!(k.clauses[0].reading, ClauseReading::Collides { at: at(2), residue: Box::new(residue) });
    assert!(k.clauses.iter().skip(1).all(|c| c.reading == ClauseReading::Converses { at: at(1) }));
    let mut later = events.clone();
    later.push(event(3, 7, admitted(&ids, Assent::Enabled)));
    let [k]: [KernelAlignment; 1] = read_alignment(&ids, &later, None).try_into().expect("one kernel");
    assert!(k.clauses.iter().all(|c| c.reading == ClauseReading::Converses { at: at(3) }));
}

/// A KERNEL CLAUSE VETOED reads vetoed; a kernel with no event reads every
/// clause unread; a version no register holds, named by an event, is read
/// after the register's clauses; the readings are per kernel.
#[test]
fn read_alignment_names_vetoed_unread_and_other_versions() {
    let ids = ids();
    let mut older = ids[0].clone();
    older.version -= 1;
    let events = vec![
        event(1, 7, admitted(&ids, Assent::Vetoed)),
        event(2, 8, Err(SigningRefusal::UnknownClause { id: older.clone() })),
    ];
    let kernels = read_alignment(&ids, &events, None);
    assert_eq!(kernels.iter().map(|k| k.kernel).collect::<Vec<_>>(), vec![[7; 32], [8; 32]], "in the order the ledger first names them");
    assert!(kernels[0].clauses.iter().all(|c| c.reading == ClauseReading::Vetoed { at: at(1) }));
    let other = &kernels[1].clauses;
    assert_eq!(other.len(), ids.len() + 1);
    assert!(other.iter().take(ids.len()).all(|c| c.reading == ClauseReading::Unread));
    assert_eq!(other[ids.len()].id, older);
    let [k]: [KernelAlignment; 1] = read_alignment(&ids, &events, Some([9; 32])).try_into().expect("the asked kernel");
    assert_eq!(k.kernel, [9; 32]);
    assert!(k.clauses.iter().all(|c| c.reading == ClauseReading::Unread));
}

/// The signings of the past are history, named and never a reading: v0.3.0's
/// two kernels at Concordance version 4, by the retired text hash.
#[test]
fn history_carries_v0_3_0_forward_as_fact() {
    let h = history();
    assert_eq!(h.iter().map(|s| s.kernel.as_str()).collect::<Vec<_>>(), ["netstratum", "xylarium"]);
    assert!(h.iter().all(|s| s.concordance == 4 && s.release == "v0.3.0" && s.mechanism == Mechanism::TextHash));
}

/// Mode 9 round-trips: the request, every reading with its stamp and
/// residue, the history, and each refusal; and the ledger's signing
/// citation reads back as written.
#[test]
fn alignment_mode_and_signing_citation_round_trip() {
    let ids = ids();
    assert_eq!(decode_laws_request(&encode_alignment_request(None)), Some(LawsRequest::Alignment(None)));
    assert_eq!(decode_laws_request(&encode_alignment_request(Some(&[5; 32]))), Some(LawsRequest::Alignment(Some([5; 32]))));
    assert_eq!(decode_laws_request(&[crate::anastome::dialogue::ACT_LAWS, MODE_ALIGNMENT, 1]), None, "a key short of 32 bytes");
    let residue = SigningRefusal::Repeated { id: ids[0].clone() };
    let events = vec![event(1, 7, admitted(&ids, Assent::Enabled)), event(2, 7, Err(residue.clone())), event(3, 8, admitted(&ids, Assent::Vetoed))];
    let mut kernels = read_alignment(&ids, &events, None);
    kernels[0].reserve = Some([6; 32]);
    let read = Ok(Alignment { kernels, history: history() });
    assert_eq!(decode_alignment_reply(&encode_alignment_reply(&read)), Some(read));
    for refused in [AlignmentRefusal::StrokeSpent, AlignmentRefusal::Unregistered { id: ids[0].clone() }] {
        let read = Err(refused);
        assert_eq!(decode_alignment_reply(&encode_alignment_reply(&read)), Some(read));
    }
    let mut out = Vec::new();
    put_alignment(&mut out, &Err(AlignmentRefusal::StrokeSpent));
    out.push(0);
    assert_eq!(take_alignment(&out), None, "a byte left over");
    let cite = Cite::Signing { kernel: [7; 32], reserve: Some([6; 32]), verdict: Box::new(Err(residue)) };
    let mut b = Vec::new();
    cite.put_canonical(&mut b).expect("written");
    let mut p = 0;
    assert_eq!(Cite::take_canonical(&b, &mut p), Some(cite));
    assert_eq!(p, b.len());
}
