// ACT_TASK and ACT_SOLVE — a task envelope out of the registry, and the
// solve that answers it.

use super::*;

/// task request: `[ACT_TASK][voice 32]`.
pub fn encode_task_request(voice: &[u8; 32]) -> Vec<u8> {
    let mut out = vec![ACT_TASK];
    out.extend_from_slice(voice);
    out
}

pub fn decode_task_request(body: &[u8]) -> Option<[u8; 32]> {
    if !act_is(body, ACT_TASK) || body.len() != 33 {
        return None;
    }
    body.get(1..33)?.try_into().ok()
}

/// 78.146, WIRE v1.2 — TASK BY ITS CITATION: `[ACT_TASK][voice 32][citation
/// 32]` — answered with that task's envelope, or an empty body where the
/// registry holds no such task. A task is named on the wire by its 32-byte
/// citation (`registry::citation`), never its content address.
pub fn encode_task_by_id(voice: &[u8; 32], citation: &[u8; 32]) -> Vec<u8> {
    let mut out = encode_task_request(voice);
    out.extend_from_slice(citation);
    out
}

/// A task-by-citation request read whole, or none (a plain request is 33
/// bytes and reads as [`decode_task_request`]).
pub fn decode_task_by_id(body: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if !act_is(body, ACT_TASK) || body.len() != 65 {
        return None;
    }
    Some((body.get(1..33)?.try_into().ok()?, body.get(33..65)?.try_into().ok()?))
}

/// work envelope (wire v1.2): `[citation 32][nt u8][(input grid, output
/// grid)·nt][nx u8][test-input grid·nx]` — train pairs to STUDY, test inputs
/// to SOLVE; the held answers never ride the wire. An empty body = no work.
pub fn encode_task_envelope(
    citation: &[u8; 32],
    train: &crate::transducer::family::Family,
    tests: &[Vec<Vec<u8>>],
) -> Option<Vec<u8>> {
    let mut out = citation.to_vec();
    crate::transducer::address::put_u8_count(&mut out, train.len())?;
    for (a, b) in train {
        push_grid(&mut out, a)?;
        push_grid(&mut out, b)?;
    }
    crate::transducer::address::put_u8_count(&mut out, tests.len())?;
    for x in tests {
        push_grid(&mut out, x)?;
    }
    Some(out)
}

/// A REGISTERED TASK as the registry hands it over: the family to learn
/// from and the inputs to answer, never the answers.
pub struct TaskEnvelope {
    /// The task's citation (wire v1.2) — what a solve names it by.
    pub id: [u8; 32],
    /// The TRAIN pairs: the family the rule is extracted from.
    pub train: Vec<crate::transducer::family::TrainPair>,
    /// The TEST inputs. Their outputs are what a solve claims; the registry
    /// never sends them.
    pub tests: Vec<Vec<Vec<u8>>>,
}

pub fn decode_task_envelope(body: &[u8]) -> Option<TaskEnvelope> {
    let id: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32usize;
    let nt = *body.get(p)? as usize;
    p += 1;
    let mut train = Vec::with_capacity(nt.min(64));
    for _ in 0..nt {
        let a = read_grid(body, &mut p)?;
        let b = read_grid(body, &mut p)?;
        train.push((a, b));
    }
    let nx = *body.get(p)? as usize;
    p += 1;
    let mut tests = Vec::with_capacity(nx.min(64));
    for _ in 0..nx {
        tests.push(read_grid(body, &mut p)?);
    }
    Some(TaskEnvelope { id, train, tests })
}

/// 78.146 — GRIDS AS THE SOLVE ACT WRITES THEM: `[n u8][grid·n]`. `None`
/// where a count or a grid passes its field.
pub fn encode_grids(grids: &[Vec<Vec<u8>>]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    crate::transducer::address::put_u8_count(&mut out, grids.len())?;
    for g in grids {
        push_grid(&mut out, g)?;
    }
    Some(out)
}

/// The grids read back whole, or none.
pub fn decode_grids(b: &[u8]) -> Option<Vec<Vec<Vec<u8>>>> {
    let [n, ..] = b else { return None };
    let n = usize::from(*n);
    let mut p = 1usize;
    let mut out = Vec::new();
    for _ in 0..n {
        out.push(read_grid(b, &mut p)?);
    }
    (p == b.len()).then_some(out)
}

/// 78.141 — the signed bytes' domain of a solve.
const SIGNED_SOLVE: &[u8] = b"mycelium solve v1";

/// The bytes a solve's voice signs: the domain, then the act up to its
/// signature.
fn solve_signed_bytes(unsigned: &[u8]) -> Vec<u8> {
    let mut b = SIGNED_SOLVE.to_vec();
    b.extend_from_slice(unsigned);
    b
}

/// solve submission (wire v1.2): `[ACT_SOLVE][voice 32][task citation
/// 32][n u8][grid·n]`,
/// optionally `[RULE][len u32][element]` so the home speaks the rule, then —
/// 78.141 — `[SIGN][sig 64]`: the voice, which the court pays, signs the act,
/// and the voice is its key's verifying half.
pub fn encode_solve(
    key: &ed25519_dalek::SigningKey,
    citation: &[u8; 32],
    outputs: &[Vec<Vec<u8>>],
) -> Option<Vec<u8>> {
    encode_solve_with_rule(key, citation, outputs, None)
}

