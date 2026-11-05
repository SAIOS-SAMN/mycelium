// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, SIGNING BY ADJUDICATED READING — a kernel's signature,
// clause by clause, over its own reconstruction of each clause's fixtures;
// the hypha admits it only where the kernel reached the same exact objects.
//
//! The kernel presents, per clause, the clause's content (its name and its
//! fixtures' inputs) and its choice: enabled, with the object it reached on
//! each fixture, or vetoed. The whole is signed under the kernel's key over
//! `"mycelium concordance v2" ‖ kernel key ‖ the clauses written out`. The
//! hypha verifies the signature, finds each clause in its register by its
//! content, and compares each presented object with its own reconstruction
//! (MU-SCIENCE §5.0). A kernel that did not reach a fixture's object is
//! refused [`SigningRefusal::NotSignatory`], naming the clause, the fixture,
//! and both objects; a veto of a network clause is refused by name. Nothing
//! is recorded yet: the signed clause set per domain, and `NotSignatory` at
//! the universal core tube, are B2's next stage and B3/B4.

use super::register::{register, ConcordanceClauseId, Fixture, Object, Registered, Standing};
use super::writing::{put_clause_id, put_content, put_object, take_clause_id, take_content, take_object};
use crate::transducer::address::{put_key_count, take_key_count};
use ed25519_dalek::Verifier;

/// The domain a kernel's signature is taken over, ahead of its key and the
/// clauses written out.
const SIGNED_CLAUSES: &[u8] = b"mycelium concordance v2";

/// A KERNEL'S CHOICE ON ONE CLAUSE: enabled, with the object it reached on
/// each of the clause's fixtures in order, or vetoed.
#[derive(Clone, Debug, PartialEq)]
pub enum Choice {
    Enabled { reached: Vec<Object> },
    Vetoed,
}

/// THE CHOICE A SIGNATURE WAS ADMITTED UNDER, named without its objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Assent {
    Enabled,
    Vetoed,
}

impl Assent {
    /// The assent's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            Assent::Enabled => 0,
            Assent::Vetoed => 1,
        }
    }

    /// The assent a byte names, or none.
    pub const fn of(b: u8) -> Option<Assent> {
        match b {
            0 => Some(Assent::Enabled),
            1 => Some(Assent::Vetoed),
            _ => None,
        }
    }
}

/// ONE CLAUSE AS A KERNEL SIGNS IT: the clause's content, and its choice.
#[derive(Clone, Debug, PartialEq)]
pub struct SignedClause {
    pub id: ConcordanceClauseId,
    pub fixtures: Vec<Fixture>,
    pub choice: Choice,
}

/// ONE CLAUSE ADMITTED: its name and the kernel's assent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Admitted {
    pub id: ConcordanceClauseId,
    pub assent: Assent,
}

/// A KERNEL'S SIGNING ACT: its key, the clauses it signs, and the Ed25519
/// signature over them.
#[derive(Clone, Debug, PartialEq)]
pub struct Signing {
    pub kernel_key: [u8; 32],
    pub clauses: Vec<SignedClause>,
    pub sig: [u8; 64],
}

/// WHY A SIGNING ACT IS REFUSED, by name and with what caused it.
#[derive(Clone, Debug, PartialEq)]
pub enum SigningRefusal {
    /// The bytes carry no signing act, or the key is no Ed25519 point.
    Malformed,
    /// The signature does not verify under the kernel's key.
    Unsigned,
    /// The register holds no clause of this content (no clause of this
    /// name, or one whose fixtures differ).
    UnknownClause { id: ConcordanceClauseId },
    /// The clause is signed twice in one act.
    Repeated { id: ConcordanceClauseId },
    /// A network clause vetoed: no kernel may veto it.
    NetworkVetoed { id: ConcordanceClauseId },
    /// The kernel did not reach a fixture's object: the clause, the
    /// fixture, the object presented and the hypha's reconstruction.
    NotSignatory { id: ConcordanceClauseId, fixture: Box<Fixture>, presented: Box<Object>, reconstructed: Box<Object> },
    /// This hypha cannot read the clause of its own register.
    Unregistered { id: ConcordanceClauseId },
}

