// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, MU-RELEASE §3a, ROADMAP track 24 B2 — THE ALIGNMENT
// READING: per kernel and per clause version, whether the kernel converses
// with the hypha on the clause, collides with it (with the residue), vetoed
// it, or has not read it — derived from the signing events the hypha's
// ledger records, never stored beside them.
//
//! Each signing act the hypha judged is an event on its receipt chain
//! (`Cite::Signing`, sealed by `Court::hear_signing`): the kernel's key, the
//! domain it was the sitting reserve of, and the verdict at the receipt's
//! stamp, whose height is the TUniversal point the act was read at. An
//! admitted act names each clause with its assent; a refused act names one
//! clause with its refusal, and a `NotSignatory` carries the fixture, the
//! kernel's object and the hypha's, both sides of the collision held as a
//! residue (Concordance A7). History is never rewritten: a re-signing is a
//! new event, and the reading takes, per kernel and per clause version, the
//! latest event that names the clause, in the ledger's stamp order (the
//! height first). A clause no event names reads [`ClauseReading::Unread`].
//! The signings made before signing by adjudicated reading are served
//! beside the reading as history (`history`), never folded into it.

use super::history::{put_history, take_history, HistoricalSigning};
use super::register::ConcordanceClauseId;
use super::signing::{put_verdict, take_verdict_at, Assent, SigningRefusal, Verdict};
use super::writing::{put_clause_id, take_clause_id};
use crate::chronometry::Stamp;
use crate::transducer::address::{put_key_count, take_key_count};

/// ONE SIGNING EVENT AS THE LEDGER RECORDS IT: the receipt's stamp (its
/// height the TUniversal point the act was read at), the kernel's key, the
/// domain the kernel was the sitting reserve of (none where it was none's),
/// and the hypha's verdict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningEvent {
    pub at: Stamp,
    pub kernel: [u8; 32],
    pub reserve: Option<[u8; 32]>,
    pub verdict: Verdict,
}

/// WHERE ONE KERNEL STANDS ON ONE CLAUSE VERSION, by its latest event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClauseReading {
    /// The kernel reached the hypha's own object on every fixture and
    /// enabled the clause, at the event's stamp.
    Converses { at: Stamp },
    /// The kernel vetoed the clause (a kernel clause), at the event's stamp.
    Vetoed { at: Stamp },
    /// The hypha refused the kernel's act on this clause, at the event's
    /// stamp; the residue is the refusal by name, a `NotSignatory` naming
    /// the fixture, the object presented and the hypha's own.
    Collides { at: Stamp, residue: Box<SigningRefusal> },
    /// No event of the kernel names the clause.
    Unread,
}

/// ONE CLAUSE VERSION'S READING FOR ONE KERNEL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClauseAlignment {
    pub id: ConcordanceClauseId,
    pub reading: ClauseReading,
}

/// ONE KERNEL'S ALIGNMENT: its key, the domain it was the sitting reserve
/// of at its latest event, and a reading per clause version: the register's
/// clauses first, in the register's order, then every other version its
/// events name, in the order first named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KernelAlignment {
    pub kernel: [u8; 32],
    pub reserve: Option<[u8; 32]>,
    pub clauses: Vec<ClauseAlignment>,
}

/// THE ALIGNMENT READING: each kernel's, in the order the ledger first
/// names it, and the signings of the past beside it as history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    pub kernels: Vec<KernelAlignment>,
    pub history: Vec<HistoricalSigning>,
}

/// WHY NO ALIGNMENT WAS READ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlignmentRefusal {
    /// The hold's stroke was spent before the ledger was read whole: a
    /// reading cut short is never served as the whole.
    StrokeSpent,
    /// This hypha cannot read the clause of its own register.
    Unregistered { id: ConcordanceClauseId },
}

/// The clause a verdict's refusal names, where it names one.
fn refused_clause(r: &SigningRefusal) -> Option<&ConcordanceClauseId> {
    match r {
        SigningRefusal::UnknownClause { id }
        | SigningRefusal::Repeated { id }
        | SigningRefusal::NetworkVetoed { id }
        | SigningRefusal::NotSignatory { id, .. }
        | SigningRefusal::Unregistered { id } => Some(id),
        SigningRefusal::Malformed | SigningRefusal::Unsigned | SigningRefusal::Unrecorded => None,
    }
}

/// What one event says of each clause it names.
fn readings_of(e: &SigningEvent) -> Vec<ClauseAlignment> {
    match &e.verdict {
        Ok(admitted) => admitted
            .iter()
            .map(|a| ClauseAlignment {
                id: a.id.clone(),
                reading: match a.assent {
                    Assent::Enabled => ClauseReading::Converses { at: e.at },
                    Assent::Vetoed => ClauseReading::Vetoed { at: e.at },
                },
            })
            .collect(),
        Err(r) => refused_clause(r)
            .map(|id| ClauseAlignment { id: id.clone(), reading: ClauseReading::Collides { at: e.at, residue: Box::new(r.clone()) } })
            .into_iter()
            .collect(),
    }
}

