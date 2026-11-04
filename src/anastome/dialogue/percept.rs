// ACT_TASK and ACT_SOLVE — a task envelope out of the registry, and the
// solve that answers it.

use super::*;
use super::samples::Member;

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
pub fn encode_task_by_citation(voice: &[u8; 32], citation: &[u8; 32]) -> Vec<u8> {
    let mut out = encode_task_request(voice);
    out.extend_from_slice(citation);
    out
}

/// A task-by-citation request read whole, or none (a plain request is 33
/// bytes and reads as [`decode_task_request`]).
pub fn decode_task_by_citation(body: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    if !act_is(body, ACT_TASK) || body.len() != 65 {
        return None;
    }
    Some((body.get(1..33)?.try_into().ok()?, body.get(33..65)?.try_into().ok()?))
}

/// work envelope (wire v1.2; track 26 S5): `[citation 32][nt u8][(input,
/// output)·nt][nx u8][test input·nx]`, each a sample of any rank as
/// `samples::push_sample` writes it, at its position `[split, pair,
/// member]` — train pairs to STUDY, test inputs to SOLVE; the held answers
/// never ride the wire. An empty body = no work. Refused by name where a
/// sample or a count passes its field.
pub fn encode_task_envelope(
    citation: &[u8; 32],
    train: &[(crate::transducer::sample::Sample, crate::transducer::sample::Sample)],
    tests: &[crate::transducer::sample::Sample],
) -> Result<Vec<u8>, SampleWire> {
    let mut out = citation.to_vec();
    super::samples::push_train(&mut out, train)?;
    super::samples::push_tests(&mut out, tests, Member::Input)?;
    Ok(out)
}

/// The envelope read back whole and exactly, or none.
pub fn decode_task_envelope(body: &[u8]) -> Option<TaskEnvelope> {
    let id: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32usize;
    let train = super::samples::read_train(body, &mut p).ok()?;
    let tests = super::samples::read_tests(body, &mut p, Member::Input).ok()?;
    // NOTHING AFTER THE LAST TEST INPUT: an envelope is read whole or
    // refused. Read leniently, the 43-byte deferral of a zero deficit was an
    // envelope of no train pair and no test input.
    (p == body.len()).then_some(TaskEnvelope { id, train, tests })
}

/// THE POSTED TASK FILE'S EXTENSION: what a hypha's corpus directory, a
/// client's feed and the soak list (ROADMAP track 26, S1).
pub const POSTED_TASK: &str = "task";

/// A TASK AS A SOURCE POSTS IT (MU-SENSING §6; ROADMAP track 26, S1, S5):
/// the source-neutral form a decoder writes and the hypha, the soak and a
/// client's feed read, so none of them parses a dataset's format. Its
/// samples, of any rank, as `samples::encode_posted_samples` writes them;
/// this writer takes a raster task and places each raster at its position.
/// `None` where a raster is no sample or a count passes its field.
pub fn encode_posted_task(task: &crate::transducer::family::Task) -> Option<Vec<u8>> {
    encode_posted_samples(&PostedTask::of_rasters(task).ok()?).ok()
}

/// A posted task read back on the alphabet's section (`PostedTask::section`):
/// none where it does not decode whole (`samples::decode_posted_samples`)
/// or where a sample is off the section, which the readers of the section
/// (the hypha's registry, the soak, a feed) do not read; such a task is
/// read as samples by `decode_posted_samples`.
pub fn decode_posted_task(b: &[u8]) -> Option<crate::transducer::family::Task> {
    decode_posted_samples(b).ok()?.section().ok()
}

/// 78.141 — the signed bytes' domain of a solve. Version 3 (track 26 S5,
/// inside wire 0x0200's ruled break): the outputs are samples of any rank.
/// Version 2 (S2) added the ground the claim was spoken under.
const SIGNED_SOLVE: &[u8] = b"mycelium solve v3";

/// The bytes a solve's voice signs: the domain, then the act up to its
/// signature.
fn solve_signed_bytes(unsigned: &[u8]) -> Vec<u8> {
    let mut b = SIGNED_SOLVE.to_vec();
    b.extend_from_slice(unsigned);
    b
}

/// solve submission (wire v1.2; track 26 S5): `[ACT_SOLVE][voice
/// 32][task citation 32][n u8][sample·n]`, output `i` a sample of any rank
/// at `[test, i, output]` (`samples::encode_samples`'s writing),
/// optionally `[RULE][len u32][element]` so the home speaks the rule, then
/// optionally — track 26 S2 — `[GRND][len u32][symbol]`, the ground the
/// claim was spoken under, then — 78.141 — `[SIGN][sig 64]`: the voice,
/// which the court pays, signs the act, and the voice is its key's
/// verifying half.
pub fn encode_solve(
    key: &ed25519_dalek::SigningKey,
    citation: &[u8; 32],
    outputs: &[crate::transducer::sample::Sample],
) -> Option<Vec<u8>> {
    encode_solve_with_rule(key, citation, outputs, None)
}

/// [`encode_solve_spoken`] stating no ground.
pub fn encode_solve_with_rule(
    key: &ed25519_dalek::SigningKey,
    citation: &[u8; 32],
    outputs: &[crate::transducer::sample::Sample],
    rule: Option<&[u8]>,
) -> Option<Vec<u8>> {
    encode_solve_spoken(key, citation, outputs, rule, None)
}