/// The clauses written out, as signed and carried.
fn put_clauses(out: &mut Vec<u8>, clauses: &[SignedClause]) {
    put_key_count(out, clauses.len());
    for c in clauses {
        put_content(out, &c.id, &c.fixtures);
        match &c.choice {
            Choice::Enabled { reached } => {
                out.push(Assent::Enabled.byte());
                reached.iter().for_each(|o| put_object(out, o));
            }
            Choice::Vetoed => out.push(Assent::Vetoed.byte()),
        }
    }
}

/// The clauses read back: an enabled clause carries one object per fixture.
fn take_clauses(b: &[u8], p: &mut usize) -> Option<Vec<SignedClause>> {
    let n = take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let (id, fixtures) = take_content(b, p)?;
            let choice = match Assent::of(*b.get(*p)?)? {
                Assent::Enabled => {
                    *p += 1;
                    Choice::Enabled { reached: fixtures.iter().map(|_| take_object(b, p)).collect::<Option<Vec<_>>>()? }
                }
                Assent::Vetoed => {
                    *p += 1;
                    Choice::Vetoed
                }
            };
            Some(SignedClause { id, fixtures, choice })
        })
        .collect()
}

/// The bytes a signature is taken over: the domain, the key, the clauses.
fn signed_bytes(kernel_key: &[u8; 32], clauses: &[SignedClause]) -> Vec<u8> {
    let mut b = SIGNED_CLAUSES.to_vec();
    b.extend_from_slice(kernel_key);
    put_clauses(&mut b, clauses);
    b
}

impl Signing {
    /// A kernel's signing act over `clauses`, by its key.
    pub fn sign(key: &ed25519_dalek::SigningKey, clauses: Vec<SignedClause>) -> Signing {
        use ed25519_dalek::Signer;
        let kernel_key = key.verifying_key().to_bytes();
        let sig = key.sign(&signed_bytes(&kernel_key, &clauses)).to_bytes();
        Signing { kernel_key, clauses, sig }
    }

    /// The act's bytes: `[kernel key 32][clause count][clause …][sig 64]`,
    /// each clause its content address, its assent byte, and, enabled, one
    /// object per fixture.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = self.kernel_key.to_vec();
        put_clauses(&mut out, &self.clauses);
        out.extend_from_slice(&self.sig);
        out
    }

    /// An act read whole, or [`SigningRefusal::Malformed`].
    pub fn from_bytes(b: &[u8]) -> Result<Signing, SigningRefusal> {
        let read = || -> Option<Signing> {
            let (body, sig) = b.split_at_checked(b.len().checked_sub(64)?)?;
            let kernel_key: [u8; 32] = body.get(0..32)?.try_into().ok()?;
            let mut p = 32;
            let clauses = take_clauses(body, &mut p)?;
            (p == body.len()).then_some(Signing { kernel_key, clauses, sig: sig.try_into().ok()? })
        };
        read().ok_or(SigningRefusal::Malformed)
    }
}

/// ADMIT A SIGNING ACT against `register`: the key an Ed25519 point and the
/// signature verifying under it; each clause found by its content, signed
/// once; a network clause never vetoed; and each enabled clause's presented
/// objects the hypha's own reconstructions, exactly. Returns each clause
/// admitted with its assent, in the act's order; each refusal named.
pub fn admit(register: &[Registered], s: &Signing) -> Result<Vec<Admitted>, SigningRefusal> {
    let key = ed25519_dalek::VerifyingKey::from_bytes(&s.kernel_key).map_err(|_| SigningRefusal::Malformed)?;
    let sig = ed25519_dalek::Signature::from_bytes(&s.sig);
    key.verify(&signed_bytes(&s.kernel_key, &s.clauses), &sig).map_err(|_| SigningRefusal::Unsigned)?;
    let mut admitted: Vec<Admitted> = Vec::with_capacity(s.clauses.len());
    for c in &s.clauses {
        let unknown = || SigningRefusal::UnknownClause { id: c.id.clone() };
        let held = register.iter().find(|r| r.clause.id == c.id).ok_or_else(unknown)?;
        if held.clause.fixtures != c.fixtures {
            return Err(unknown());
        }
        if admitted.iter().any(|a| a.id == c.id) {
            return Err(SigningRefusal::Repeated { id: c.id.clone() });
        }
        let assent = match (&c.choice, held.clause.standing) {
            (Choice::Vetoed, Standing::Network) => return Err(SigningRefusal::NetworkVetoed { id: c.id.clone() }),
            (Choice::Vetoed, Standing::Kernel) => Assent::Vetoed,
            (Choice::Enabled { reached }, _) => {
                let unreached = c.fixtures.iter().zip(&held.reached).zip(reached).find(|((_, ours), theirs)| ours != theirs);
                if let Some(((fixture, ours), theirs)) = unreached {
                    return Err(SigningRefusal::NotSignatory {
                        id: c.id.clone(),
                        fixture: Box::new(fixture.clone()),
                        presented: Box::new(theirs.clone()),
                        reconstructed: Box::new(ours.clone()),
                    });
                }
                Assent::Enabled
            }
        };
        admitted.push(Admitted { id: c.id.clone(), assent });
    }
    Ok(admitted)
}