pub fn encode_solve_with_rule(
    key: &ed25519_dalek::SigningKey,
    citation: &[u8; 32],
    outputs: &[Vec<Vec<u8>>],
    rule: Option<&[u8]>,
) -> Option<Vec<u8>> {
    use ed25519_dalek::Signer;
    let mut out = vec![ACT_SOLVE];
    out.extend_from_slice(&key.verifying_key().to_bytes());
    out.extend_from_slice(citation);
    crate::transducer::address::put_u8_count(&mut out, outputs.len())?;
    for g in outputs {
        push_grid(&mut out, g)?;
    }
    if let Some(r) = rule {
        out.extend_from_slice(b"RULE");
        crate::transducer::address::put_bytes(&mut out, r)?;
    }
    let sig = key.sign(&solve_signed_bytes(&out)).to_bytes();
    out.extend_from_slice(b"SIGN");
    out.extend_from_slice(&sig);
    Some(out)
}

/// A solve read back: `None` where it does not decode, or where bytes stand
/// after it that are no signature; `signed` whether its voice signed it.
pub fn decode_solve(body: &[u8]) -> Option<SolveAct> {
    if !act_is(body, ACT_SOLVE) {
        return None;
    }
    let voice: [u8; 32] = body.get(1..33)?.try_into().ok()?;
    let id: [u8; 32] = body.get(33..65)?.try_into().ok()?;
    let mut p = 65usize;
    let n = *body.get(p)? as usize;
    p += 1;
    let mut outputs = Vec::with_capacity(n.min(64));
    for _ in 0..n {
        outputs.push(read_grid(body, &mut p)?);
    }
    let rule = if body.get(p..p + 4) == Some(&b"RULE"[..]) {
        p += 4;
        let len = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        Some(body.get(p..p + len)?.to_vec())
    } else {
        None
    };
    p += rule.as_ref().map_or(0, Vec::len);
    let signed = match body.get(p..) {
        Some([]) => false,
        Some([b'S', b'I', b'G', b'N', sig @ ..]) => {
            let sig: [u8; 64] = sig.try_into().ok()?;
            crate::court::identity::verify(&voice, &solve_signed_bytes(body.get(..p)?), &sig)
        }
        _ => return None,
    };
    Some(SolveAct { voice, task: id, outputs, rule, signed })
}


pub fn encode_solve_receipt(code: u8, mu: &[u64; 4]) -> Vec<u8> {
    let mut out = vec![code];
    push_mu(&mut out, mu);
    out
}

pub fn decode_solve_receipt(body: &[u8]) -> Option<(u8, [u64; 4])> {
    let code = act_tag(body)?;
    let mut p = 1usize;
    let mu = read_mu(body, &mut p)?;
    Some((code, mu))
}

/// WHY A SOLVE WAS REFUSED, as the court read it (the actions' physics): the
/// replay's reason and its obstruction kind (78.9), carried behind the
/// receipt so the voice learns where its rule broke. A receipt reader that
/// knows only `[code][μ]` reads the prefix and is unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolveRefusal {
    pub reason: Vec<u8>,
    pub kind: u8,
}

/// `[code][μ 4×u64]` then, for a refusal, `[kind u8][reason, length-prefixed]`.
/// The tag a paid verdict's lane reading rides behind the receipt under: no
/// obstruction kind (78.9's are below seven) is it, so a refusal and a
/// reading are told apart by their first byte.
pub const SOLVE_READS_TAG: u8 = 0xFF;

/// The receipt and, behind it, the refusal where one was refused, or — paid —
/// the pay read lane by lane (`LaneRead`: fell, held, rose, unread). A
/// receipt reader that knows only `[code][μ]` reads the prefix unchanged.
pub fn encode_solve_verdict(
    code: u8,
    mu: &[u64; 4],
    refusal: Option<&SolveRefusal>,
    reads: Option<&[crate::economy::LaneRead; 4]>,
) -> Vec<u8> {
    let mut out = encode_solve_receipt(code, mu);
    if let Some(r) = refusal {
        out.push(r.kind);
        crate::transducer::address::put_key_bytes(&mut out, &r.reason);
    } else if let Some(reads) = reads {
        out.push(SOLVE_READS_TAG);
        for r in reads {
            r.put(&mut out);
        }
    }
    out
}

/// A VERDICT READ: the receipt, the refusal where one rode behind it, and
/// the lane reading where the pay carried one.
pub type SolveVerdictRead = (u8, [u64; 4], Option<SolveRefusal>, Option<[crate::economy::LaneRead; 4]>);

/// The receipt and, where one rode behind it, the refusal or the reading.
pub fn decode_solve_verdict(body: &[u8]) -> Option<SolveVerdictRead> {
    let (code, mu) = decode_solve_receipt(body)?;
    let mut p = 1 + 32;
    if p == body.len() {
        return Some((code, mu, None, None));
    }
    let kind = *body.get(p)?;
    p += 1;
    if kind == SOLVE_READS_TAG {
        let mut reads = [crate::economy::LaneRead::Unread; 4];
        for r in reads.iter_mut() {
            *r = crate::economy::LaneRead::take(body, &mut p)?;
        }
        return (p == body.len()).then_some((code, mu, None, Some(reads)));
    }
    let reason = crate::transducer::address::take_key_bytes(body, &mut p)?;
    (p == body.len()).then_some((code, mu, Some(SolveRefusal { reason, kind }), None))
}