/// A SOLVE THAT STATES THE GROUND ITS CLAIM WAS SPOKEN UNDER (MU-SENSING
/// §6): `ground` is one symbol at the source's width, or `None`. `None`
/// where an output or a count passes its field.
pub fn encode_solve_spoken(
    key: &ed25519_dalek::SigningKey,
    citation: &[u8; 32],
    outputs: &[crate::transducer::sample::Sample],
    rule: Option<&[u8]>,
    ground: Option<&[u8]>,
) -> Option<Vec<u8>> {
    use ed25519_dalek::Signer;
    let mut out = vec![ACT_SOLVE];
    out.extend_from_slice(&key.verifying_key().to_bytes());
    out.extend_from_slice(citation);
    super::samples::push_tests(&mut out, outputs, Member::Output).ok()?;
    if let Some(r) = rule {
        out.extend_from_slice(b"RULE");
        crate::transducer::address::put_bytes(&mut out, r)?;
    }
    if let Some(g) = ground {
        out.extend_from_slice(b"GRND");
        crate::transducer::address::put_bytes(&mut out, g)?;
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
    let outputs = super::samples::read_tests(body, &mut p, Member::Output).ok()?;
    let rule = if body.get(p..p + 4) == Some(&b"RULE"[..]) {
        p += 4;
        let len = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
        Some(body.get(p..p + len)?.to_vec())
    } else {
        None
    };
    p += rule.as_ref().map_or(0, Vec::len);
    let ground = match body.get(p..p + 4) == Some(&b"GRND"[..]) {
        true => {
            p += 4;
            let g = crate::transducer::address::take_bytes(body, &mut p)?;
            // A stated ground is one symbol: never empty.
            if g.is_empty() {
                return None;
            }
            Some(g)
        }
        false => None,
    };
    let signed = match body.get(p..) {
        Some([]) => false,
        Some([b'S', b'I', b'G', b'N', sig @ ..]) => {
            let sig: [u8; 64] = sig.try_into().ok()?;
            crate::court::identity::verify(&voice, &solve_signed_bytes(body.get(..p)?), &sig)
        }
        _ => return None,
    };
    Some(SolveAct { voice, task: id, outputs, rule, ground, signed })
}


/// `[code u8][μ⃗]`: the verdict and what it paid, four exact rational lanes
/// (`Mu::put`).
pub fn encode_solve_receipt(code: u8, mu: &crate::economy::Mu) -> Vec<u8> {
    let mut out = vec![code];
    mu.put(&mut out);
    out
}

/// The receipt's code and μ, and where its bytes end.
fn take_solve_receipt(body: &[u8]) -> Option<(u8, crate::economy::Mu, usize)> {
    let code = act_tag(body)?;
    let mut p = 1usize;
    let mu = crate::economy::Mu::take_from(body, &mut p)?;
    Some((code, mu, p))
}

pub fn decode_solve_receipt(body: &[u8]) -> Option<(u8, crate::economy::Mu)> {
    take_solve_receipt(body).map(|(code, mu, _)| (code, mu))
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

/// `[code][μ⃗]` then, for a refusal, `[kind u8][reason, length-prefixed]`.
/// The tag a paid verdict's modes ride behind the receipt under: no
/// obstruction kind (78.9's are below seven) is it, so a refusal and the
/// modes are told apart by the first byte.
/// FORMAT: tag marking a solve receipt's modes.
pub const SOLVE_MODES_TAG: u8 = 0xFE;

/// The receipt and, behind it, the refusal where one was refused, or — paid —
/// the modes the closing form moved (78.162, the `Modes` codec: a u32 count,
/// then each mode and its exact amount). A receipt reader that knows only
/// `[code][μ]` reads the prefix unchanged. `None` where the modes do not
/// write.
pub fn encode_solve_verdict(
    code: u8,
    mu: &crate::economy::Mu,
    refusal: Option<&SolveRefusal>,
    modes: Option<&crate::economy::modes::Modes>,
) -> Option<Vec<u8>> {
    let mut out = encode_solve_receipt(code, mu);
    if let Some(r) = refusal {
        out.push(r.kind);
        crate::transducer::address::put_key_bytes(&mut out, &r.reason);
    } else if let Some(m) = modes {
        out.push(SOLVE_MODES_TAG);
        m.put(&mut out)?;
    }
    Some(out)
}

/// A VERDICT READ: the receipt, the refusal where one rode behind it, and
/// the modes where the pay carried them.
pub type SolveVerdictRead = (u8, crate::economy::Mu, Option<SolveRefusal>, Option<crate::economy::modes::Modes>);

/// The receipt and, where one rode behind it, the refusal or the modes.
pub fn decode_solve_verdict(body: &[u8]) -> Option<SolveVerdictRead> {
    let (code, mu, mut p) = take_solve_receipt(body)?;
    if p == body.len() {
        return Some((code, mu, None, None));
    }
    let kind = *body.get(p)?;
    p += 1;
    if kind == SOLVE_MODES_TAG {
        let modes = crate::economy::modes::Modes::take_from(body, &mut p)?;
        return (p == body.len()).then_some((code, mu, None, Some(modes)));
    }
    let reason = crate::transducer::address::take_key_bytes(body, &mut p)?;
    (p == body.len()).then_some((code, mu, Some(SolveRefusal { reason, kind }), None))
}