/// The verdict on a carried act's bytes, read and admitted against this
/// hypha's register; a register this hypha cannot read refuses
/// `Unregistered`, naming the clause.
pub fn admit_bytes(b: &[u8]) -> Result<Vec<Admitted>, SigningRefusal> {
    let held = register().map_err(|r| SigningRefusal::Unregistered { id: r.id })?;
    admit(&held, &Signing::from_bytes(b)?)
}

/// THE VERDICT'S BYTES: `[0][count][(clause name, assent) …]` admitted, or
/// `[1][refusal]`: its tag, then what it names.
pub fn put_verdict(out: &mut Vec<u8>, verdict: &Result<Vec<Admitted>, SigningRefusal>) {
    match verdict {
        Ok(admitted) => {
            out.push(0);
            put_key_count(out, admitted.len());
            for a in admitted {
                put_clause_id(out, &a.id);
                out.push(a.assent.byte());
            }
        }
        Err(r) => {
            out.push(1);
            match r {
                SigningRefusal::Malformed => out.push(0),
                SigningRefusal::Unsigned => out.push(1),
                SigningRefusal::UnknownClause { id } | SigningRefusal::Repeated { id } | SigningRefusal::NetworkVetoed { id } | SigningRefusal::Unregistered { id } => {
                    out.push(match r {
                        SigningRefusal::UnknownClause { .. } => 2,
                        SigningRefusal::Repeated { .. } => 3,
                        SigningRefusal::NetworkVetoed { .. } => 4,
                        _ => 6,
                    });
                    put_clause_id(out, id);
                }
                SigningRefusal::NotSignatory { id, fixture, presented, reconstructed } => {
                    out.push(5);
                    put_content(out, id, std::slice::from_ref(fixture));
                    put_object(out, presented);
                    put_object(out, reconstructed);
                }
            }
        }
    }
}

/// The verdict a reply carries, read whole.
pub fn take_verdict(b: &[u8]) -> Option<Result<Vec<Admitted>, SigningRefusal>> {
    let mut p = 1;
    let (head, _) = b.split_first()?;
    let verdict = match head {
        0 => {
            let n = take_key_count(b, &mut p)?;
            let admitted = (0..n)
                .map(|_| {
                    let id = take_clause_id(b, &mut p)?;
                    let assent = Assent::of(*b.get(p)?)?;
                    p += 1;
                    Some(Admitted { id, assent })
                })
                .collect::<Option<Vec<_>>>()?;
            Ok(admitted)
        }
        1 => {
            let tag = *b.get(p)?;
            p += 1;
            Err(match tag {
                0 => SigningRefusal::Malformed,
                1 => SigningRefusal::Unsigned,
                2 => SigningRefusal::UnknownClause { id: take_clause_id(b, &mut p)? },
                3 => SigningRefusal::Repeated { id: take_clause_id(b, &mut p)? },
                4 => SigningRefusal::NetworkVetoed { id: take_clause_id(b, &mut p)? },
                5 => {
                    let (id, mut fixtures) = take_content(b, &mut p)?;
                    let fixture = fixtures.pop().filter(|_| fixtures.is_empty())?;
                    let presented = take_object(b, &mut p)?;
                    let reconstructed = take_object(b, &mut p)?;
                    SigningRefusal::NotSignatory { id, fixture: Box::new(fixture), presented: Box::new(presented), reconstructed: Box::new(reconstructed) }
                }
                6 => SigningRefusal::Unregistered { id: take_clause_id(b, &mut p)? },
                _ => return None,
            })
        }
        _ => return None,
    };
    (p == b.len()).then_some(verdict)
}