/// THE ALIGNMENT DERIVED FROM THE EVENTS, given in the ledger's stamp
/// order: per kernel (only `only` where named, read Unread on every clause
/// where it has no event), each clause version's latest reading, the
/// register's clauses first. Nothing is stored: the same events give the
/// same reading.
pub fn read_alignment(register: &[ConcordanceClauseId], events: &[SigningEvent], only: Option<[u8; 32]>) -> Vec<KernelAlignment> {
    let mut kernels: Vec<KernelAlignment> = Vec::new();
    let unread = |id: &ConcordanceClauseId| ClauseAlignment { id: id.clone(), reading: ClauseReading::Unread };
    if let Some(k) = only {
        kernels.push(KernelAlignment { kernel: k, reserve: None, clauses: register.iter().map(unread).collect() });
    }
    for e in events.iter().filter(|e| only.is_none_or(|k| k == e.kernel)) {
        let at = match kernels.iter().position(|k| k.kernel == e.kernel) {
            Some(at) => at,
            None => {
                kernels.push(KernelAlignment { kernel: e.kernel, reserve: None, clauses: register.iter().map(unread).collect() });
                kernels.len() - 1
            }
        };
        let Some(k) = kernels.get_mut(at) else { continue };
        k.reserve = e.reserve;
        for read in readings_of(e) {
            match k.clauses.iter_mut().find(|c| c.id == read.id) {
                Some(held) => held.reading = read.reading,
                None => k.clauses.push(read),
            }
        }
    }
    kernels
}

fn put_key(out: &mut Vec<u8>, key: &Option<[u8; 32]>) {
    match key {
        None => out.push(0),
        Some(k) => {
            out.push(1);
            out.extend_from_slice(k);
        }
    }
}

fn take_key(b: &[u8], p: &mut usize) -> Option<Option<[u8; 32]>> {
    let flag = *b.get(*p)?;
    *p += 1;
    match flag {
        0 => Some(None),
        1 => {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(Some(k))
        }
        _ => None,
    }
}

/// A SIGNING EVENT'S CITATION, as a receipt carries it after its tag: the
/// kernel's key, the reserve's domain or none, and the verdict
/// (`signing::put_verdict`).
pub fn put_signing(out: &mut Vec<u8>, kernel: &[u8; 32], reserve: &Option<[u8; 32]>, verdict: &Verdict) {
    out.extend_from_slice(kernel);
    put_key(out, reserve);
    put_verdict(out, verdict);
}

/// The citation read back from `p` on: the kernel, the reserve, the verdict.
pub fn take_signing(b: &[u8], p: &mut usize) -> Option<([u8; 32], Option<[u8; 32]>, Verdict)> {
    let kernel: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    let reserve = take_key(b, p)?;
    Some((kernel, reserve, take_verdict_at(b, p)?))
}

fn put_reading(out: &mut Vec<u8>, r: &ClauseReading) {
    match r {
        ClauseReading::Converses { at } => {
            out.push(0);
            at.put_canonical(out);
        }
        ClauseReading::Vetoed { at } => {
            out.push(1);
            at.put_canonical(out);
        }
        ClauseReading::Collides { at, residue } => {
            out.push(2);
            at.put_canonical(out);
            put_verdict(out, &Err((**residue).clone()));
        }
        ClauseReading::Unread => out.push(3),
    }
}

fn take_reading(b: &[u8], p: &mut usize) -> Option<ClauseReading> {
    let tag = *b.get(*p)?;
    *p += 1;
    Some(match tag {
        0 => ClauseReading::Converses { at: Stamp::take_canonical(b, p)? },
        1 => ClauseReading::Vetoed { at: Stamp::take_canonical(b, p)? },
        2 => {
            let at = Stamp::take_canonical(b, p)?;
            let Err(residue) = take_verdict_at(b, p)? else { return None };
            ClauseReading::Collides { at, residue: Box::new(residue) }
        }
        3 => ClauseReading::Unread,
        _ => return None,
    })
}

/// THE READING'S BYTES: `[0][kernel count][(kernel 32, reserve, clause
/// count, (clause name, reading) …) …][history]`, a reading's tag (0
/// converses, 1 vetoed, 2 collides, 3 unread) then its stamp and, for a
/// collision, the refusal as a verdict writes it; or `[1][0]` the stroke
/// spent, `[1][1][clause name]` a register this hypha cannot read.
pub fn put_alignment(out: &mut Vec<u8>, read: &Result<Alignment, AlignmentRefusal>) {
    match read {
        Ok(a) => {
            out.push(0);
            put_key_count(out, a.kernels.len());
            for k in &a.kernels {
                out.extend_from_slice(&k.kernel);
                put_key(out, &k.reserve);
                put_key_count(out, k.clauses.len());
                for c in &k.clauses {
                    put_clause_id(out, &c.id);
                    put_reading(out, &c.reading);
                }
            }
            put_history(out, &a.history);
        }
        Err(AlignmentRefusal::StrokeSpent) => out.extend_from_slice(&[1, 0]),
        Err(AlignmentRefusal::Unregistered { id }) => {
            out.extend_from_slice(&[1, 1]);
            put_clause_id(out, id);
        }
    }
}

/// The reading a reply carries, read whole.
pub fn take_alignment(b: &[u8]) -> Option<Result<Alignment, AlignmentRefusal>> {
    let mut p = 1;
    let (head, _) = b.split_first()?;
    let read = match head {
        0 => {
            let n = take_key_count(b, &mut p)?;
            let kernels = (0..n)
                .map(|_| {
                    let kernel: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
                    p += 32;
                    let reserve = take_key(b, &mut p)?;
                    let m = take_key_count(b, &mut p)?;
                    let clauses = (0..m)
                        .map(|_| Some(ClauseAlignment { id: take_clause_id(b, &mut p)?, reading: take_reading(b, &mut p)? }))
                        .collect::<Option<Vec<_>>>()?;
                    Some(KernelAlignment { kernel, reserve, clauses })
                })
                .collect::<Option<Vec<_>>>()?;
            Ok(Alignment { kernels, history: take_history(b, &mut p)? })
        }
        1 => {
            let tag = *b.get(p)?;
            p += 1;
            Err(match tag {
                0 => AlignmentRefusal::StrokeSpent,
                1 => AlignmentRefusal::Unregistered { id: take_clause_id(b, &mut p)? },
                _ => return None,
            })
        }
        _ => return None,
    };
    (p == b.len()).then_some(read)
}
